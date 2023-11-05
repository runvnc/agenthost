use std::collections::HashMap;
use std::sync::{Mutex};
use tokio::runtime::Runtime;
use crate::agent::Agent;
use std::sync::Arc;
use tokio::sync::mpsc;
use flume::*;
use tokio::runtime::*;
use tokio::*;
use std::thread;
use crate::api::ChatUIMessage;


#[derive(Debug, Clone)]
pub struct SessionCache {
    cache: HashMap<usize, (flume::Sender<String>, flume::Receiver<ChatUIMessage>)>,
}

#[derive(Debug, Clone)]
pub struct AgentManager {
    user_cache: Arc<Mutex<HashMap<String, SessionCache>>>,
}

impl AgentManager {
    pub fn new() -> Self {
        AgentManager {
            user_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_agent(&self, username: String, id: usize, script_path: String) -> 
        (flume::Sender<String>, flume::Receiver<ChatUIMessage>) {

        let mut user_cache = self.user_cache.lock().unwrap();

        let session_cache = user_cache.entry(username).or_insert_with(|| SessionCache { cache: HashMap::new() });

        if let Some((sender, reply_receiver)) = session_cache.cache.get(&id) {
            return (sender.clone(), reply_receiver.clone());
        }

        let (sender, mut receiver) = flume::bounded(500);
        let (reply_sender, reply_receiver) = flume::bounded(500);
        session_cache.cache.insert(id, (sender.clone(), reply_receiver.clone()));
        let session_id = id.clone();

        thread::spawn(move || {
            let future = async move {
                let mut agent = Agent::new(username, session_id, script_path, receiver, reply_sender).expect("no agent");
                agent.run().await;
            };
            let rt = Runtime::new().unwrap();
            rt.block_on(future);
        });

        (sender, reply_receiver)
    }
}

