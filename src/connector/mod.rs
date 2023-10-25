#![allow(warnings)]

use serde_json::json;
use tokio::sync::mpsc;
use warp::Reply;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::agent::*;

pub struct Connector {
    sender: mpsc::Sender<Result<Box<dyn Reply>, warp::Rejection>>,
    script_path: String,
    interrupt_receiver: Arc<Mutex<mpsc::Receiver<()>>>,
}

pub trait BasicChatInterface {
    fn receive_message(&mut self, message: AgentMessage);
}



impl Connector {
    pub fn new(
        sender: mpsc::Sender<Result<Box<dyn Reply>, warp::Rejection>>, 
        script_path: String, 
        interrupt_receiver: Arc<Mutex<mpsc::Receiver<()>>>
    ) -> Self {
        Self { sender, script_path, interrupt_receiver }
    }

    pub async fn start_agent(&mut self, user_input: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sender_clone = self.sender.clone();
        let script_path_clone = self.script_path.clone();
        let interrupt_receiver_clone = self.interrupt_receiver.clone();

        tokio::spawn(async move {
            /*let mut agent = Agent::new(script_path_clone, interrupt_receiver_clone).unwrap();
            
            let mut connector = Connector::new(sender_clone.clone(), 
                    script_path_clone, interrupt_receiver_clone);
            if let Err(e) = agent.run_some(Some(&user_input), &mut connector).await {
                eprintln!("Error processing chat: {:?}", e);
            }
            
            drop(sender_clone); */
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        }).await??;
        Ok(())
    }

}

impl BasicChatInterface for Connector {
    fn receive_message(&mut self, message: AgentMessage) {
        let json_message = match message {
            AgentMessage::Fragment(fragment) => json!({ "type": "fragment", "content": fragment }),
            AgentMessage::Complete(complete_message) => json!({ "type": "complete", "content": complete_message }),
        };

        tokio::spawn(async move {
            self.sender.send(Ok(Box::new(warp::reply::json(&json_message)))).await.unwrap();
        });
    }
}
