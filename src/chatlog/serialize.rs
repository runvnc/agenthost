use serde_json::json;

pub fn serialize_message(message: &ChatCompletionRequestMessage) -> String {
    match message {
        ChatCompletionRequestMessage::User(user_msg) => {
            let name = user_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", user_msg.role);
            let content = user_msg.content.clone().unwrap_or("".to_string());
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
        ChatCompletionRequestMessage::System(system_msg) => {
            let name = system_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", system_msg.role);
            let content = system_msg.content.clone().unwrap_or_default();
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
        ChatCompletionRequestMessage::Assistant(assistant_msg) => {
            let name = assistant_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", assistant_msg.role);
            let content = assistant_msg.content.clone().unwrap_or_default();
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
    }
}


use serde_json::{Value, from_value};

pub fn deserialize_message(json_str: &str) -> Result<ChatCompletionRequestMessage, serde_json::Error> {
    let json_value: Value = serde_json::from_str(json_str)?;
    let name = json_value["name"].as_str().unwrap_or_default().to_string();
    let role = json_value["role"].as_str().unwrap_or_default().to_string();
    let content = json_value["content"].as_str().unwrap_or_default().to_string();
    let role_enum = from_value(json_value["role"].clone())?;

    match role_enum {
        Role::User => Ok(ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            name: Some(name),
            role: role_enum,
            content: Some(content),
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
        })),
        // ... handle other roles
    }
}


// Remove the test code or move it inside a function if it was intended for testing purposes.


use crate::chatlog::{ChatCompletionRequestMessage, Role};
use async_openai::types::{ChatCompletionRequestUserMessage, ChatCompletionRequestSystemMessage, ChatCompletionRequestAssistantMessage};
