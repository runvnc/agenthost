#![allow(warnings)]

use std::io;
use std::io::Write;
use std::io::Read;

use anyhow::{Result};

mod connector;
mod cat;
mod shorthands;
mod chatlog;
mod scripts;
mod openai_chat;
mod agent;
mod api;

use api::server;

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
#[tokio::main]
async fn main() -> Result<()> {
    api::server()
}


//    io::stdout().flush().unwrap();
//    io::stdin().read_line(&mut input).unwrap();

