use clap::Parser;
use std::fs;
use anyhow::Result;

#[derive(Parser)]
struct Args {
    #[arg(help = "Path to ARM Thumb ELF file")]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let data = fs::read(&args.file)?;
    println!("Loaded ARM Thumb binary: {} ({} bytes)", args.file, data.len());
    println!("First 16 bytes: {:02X?}", 
             &data[..data.len().min(16)]);
    Ok(())
}
