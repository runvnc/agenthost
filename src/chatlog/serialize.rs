use serde_json::json;

fn serialize_message(message: &ChatCompletionRequestMessage) -> String {
    match message {
        ChatCompletionRequestMessage::User(user_msg) => {
            let name = user_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", user_msg.role);
            let content = user_msg.content.clone().unwrap_or_default();
            (name, role, content)
        },
        ChatCompletionRequestMessage::System(system_msg) => {
            let name = system_msg.name.clone().unwrap_or_default();
            let role = format!("{:?}", system_msg.role);
            let content = system_msg.content.clone().unwrap_or_default();
            (name, role, content)
        },
    }
}


fn deserialize_message(name: String, role: String, content: String) -> ChatCompletionRequestMessage {
    let role_enum = match role.as_str() {
        "User" => Role::User,
        "System" => Role::System,
        // ... handle other roles
        _ => Role::User, // Default role
    };

    if role_enum == Role::User {
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            name: Some(name),
            role: role_enum,
            content: Some(content),
        })
    } else {
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            name: Some(name),
            role: role_enum,
            content: Some(content),
        })
    }
}


let msg = ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
    content: Some("Hello, World!".to_string()),
    role: Role::User,
    name: Some("Alice".to_string()),
});

let serialized = serialize_message(&msg);
let deserialized = deserialize_message(serialized.0, serialized.1, serialized.2);


