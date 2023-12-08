use anyhow::Result;
use once_cell::sync::OnceCell;
use termion::style;
use std::collections::HashMap;

use crate::s;

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestFunctionMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, FunctionCall, Role,
};

use tiktoken_rs::{cl100k_base, CoreBPE};

static BPE: OnceCell<CoreBPE> = OnceCell::new();

mod serialize;
//mod shorthands;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub message: ChatCompletionRequestMessage,
    pub length: usize,
}

impl ChatMessage {
    pub fn new(message: ChatCompletionRequestMessage) -> Self {
        let mut instance = Self { message, length: 0 };
        instance.calc_length();
        instance
    }

    pub fn new_with_len(message: ChatCompletionRequestMessage, length: usize) -> Self {
        Self { message, length }
    }

    pub fn calc_length(&mut self) {
        let str_msg = format!("{:?}", self.message);
        let tokens = BPE
            .get()
            .expect("No tokenizer")
            .encode_with_special_tokens(str_msg.as_ref());
        //#.encode_with_special_tokens(&(self.message.content.as_deref().unwrap_or("")));

        self.length = tokens.len()
    }
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::chatlog::serialize::{serialize_message, deserialize_message};

#[derive(Serialize, Deserialize)]
pub struct ChatLog {
    username: String,
    session_id: usize,
    pub messages: Vec<ChatMessage>,
}

pub fn sys_msg(text: &String) -> Result<ChatMessage> {
    let msg = ChatCompletionRequestSystemMessageArgs::default()
        .content(text)
        .build()?
        .into();
    Ok(ChatMessage::new(msg))
}

pub fn user_msg(text: &String) -> Result<ChatMessage> {
    let msg = ChatCompletionRequestUserMessageArgs::default()
        .content(text.as_ref())
        .build()?
        .into();
    Ok(ChatMessage::new(msg))
}

pub fn agent_msg(text: &String) -> Result<ChatMessage> {
    let msg = ChatCompletionRequestAssistantMessageArgs::default()
        .content(text)
        .build()?
        .into();
    Ok(ChatMessage::new(msg))
}

pub fn fn_call_msg(fn_name: &String, args_json: &String) -> Result<ChatMessage> {
    let msg = ChatCompletionRequestAssistantMessageArgs::default()
        .function_call(FunctionCall {
            name: fn_name.to_string(),
            arguments: args_json.to_string(),
        })
        .build()?
        .into();
    Ok(ChatMessage::new(msg))
}

pub fn fn_result_msg(fn_name: &String, result: &String) -> Result<ChatMessage> {
    let msg = ChatCompletionRequestFunctionMessageArgs::default()
        .name(fn_name)
        .content(result)
        .build()?
        .into();
    Ok(ChatMessage::new(msg))
}

impl ChatLog {
    pub fn new(username: String, session_id: usize) -> Self {
        let path = format!("data/{}/sessions/{}.json", username, session_id);
        if Path::new(&path).exists() {
            println!("Found json file, loading chat log from {}", path);
            let data = fs::read_to_string(&path).unwrap();
            let messages: Vec<ChatMessage> = serde_json::from_str::<Vec<HashMap<String, String>>>(&data)
                .unwrap()
                .into_iter()
                .map(|serialized_msg| {
                    println!("A");
                    let message = deserialize_message(serialized_msg.into_iter().map(|(k, v)| (k, Value::String(v))).collect()).unwrap();
                    println!("B");
                    let length = serialized_msg.len(); // This is a simplification, actual token length should be calculated
                    println!("C");
                    ChatMessage::new_with_len(message, length)
                })
                .collect();
            Self {
                username,
                session_id,
                messages,
            }
        } else {
            println!("Did not find json file, creating dir and returning empty log.");
            fs::create_dir_all(format!("data/{}/sessions", username)).unwrap();
            Self {
                username,
                session_id,
                messages: Vec::<ChatMessage>::new(),
            }
        }
    }

    pub fn add(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
        self.save();
    }

    pub fn change_sys_msg(&mut self, msg: ChatMessage) {
        if self.messages.len() > 0 {
            println!("change sys message at 0");
            self.messages[0] = msg;
        } else {
            println!("change sys message, push");
            self.messages.push(msg);
        }
        self.save();
    }

    fn save(&self) {
        let path = format!("data/{}/sessions/{}.json", self.username, self.session_id);
        let mut outjson = s!("[\n");
        let mut i: u32 = 0;
        for msg in &self.messages {
            if i > 0 { outjson.push_str(",\n") }
            let json = serialize_message(&msg.message);
            outjson.push_str(&json);
            i += 1;
        }
        outjson.push_str("]");
        fs::write(&path, outjson).unwrap();
        
        println!("Saved chat log to {}", path);
    }

    pub fn to_request_msgs(&mut self, model: &str) -> Result<Vec<ChatCompletionRequestMessage>> {
        let max_tokens = match model {
            "gpt-3.5-turbo" => 3000,
            _other => 7000,
        };
        let mut msgs: Vec<ChatCompletionRequestMessage> = vec![self.messages[0].message.clone()];
        let mut tokens = self.messages[0].length;
        let mut i: usize = 0;

        println!("to request messages len: {}", self.messages.len());

        for msg in self.messages.iter().rev() {
            tokens += msg.length;
            if tokens <= max_tokens && i < (self.messages.len() - 1) {
                println!("to request msgs adding message");
                msgs.insert(1, msg.message.clone());
            } else {
                println!("to request msgs break");
                break;
            }
            i += 1;
        }
        //println!("{}({} of {} max tokens)", style::Reset,tokens, max_tokens);
        Ok(msgs)
    }
}

pub fn init() {
    if !(BPE.get().is_some()) {
        println!("chat_log: initializing tokenizer..");
        BPE.set(cl100k_base().unwrap()).unwrap();
    }
}

/*
    pub fn trimmed(&mut self, max_tokens: i64) -> Vec<ChatCompletionRequestMessage> {

    }

    pub fn add_function_call(&mut self, name: String, args: serde_json::Value) {

    }
*/
