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

use crate::{s, json_str, dyn_str, dyn_map};

pub struct Agent {
    functions: Vec::<ChatCompletionFunctions>,
    log: ChatLog,
    model: String,
    chat: OpenAIChat,
    handler: Handler 
}

impl Agent {
    pub fn new(functions: Vec::<ChatCompletionFunctions>, 
        log: ChatLog, chat:OpenAIChat,
        model: String,
        handler: Handler) -> Self {
        Self { functions, model, log, chat, handler }
    }

    pub fn startup(script_path: &str, 
               model: &str) -> Result<Self> {
        println!("AgentHost 0.1 Startup..");
        chatlog::init();

        let mut log = ChatLog::new();
        let chat = OpenAIChat::new(model.to_string());
        let mut handler = scripts::init(script_path)?;
       
        let mut functions = Vec::<ChatCompletionFunctions>::new();

        call_function(&mut handler, "expand_actions", "{}");
        let actions = get_actions(&mut handler)?;
        
        for (fn_name, info) in &actions {
            let info_map = dyn_map!(info, "")?;
            let description = dyn_str!(info_map, "description")?;
            let info_json = json!(&info_map);
            functions.push(chat_fn(fn_name.to_string(), description, info_json)?); 
        }
        Ok( Self::new(functions, log, chat, model.to_string(), handler) )
    }

    pub async fn next_stage(&mut self, stage: &String) -> Result<()> {
        goto_stage(&mut self.handler, stage);
        let mut functions = Vec::<ChatCompletionFunctions>::new();

        call_function(&mut self.handler, "expand_actions", "{}");
        let actions = get_actions(&mut self.handler)?;
        
        for (fn_name, info) in &actions {
            let info_map = dyn_map!(info, "")?;
            let description = dyn_str!(info_map, "description")?;
            let info_json = json!(&info_map);
            functions.push(chat_fn(fn_name.to_string(), description, info_json)?); 
        }
        self.functions = functions;

        Ok(())
    }

    pub async fn run(&mut self, mut user_input:bool) -> Result<()> {
        let mut input = String::new();

        loop {
            if user_input {
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                self.log.add(user_msg(&input)?);
            }
            user_input = true;
            println!();

            let mut data:HashMap<String, String> = HashMap::new();
            let timestamp_str = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();
            data.insert("timestamp".into(), timestamp_str);
            let json = json!(data);

            let json_string = &json.to_string();
            let sys_str = call_function(&mut self.handler, "renderSysMsg", json_string)?;
            let sys_str_ = json_str!(sys_str);

            self.log.change_sys_msg(sys_msg(&sys_str_)?);
     
            let msgs = self.log.to_request_msgs(self.model.as_str())?;
            print!("");

            let (text, fn_name, fn_args) = self.chat.send_request(
                msgs.clone(), 
                self.functions.clone()
            ).await?;

            print!("");

            if fn_name != "" { 
                self.log.add(fn_call_msg(&fn_name, &fn_args)?);
                
                let output = call_function(&mut self.handler, 
                                           fn_name.as_str(),
                                           fn_args.as_str())?; 
                self.log.add(fn_result_msg(&fn_name, &output)?);

                let next_step_ = call_function( &mut self.handler, "evalExitStage", "{}" )?;
                let next_step = json_str!(next_step_);
                if next_step.contains("Function not found") {
                } else {
                    if next_step != "" && next_step != "()" {
                        self.next_stage(&next_step).await?;
                     }
                }
                user_input = false;
            } else {
                self.log.add(agent_msg(&text)?);
                println!();
            }
            input.clear();
        }
    }
}

use std::io::{self, Write};

use termion::{color, style};
