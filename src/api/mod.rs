use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::{Extension, Json, Path, Query},
    http::{Request, Response, StatusCode},
    middleware::{self, Next},
    response::sse::{Event, Sse},
    response::IntoResponse,
    routing::{get, get_service, post},
    Router,
};
use http::header;
use hyper::http;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use serde_urlencoded::*;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use flume::*;
use rhai::Engine;
use tokio::runtime::Runtime;

use crate::agent::Agent;
use crate::agentmgr;
use crate::agentmgr::{agent_mgr, init, AgentManager};
use crate::jwt_util::{create_token, verify_token, Claims};
use crate::s;
use flume::Receiver;
use futures::{
    stream::Stream,
    task::{Context, Poll},
    StreamExt,
};
use serde::Deserialize;
use std::convert::Infallible;
use std::pin::Pin;
use std::task::Waker;
use tokio_stream::wrappers::ReceiverStream;

use maplit::hashmap;

use axum::response::Html;
use rand::Rng;
use tokio::fs::read_to_string;
use tokio::time::{self, Duration};

pub mod chatuimessage;
use chatuimessage::*;

async fn user_input(
    params: Query<HashMap<String, String>>,
    Extension(claims): Extension<Claims>,
    Extension(connected_users): Extension<ConnectedUsers>,
) -> Result<Json<HashMap<&'static str, bool>>, (StatusCode, &'static str)> {
    let mut session_id = 1;
    if let Some(session) = params.get("session_id") {
        session_id = session.parse::<usize>().expect("Invalid session id");
    }
    let mut msg = s!("Error: no msg");

    if let Some(msg_) = params.get("msg") {
        msg = msg_.clone();
        println!("************ READ MSG: {} ****************", msg);
    }

    let (sender, reply_receiver, cancellation_token) = agent_mgr
        .get()
        .expect("Could not access Agent Manager.")
        .get_or_create_agent(claims.username.clone(), session_id, s!("scripts/dm.rhai"))
        .await;

    sender.send_async(msg).await.unwrap();

    loop {
        println!("Top of loop");
        let reply = reply_receiver.recv_async().await.unwrap();
        let mut locked_users = connected_users.user_cache.lock().unwrap();
        let sse_streams = locked_users.get(&claims.username.clone());
        let tx = sse_streams
            .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "User stream not found"))?
            .cache
            .get(&session_id)
            .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "Session not found"))?;
        //println!("Received reply:");
        tx.send(reply.clone());
        if let ChatUIMessage::Reply {
            role,
            name,
            content,
        } = reply
        {
            if name == "__READY__" {
                println!("Agent ready for more input.");
                break;
            }
            println!("ok");
        }
    }

    Ok(Json(hashmap! {"ok" => true}))
}

async fn login_handler(
    Json(credentials): Json<Credentials>,
) -> Result<Json<LoginResponse>, (StatusCode, &'static str)> {
    if true
        || credentials.username.starts_with("anon")
        || (credentials.username == "user" && credentials.password == "password")
    {
        let token = create_token(&credentials.username)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"))?;
        Ok(Json(LoginResponse { token }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid credentials"))
    }
}

#[derive(Debug, Clone)]
pub struct SessionSseStreams {
    cache: HashMap<usize, UnboundedSender<ChatUIMessage>>,
}

#[derive(Debug, Clone)]
pub struct ConnectedUsers {
    user_cache: Arc<Mutex<HashMap<String, SessionSseStreams>>>,
}

async fn chat_events(
    params: Query<HashMap<String, String>>,
    Extension(claims): Extension<Claims>,
    Extension(connected_users): Extension<ConnectedUsers>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("username: {}", claims.username);
    let mut session_id = 10;
    if let Some(session) = params.get("session_id") {
        session_id = session.parse::<usize>().expect("Invalid session id");
    }
    println!("{}", session_id);

    let stream = user_connected(&claims, &connected_users, session_id);
    println!("Returning Ssse stream!");
    Sse::new(stream)
}

pub async fn server() -> Result<(), hyper::Error> {
    pretty_env_logger::init();

    agentmgr::init();

    let connected_users = ConnectedUsers {
        user_cache: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/chat", get(chat_events))
        .route("/send", get(user_input))
        .route("/stop", get(stop_handler))
        .route("/sessions", get(list_sessions_handler))
        .route("/", get(index_handler))
        .layer(Extension(connected_users))
        .layer(middleware::from_fn(logging_middleware))
        .layer(middleware::from_fn(auth_middleware))
        .fallback(get_service(ServeDir::new("static")).handle_error(
            |error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            },
        ));

    println!("Listening at https://hostdev.padhub.xyz/");

    axum::Server::bind(&"127.0.0.1:3132".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
}

async fn index_handler() -> Result<Html<String>, StatusCode> {
    let index_html = read_to_string("static/index.html")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(index_html))
}

async fn list_sessions_handler(
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<String>>, (StatusCode, &'static str)> {
    let sessions = agent_mgr
        .get()
        .expect("Could not access Agent Manager.")
        .list_sessions(&claims.username);
    Ok(Json(sessions))
}

fn user_connected(
    claims: &Claims,
    users: &ConnectedUsers,
    session_id: usize,
) -> impl Stream<Item = Result<Event, Infallible>> + Send + 'static {
    println!("user_connected");
    let mut locked_users = users.user_cache.lock().unwrap();
    let sse_streams = locked_users
        .entry(claims.username.clone())
        .or_insert_with(|| SessionSseStreams {
            cache: HashMap::new(),
        });
    println!("Got locked users");
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tx.send(ChatUIMessage::UserId(claims.username.clone()))
        .unwrap();
    println!("sent userid msg");

    let mapped = rx
        .map(|msg| match msg {
            ChatUIMessage::UserId(my_id) => {
                Ok(Event::default().event("user").data(my_id.to_string()))
            }
            ChatUIMessage::Fragment(fragment) => {
                //print!("[{}]", fragment);
                Ok(Event::default().event("fragment").data(fragment))
            }
            ChatUIMessage::Reply {
                name,
                role,
                content,
            } => {
                let data = serde_json::json!({
                    "name": name,
                    "role": role,
                    "content": content
                });
                Ok(Event::default().event("msg").data(s!(data)))
            }
            ChatUIMessage::FunctionCall {
                name,
                params,
                result,
            } => {
                let data = serde_json::json!({
                    "name": name,
                    "params": params,
                    "result": result
                });
                println!("OK 2");
                Ok(Event::default()
                    .event("functionCall")
                    .data(data.to_string()))
            }
        })
        .chain(futures::stream::once(async move {
            println!("Detected client disconnect from event source!!!!!!!!!!!");
            Ok(Event::default().event("disconnect").data("disconnected"))
        }));

    sse_streams.cache.insert(session_id, tx);

    println!("returning from user_connected");
    mapped
}

#[derive(serde::Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(serde::Serialize)]
struct LoginResponse {
    token: String,
}

async fn auth_middleware(mut req: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    //) -> Result<Response<Body>, (StatusCode, &'static str) > {
    println!("Request URI: {}", req.uri());
    println!("Headers: {:?}", req.headers());
    let needs_auth = vec!["/chat", "/send", "/stop", "/sessions"];
    //let claims = Claims { username: s!("bob"), exp: 1800446206315 };
    //req.extensions_mut().insert(claims);
    if needs_auth
        .iter()
        .any(|path| req.uri().path().starts_with(path))
    {
        println!("Needs auth");
        if let Some(query_string) = req.uri().query() {
            println!("Found query");
            let query_params: HashMap<String, String> =
                serde_urlencoded::from_str(query_string).unwrap_or_default();

            if let Some(token) = query_params.get("token") {
                println!("Token: {}", token);
                let claims = verify_token(token).expect("Invalid token");
                println!("Inserting claims!");
                req.extensions_mut().insert(claims);
                println!("Claims extension inserted.");
            } else {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Auth failed"));
            }
        } else {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Auth failed"));
        }
    }
    let response = next.run(req).await;
    Ok(response)
    /* let (parts, body) = response.into_parts();
    let body = Body::from(hyper::body::to_bytes(body).await.unwrap());
    Ok(Response::from_parts(parts, body)) */
}


async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    println!("Request URI: {}", req.uri());
    println!("Headers: {:?}", req.headers());
    next.run(req).await

    /*
    let (parts, body) = response.into_parts();
    let body = Body::from(hyper::body::to_bytes(body).await.unwrap());
    Ok(Response::from_parts(parts, body)) */
}
async fn stop_handler(
    params: Query<HashMap<String, String>>,
    Extension(claims): Extension<Claims>,
    Extension(connected_users): Extension<ConnectedUsers>,
) -> Result<Json<HashMap<&'static str, bool>>, (StatusCode, &'static str)> {
    let mut session_id = 1;
    if let Some(session) = params.get("session_id") {
        session_id = session.parse::<usize>().expect("Invalid session id");
    }

    let (_, _, cancellation_token) = agent_mgr
        .get()
        .expect("Could not access Agent Manager.")
        .get_or_create_agent(claims.username.clone(), session_id, s!("scripts/dm.rhai"))
        .await;

    cancellation_token.cancel();

    Ok(Json(hashmap! {"ok" => true}))
}
