use std::collections::HashMap;
use std::sync::{Mutex};
use std::thread;
use tokio::runtime::Runtime;
use crate::agent::Agent;
use std::sync::Arc;
use tokio::sync::mpsc;
use flume::*;

#[derive(Debug, Clone)]
pub struct AgentManager {
    cache: Arc<Mutex<HashMap<usize, (flume::Sender<String>, flume::Receiver<String>)>>>,
}

impl AgentManager {
    pub fn new() -> Self {
        AgentManager {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_or_create_agent(&self, id: usize, script_path: String) -> 
        (flume::Sender<String>, flume::Receiver<String>) {

        let mut cache = self.cache.lock().unwrap();

        if let Some((sender, reply_receiver)) = cache.get(&id) {
            return (sender.clone(), reply_receiver.clone());
        }

        let (sender, mut receiver) = flume::unbounded();
        let (reply_sender, reply_receiver) = flume::unbounded();

        thread::spawn(move || {
            // Create a new Tokio Runtime for this thread.
            let rt = Runtime::new().unwrap();
            
            // Create and run the agent inside the thread.
            let agent = Agent::new(script_path, receiver, reply_sender);
            rt.block_on(agent.expect("No agent").run()); // Block on the async function.
        });

        cache.insert(id, (sender.clone(), reply_receiver.clone()));
        (sender, reply_receiver)
    }
}

