use crate::agent::Agent;
use crate::api::chatuimessage::ChatUIMessage;
use flume::*;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::fs;
use std::path::Path;
use tokio::runtime::Runtime;
use tokio::runtime::*;
use tokio::sync::mpsc;
use tokio::*;
use tokio_util::sync::CancellationToken;

pub static agent_mgr: OnceCell<AgentManager> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct SessionCache {
    cache: HashMap<usize, (flume::Sender<String>, flume::Receiver<ChatUIMessage>)>,
}

#[derive(Debug, Clone)]
pub struct AgentManager {
    user_cache: Arc<Mutex<HashMap<String, SessionCache>>>,
}

pub fn init() {
    if !(agent_mgr.get().is_some()) {
        println!("agentmgr: init..");
        agent_mgr
            .set(AgentManager::new())
            .expect("Failed to set AgentManager");
    }
}

impl AgentManager {
    pub fn new() -> Self {
        AgentManager {
            user_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn list_sessions(&self, username: &str) -> Result<Vec<String>, std::io::Error> {
        let path = format!("data/{}/sessions", username);
        let mut sessions = Vec::new();

        for entry in fs::read_dir(Path::new(&path))? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(session_id) = entry.path().file_stem() {
                    sessions.push(session_id.to_string_lossy().into_owned());
                }
            }
        }

        Ok(sessions)
    }

    pub async fn get_or_create_agent(
        &self,
        username: String,
        id: usize,
        script_path: String,
    ) -> (flume::Sender<String>, flume::Receiver<ChatUIMessage>, ) {
        let mut user_cache = self.user_cache.lock().unwrap();

        let session_cache = user_cache
            .entry(username.clone())
            .or_insert_with(|| SessionCache {
                cache: HashMap::new(),
            });

        if let Some((sender, reply_receiver)) = session_cache.cache.get(&id) {
            return (sender.clone(), reply_receiver.clone());
        }

        let (sender, mut receiver) = flume::bounded(500);
        let (reply_sender, reply_receiver) = flume::bounded(500);

        session_cache
            .cache
            .insert(id, (sender.clone(), reply_receiver.clone()));
        let session_id = id.clone();

        thread::spawn(move || {
            let future = async move {
                let mut agent =
                    Agent::new(username, session_id, script_path, receiver, reply_sender)
                        .expect("no agent");
                agent.run().await;
            };
            let rt = Runtime::new().unwrap();
            rt.block_on(future);
        });

        (sender, reply_receiver)
    }
}
