use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub async fn download_model_if_not_exists(
    url: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        println!(
            "File does not exist. Downloading from {} to {}",
            url, file_path
        );

        let mut child = Command::new("wget")
            .arg(url)
            .arg("-O")
            .arg(file_path)
            .stdout(Stdio::inherit()) // Inherit the stdout to display in console
            .stderr(Stdio::inherit()) // Inherit the stderr to display in console
            .spawn()
            .expect("Failed to start wget command");

        // Wait for wget to finish
        let result = child.wait().expect("Failed to wait on child");

        // Check the result
        if result.success() {
            println!("Download completed successfully.");
        } else {
            eprintln!("Error occurred during download.");
        }
        println!("Download complete.");
    } else {
        println!("File already exists. No download needed.");
    }

    Ok(())
}
