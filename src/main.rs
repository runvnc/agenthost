use std::error::Error;
use anyhow::{Result, anyhow};

mod cat;
mod shorthands;
mod chatlog;
mod scripts;
mod openai_chat;
mod agent;

use agent::{startup, run};

use shorthands::*;

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
#[tokio::main]
async fn main() -> Result<()> {
    let mut agent = startup("scripts/dm.rhai",
                            "gpt-4")?;
    run(&mut agent).await?;
    Ok(())
}

