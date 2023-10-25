use warp::*;
use crate::connector::*;
use anyhow::{Result};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio::sync::Mutex;

#[tokio::main]
pub async fn server() -> Result<()> {
    let chat_sessions: Arc<RwLock<HashMap<String, mpsc::Sender<()>>>> = Arc::new(RwLock::new(HashMap::new()));

    let chat_route = warp::path!("chat" / String / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::sse::keep_alive().map(Some).or_else(|_| async { Ok::<(Option<_>,), std::convert::Infallible>((None,)) }))
        .map({
            let chat_sessions = Arc::clone(&chat_sessions);

            move |id, script_name, user_input: String, keep_alive| {
                let (sender, receiver) = mpsc::channel(16);
                let (interrupt_sender, _interrupt_receiver) = mpsc::channel(16);

                tokio::spawn(async move {
                    chat_sessions.write().await.insert(id.clone(), interrupt_sender.clone());
                });

                let mut connector = Connector::new(sender.clone(), script_name, Arc::new(Mutex::new(interrupt_sender)));

                if let Some(keep_alive) = keep_alive {
                    tokio::spawn(async move {
                        if keep_alive.await.is_err() {
                            tokio::spawn(async move {
                                if let Some(interrupt_sender) = chat_sessions.write().await.remove(&id) {
                                    let _ = interrupt_sender.send(()).await;
                                }
                                chat_sessions.write().await.remove(&id);
                            });
                        }
                    });
                }

                tokio::spawn(async move {
                    connector.start_agent(user_input).await;
                });

                warp::sse::reply(warp::sse::keep(receiver))
            }
        });

    let cancel_chat_route = warp::path!("cancelchat" / String)
        .map({
            let chat_sessions = Arc::clone(&chat_sessions);

            move |id: String| {
                tokio::spawn(async move {
                    if let Some(interrupt_sender) = chat_sessions.write().await.remove(&id) {
                        let _ = interrupt_sender.send(()).await;
                        format!("Chat {} cancelled", id)
                    } else {
                        format!("No active chat found with ID {}", id)
                    }
                })
            }
        });

    let routes = chat_route.or(cancel_chat_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}