use async_openai::types::{ChatCompletionRequestMessage};
use crate::chatlog::serialize::{AnyChatMessage, to_anychatmessage};
use super::model::{Model, ModelInfo};

#[derive(Default)]
pub struct OrcaModel {
    info: ModelInfo
}

impl OrcaModel {
    pub fn new() {
        let model_info = ModelInfo {
            type_name: "Orca",
            model_file: "orca-2-7b.Q4_0.gguf",
            url: "https://huggingface.co/TheBloke/Orca-2-7B-GGUF/resolve/main/orca-2-7b.Q4_0.gguf?download=true",
            max_context: 2048
        };
        Self { model_info }
    }
}

//<|im_start|>system\n{system_message}<|im_end|>\n<|im_start|>user\n{user_message}<|im_end|>\n<|im_start|>assistant


impl Model for OrcaModel {
    pub fn to_instruct_string(msgs: &Vec<ChatCompletionRequestMessage>) -> String {
        let mut outs = String::new();
        for msg in msgs {
            let msg_ = to_anychatmessage(msg);
            let io_str = format!("<|im_start|>{}\n{}<|im_end|>\n", msg_.role.as_str(), msg_.content.as_str());
            outs.push_str(&io_str);
        }
        outs
    }
}
