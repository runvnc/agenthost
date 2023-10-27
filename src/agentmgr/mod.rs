use std::collections::HashMap;
use std::sync::{mpsc, Mutex};
use std::thread;
use tokio::runtime::Runtime;

struct AgentManager {
    cache: Mutex<HashMap<i32, (mpsc::Sender<String>, mpsc::Receiver<String>)>>,
}

impl AgentManager {
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

fn main() {
    let manager = AgentManager::new();
    
    let (sender1, reply_receiver1) = manager.get_or_create_agent(1);
    let (_sender2, _reply_receiver2) = manager.get_or_create_agent(2);

    sender1.send("Hello, agent 1!".to_string()).unwrap();

    // Get the reply from the agent
    let reply = reply_receiver1.recv().unwrap();
    println!("Reply from agent 1: {}", reply);
}

