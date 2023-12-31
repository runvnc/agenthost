use super::model::{Model, ModelInfo};
use crate::chatlog::serialize::{to_anychatmessage, AnyChatMessage};
use async_openai::types::ChatCompletionRequestMessage;

use crate::s;

#[derive(Debug,Default)]
pub struct MistralModel {
    info: ModelInfo,
}

impl MistralModel {
    pub fn new() -> MistralModel {
        let info = ModelInfo {
              type_name: s!("Mistral"),
              model_file: s!("mistral-7b-instruct-v0.2.Q5_K_M.gguf"),
              url: s!("https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q5_K_M.gguf?download=true"),
              max_context: 4096
        };
        MistralModel { info }
    }
}

/*
https://docs.mistral.ai/models/


<s>[INST] Instruction [/INST] Model answer</s>[INST] Follow-up instruction [/INST]


[START_SYMBOL_ID] +
tok("[INST]") + tok(USER_MESSAGE_1) + tok("[/INST]") +
tok(BOT_MESSAGE_1) + [END_SYMBOL_ID] +
…
tok("[INST]") + tok(USER_MESSAGE_N) + tok("[/INST]") +
tok(BOT_MESSAGE_N) + [END_SYMBOL_ID]

*/

impl Model for MistralModel {
    fn to_instruct_string(&self, msgs: &Vec<ChatCompletionRequestMessage>) -> String {
        let mut outs = String::from("<s>");
        for msg in msgs {
            let msg_ = to_anychatmessage(msg);
            let role = msg_.role.as_str();
            let name = msg_.name.as_str();
            let content = msg_.content.as_str();
            let io_str = match (role, name) {
                (_, "SYSTEM OUTPUT") | (_, "eval") => format!("[INST] {} [/INST]\n Using this system output, ", content),
                ("system", _) => format!("[INST] {} [/INST] System instructions understood.</s>", content),
                ("user", _) => format!("[INST] {} [/INST]", content),
                ("assistant", _) => format!("{}</s>", content),
                _ => "".to_string(),
            };
            outs.push_str(&io_str);
        }
        //outs.push_str("</s>");
        outs
    }

    fn model_info(&self) -> ModelInfo {
        self.info.clone()
    }
}
