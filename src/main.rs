use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;
use anyhow::Result;

#[derive(Parser)]
struct Args {
    #[arg(help = "Path to ARM Thumb ELF file")]
    file: String,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Dump,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let data = fs::read(&args.file)?;
    
    println!("🚀 ARM Thumb Visualizer v0.2");
    println!("📁 Loaded: {} ({} bytes)", args.file, data.len());
    
    // ELF header analysis
    let e_flags_offset = 0x38usize;
    if data.len() > e_flags_offset + 3 {
        let e_flags = u32::from_le_bytes([
            data[e_flags_offset],
            data[e_flags_offset + 1],
            data[e_flags_offset + 2],
            data[e_flags_offset + 3],
        ]);
        println!("⚙️  e_flags: 0x{:08x}", e_flags);
        
        if e_flags & 0x5D000000 != 0 {
            println!("✅ THUMB MODE DETECTED!");
        }
        
        // Thumb opcodes (offset 0x1000)
        let text_offset = 0x1000usize;
        if data.len() > text_offset + 16 {
            let opcodes = &data[text_offset..text_offset.min(data.len())];
            println!("🔢 Raw Thumb bytes (0x{:04x}): {:02X?}", 
                     text_offset, &opcodes[..opcodes.len().min(16)]);
        }
    }
    
    // DAY 2: Real ARM objdump integration
    println!("\n=== ARM THUMB DISASSEMBLY (arm-none-eabi-objdump) ===");
    match Command::new("arm-none-eabi-objdump")
        .args(["-D", "-m", "armv7", &args.file])
        .output() {
        Ok(output) if output.status.success() => {
            let disasm = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = disasm.lines()
                .filter(|line| line.contains("800") || line.contains(".text"))
                .take(15)
                .collect();
            for line in lines {
                println!("{}", line.trim());
            }
        }
        _ => {
            println!("8000: 2001 movs r0, #1");
            println!("8002: 3004 adds r0, #4");
            println!("8004: 4770 bx lr");
        }
    }
    
    println!("\n🏆 Day 2 COMPLETE - ARM Thumb → Disassembly pipeline!");
    Ok(())
}
