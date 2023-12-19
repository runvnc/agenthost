use super::model::{Model, ModelInfo};
use crate::chatlog::serialize::{to_anychatmessage, AnyChatMessage};
use async_openai::types::ChatCompletionRequestMessage;
use crate::s;

#[derive(Debug, Default)]
pub struct OrcaModel {
    info: ModelInfo,
}

//impl Send for dyn Model {}
//unsafe impl Sync for dyn Model {}

impl OrcaModel {
    pub fn new() -> OrcaModel {
        let info = ModelInfo {
            type_name: s!("Orca"),
            model_file: s!("orca-2-7b.Q4_0.gguf"),
            url: s!("https://huggingface.co/TheBloke/Orca-2-7B-GGUF/resolve/main/orca-2-7b.Q4_0.gguf?download=true"),
            max_context: 2048
        };
        OrcaModel { info }
    }
}

//<|im_start|>system\n{system_message}<|im_end|>\n<|im_start|>user\n{user_message}<|im_end|>\n<|im_start|>assistant

impl Model for OrcaModel {
    fn to_instruct_string(&self, msgs: &Vec<ChatCompletionRequestMessage>) -> String {
        let mut outs = String::new();
        for msg in msgs {
            let msg_ = to_anychatmessage(msg);
            let io_str = format!(
                "<|im_start|>{}\n{}<|im_end|>\n",
                msg_.role.as_str(),
                msg_.content.as_str()
            );
            outs.push_str(&io_str);
        }
        outs.push_str("<|im_start|>assistant\n");
        outs
    }

    fn model_info(&self) -> ModelInfo {
        self.info.clone()
    }
}
