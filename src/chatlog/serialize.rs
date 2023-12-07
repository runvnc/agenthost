use serde_json::json;

use serde_json::json;

fn serialize_message(message: &ChatCompletionRequestMessage) -> String {
    match message {
        ChatCompletionRequestMessage::User(user_msg) => {
            let name = user_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", user_msg.role);
            let content = user_msg.content.clone().unwrap_or_default();
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
        ChatCompletionRequestMessage::System(system_msg) => {
            let name = system_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", system_msg.role);
            let content = system_msg.content.clone().unwrap_or_default();
            json!({ "name": name, "role": role, "content": content }).to_string()
        },
    }
}


use serde_json::{Value, from_value};

fn deserialize_message(json_str: &str) -> Result<ChatCompletionRequestMessage, serde_json::Error> {
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
        // ... handle other roles
    }
}


let msg = ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
    content: Some("Hello, World!".to_string()),
    role: Role::User,
    name: Some("Alice".to_string()),
});

let serialized = serialize_message(&msg);
let deserialized = deserialize_message(serialized.0, serialized.1, serialized.2);


