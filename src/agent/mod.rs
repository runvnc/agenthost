use smartstring::alias::String as SmartString;

use std::error::Error;
use anyhow::{Result, anyhow};

use crate::chatlog;
use crate::chatlog::{ChatLog, sys_msg, user_msg, agent_msg, fn_call_msg, fn_result_msg};
use rhai::{format_map_as_json};
use crate::scripts::{init, Handler, print_scope_ex, get_actions, call_function};

use crate::scripts;

use crate::openai_chat::{OpenAIChat, chat_fn};
use serde_json::{json, Value};

use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::ChatCompletionFunctions;

use crate::shorthands::*;
use crate::{s, dyn_str, dyn_map};

pub struct Agent {
    functions: Vec::<ChatCompletionFunctions>,
    log: ChatLog,
    model: &String,
    chat: OpenAIChat,
    handler: Handler 
}

impl Agent {
    pub fn new(functions: Vec::<ChatCompletionFunctions>, 
        log: ChatLog, chat:OpenAIChat,
        model: &String,
        handler: Handler) -> Self {
        Self { functions, model, log, chat, handler }
    }
}

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
pub fn startup(sys: &String, script_path: &str, 
               model: &str) -> Result<Agent> {
    println!("AgentHost 0.1 Startup..");
    chatlog::init();

    let mut log = ChatLog::new();
    let mut chat = OpenAIChat::new(model.to_string());
    
    log.add(sys_msg(&sys)?);
    let mut functions = Vec::<ChatCompletionFunctions>::new();

    let mut handler = scripts::init(script_path)?;

    call_function(&mut handler, "expand_actions", "{}");
    let actions = get_actions(&mut handler)?;
    
    for (fn_name, info) in &actions {
        let info_map = dyn_map!(info, "")?;
        let description = dyn_str!(info_map, "description")?;
        let info_json = json!(&info_map);
        println!("descr={} json={}", description, info_json);
        functions.push(chat_fn(fn_name.to_string(), description, info_json)?); 
        println!("Found function: {}", fn_name);
    }
    Ok( Agent::new(functions, log, chat, model, handler) )
}


use std::io::{self, Write};
use tokio::io::AsyncReadExt;
use termion::{color, style};


pub async fn run(agent: &mut Agent) -> Result<()> {
    println!("Run agent..");

    let mut input = String::new();
    let mut user_input = true;

    loop {
        if user_input {
            print!("{}> {}", color::Fg(color::LightCyan), color::Fg(color::Yellow));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            agent.log.add(user_msg(&input)?);
        }
        user_input = true;

        let msgs = agent.log.to_request_msgs(agent.model.as_str())?;
        println!("{}", color::Fg(color::White));

        let (text, fn_name, fn_args) = agent.chat.send_request(
            msgs.clone(), 
            agent.functions.clone()
        ).await?;

        println!("{}", style::Reset);

        if fn_name != "" { 
            agent.log.add(fn_call_msg(&fn_name, &fn_args)?);
            
            let output = call_function(&mut agent.handler, 
                                       fn_name.as_str(),
                                       fn_args.as_str())?; 
            println!("Call result: {}", output);
            agent.log.add(fn_result_msg(&fn_name, &output)?);
            user_input = false;
        } else {
            agent.log.add(agent_msg(&text)?);
            println!();
        }
        input.clear();
    }
}

