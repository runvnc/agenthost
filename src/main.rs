use std::error::Error;

mod shorthands;
mod chatlog;
mod scripts;
mod openai_chat;

use chatlog::{ChatLog, sys_msg, user_msg, agent_msg};

use scripts::{init, print_scope_ex, get_actions, call_function};

use openai_chat::{OpenAIChat, chat_fn};
use serde_json::json;

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
async fn main() -> Res<()> {
    println!("AgentHost 0.1 Startup..");
    chatlog::init();

    let mut log = ChatLog::new();
    let mut chat = OpenAIChat::new(s!("gpt-3.5-turbo"));
    
    log.add(sys_msg(s!("You are an AI assistant."))?);
    log.add(user_msg(s!("Hello. What's the weather like in Boston?"))?);

    let mut functions = Vec::<ChatCompletionFunctions>::new();
    functions.push(chat_fn(s!("get_weather"), s!("Get weather report"), test_args())?);
     
    let (fn_name, fn_args) = chat.send_request(log.to_request_msgs()?, functions).await?;

    println!("Function name: {}", fn_name);
    println!("Function arguments: {}", fn_args);
 
    let mut handler = scripts::init("script.rhai")?;
    print_scope_ex(&handler.scope);
    let actions = get_actions(&mut handler)?;
    println!("Actions found in script: {}", actions);

    call_function(&mut handler, "rollDice", "{ \"sides\": 20 }"); 

    Ok(())
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut chat = OpenAIChat::new();

    let current_weather_args = json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA",
                    },
                    "unit": { "type": "string", "enum": ["celsius", "fahrenheit"] },
                },
                "required": ["location"],
            });

    chat.add_function("get_current_weather".to_string(), 
                      "Get the current weather in a given location.".to_string(),
                      current_weather_args);

    let (fn_name, fn_args) = chat.create_chat_request("What's the weather like in Boston?").await?;

    println!("Function name: {}", fn_name);
    println!("Function arguments: {}", fn_args);

    Ok(())
}
*/

//fn get_current_weather(location: &str, unit: &str) -> serde_json::Value {
//    let weather_info = serde_json::json!({
//            "location": location,
//  }
