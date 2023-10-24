#![allow(warnings)]
use anyhow::{Result, anyhow};

use crate::chatlog;
use crate::chatlog::{ChatLog, sys_msg, user_msg, agent_msg, fn_call_msg, fn_result_msg};

use crate::scripts::{Handler, get_actions, goto_stage, call_function};

use crate::scripts;

use crate::openai_chat::{OpenAIChat, chat_fn};
use serde_json::{json};
use chrono::{Utc, DateTime};

use async_openai::types::ChatCompletionFunctions;
use std::collections::HashMap;
use serde_json::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::connector::*;

use tokio::sync::mpsc;

use crate::{s, json_str, dyn_str, dyn_map};

pub enum AgentMessage {
    Fragment(String),
    Complete(String),
    FunctionCall { 
        name: String, 
        params: Vec<String>, 
        result: String 
    }
}


pub struct Agent {
    functions: Vec::<ChatCompletionFunctions>,
    log: ChatLog,
    model: String,
    interrupt_receiver: Arc<Mutex<mpsc::Receiver<()>>>, 
    chat: OpenAIChat,
    handler: Handler 
}

impl Agent {
    pub fn new(script_path: String, 
              interrupt_receiver: Arc<Mutex<mpsc::Receiver<()>>>) -> Result<Self> {
        println!("AgentHost 0.1 Startup..");
        chatlog::init();
        let model = "gpt-4".to_string();
        let mut log = ChatLog::new();
        let chat = OpenAIChat::new(model);
        let mut handler = scripts::init(&script_path)?;
    
        let mut instance = Self{ functions: Vec::<ChatCompletionFunctions>::new(),
                              model, 
                              interrupt_receiver, log, chat, handler };

        instance.functions = instance.load_actions()?;

        Ok( instance )
    }

    fn load_actions(&mut self) -> Result<Vec::<ChatCompletionFunctions>> {
        let mut functions = Vec::<ChatCompletionFunctions>::new();

        call_function(&mut self.handler, "expand_actions", "{}");
        let actions = get_actions(&mut self.handler)?;
        
        for (fn_name, info) in &actions {
            let info_map = dyn_map!(info, "")?;
            let description = dyn_str!(info_map, "description")?;
            let info_json = json!(&info_map);
            functions.push(chat_fn(fn_name.to_string(), description, info_json)?); 
        }
        Ok( functions )
    }

    pub async fn next_stage(&mut self, stage: &String) -> Result<()> {
        goto_stage(&mut self.handler, stage);
        self.functions = self.load_actions()?;
        Ok( () )
    }

    pub fn call_ret_string(&mut self, fn_name: &str, args_json: &str) -> Result<String> {
        let res_json = call_function(&mut self.handler, fn_name, args_json)?;
        let str_ = json_str!(res_json);
        Ok( str_ )
    }

    pub fn call(&mut self, fn_name: &str, args_json: &str) -> Result<String> {
        call_function(&mut self.handler, fn_name, args_json)
    }

    pub async fn process_fn_call(&mut self, fn_name: &str, fn_args: &str) -> Result<()> {
        self.log.add(fn_call_msg(&fn_name.to_string(), &fn_args.to_string())?);
                    
        let output = self.call(fn_name, fn_args)?; 
        self.log.add(fn_result_msg(&fn_name.to_string(), &output.to_string())?);

        let next_step = self.call_ret_string("evalExitStage", "{}" )?;
        if next_step.contains("Function not found") {
        } else {
            if next_step != "" && next_step != "()" {
                self.next_stage(&next_step).await?;
             }
        };

        Ok( () )
    }

    pub fn update_sys_msg(&mut self) -> Result<()> {
        let mut data:HashMap<String, String> = HashMap::new();

        let timestamp_str = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();
        data.insert("timestamp".into(), timestamp_str);
 
        let json = json!(data);
        let json_string = &json.to_string();

        let sys_str = self.call_ret_string("renderSysMsg", json_string)?;

        self.log.change_sys_msg(sys_msg(&sys_str)?);

        Ok( () )
    }

    pub async fn run_some(&mut self, input: Option<&str>, 
                          connector: &Connector) -> Result<()> {
        loop {
            if let Some(input_str) = input {
                self.log.add(user_msg(&input_str.to_string())?);
            }

            self.update_sys_msg();

            let msgs = self.log.to_request_msgs(self.model.as_str())?;

            let (text, fn_name, fn_args) = self.chat.send_request(
                msgs.clone(), 
                self.functions.clone()
            ).await?;

            if fn_name != "" {
                self.process_fn_call(&fn_name, &fn_args).await?;
            } else {
                self.log.add(agent_msg(&text)?);
                break;
            }
        }
        Ok( () )
    }
}

