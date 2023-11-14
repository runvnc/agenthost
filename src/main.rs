#![allow(warnings)]

use std::io;
use std::io::Read;
use std::io::Write;

use anyhow::Result;

//mod connector;
mod agent;
mod agentmgr;
mod api;
mod cat;
mod chatlog;
//mod errors;
mod jwt_util;
mod openai_chat;
mod scripts;
mod shorthands;

use api::server;

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
#[tokio::main]
async fn main() -> Result<()> {
    api::server().await;
    Ok(())
}

//    io::stdout().flush().unwrap();
//    io::stdin().read_line(&mut input).unwrap();
