use futures_util::{Stream, StreamExt};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{sse::Event, Filter, reject};
use rhai::{Engine};
use tokio::runtime::Runtime;
use flume::*;

use crate::agent::Agent;
use crate::agentmgr::AgentManager;
use crate::jwt_util::{Claims, create_token, verify_token};
use crate::{s};

pub async fn server() {
    pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is an event stream sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());
    let manager = AgentManager::new();
    let manager = warp::any().map(move || manager.clone());

    // POST /chat -> send message
    let chat_send = warp::path("chat")
        .and(warp::post())
        .and(warp::path::param::<usize>())
        .and(warp::header("authorization"))
        .and_then(|authorization: String| async move {
            let token = authorization.strip_prefix("Bearer ").ok_or(warp::reject::custom(InvalidTokenFormat))?;
            let claims = verify_token(token)?;
            claims.username
        })
        .and(warp::body::content_length_limit(500))
        .and(
            warp::body::bytes().and_then(|body: bytes::Bytes| async move {
                std::str::from_utf8(&body)
                    .map(String::from)
                    .map_err(|_e| warp::reject::custom(NotUtf8))
            }),
        )
        .and(users.clone())
        .and(manager.clone())
        .and_then(handle_msg);
        
   // GET /chat -> messages stream
    let chat_recv = warp::path("chat").and(warp::get()).
        .and(warp::header("authorization"))
        .and_then(|authorization: String| async move {
            let token = authorization.strip_prefix("Bearer ").ok_or(warp::reject::custom(InvalidTokenFormat))?;
            let claims = verify_token(token)?;
            println!("User connected: {}", claims.username);
        })
        .and(users).map(|users| {
         let stream = user_connected(users);
          warp::sse::reply(warp::sse::keep_alive().stream(stream))
        });


    let login = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|credentials: Credentials| async move {
            
            if credentials.username.starts_with("anon") || 
               (credentials.username == "user" && credentials.password == "password") {
                let token = create_token(&credentials.username)?;
                Ok(warp::reply::json(&LoginResponse { token }))
            } else {
                Err(warp::reject::custom(InvalidCredentials))
            }
        });

    // Authenticated route
    let auth_route = warp::path!("auth_route")
        .and(warp::header("authorization"))
        .and_then(|authorization: String| async move {
            let token = authorization.strip_prefix("Bearer ").ok_or(warp::reject::custom(InvalidTokenFormat))?;
            let claims = verify_token(token)?;
            // Now claims contains the user info
            Ok(warp::reply::json(&claims))
        });

    // GET / -> index html
    let index = warp::path::end()
        .and(warp::fs::file("static/chat.html"));

    // Serve static files from static/ directory
    let static_files = warp::path("static")
        .and(warp::fs::dir("static"));

    let routes = index.or(chat_recv).or(chat_send).or(static_files).or(login).or(auth_route);

    warp::serve(routes).run(([0, 0, 0, 0], 3132)).await;
}

async fn handle_msg(my_id: usize, userid: String, msg:String, users: Users,
                    manager: AgentManager) -> Result<impl warp::Reply, Infallible> {
    let (sender, reply_receiver) = manager.get_or_create_agent(my_id, s!("scripts/dm.rhai")).await;

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

#[derive(Debug)]
struct SimpleRejection(String);
impl reject::Reject for SimpleRejection {}


type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<ChatUIMessage>>>>;

fn user_connected(users: Users) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
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
