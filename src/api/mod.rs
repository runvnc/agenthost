use futures_util::{Stream, StreamExt};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{sse::Event, Filter};
use rhai::{Engine};
use tokio::runtime::Runtime;
use flume::*;

use crate::agent::Agent;
use crate::agentmgr::AgentManager;

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
    let chat_recv = warp::path("chat").and(warp::get()).and(users).map(|users| {
        // reply using server-sent events
        let stream = user_connected(users);
        warp::sse::reply(warp::sse::keep_alive().stream(stream))
    });

    // GET / -> index html
    let index = warp::path::end().map(|| {
        let html = std::fs::read_to_string("chat.html").unwrap();
        warp::http::Response::builder()
            .header("content-type", "text/html; charset=utf-8")
            .body(html)
    });

    let routes = index.or(chat_recv).or(chat_send);

    warp::serve(routes).run(([0, 0, 0, 0], 3132)).await;
}

async fn handle_msg(my_id: usize, msg:String, users: Users,
                    manager: AgentManager) -> Result<impl warp::Reply, Infallible> {
    let (sender, reply_receiver) = manager.get_or_create_agent(my_id, s!("scripts/dm.rhai")).await;

    println!("Sending message from API");
    sender.send_async(msg).await.unwrap();
    println!("Sent message from API to agent");

    loop {
      let reply = reply_receiver.recv_async().await.unwrap();
      println!("Received reply from agent: {}", reply);
      {
        let users_lock = users.lock().unwrap();
        let tx = users_lock.get(&my_id).unwrap().clone();
        tx.send(reply); //Message::Reply(reply));
      }
      println!("Forwarded reply as SSE.");
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
    Fragment(String)
}

#[derive(Debug)]
pub struct NotUtf8;
impl warp::reject::Reject for NotUtf8 {}

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `Message`
type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<ChatUIMessage>>>>;

fn user_connected(users: Users) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

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
        ChatUIMessage::Fragment(fragment) => Ok(Event::default().event("fragment").data(fragment), 
        ChatUIMessage::Reply(reply) => Ok(Event::default().data(reply)),
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

