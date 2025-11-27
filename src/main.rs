use md5::Md5;
use md5::Digest;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    let result = hasher.finalize();
    println!("MD5 checksum: {}", hex::encode(result));

    Ok(())
}
