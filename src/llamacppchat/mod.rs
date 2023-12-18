use llama_cpp_rs::{LlamaCppSimple, LlamaOptions};
use tokio_util::sync::CancellationToken;
use std::{env, io::{self, Write}, sync::{Arc, Mutex}};
use async_openai::types::ChatCompletionRequestMessage;
use flume::Sender;
use once_cell::sync::OnceCell;
pub mod download_model;
use crate::{s, api::chatuimessage::ChatUIMessage};
use download_model::*;
mod model;
use model::*;
mod orca;
mod mixtral;
use orca::*;
use mixtral::*;

const AGENTHOST_DEFAULT_MODEL = "orca";

pub static llama_cpp_chat: OnceCell<LlamaCppChat> = OnceCell::new();

#[derive(Debug)]
pub struct LlamaCppChat {
    model_options: LlamaOptions,
    model_file: String,
    llama: Arc<Mutex<LlamaCppSimple>>,
}

pub async fn init_llama_cpp_chat() {
    if !(llama_cpp_chat.get().is_some()) {
        println!("llama_cpp_chat: init..");
        llama_cpp_chat
            .set(LlamaCppChat::new_default_model().await)
            .expect("Failed to create LlamaCppChat with default model");
    }
}

impl LlamaCppChat {
    pub async fn new(model_name: String) -> LlamaCppChat {
        let model = match model_name: {
            "orca" | _ => Orca::default()
            "mixtral" => Mixtral::default()
        }
        download_model_if_not_exists(&model.download_url(), &model.file_path()).await.unwrap();

        let model_options = LlamaOptions {
            model_path: model.file_path().to_string(),
            context: 2048,
            ..Default::default()
        };

        let llama = Arc::new(Mutex::new(LlamaCppSimple::new(model_options))).unwrap();

         LlamaCppChat {
             model_options,
             model_file,
             llama
         }
     }

     pub async fn new_default_model() -> LlamaCppChat {
         let model_name = env::var("AGENTHOST_DEFAULT_MODEL").unwrap_or(AGENTHOST_DEFAULT_MODEL.to_string());

         LlamaCppChat::new(model_name).await
     }


    pub async fn generate(self: &LlamaCppChat,
                messages: Vec<ChatCompletionRequestMessage>,
                reply_sender: flume::Sender<ChatUIMessage>,
                token: CancellationToken,
       ) -> (String, String, String) {
        let another_sender = Arc::new(Mutex::new(reply_sender.clone()));

        let llama = self.llama.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        llama.generate_text(
            to_instruct_string(&messages),
            512,
            Box::new(move |token| {
                another_sender.lock().unwrap().send(ChatUIMessage::Fragment(format!("*{}*", token))).unwrap();
                true
            }) 
        );
        (s!("ok"), s!(""), s!(""))
    }

}
