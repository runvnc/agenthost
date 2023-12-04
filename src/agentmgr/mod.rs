use crate::agent::Agent;
use crate::api::chatuimessage::ChatUIMessage;
use flume::*;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tokio::runtime::Runtime;
use tokio::runtime::*;
use tokio::sync::mpsc;
use tokio::*;
use tokio_util::sync::CancellationToken;

pub static agent_mgr: OnceCell<AgentManager> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct SessionCache {
    cache: HashMap<
        usize,
        (
            flume::Sender<String>,
            flume::Receiver<ChatUIMessage>,
            CancellationToken,
        ),
    >,
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

    pub fn list_sessions(&self, username: &str) -> Vec<String> {
        let path = format!("data/{}/sessions", username);
        let mut sessions = Vec::new();

        if let Ok(entries) = fs::read_dir(Path::new(&path)) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    if let Some(session_id) = entry.path().file_stem() {
                        sessions.push(session_id.to_string_lossy().into_owned());
                    }
                }
            }
        }

        sessions
    }

    pub async fn get_or_create_agent(
        &self,
        username: String,
        id: usize,
        script_path: String,
    ) -> (
        flume::Sender<String>,
        flume::Receiver<ChatUIMessage>,
        CancellationToken,
    ) {
        let mut user_cache = self.user_cache.lock().unwrap();

        let session_cache = user_cache
            .entry(username.clone())
            .or_insert_with(|| SessionCache {
                cache: HashMap::new(),
            });

        if let Some((sender, reply_receiver, cancellation_token)) = session_cache.cache.get(&id) {
            return (
                sender.clone(),
                reply_receiver.clone(),
                cancellation_token.clone(),
            );
        }

        let (sender, mut receiver) = flume::bounded(500);
        let (reply_sender, reply_receiver) = flume::bounded(500);
        let cancellation_token = CancellationToken::new();
        let cancellation_token_clone = cancellation_token.clone();

        session_cache.cache.insert(
            id,
            (
                sender.clone(),
                reply_receiver.clone(),
                cancellation_token_clone.clone(),
            ),
        );
        let session_id = id.clone();

        let user_cache_clone = Arc::clone(&self.user_cache);
        let username_clone = username.clone(); // Clone username before moving it
        let cancellation_token_clone_for_return = cancellation_token_clone.clone(); // Clone token for return
        thread::spawn(move || {
            let future = async move {
                let mut agent =
                    Agent::new(username_clone, session_id, script_path, receiver, reply_sender)
                        .expect("no agent");
                agent.run(cancellation_token_clone).await;
            };
            let rt = Runtime::new().unwrap();
            rt.block_on(future);
            let mut user_cache = user_cache_clone.lock().unwrap();
            if let Some(session_cache) = user_cache.get_mut(&username_clone) {
                session_cache.cache.remove(&session_id);
            }
        });

        (sender, reply_receiver, cancellation_token_clone_for_return) // Use the cloned token for return
    }
}
