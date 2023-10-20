use smartstring::alias::String as SmartString;

use std::error::Error;
use anyhow::{Result, anyhow};

mod shorthands;
mod chatlog;
mod scripts;
mod openai_chat;

use chatlog::{ChatLog, sys_msg, user_msg, agent_msg};
use rhai::{format_map_as_json};
use scripts::{init, print_scope_ex, get_actions, call_function};

use openai_chat::{OpenAIChat, chat_fn};
use serde_json::{json, Value};

use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::ChatCompletionFunctions;

use shorthands::*;

fn test_args() -> serde_json::Value {
    json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA",
                    },
                    "unit": { "type": "string", "enum": ["celsius", "fahrenheit"] },
                },
                "required": ["location"],
    })
}

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
#[tokio::main]
async fn main() -> Result<()> {
    println!("AgentHost 0.1 Startup..");
    chatlog::init();

    let mut log = ChatLog::new();
    let mut chat = OpenAIChat::new(s!("gpt-3.5-turbo"));
    
    log.add(sys_msg(s!("You are a dungeon master."))?);
    log.add(user_msg(s!("Hello. Please roll a d8."))?);

    let mut functions = Vec::<ChatCompletionFunctions>::new();

    let mut handler = scripts::init("script.rhai")?;
    print_scope_ex(&handler.scope);
    let actions = get_actions(&mut handler)?;
    println!("Actions found in script: {}", 
             format_map_as_json(&actions));
    
    for (fn_name, info) in &actions {
        let info_map = dyn_map!(info, "")?;
        let description = dyn_str!(info_map, "description")?;
        let info_json = json!(&info_map);
        println!("descr={} json={}", description, info_json);
        functions.push(chat_fn(fn_name.to_string(), description, info_json)?); 
        println!("Found function: {}", fn_name);
    }
    println!("test_args: {}", test_args());
    functions.push(chat_fn(s!("get_weather"), s!("Get the current weather"), test_args())?);

    let (fn_name, fn_args) = chat.send_request(log.to_request_msgs()?, functions).await?;

    println!("Function name: {}", fn_name);
    println!("Function arguments: {}", fn_args);
 
    //call_function(&mut handler, "rollDice", "{ \"sides\": 20 }"); 

    Ok(())
}

