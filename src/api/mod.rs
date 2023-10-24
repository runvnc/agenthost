use warp::*;
use crate::connector::*;
use anyhow::{Result};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

#[tokio::main]
pub async fn server() -> Result<()> {
    // Create a hashmap to store interrupt_senders associated with chat IDs
    let chat_sessions: Arc<RwLock<HashMap<String, mpsc::Sender<()>>>> = Arc::new(RwLock::new(HashMap::new()));

    // Create a Warp route to handle chat requests
    let chat_route = warp::path!("chat" / String / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::sse::keep_alive().map(Some).or_else(|_| async { Ok::<(Option<_>,), std::convert::Infallible>((None,)) }))
        .map({
            let chat_sessions = Arc::clone(&chat_sessions);

            move |id, script_name, user_input: String, keep_alive| {
                let (sender, receiver) = mpsc::channel(16);
                let (interrupt_sender, _interrupt_receiver) = mpsc::channel(16);

                chat_sessions.write().await.unwrap().insert(id.clone(), interrupt_sender.clone());

                let mut connector = Connector::new(sender.clone(), script_name, Arc<Mutex<interrupt_sender>>);

                if let Some(keep_alive) = keep_alive {
                    tokio::spawn(async move {
                        if keep_alive.await.is_err() {
                            // The client has disconnected
                            if let Some(interrupt_sender) = chat_sessions.write().unwrap().remove(&id) {
                                interrupt_sender.send(()).unwrap();
                            }
                            chat_sessions.write().unwrap().remove(&id);
                        }
                    });
                }

                tokio::spawn(async move {
                    connector.start_agent(user_input).await;
                });

                warp::sse::reply(receiver)
            }
        });

    // Cancel chat route
    let cancel_chat_route = warp::path!("cancelchat" / String)
        .map({
            let chat_sessions = Arc::clone(&chat_sessions);

            move |id: String| {
                if let Some(interrupt_sender) = chat_sessions.write().await.unwrap().remove(&id) {
                    interrupt_sender.send(()).unwrap();
                    format!("Chat {} cancelled", id)
                } else {
                    format!("No active chat found with ID {}", id)
                }
            }
        });

    // Combine the routes
    let routes = chat_route.or(cancel_chat_route);

    // Start the Warp server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
