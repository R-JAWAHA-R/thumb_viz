use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let data = fs::read(&args.file)?;
    println!("Loaded ARM Thumb binary: {} ({} bytes)", args.file, data.len());
    println!("First 16 bytes: {:02X?}", &data[..data.len().min(16)]);
    Ok(())
}
