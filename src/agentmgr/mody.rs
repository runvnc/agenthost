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

        //tokio::spawn(async move {
        
        /* let runtime = Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("my-custom-name")
            .thread_stack_size(3 * 1024 * 1024)
            .build()
            .unwrap(); */

        thread::spawn(move || {
            let runtime = Builder::new_multi_thread()
             .worker_threads(4)
             .thread_name("run_agent")
             .thread_stack_size(3 * 1024 * 1024)
             .enable_io()
             .enable_time()
             .build()
             .unwrap();

            //let runtime = Runtime::new().unwrap();
            runtime.block_on( async {
                let local = task::LocalSet::new();

                    local.run_until(async move {
                        //let nonsend_data = nonsend_data.clone();
                        // `spawn_local` ensures that the future is spawned on the local
                        // task set.
                        task::spawn_local(async move {
                            let mut agent = Agent::new(script_path, receiver, reply_sender).expect("no agent");
                            agent.run().await;
                            // ...
                        }).await.unwrap();
                    }).await;
            });
        });

        (sender, reply_receiver)
    }
}

/*
   // Construct a local task set that can run `!Send` futures.
    let local = task::LocalSet::new();

    // Run the local task set.
    local.run_until(async move {
        let nonsend_data = nonsend_data.clone();
        // `spawn_local` ensures that the future is spawned on the local
        // task set.
        task::spawn_local(async move {
            println!("{}", nonsend_data);
            // ...
        }).await.unwrap();
    }).await;

*/

