
use anyhow::{Result};

mod cat;
mod shorthands;
mod chatlog;
mod scripts;
mod openai_chat;
mod agent;

use agent::{Agent};

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
#[tokio::main]
async fn main() -> Result<()> {
    let mut agent = Agent::new("scripts/dm.rhai", "gpt-4")?;
/*           if user_input {
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                self.log.add(user_msg(&input)?);
            }
  */
    agent.run(false).await?;
    Ok(())
}

