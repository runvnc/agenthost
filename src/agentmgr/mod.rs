use std::collections::HashMap;
use std::sync::{mpsc, Mutex};
use std::thread;
use tokio::runtime::Runtime;

pub struct AgentManager {
    cache: Mutex<HashMap<i32, (mpsc::Sender<String>, mpsc::Receiver<String>)>>,
}

pub impl AgentManager {
    fn new() -> Self {
        AgentManager {
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn get_or_create_agent(&self, id: i32, script_path: String) -> (mpsc::Sender<String>, mpsc::Receiver<String>) {
        let mut cache = self.cache.lock().unwrap();

        if let Some((sender, reply_receiver)) = cache.get(&id) {
            return (sender.clone(), reply_receiver.clone());
        }

        let (sender, receiver) = mpsc::channel();
        let (reply_sender, reply_receiver) = mpsc::channel();

        // Spawn a new thread for each agent.
        thread::spawn(move || {
            // Create a new Tokio Runtime for this thread.
            let rt = Runtime::new().unwrap();
            
            // Create and run the agent inside the thread.
            let agent = Agent::new(script_path, receiver, reply_sender);
            rt.block_on(agent.run()); // Block on the async function.
        });

        cache.insert(id, (sender.clone(), reply_receiver.clone()));
        (sender, reply_receiver)
    }
}

