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

mod extract_code;
use extract_code::*;

const AGENTHOST_DEFAULT_MODEL: &str = "orca";

pub static llama_cpp_chat: OnceCell<LlamaCppChat> = OnceCell::new();

use std::fmt;

pub struct LlamaCppChat {
    model_options: LlamaOptions,
    model: Box<dyn Model>,
    llama: Arc<Mutex<LlamaCppSimple>>,
}

impl fmt::Debug for LlamaCppChat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LlamaCppChat")
            .field("model_options", &self.model_options)
            .field("llama", &self.llama)
            .finish()
    }
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
            "orca" => Box::new(OrcaModel::new()),
            "mixtral" => Box::new(MixtralModel::new()),
            _ => Box::new(OrcaModel::new()),
        };
        println!("---------------------------------------------------------------------------\n");
        println!("created model. info: {:?}", model.model_info());
        let full_model_path = format!("models/{}", &model.model_info().model_file);
        download_model_if_not_exists(
            &model.model_info().url,
            &full_model_path,
        )
        .await
        .unwrap();

        let model_options = LlamaOptions {
            model_path: full_model_path,
            context: model.model_info().max_context,
            ..Default::default()
        };

        let model_options_clone = model_options.clone();
        let llama_simple = LlamaCppSimple::new(model_options_clone).unwrap();
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
        println!("new_default_model(), model_name = {}", model_name);
        LlamaCppChat::new(&model_name).await
    }

    pub async fn generate(
        self: &LlamaCppChat,
        messages: Vec<ChatCompletionRequestMessage>,
        reply_sender: flume::Sender<ChatUIMessage>,
        token: CancellationToken,
    ) -> (String, String, String) {

        let another_sender = Arc::new(Mutex::new(reply_sender.clone()));
        let reply_str = String::new();

        let reply_str_clone = Arc::new(Mutex::new(reply_str.clone()));
        let reply_str_clone_for_closure = Arc::clone(&reply_str_clone);

        let llama = self
            .llama
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let last_msg = messages[messages.len() - 1..].to_vec();

        let text = &self.model.to_instruct_string(&messages);
        //let text = &self.model.to_instruct_string(&last_msg);
 
        println!("{}", text);

        llama.generate_text(
            &self.model.to_instruct_string(&messages),
            //&self.model.to_instruct_string(&last_msg),
            256,
            Box::new(move |tokenString| {
                let mut reply = reply_str_clone_for_closure
                    .lock()
                    .unwrap();
                reply.push_str(&tokenString);

                another_sender
                    .lock()
                    .unwrap()
                    .send(ChatUIMessage::Fragment(format!("*{}*", tokenString)))
                    .unwrap();

                !check_for_code(&reply.clone())
            }),
        );
        let result_str = reply_str_clone.lock().unwrap();
        let result_str_ = &result_str.to_string(); 
        let code_ = extract_code(result_str_);
        println!("Code extracted: [ {} ]", code_);

        let code = code_.unwrap();
        println!("Code extracted: [ {} ]", code);
        if code != "" {
            (result_str.to_string(), s!("eval"), s!(code))
        } else {
            (result_str.to_string(), s!(""), s!(""))
        }
    }
}
