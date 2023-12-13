use std::fs::File;
use std::io::Write;
use std::path::Path;
use reqwest;

pub async fn download_model_if_not_exists(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        println!("File does not exist. Downloading from {}", url);

        let response = reqwest::get(url).await?;
        let mut file = File::create(file_path)?;
        let content = response.bytes().await?;
        file.write_all(&content)?;

        println!("Download complete.");
    } else {
        println!("File already exists. No download needed.");
    }

    Ok(())
}


