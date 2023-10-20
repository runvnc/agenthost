use smartstring::alias::String as SmartString;

use std::error::Error;
use anyhow::{Result, anyhow};

use crate::chatlog;
use crate::chatlog::{ChatLog, sys_msg, user_msg, agent_msg};
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
    chat: OpenAIChat,
    handler: Handler 
}

impl Agent {
    pub fn new(functions: Vec::<ChatCompletionFunctions>, 
        log: ChatLog, chat:OpenAIChat, 
        handler: Handler) -> Self {
        Self { functions, log, chat, handler }
    }
}

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
pub fn startup() -> Result<Agent> {
    println!("AgentHost 0.1 Startup..");
    chatlog::init();

    let mut log = ChatLog::new();
    let mut chat = OpenAIChat::new(s!("gpt-3.5-turbo"));
    
    log.add(sys_msg(&s!("You are a dungeon master."))?);
    let mut functions = Vec::<ChatCompletionFunctions>::new();

    let mut handler = scripts::init("scripts/script.rhai")?;

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
    Ok( Agent::new(functions, log, chat, handler) )
}


use std::io::{self, Write};
use tokio::io::AsyncReadExt;

pub async fn run(agent: &mut Agent) -> Result<()> {
    println!("Run agent..");

    let mut input = String::new();
    let user_input = true;

    loop {
        if (user_input) {
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            agent.log.add(user_msg(&input)?);
        }
        user_input = true;

        let msgs = agent.log.to_request_msgs()?;

        let (text, fn_name, fn_args) = agent.chat.send_request(
            msgs.clone(), 
            agent.functions.clone()
        ).await?;

        println!();

        if fn_name != "" { 
            println!("Function name: {}", fn_name);
            println!("Function arguments: {}", fn_args);

            agent.log.add(fn_call_msg(fn_name, fn_args));
            
            let output = call_function(&mut agent.handler, 
                                       fn_name.as_str(),
                                       fn_args.as_str()); 
            agent.log.add(fn_result_msg(fn_name, output));
            user_input = false;
        } else {
            agent.log.add(agent_msg(text)?);
        }
        input.clear();
    }
}

