use md5::{Digest, Md5};
use std::{env, process::Stdio};
use std::process::Output;
use tokio::process::Command;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct API_data {
    md5: String,
    sha256: String,
    filename: String,
    source: String
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let json_data = r#"
        {
          "md5": "...",
          "sha256": "...",
          "filename": "...",
          "source": "rust"
        }"#;
    
    let json_data: API_data = serde_json::from_str(json_data)?;
    
    let python_output: Output = python_script(json_data).await?;
    let md5_hash = md5_generate().await?;

    Ok(())
}

async fn md5_generate() -> std::io::Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path).await?;
    let mut reader = file;
    let mut hasher = Md5::new();
    let mut buffer = [0u8; 4096];

    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hex::encode(hasher.finalize());

    Ok(result)
}

async fn python_script(json_data: serde_json::Value) -> std::io::Result<Output> {
    // Running python script
    let python_output = Command::new("python3")
        .arg("python/main.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    // Handling python script errors
    match python_output {
        Ok(output) => Ok(output),
        Err(e) => {
            eprintln!("Error while executing python script: {}", e);
            Err(e)
        }
    }
}
