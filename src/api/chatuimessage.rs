use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs, FunctionCall, Role,
};


#[derive(Clone, Debug)]
pub enum ChatUIMessage {
    UserId(String),
    Reply {
        role: String,
        name: String,
        content: String
    },
    Fragment(String),
    FunctionCall {
        name: String,
        params: String,
        result: String,
    },
}


impl From<ChatCompletionRequestMessage> for ChatUIMessage {
    fn from(item: ChatCompletionRequestMessage) -> Self {
        match item {
            ChatCompletionRequestMessage::User(user_message) => {
                match user_message.content {
                    Some(ChatCompletionRequestUserMessageContent::Text(text)) => {
                        ChatUIMessage::Reply {
                            role: user_message.role.to_string(),
                            name: "User".to_string(), // Name is set as "User"
                            content: text,
                        }
                    },
                    _ => ChatUIMessage::Fragment("Unsupported User Message Content".to_string()),
                }
            },
            ChatCompletionRequestMessage::Assistant(assistant_message) => {
                if let Some(content) = assistant_message.content {
                    ChatUIMessage::Reply {
                        role: assistant_message.role.to_string(),
                        name: "Assistant".to_string(), // Name is set as "Assistant"
                        content,
                    }
                } else {
                    ChatUIMessage::Fragment("Empty Assistant Message".to_string())
                }
            },
            _ => ChatUIMessage::Fragment("Unsupported Message Type".to_string()),
        }
    }
}
