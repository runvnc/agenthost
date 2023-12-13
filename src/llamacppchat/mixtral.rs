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

pub fn to_instruct_strings(msgs: &Vec<ChatCompletionRequestMessage>) -> String {
    let mut outs = String::from("<s>");
    for msg in msgs {
        outs.push_str(
            let msg_ msg.to_anychatmessage()
            match msg_.role {
                "user" => format!(" [INST] {} [/INST]", msg_.content),
                "assistant" => format!(" {}</s>", msg_.content),
                _ => ""
            }
        );
    }
    outs.push_str("</s>");
    outs
}

