use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use tokio_util::sync::CancellationToken;
use std::io::{self, Write};
use std::env;
use async_openai::{
    types::{
        //ChatCompletionFunctions,
        ChatCompletionRequestMessage
    }
};

use std::sync::{Arc, Mutex};
use flume::Sender;
pub mod download_model;
use crate::s;
use crate::api::chatuimessage::ChatUIMessage;

use download_model::*;

mod mixtral;
use mixtral::to_instruct_string;

const AGENTHOST_MODEL: &str = "models/mixtral-8x7b-instruct-v0.1.Q4_K_M.gguf";
const AGENTHOST_MODEL_URL: &str = "https://huggingface.co/TheBloke/Mixtral-8x7B-Instruct-v0.1-GGUF/resolve/main/mixtral-8x7b-instruct-v0.1.Q4_K_M.gguf?download=true"; 

pub struct LlamaCppChat {
  model_options: ModelOptions,
  model_file: String,
  llama: LLama
}

impl LlamaCppChat {
    pub async fn new(model_file: String) -> LlamaCppChat {
        let model_options = ModelOptions {
            n_gpu_layers: 1000,
            ..Default::default()
        };
        let llama = LLama::new(model_file.clone(), &model_options).unwrap();

        LlamaCppChat {
            model_options,
            model_file,
            llama
        }
    }

    pub async fn new_default_model() -> LlamaCppChat {
        let model_file = env::var("AGENTHOST_MODEL").unwrap_or(AGENTHOST_MODEL.to_string());
        let model_url = env::var("AGENTHOST_MODEL_URL").unwrap_or(AGENTHOST_MODEL_URL.to_string());
 
        download_model_if_not_exists(&model_url, &model_file).await.unwrap();

        LlamaCppChat::new(model_file).await
    }


    pub async fn generate(self: &LlamaCppChat, 
                messages: Vec<ChatCompletionRequestMessage>,
                reply_sender: flume::Sender<ChatUIMessage>,
                token: CancellationToken,
       ) -> (String, String, String) {
        let another_sender = Arc::new(Mutex::new(reply_sender.clone()));
        let predict_options = PredictOptions {
            tokens: 0,
            threads: 10,
            temperature: 0.0001,

            token_callback: Some(Box::new(move |token| {
                another_sender.lock().unwrap().send(ChatUIMessage::Fragment(format!("*{}*", token))).unwrap();
                true
            })),
            ..Default::default()
        };
        //top_k: 90,
        //top_p: 0.86,
        
        self.llama
            .predict(
                to_instruct_string(&messages),
                predict_options,
            )
            .unwrap();
        (s!("ok"), s!(""), s!(""))
    }

}
