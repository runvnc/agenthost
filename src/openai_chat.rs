use std::error::Error;
use std::io::{stdout, Write};

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs,
        ChatCompletionFunctions,
        ChatCompletionFunctionsArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

use async_openai::config::OpenAIConfig;
use futures::StreamExt;

pub struct OpenAIChat {
    client: Client<OpenAIConfig>,
    functions: Vec<ChatCompletionFunctions>,
}

impl OpenAIChat {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            functions: Vec::<ChatCompletionFunctions>::new(),
        }
    }

    pub fn add_function(&mut self, name: String, descr: String, args: serde_json::Value) {
        let fnargs = ChatCompletionFunctionsArgs::default()
            .name(name)
            .description(descr)
            .parameters(args)
            .build().unwrap();
        self.functions.push(fnargs);
    }


    pub async fn create_chat_request(&self, user_message: &str) -> Result<(String, String), Box<dyn Error>> {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo-0613")
            .messages([ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user_message)
                .build()?])
            .functions(self.functions.clone())
            .function_call("auto")
            .build()?;

        let mut stream = self.client.chat().create_stream(request).await?;

        let mut fn_name = String::new();
        let mut fn_args = String::new();

        let mut lock = stdout().lock();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    for chat_choice in response.choices {
                        if let Some(fn_call) = &chat_choice.delta.function_call {
                            writeln!(lock, "function_call: {:?}", fn_call).unwrap();
                            if let Some(name) = &fn_call.name {
                                fn_name = name.clone();
                            }
                            if let Some(args) = &fn_call.arguments {
                                fn_args.push_str(args);
                            }
                        }
                        if let Some(finish_reason) = &chat_choice.finish_reason {
                            if finish_reason == "function_call" {
                                return Ok((fn_name, fn_args));
                            }
                        } else if let Some(content) = &chat_choice.delta.content {
                            write!(lock, "{}", content).unwrap();
                        }
                    }
                }
                Err(err) => {
                    writeln!(lock, "error: {err}").unwrap();
                }
            }
            stdout().flush()?;
        }

        Err("No function call found".into())
    }
}
