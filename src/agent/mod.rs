#![allow(warnings)]
use anyhow::{anyhow, Context, Result};

use crate::chatlog;
use crate::chatlog::{agent_msg, fn_call_msg, fn_result_msg, sys_msg, user_msg, ChatLog};

use crate::scripts::{call_function, get_actions, eval_expr, goto_stage, Handler};

use crate::scripts;

use crate::llamacppchat::llama_cpp_chat;

use crate::openai_chat::{chat_fn, OpenAIChat};
use chrono::{DateTime, Utc};
use serde_json::json;

use async_openai::types::{
    ChatCompletionFunctions, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage, Role,
};
use serde_json::Value;
use std::collections::HashMap;

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use flume::*;
use tokio::sync::mpsc;

use crate::{dyn_map, dyn_str, json_str, s};

use crate::api;
use crate::api::chatuimessage::ChatUIMessage;
use crate::api::chatuimessage::*;

//unsafe impl Send for Agent {}

pub enum AgentMessage {
    Fragment(String),
    Complete(String),
    FunctionCall {
        name: String,
        params: Vec<String>,
        result: String,
    },
}

pub struct Agent {
    username: String,
    functions: Vec<ChatCompletionFunctions>,
    session_id: usize,
    log: ChatLog,
    //model: String,
    handler: Handler,
    receiver: flume::Receiver<String>,
    reply_sender: flume::Sender<ChatUIMessage>,
}

impl Agent {
    pub async fn new(
        username: String,
        session_id: usize,
        script_path: String,
        receiver: flume::Receiver<String>,
        reply_sender: flume::Sender<ChatUIMessage>,
    ) -> Result<Self> {
        println!("AgentHost 0.1 Startup agent..");
        chatlog::init();
        //let model = s!("gpt-3.5-turbo");
        //let model = s!("gpt-4-1106-preview");
        let mut log = ChatLog::new(username.clone(), session_id);
        //let chat = OpenAIChat::new(model.clone());
        let mut handler = scripts::init(&script_path, session_id, &username)?;

        let mut instance = Self {
            username,
            functions: Vec::<ChatCompletionFunctions>::new(),
            session_id,
            log,
            handler,
            receiver,
            reply_sender,
        };

        instance.functions = instance.load_actions()?;
        instance.update_sys_msg();

        Ok(instance)
    }

    fn load_actions(&mut self) -> Result<Vec<ChatCompletionFunctions>> {
        let mut functions = Vec::<ChatCompletionFunctions>::new();

        call_function(&mut self.handler, "expand_actions", "{}");
        let actions = get_actions(&mut self.handler)?;

        for (fn_name, info) in &actions {
            let info_map = dyn_map!(info, "")?;
            let description = dyn_str!(info_map, "description")?;
            let info_json = json!(&info_map);
            functions.push(chat_fn(fn_name.to_string(), description, info_json)?);
        }
        Ok(functions)
    }

    pub async fn next_stage(&mut self, stage: &String) -> Result<()> {
        goto_stage(&mut self.handler, stage);
        self.functions = self.load_actions()?;
        Ok(())
    }

    pub fn call_ret_string(&mut self, fn_name: &str, args_json: &str) -> Result<String> {
        let res_json = call_function(&mut self.handler, fn_name, args_json)?;
        let str_ = json_str!(res_json);
        Ok(str_)
    }

    pub fn call(&mut self, fn_name: &str, args_json: &str) -> Result<String> {
        call_function(&mut self.handler, fn_name, args_json)
    }

    pub async fn eval_rhai(&mut self, expr: &str) -> Result<()> {
        let output = eval_expr(&mut self.handler, expr)?;
        self.log
            .add(fn_result_msg(&s!("eval"), &output.to_string())?);
        println!("Trying to send expr call back");
        self.reply_sender
            .send_async(ChatUIMessage::FunctionCall {
                name: s!("result"),
                params: s!("="),
                result: output,
            })
            .await?;
        println!("Eval expr: ({})", expr);

        let next_step = self.call_ret_string("evalExitStage", "{}")?;
        if next_step.contains("Function not found") {
        } else {
            if next_step != "" && next_step != "()" {
                self.next_stage(&next_step).await?;
            }
        };

        Ok(())
    }

    pub async fn process_fn_call(&mut self, fn_name: &str, fn_args: &str) -> Result<()> {
        if fn_name == "eval" {
            return self.eval_rhai(fn_args).await;
        }
        self.log
            .add(fn_call_msg(&fn_name.to_string(), &fn_args.to_string())?);

        let output = self.call(fn_name, fn_args)?;
        self.log
            .add(fn_result_msg(&s!(fn_name), &output.to_string())?);
        println!("Trying to send func call back");
        self.reply_sender
            .send_async(ChatUIMessage::FunctionCall {
                name: s!(fn_name),
                params: s!(fn_args),
                result: output,
            })
            .await?;
        println!("Function call: {}({})", fn_name, fn_args);

        let next_step = self.call_ret_string("evalExitStage", "{}")?;
        if next_step.contains("Function not found") {
        } else {
            if next_step != "" && next_step != "()" {
                self.next_stage(&next_step).await?;
            }
        };

        Ok(())
    }

    pub fn update_sys_msg(&mut self) -> Result<()> {
        let mut data: HashMap<String, String> = HashMap::new();

        let timestamp_str = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();
        data.insert("timestamp".into(), timestamp_str);

        let json = json!(data);
        let json_string = &json.to_string();

        let sys_str = self.call_ret_string("renderSysMsg", json_string)?;

        self.log.change_sys_msg(sys_msg(&sys_str)?);

        Ok(())
    }

    pub fn render_user_msg(&mut self, user_msg: String) -> Result<String> {
        let mut data: HashMap<String, String> = HashMap::new();

        let timestamp_str = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();
        data.insert(s!("timestamp"), timestamp_str);
        data.insert(s!("user_msg"), user_msg);
        let json = json!(data);
        let json_string = &json.to_string();

        self.call_ret_string("renderUserMsg", json_string)
    }

    async fn handle_command(&mut self, cmd: String) -> Result<bool> {
        let cmd_prefix = "//";
        if cmd.starts_with(cmd_prefix) {
            println!("Found command");
            match cmd.as_str() {
                "//history" => {
                    println!("Found history command.");
                    for msg in &self.log.messages[..] {
                        println!("Sending message..");
                        println!("{:?}", msg.message.clone());
                        match &msg.message {
                            ChatCompletionRequestMessage::System(
                                ChatCompletionRequestSystemMessage { content, role, .. },
                            ) => {
                                if *role != Role::System {
                                    self.reply_sender
                                        .send_async(msg.message.clone().into())
                                        .await
                                        .unwrap();
                                }
                            }
                            _ => {
                                self.reply_sender
                                    .send_async(msg.message.clone().into())
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                    return Ok(true);
                }
                _ => return Ok(false),
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn run(&mut self, cancellation_token: CancellationToken) -> Result<()> {
        let mut need_user_input = true;
        while !cancellation_token.is_cancelled() {
            if need_user_input {
                let input_str = self.receiver.recv_async().await.context("error")?;
                if self.handle_command(s!(input_str)).await? {
                    //continue;
                    break;
                }
                let msg = self.render_user_msg(s!(input_str))?;
                self.log.add(user_msg(&msg)?);
            }
            self.update_sys_msg();

            let msgs = self.log.to_request_msgs("gpt-4")?; //self.model.as_str())?;
            let token = cancellation_token.child_token();
            let (text, fn_name, fn_args) = llama_cpp_chat
                .get()
                .expect("Could not access llama_cpp_chat")
                .generate(
                    msgs.clone(),
                    //self.functions.clone(),
                    self.reply_sender.clone(),
                    token.clone(),
                )
                .await;
            println!("generate result fn: {}({})", fn_name, fn_args);
            if fn_name != "" {
                self.log.add(agent_msg(&text)?); 
                println!("Processing function call!");
                self.process_fn_call(&fn_name, &fn_args).await?;
                need_user_input = false;
            } else {
                self.log.add(agent_msg(&text)?);
                let reply = ChatUIMessage::Reply {
                    name: s!("agent"),
                    role: s!("assistant"),
                    content: text,
                };
                self.reply_sender.send_async(reply.clone()).await?;
                break;
                //need_user_input = true;
            }
        }
        let reply = ChatUIMessage::Reply {
            name: s!("__READY__"),
            role: s!("SYSTEM"),
            content: s!(""),
        };
        self.reply_sender.send_async(reply.clone()).await?;

        println!("Agent finished running.");
        Ok(())
    }
}
