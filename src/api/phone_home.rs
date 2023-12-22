

use std::process::Command;

pub async fn phone_home() -> std::io::Result<()> {
    let result = Command::new("./runpod/phone_home.sh").status();
    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "phone_home script did not run successfully",
        )),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::io::{self, Write};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_phone_home() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "#!/bin/sh\nexit 0").unwrap();
        let path = file.path().to_str().unwrap();
        Command::new("chmod").args(&["+x", path]).status().unwrap();

        let result = phone_home().await;
        assert!(result.is_ok());
    }
}
