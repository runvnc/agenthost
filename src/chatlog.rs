//use smartstring::alias::String;

use std::error::Error;
use once_cell::sync::OnceCell;
use anyhow::{Result,anyhow};


use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        ChatCompletionRequestMessage,
        Role,
    }
};


use tiktoken_rs::{CoreBPE, cl100k_base};

static BPE: OnceCell<CoreBPE> = OnceCell::new();

//mod shorthands;
use crate::shorthands::*;


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
        let tokens = BPE.get().expect("No tokenizer").encode_with_special_tokens(
            &(self.message.content.as_deref().unwrap_or(""))
        );

        self.length = tokens.len()
    }
}

pub struct ChatLog {
    messages: Vec<ChatMessage>,
}

pub fn sys_msg(text: String) -> Result<ChatMessage> {
    let msg = ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content(text.as_str())
        .build()?;
    Ok( ChatMessage::new(msg) )
}

pub fn user_msg(text: String) -> Result<ChatMessage> {
   let msg = ChatCompletionRequestMessageArgs::default()
        .role(Role::User)
        .content(text)
        .build()?;
   Ok( ChatMessage::new(msg) )
}

pub fn agent_msg(text: String) -> Result<ChatMessage> {
   let msg = ChatCompletionRequestMessageArgs::default()
            .role(Role::Assistant)
            .content(text)
            .build()?;
   Ok( ChatMessage::new(msg) )
}

impl ChatLog {
    pub fn new() -> Self {
      Self { messages: Vec::<ChatMessage>::new() }
    }

    pub fn add(&mut self, msg: ChatMessage) {
        self.messages.push(msg)
    }

    pub fn to_request_msgs(&mut self) -> Result<Vec<ChatCompletionRequestMessage>> {
       Ok( self.messages.iter()
           .map(|msg| msg.message.clone())
           .collect::<Vec<_>>() 
        )
    }
}

pub fn init() {
    println!("chat_log: initializing tokenizer..");
    BPE.set(cl100k_base().unwrap()).unwrap();
}

/*
    pub fn trimmed(&mut self, max_tokens: i64) -> Vec<ChatCompletionRequestMessage> {
        
    }

    pub fn add_function_call(&mut self, name: String, args: serde_json::Value) {

    }
*/
