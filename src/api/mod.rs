use hyper::http;
use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::{Extension, Json, Path, Query},
    http::{Request, Response, StatusCode}, 
    middleware::{self, Next},
    response::IntoResponse,
    response::sse::{Event, Sse},
    routing::{get, post, get_service},
    Router,
};
use futures_util::{Stream, StreamExt};
use http::header;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tower_http::{
    cors::{CorsLayer},
    trace::TraceLayer,
    services::{ServeDir}
};

use flume::*;
use rhai::Engine;
use tokio::runtime::Runtime;

use crate::agentmgr;
use crate::agent::Agent;
use crate::agentmgr::{AgentManager, agent_mgr, init};
use crate::jwt_util::{create_token, verify_token, Claims};
use crate::s;


async fn hello_world() -> &'static str {
    "Hello, world!"
}

/*
async fn chat_send_handler(
    Path(user_id): Path<usize>,
    Header(authorization): Header<String>,
    ContentLengthLimit(Json(msg)): ContentLengthLimit<Json<String>, { 500 }>,
) -> Result<AxumJson<&'static str>, StatusCode> {
    let token = authorization
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::BAD_REQUEST, "Invalid token format"))?;
    let claims = verify_token(token).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

    // Call the existing handle_msg function to process the message
    match handle_msg(user_id, claims.username, msg, users, manager).await {
        Ok(_) => AxumJson("ok"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to handle message"
        )
    }
}

async fn handle_msg(
    my_id: usize,
    username: String,
    msg: String,
    users: Users,
    manager: AgentManager,
) -> Result<(), (StatusCode, &str)> {
    let (sender, reply_receiver) = manager
        .get_or_create_agent(username, my_id, s!("scripts/dm.rhai"))
        .await;

    sender
        .send_async(msg)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send message"))?;

    if let Some(reply) = reply_receiver.recv_async().await {
        let users_lock = users
            .lock()
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock users"))?;
        if let Some(tx) = users_lock.get(&my_id) {
            tx.send(ChatUIMessage::Reply(reply))
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send reply"))?;
        }
    }

    Ok(())
}
*/

async fn login_handler(
    Json(credentials): Json<Credentials>,
) -> Result<Json<LoginResponse>, (StatusCode, &'static str)>  {
    if true || credentials.username.starts_with("anon")
        || (credentials.username == "user" && credentials.password == "password")
    {
        let token = create_token(&credentials.username)
             .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"))?;
        Ok(Json(LoginResponse { token }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid credentials"))
    }
}

/*

async fn auth_route_handler(
    Header(authorization): Header<String>,
) -> Result<Json<Claims>, (StatusCode, &'static str)> {
    let token = authorization
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::BAD_REQUEST, "Invalid token format"))?;
    let claims = verify_token(token).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;
    Ok(Json(claims))
}
*/

pub async fn server() -> Result<(), hyper::Error> {
    pretty_env_logger::init();

    agentmgr::init();

    /*
    let app = app.route("/chat/:user_id", post(chat_send_handler));

    */

    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/login", post(login_handler))
        .route("/chat", get(user_connected))
        .fallback(get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }));

    /*
        .route(chat_recv)
        .route(chat_send_handler)
        .merge(static_files);
    */
   // https://docs.rs/tower-http/latest/tower_http/services/fs/struct.ServeFile.html 
    println!("Listening at http://45.79.139.237:3132/");

    axum::Server::bind(&"0.0.0.0:3132".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
}

/*
async fn chat_input(
    userid: String,
    session_id: usize,
    msg: String,
) -> Result<impl warp::Reply, Infallible> {
    let (sender, reply_receiver) = manager
        .get_or_create_agent(userid, session_id, s!("scripts/dm.rhai"))
        .await?;

    println!("Received: {}", msg);

    sender.send_async(msg).await.unwrap();

    loop {
        let reply = reply_receiver.recv_async().await.unwrap();
        println!(reply)
        {
            let users_lock = users.lock().unwrap();
            let tx = users_lock.get(&my_id).unwrap().clone();
            tx.send(reply);
        }
    }

    Ok("ok")
} */

#[derive(Debug)]
pub enum ChatUIMessage {
    UserId(usize),
    Reply(String),
    Fragment(String),
    FunctionCall {
        name: String,
        params: String,
        result: String,
    },
}

async fn user_connected(Query(params): Query<HashMap<String, String>>)  
  -> impl Stream<Item = Result<axum::response::sse::Event, Infallible>> + Send + 'static {
    let userid = s!("failuser");

    if let Some(token) = params.get("token") {                                                                                  
        println!("Token: {}", token);
        let claims = verify_token(token)?;
        userid = claims.username;
    }
    let session_id = 1;
    if let Some(session) = params.get("session_id") {
       session_id = session.parse::<usize>()?; 
    }
    eprintln!("chat user connected: {} {}", userid, session_id);

    let (tx, rx) = agent_mgr.get().expect("No Agent Manager!")
        .get_or_create_agent(userid, session_id, s!("scripts/dm.rhai"))
        .await?;

    //tx.send(ChatUIMessage::UserId(session_id))
    //    .unwrap();

    let events = rx.map(|msg| match msg {
        ChatUIMessage::UserId(session_id) => Ok(Event::default().event("user").data(session_id.to_string())),
        ChatUIMessage::Fragment(fragment) => Ok(Event::default().event("fragment").data(fragment)),
        ChatUIMessage::Reply(reply) => Ok(Event::default().data(reply)),
        ChatUIMessage::FunctionCall {
            name,
            params,
            result,
        } => {
            println!("Sending fn call as json");
            let data = serde_json::json!({
                "name": name,
                "params": params,
                "result": result
            });
            Ok(Event::default()
                .event("functionCall")
                .data(data.to_string()))
        }
    });
   tx
    .send_async("Chat session initiated.")
    .await?;
 
    Sse::new(events)
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

