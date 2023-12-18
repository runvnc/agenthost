use async_openai::types::ChatCompletionRequestMessage;
use flume::Sender;
use llama_cpp_rs::{LlamaCppSimple, LlamaOptions};
use once_cell::sync::OnceCell;
use std::{
    env,
    io::{self, Write},
    sync::{Arc, Mutex},
};
use tokio_util::sync::CancellationToken;
pub mod download_model;
use crate::{api::chatuimessage::ChatUIMessage, s};
use download_model::*;
pub mod model;
use model::*;
pub mod mixtral;
pub mod orca;
use mixtral::*;
use orca::*;

const AGENTHOST_DEFAULT_MODEL: &str = "orca";

pub static llama_cpp_chat: OnceCell<LlamaCppChat> = OnceCell::new();

#[derive(Debug)]
pub struct LlamaCppChat {
    model_options: LlamaOptions,
    model: Box<dyn Model>,
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
    pub async fn new(model_name: &str) -> LlamaCppChat {
        let model: Box<dyn Model> = match model_name {
            "orca" => Box::new(OrcaModel::default()),
            "mixtral" => Box::new(MixtralModel::default()),
            _ => Box::new(OrcaModel::default()),
        };
        download_model_if_not_exists(
            &model.model_info().url,
            &model.model_info().model_file,
        )
        .await
        .unwrap();

        let model_options = LlamaOptions {
            model_path: model.model_info().model_file,
            context: model.model_info().max_context,
            ..Default::default()
        };

        let llama_simple = LlamaCppSimple::new(&model_options).unwrap();
        let llama = Arc::new(Mutex::new(llama_simple));

        LlamaCppChat {
            model_options,
            model,
            llama
        }
    }

    pub async fn new_default_model() -> LlamaCppChat {
        let model_name =
            env::var("AGENTHOST_DEFAULT_MODEL").unwrap_or(AGENTHOST_DEFAULT_MODEL.to_string());

        LlamaCppChat::new(&model_name).await
    }

    pub async fn generate(
        self: &LlamaCppChat,
        messages: Vec<ChatCompletionRequestMessage>,
        reply_sender: flume::Sender<ChatUIMessage>,
        token: CancellationToken,
    ) -> (String, String, String) {
        let another_sender = Arc::new(Mutex::new(reply_sender.clone()));

        let llama = self
            .llama
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        llama.generate_text(
            &self.model.to_instruct_string(&messages),
            512,
            Box::new(move |tokenString| {
                another_sender
                    .lock()
                    .unwrap()
                    .send(ChatUIMessage::Fragment(format!("*{}*", tokenString)))
                    .unwrap();
                true
            }),
        );
        (s!("ok"), s!(""), s!(""))
    }
}
