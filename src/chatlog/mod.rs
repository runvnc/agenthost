
use once_cell::sync::OnceCell;
use anyhow::{Result};
use termion::{style};


use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        ChatCompletionRequestMessage,
        FunctionCall,
        Role,
    }
};


use tiktoken_rs::{CoreBPE, cl100k_base};

static BPE: OnceCell<CoreBPE> = OnceCell::new();

//mod shorthands;



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

pub fn sys_msg(text: &String) -> Result<ChatMessage> {
    let msg = ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content(text.as_str())
        .build()?;
    Ok( ChatMessage::new(msg) )
}

pub fn user_msg(text: &String) -> Result<ChatMessage> {
   let msg = ChatCompletionRequestMessageArgs::default()
        .role(Role::User)
        .content(text)
        .build()?;
   Ok( ChatMessage::new(msg) )
}

pub fn agent_msg(text: &String) -> Result<ChatMessage> {
   let msg = ChatCompletionRequestMessageArgs::default()
            .role(Role::Assistant)
            .content(text)
            .build()?;
   Ok( ChatMessage::new(msg) )
}

pub fn fn_call_msg(fn_name: &String, args_json:&String) -> Result<ChatMessage> {
   let msg = ChatCompletionRequestMessageArgs::default()
            .role(Role::Assistant)
            .function_call(
                FunctionCall{ 
                    name: fn_name.to_string(),
                    arguments: args_json.to_string()
                }
            ).build()?;
   Ok( ChatMessage::new(msg) )
}

pub fn fn_result_msg(fn_name: &String, result:&String) -> Result<ChatMessage> {
   let msg = ChatCompletionRequestMessageArgs::default()
            .role(Role::Function)
            .name(fn_name)
            .content(result)
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

    pub fn change_sys_msg(&mut self, msg: ChatMessage) {
        if self.messages.len() > 0 {
            self.messages[0] = msg;
        } else {
            self.messages.push(msg);
        }
        let content: String = self.messages[0].message.content.as_ref().expect("The value is None").to_string();
        println!("System message changed to:\n{}", content);
    }

    pub fn to_request_msgs(&mut self, model: &str) -> Result<Vec<ChatCompletionRequestMessage>> {
        let _i:i32 = 0;
        let max_tokens = match model {
            "gpt-3.5-turbo" => 3000,
            _other           => 7000
        };
        let mut msgs: Vec<ChatCompletionRequestMessage> = vec![ self.messages[0].message.clone() ];
        let mut tokens = self.messages[0].length;

        for msg in self.messages.iter().rev() {
           tokens += msg.length; 
           if tokens <= max_tokens {
                msgs.insert(1, msg.message.clone());
           } else {
                break;
           }
        };
        //println!("{}({} of {} max tokens)", style::Reset,tokens, max_tokens);
        Ok( msgs )
    }
}

pub fn init() {
    //println!("chat_log: initializing tokenizer..");
    BPE.set(cl100k_base().unwrap()).unwrap();
}

/*
    pub fn trimmed(&mut self, max_tokens: i64) -> Vec<ChatCompletionRequestMessage> {
        
    }

    pub fn add_function_call(&mut self, name: String, args: serde_json::Value) {

    }
*/
