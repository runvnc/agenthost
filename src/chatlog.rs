use std::error::Error;
use once_cell::sync::OnceCell;

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        ChatCompletionRequestMessage,
        Role,
    }
};

use tiktoken_rs::{CoreBPE, cl100k_base};

static BPE: OnceCell<CoreBPE> = OnceCell::new();

type Res<T> = Result<T, Box<dyn Error>>;

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


impl ChatLog {
    pub fn new() -> Self {
      Self { messages: Vec::<ChatMessage>::new() }
    }

    pub fn add_user_message(&mut self, text: String) -> Res<usize> {
       let msg = ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(text)
                .build()?;
       let chatmsg = ChatMessage::new(msg);
       let length = chatmsg.length;
       self.messages.push(chatmsg);
       Ok(length)
    }

    pub fn add_agent_message(&mut self, text: String) -> Res<()> {
       let msg = ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content(text)
                .build()?;
       self.messages.push(ChatMessage::new(msg));
       Ok(())
    }
}

pub fn init() {
    BPE.set(cl100k_base().unwrap()).unwrap();
}

/*
    pub fn trimmed(&mut self, max_tokens: i64) -> Vec<ChatCompletionRequestMessage> {
        
    }

    pub fn add_function_call(&mut self, name: String, args: serde_json::Value) {

    }
*/
