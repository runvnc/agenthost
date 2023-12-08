use serde_json::json;

use crate::s;

pub fn serialize_message(message: &ChatCompletionRequestMessage) -> String {
    match message {
        ChatCompletionRequestMessage::User(user_msg) => {
            let name = s!("test");// user_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", user_msg.role).to_lowercase();
            let content = user_msg.content.clone().unwrap_or(async_openai::types::ChatCompletionRequestUserMessageContent::Text("".to_string()));
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
        ChatCompletionRequestMessage::System(system_msg) => {
            let name = s!("test");// system_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", system_msg.role).to_lowercase();
            let content = system_msg.content.clone().unwrap_or("".to_string());
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
        ChatCompletionRequestMessage::Assistant(assistant_msg) => {
            let name = s!("test");// assistant_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", assistant_msg.role).to_lowercase();
            let content = assistant_msg.content.clone().unwrap_or("".to_string());
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
        ChatCompletionRequestMessage::Tool(_) | ChatCompletionRequestMessage::Function(_) => {
            s!(json!({ "name": s!(""), "role": s!("system"), "content":s!("")}))
        }
    }
}


use std::collections::HashMap;
use serde_json::{Value, from_value};

pub fn deserialize_message(json_value: HashMap<String, Value>) -> Result<ChatCompletionRequestMessage, serde_json::Error> {
    let name = s!("test");// json_value.get("name").and_then(Value::as_str).unwrap_or_default().to_string();
    let role = json_value.get("role").and_then(Value::as_str).unwrap_or_default().to_string();
    let content = json_value.get("content").and_then(Value::as_str).unwrap_or_default().to_string();
    let role_enum = from_value(json_value.get("role").cloned().unwrap_or(Value::Null))?;
    println!("6");
    match role_enum {
        Role::User => Ok(ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            name: Some(name),
            role: role_enum,
            content: Some(async_openai::types::ChatCompletionRequestUserMessageContent::Text(content)),
        })),
        Role::System => Ok(ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            name: Some(name),
            role: role_enum,
            content: Some(content),
        })),
        Role::Assistant => Ok(ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage {
            name: Some(name),
            role: role_enum,
            content: Some(content),
            function_call: None,
            tool_calls: None,
        })),
        Role::Tool | Role::Function => {
            unimplemented!()
        }
    }
}


// Remove the test code or move it inside a function if it was intended for testing purposes.


use crate::chatlog::{ChatCompletionRequestMessage, Role};
use async_openai::types::{ChatCompletionRequestUserMessage, ChatCompletionRequestSystemMessage, ChatCompletionRequestAssistantMessage};
