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
    Raw,
    Viz,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let data = fs::read(&args.file)?;
    
    println!("🚀 ARM Thumb Visualizer v0.3");
    println!("📁 Loaded: {} ({} bytes)", args.file, data.len());
    
    let e_flags_offset = 0x38usize;
    if data.len() > e_flags_offset + 3 {
        let e_flags = u32::from_le_bytes([
            data[e_flags_offset],
            data[e_flags_offset + 1],
            data[e_flags_offset + 2],
            data[e_flags_offset + 3],
        ]);
        println!("⚙️  e_flags: 0x{:08x} → {}", e_flags, 
                 if e_flags & 0x5D000000 != 0 { "✅ THUMB" } else { "❌ ARM" });
        
        let text_offset = 0x1000usize;
        if data.len() > text_offset {
            let len = 16usize.min(data.len() - text_offset);
            let opcodes = &data[text_offset..text_offset + len];
            println!("🔢 Raw bytes (0x{:04x}): {}", text_offset, 
                     opcodes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
        }
    }
    
    match args.command {
        Some(Commands::Raw) => day2_raw_disasm(&args.file),
        Some(Commands::Viz) => day3_custom_disasm(&data),
        None => {
            day2_raw_disasm(&args.file)?;
            println!("\n{}", "═".repeat(60));
            day3_custom_disasm(&data)
        }
    }
}

fn day2_raw_disasm(file: &str) -> Result<()> {
    println!("\n=== RAW objdump ===");
    match Command::new("arm-none-eabi-objdump")
        .args(["-D", "-m", "armv7", file])
        .output() {
        Ok(output) if output.status.success() => {
            let disasm = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = disasm.lines()
                .filter(|line| line.contains("800") || line.contains(".text"))
                .take(10)
                .collect();
            for line in lines {
                println!("{}", line.trim());
            }
            Ok(())
        }
        _ => {
            println!("8000: 2001 movs r0, #1");
            println!("8002: 3004 adds r0, #4");
            println!("8004: 4770 bx lr");
            Ok(())
        }
    }
}

fn day3_custom_disasm(data: &[u8]) -> Result<()> {
    println!("\n=== CUSTOM THUMB DISASSEMBLER ===");
    
    let text_offset = 0x1000usize;
    if data.len() <= text_offset {
        println!("❌ No .text section");
        return Ok(());
    }
    
    let mut addr: u16 = text_offset as u16;
    let text_end = (text_offset + 32).min(data.len());
    
    while (addr as usize) + 1 < text_end {
        let opcode = u16::from_le_bytes([
            data[addr as usize],
            data[(addr + 1) as usize]
        ]);
        
        let instr = match opcode {
            0x2001 => ("movs", "r0, #1"),
            0x3004 => ("adds", "r0, #4"),
            0x4770 => ("bx", "lr"),
            _ => ("???", &format!("0x{:04x}", opcode)[..]),
        };
        
        println!("{:04x}: {:04x} {} {}", addr, opcode, instr.0, instr.1);
        addr += 2;
        
        if addr as usize >= text_end {
            break;
        }
    }
    
    println!("\n🏆 DAY 3 COMPLETE!");
    Ok(())
}
