use std::error::Error;

mod chatlog;
use chatlog::{ChatLog};

mod scripts;
use scripts::{init, call_function};

mod openai_chat;
use openai_chat::OpenAIChat;
use serde_json::json;

fn test_args() -> serde_json::json {
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
    });
}

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("AgentHost 0.1 Startup..");
    chatlog::init();
    let mut chat = ChatLog::new();
    let msg_len = chat.add_user_message("hello, how are you?".to_string())?;
    println!("Message token length: {}", msg_len);

    let mut chat = OpenAIChat::new();
    
    Vec<Chat> chatlog::sys_msg("You are a dungeon master.");
     
    let (fn_name, fn_args) = chat.send_request("What's the weather like in Boston?").await?;

    println!("Function name: {}", fn_name);
    println!("Function arguments: {}", fn_args);

 
    let mut handler = scripts::init("script.rhai")?;
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
