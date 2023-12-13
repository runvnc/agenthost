use async_openai::types::{ChatCompletionRequestMessage};
use crate::chatlog::serialize::{AnyChatMessage, to_anychatmessage};

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

pub fn to_instruct_string(msgs: &Vec<ChatCompletionRequestMessage>) -> String {
    let mut outs = String::from("<s>");
    for msg in msgs {
        let msg_ = to_anychatmessage(msg);
        let io_str = match msg_.role.as_str() {
            "user" => format!(" [INST] {} [/INST]", msg_.content.as_str()),
            "assistant" => format!(" {}</s>", msg_.content.as_str()),
            _ => "".to_string()
        }; 
        outs.push_str(&io_str);
    }
    outs.push_str("</s>");
    outs
}

