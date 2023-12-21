

use std::process::Command;

pub async fn phone_home() -> std::io::Result<()> {
    Command::new("./runpod/phone_home.sh").status().await?;
    Ok(())
}
