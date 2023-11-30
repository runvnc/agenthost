use async_openai::types::{ChatCompletionRequestMessage, FunctionCall, Role};

use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
};

use crate::s;

#[derive(Clone, Debug)]
pub enum ChatUIMessage {
    UserId(String),
    Reply {
        role: String,
        name: String,
        content: String,
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
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                content: user_message,
                role,
                ..
            }) => {
                match user_message {
                    Some(ChatCompletionRequestUserMessageContent::Text(text)) => {
                        ChatUIMessage::Reply {
                            role: role.to_string(),
                            name: "SYSTEM".to_string(), 
                            content: text,
                        }
                    }
                    _ => ChatUIMessage::Fragment("Unsupported User Message Content".to_string()),
                }
            }
            ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage {
                content: assistant_message,
                role,
                ..
            }) => {
                if let Some(content) = assistant_message {
                    ChatUIMessage::Reply {
                        role: role.to_string(),
                        name: "SYSTEM".to_string(),
                        content,
                    }
                } else {
                    ChatUIMessage::Fragment("Empty Assistant Message".to_string())
                }
            }
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                content: message,
                role,
                ..
            }) => {
                if let Some(content) = message {
                    ChatUIMessage::Reply {
                        role: s!(role),
                        name: s!("SYSTEM"),
                        content,
                    }
                } else {
                    ChatUIMessage::Fragment("Empty System Message".to_string())
                }
            }
            _ => ChatUIMessage::Fragment("Unsupported message type".to_string())
        }
    }
}
