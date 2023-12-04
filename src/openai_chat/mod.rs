//use smartstring::alias::String;

use anyhow::Result;
use std::io::{stdout, Write};
use tokio_util::sync::CancellationToken;

use async_openai::error::OpenAIError;
use async_openai::types::ChatCompletionRequestUserMessageArgs;
use async_openai::{
    types::{
        ChatCompletionFunctions, ChatCompletionFunctionsArgs, ChatCompletionRequestMessage,
        CreateChatCompletionRequestArgs, FinishReason,
    },
    Client,
};

use async_openai::config::OpenAIConfig;
use futures::StreamExt;

use std::error::Error;

use flume::*;

use crate::s;

use crate::api::chatuimessage::ChatUIMessage;

pub struct OpenAIChat {
    model: String,
    client: Client<OpenAIConfig>,
}

pub fn chat_fn(
    func_name: String,
    descr: String,
    params: serde_json::Value,
) -> Result<ChatCompletionFunctions> {
    Ok(ChatCompletionFunctionsArgs::default()
        .name(func_name)
        .description(descr)
        .parameters(params)
        .build()?)
}

impl OpenAIChat {
    pub fn new(model: String) -> Self {
        Self {
            model,
            client: Client::new(),
        }
    }

    pub async fn send_request(
        &self,
        messages: Vec<ChatCompletionRequestMessage>,
        functions: Vec<ChatCompletionFunctions>,
        reply_sender: flume::Sender<ChatUIMessage>,
        token: CancellationToken,
    ) -> Result<(String, String, String)> {
        /*        let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(512u16)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content("Write a marketing blog praising and introducing Rust library async-openai")
            .build()?
            .into()])
        .build()?; */
        let request = CreateChatCompletionRequestArgs::default()
            .model(&*self.model)
            .max_tokens(512u16)
            .messages(messages)
            .functions(functions)
            .function_call("auto")
            .build()?;

        println!("Request data: {:?}", request);
        let mut stream = self.client.chat().create_stream(request).await?;

        let mut fn_name = String::new();
        let mut fn_args = String::new();
        let mut text = String::new();

        //let mut lock = stdout().lock();
        while let Some(result) = stream.next().await {
            if token.is_cancelled() {
                println!("Cancellation received. Stopping request.");
                break;
            }
            match result {
                Ok(response) => {
                    for chat_choice in response.choices {
                        if let Some(fn_call) = &chat_choice.delta.function_call {
                            if let Some(name) = &fn_call.name {
                                fn_name = name.clone();
                                //write!(lock, "{}", name).unwrap();
                            }
                            if let Some(args) = &fn_call.arguments {
                                fn_args.push_str(args);
                                //write!(lock, "{}", args).unwrap();
                            }
                        }
                        if let Some(finish_reason) = &chat_choice.finish_reason {
                            if *finish_reason == FinishReason::FunctionCall {
                                return Ok((text, fn_name, fn_args));
                            }
                        } else if let Some(content) = &chat_choice.delta.content {
                            text.push_str(content);
                            print!("<{}>", content);
                            reply_sender
                                .send_async(ChatUIMessage::Fragment(format!("*{}*", content)))
                                .await?;
                        }
                    }
                }

                Err(error) => match error {
                    OpenAIError::ApiError(api_error) => {
                        println!("API Error: {:?}", api_error);
                    }
                    OpenAIError::Reqwest(er) => {
                        println!("Reqwest error: {:?}", er);
                    }
                    OpenAIError::StreamError(msg) => {
                        println!("Stream Error: {:?}", msg);
                    }
                    _ => {
                        println!("Other OpenAI Error: {:?}", error);
                    }
                },
            }
            stdout().flush()?;
        }

        Ok((text, s!(""), s!("")))
    }
}
