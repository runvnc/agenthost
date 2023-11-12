use futures_util::{Stream, StreamExt};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use axum::{
    Router,
    routing::{get, post},
    response::sse::Event,
    extract::{Path, Json, ContentLengthLimit, Extension},
    http::{StatusCode, Request, Response},
    response::IntoResponse,
    middleware::{self, Next},
    body::Body,
    error_handling::HandleErrorLayer,
};
use tower_http::{
    trace::TraceLayer,
    cors::{CorsLayer, Origin},
};
use http::header;

// Define a custom error type for your application
#[derive(Debug)]
enum ApiError {
    SimpleRejection(String),
    InternalError,
    // Add other error variants as needed
}

// Implement `IntoResponse` for `ApiError`
impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        match self {
            ApiError::SimpleRejection(message) => (
                StatusCode::BAD_REQUEST,
                format!("Error: {}", message),
            )
                .into_response(),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response(),
            // Handle other error variants as needed
        }
    }
}

use rhai::{Engine};
use tokio::runtime::Runtime;
use flume::*;

use crate::agent::Agent;
use crate::agentmgr::AgentManager;
use crate::jwt_util::{Claims, create_token, verify_token};
use crate::{s};

use axum::{
    Router,
    routing::get,
    http::StatusCode,
    response::IntoResponse,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn server() -> Result<(), hyper::Error> {
    pretty_env_logger::init();
    let app = Router::new()
        .route("/", get(hello_world));

    // Keep track of all connected users, key is usize, value
    // is an event stream sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    let manager = AgentManager::new();

    // POST /chat -> send message
use axum::{
    extract::{Path, Header, ContentLengthLimit, Json, Extension},
    response::IntoResponse,
    Json as AxumJson,
};

async fn chat_send_handler(
    Path(user_id): Path<usize>,
    Header(authorization): Header<String>,
    ContentLengthLimit(Json(msg)): ContentLengthLimit<Json<String>, { 500 }>,
    Extension(users): Extension<Users>,
    Extension(manager): Extension<AgentManager>
) -> impl IntoResponse {
    // Extract the token from the authorization header
    let token = authorization.strip_prefix("Bearer ").ok_or(SimpleRejection("Invalid token format".into()))?;
    let claims = verify_token(token).map_err(|_| SimpleRejection("Invalid token".into()))?;

    // Call the existing handle_msg function to process the message
    match handle_msg(user_id, claims.username, msg, users, manager).await {
        Ok(_) => AxumJson("ok"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to handle message"
        ).into_response(),
    }
}

// Since handle_msg is now an async function, we need to update its signature and implementation
async fn handle_msg(
    my_id: usize,
    username: String,
    msg: String,
    users: Users,
    manager: AgentManager
) -> Result<(), SimpleRejection> {
    let (sender, reply_receiver) = manager.get_or_create_agent(username, my_id, s!("scripts/dm.rhai")).await;

    sender.send_async(msg).await.map_err(|_| SimpleRejection("Failed to send message".into()))?;

    if let Some(reply) = reply_receiver.recv_async().await {
        let users_lock = users.lock().map_err(|_| SimpleRejection("Failed to lock users".into()))?;
        if let Some(tx) = users_lock.get(&my_id) {
            tx.send(ChatUIMessage::Reply(reply)).map_err(|_| SimpleRejection("Failed to send reply".into()))?;
        }
    }

    Ok(())
}

// ... (rest of the existing code)

    let app = app.route("/chat/:user_id", post(chat_send_handler));
        
   // GET /chat -> messages stream
    let chat_recv = Router::new()
        .route("/chat", get(chat_recv_handler))
        .layer(Extension(users));


    let login = Router::new()
        .route("/login", post(login_handler))
        .layer(Extension(manager.clone()));

    async fn login_handler(
        Json(credentials): Json<Credentials>,
        Extension(manager): Extension<AgentManager>
    ) -> impl IntoResponse {
        if credentials.username.starts_with("anon") || 
           (credentials.username == "user" && credentials.password == "password") {
            let token = create_token(&credentials.username)
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"))?;
            (StatusCode::OK, Json(LoginResponse { token }))
        } else {
            (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
        }
    }

    let auth_route = Router::new()
        .route("/auth_route", get(auth_route_handler));

    async fn auth_route_handler(
        Header(authorization): Header<String>
    ) -> Result<Json<Claims>, (StatusCode, &'static str)> {
        let token = authorization.strip_prefix("Bearer ")
            .ok_or((StatusCode::BAD_REQUEST, "Invalid token format"))?;
        let claims = verify_token(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;
        Ok(Json(claims))
    }

    // GET / -> index html
    let index = Router::new()
        .route("/", get(serve_file("static/chat.html")));

    use tower_http::services::ServeDir;

    // Serve static files from static/ directory
    let static_files = Router::new()
        .nest("/static", service(ServeDir::new("static")));

    let app = Router::new()
        .merge(index)
        .merge(chat_recv)
        .merge(chat_send)
        .merge(login)
        .merge(auth_route)
        .merge(static_files);

    axum::Server::bind(&"0.0.0.0:3132".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await?;
}

async fn handle_msg(my_id: usize, userid: String, msg:String, users: Users,
                    manager: AgentManager) -> Result<impl warp::Reply, Infallible> {
    let (sender, reply_receiver) = manager.get_or_create_agent("username".to_string(), my_id, s!("scripts/dm.rhai")).await;

    sender.send_async(msg).await.unwrap();

    loop {
      let reply = reply_receiver.recv_async().await.unwrap();
      {
        let users_lock = users.lock().unwrap();
        let tx = users_lock.get(&my_id).unwrap().clone();
        tx.send(reply); 
      }
    }

    Ok( "ok" )
}
 
/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Message variants.
#[derive(Debug)]
pub enum ChatUIMessage {
    UserId(usize),
    Reply(String),
    Fragment(String),
    FunctionCall {
        name: String,
        params: String,
        result: String
    }
}

#[derive(Debug)]
pub struct NotUtf8;
impl warp::reject::Reject for NotUtf8 {}

// Import SimpleRejection from the errors module.
use crate::errors::SimpleRejection;


type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<ChatUIMessage>>>>;

fn user_connected(users: Users) -> impl Stream<Item = Result<axum::response::sse::Event, Infallible>> + Send + 'static {
    // Use a counter to assign a new unique ID for this user.
    //let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    let my_id = 1;
    eprintln!("new chat user: {}", my_id);

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the event source...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tx.send(ChatUIMessage::UserId(my_id))
        // rx is right above, so this cannot fail
        .unwrap();

    // Save the sender in our list of connected users.
    users.lock().unwrap().insert(my_id, tx);

    // Convert messages into Server-Sent Events and return resulting stream.
    rx.map(|msg| match msg {
        ChatUIMessage::UserId(my_id) => Ok(Event::default().event("user").data(my_id.to_string())),
        ChatUIMessage::Fragment(fragment) => Ok(Event::default().event("fragment").data(fragment)), 
        ChatUIMessage::Reply(reply) => Ok(Event::default().data(reply)),
        ChatUIMessage::FunctionCall { name, params, result } => {
            println!("Sending fn call as json");
            let data = serde_json::json!({
                "name": name,
                "params": params,
                "result": result
            });
            println!("OK 2");
            Ok(Event::default().event("functionCall").data(data.to_string()))
        }
    })
}

fn user_message(my_id: usize, msg: String, users: &Users) {
    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    //
    // We use `retain` instead of a for loop so that we can reap any user that
    // appears to have disconnected.
    users.lock().unwrap().retain(|uid, tx| {
        if my_id == *uid {
            // don't send to same user, but do retain
            true
        } else {
            // If not `is_ok`, the SSE stream is gone, and so don't retain
            tx.send(ChatUIMessage::Reply(new_msg.clone())).is_ok()
        }
    });
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
