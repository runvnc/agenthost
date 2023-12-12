use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use tokio_util::sync::CancellationToken;
use download_model::*;
use std::io::{self, Write};
use std::env;
use async_openai::{
    types::{
        //ChatCompletionFunctions,
        ChatCompletionRequestMessage
    }
};

use flume::Sender;

use crate::api::chatuimessage::*;

const AGENTHOST_MODEL: &str = "models/mixtral-8x7b-instruct-v0.1.Q4_K_M.gguf";
const AGENTHOST_MODEL_URL: &str = "https://huggingface.co/TheBloke/Mixtral-8x7B-Instruct-v0.1-GGUF/resolve/main/mixtral-8x7b-instruct-v0.1.Q4_K_M.gguf?download=true"; 

struct LlamaCppChat {
  model_options: ModelOptions,
  model_file: String,
  llama: Llama
}

impl LlamaCppChat {
    async fn new(model_file: String) {
        let model_options = ModelOptions {
            n_gpu_layers: 1000,
            ..Default::default()
        };
        let llama = LLama::new(&model_file, &model_options).unwrap();

        LlamaCppChat {
            model_options,
            model_file,
            llama
        }
    } -> LLamaCppChat;

    async fn new_default_model() -> LLamaCppChat {
        let model_file = env::var("AGENTHOST_MODEL").unwrap_or(AGENTHOST_MODEL.to_string());
        let model_url = env::var("AGENTHOST_MODEL_URL").unwrap_or(AGENTHOST_MODEL_URL.to_string());
 
        download_model_if_not_exist(&model_url, &model_file).await.unwrap();

        LlamaCppChat::new(model_file)
    }

    async fn generate(self: &LlamaCppChat, 
                messages: Vec<ChatCompletionRequestMessage>,
                reply_sender: flume::Sender<ChatUIMessage>,
                token: CancellationToken,
       ) -> (String, String, String) {

        let predict_options = PredictOptions {
            tokens: 0,
            threads: 10,
            layers: 1000,
            temperature: 0.0001,
            token_callback: Some(Box::new(|token| {
            reply_sender
                .send_async(ChatUIMessage::Fragment(format!("*{}*", token)))
                .await?;
                true
            })),
            ..Default::default()
        };
        //top_k: 90,
        //top_p: 0.86,
 
        llama
            .predict(
                messages.into(),
                predict_options,
            )
            .unwrap();
        (s!("ok"), s!(""), s!(""))
    }

}
