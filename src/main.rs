use std::error::Error;

mod chatlog;
use chatlog::init;

/*
mod openai_chat;
use openai_chat::OpenAIChat;
use serde_json::json;
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init();
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

fn get_current_weather(location: &str, unit: &str) -> serde_json::Value {
    let weather_info = serde_json::json!({
            "location": location,
            "temperature": "72",
            "unit": unit,
            "forecast": ["sunny", "windy"]
    });

    weather_info

} */
