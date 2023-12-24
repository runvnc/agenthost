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
              model_file: s!("mistral-ft-optimized-1218.Q4_0.gguf"),
              url: s!("https://huggingface.co/TheBloke/mistral-ft-optimized-1218-GGUF/resolve/main/mistral-ft-optimized-1218.Q4_0.gguf?download=true"),
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
            let io_str = match msg_.role.as_str() {
                "user" | "system" => format!(" [INST] {} [/INST]", msg_.content.as_str()),
                "assistant" => format!(" {}</s>", msg_.content.as_str()),
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
