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

    pub async fn get_or_create_agent(&self, id: usize, script_path: String) -> 
        (flume::Sender<String>, flume::Receiver<String>) {

        let mut cache = self.cache.lock().unwrap();

        if let Some((sender, reply_receiver)) = cache.get(&id) {
            return (sender.clone(), reply_receiver.clone());
        }

        let (sender, mut receiver) = flume::bounded(500);
        let (reply_sender, reply_receiver) = flume::bounded(500);
        cache.insert(id, (sender.clone(), reply_receiver.clone()));

        thread::spawn(|| {
            let future = async move {
                let mut agent = Agent::new(script_path, receiver, reply_sender).expect("no agent");
                agent.run().await;
            };
            let rt = Runtime::new().unwrap();
            rt.block_on(future);
        });

        //thread::spawn(|| async move {
        //    let mut agent = Agent::new(script_path, receiver, reply_sender).expect("no agent");
        //    agent.run().await;
        //});

        (sender, reply_receiver)
    }
}

