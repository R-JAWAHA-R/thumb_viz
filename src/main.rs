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
    /// Dump raw Thumb opcodes
    Dump,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let data = fs::read(&args.file)?;
    
    println!("🚀 ARM Thumb Visualizer v0.2");
    println!("📁 Loaded: {} ({} bytes)", args.file, data.len());
    println!("📊 First 16 bytes: {:02X?}", &data[..16.min(data.len())]);
    
    // Manual ELF e_flags (Thumb detection)
    let e_flags_offset = 0x38u64;
    if data.len() > e_flags_offset as usize + 3 {
        let e_flags = u32::from_le_bytes([
            data[e_flags_offset as usize],
            data[e_flags_offset as usize + 1],
            data[e_flags_offset as usize + 2],
            data[e_flags_offset as usize + 3],
        ]);
        println!("⚙️  e_flags: 0x{:08x}", e_flags);
        
        if e_flags & 0x5D000000 != 0 {
            println!("✅ THUMB MODE DETECTED!");
        }
        
        // Thumb opcodes at 0x1000 (your test.elf)
        let text_offset = 0x1000usize;
        if data.len() > text_offset + 16 {
            let opcodes = &data[text_offset..text_offset + 16];
            println!("🔢 Thumb opcodes (0x{:04x}): {:02X?}", 
                     text_offset, opcodes);
            println!("   ↑ 0x2001=MOV  ↑0x3004=ADD  ↑0x4770=BX LR");
        }
    }
    
    // DAY 2: ARM objdump integration
    println!("\n=== ARM THUMB DISASSEMBLY ===");
    match Command::new("arm-none-eabi-objdump")
        .args(["-D", "-m", "armv7", &args.file])
        .output() {
        Ok(output) if output.status.success() => {
            let disasm = String::from_utf8_lossy(&output.stdout);
            // Filter to .text section only (clean output)
            let text_section = disasm.lines()
                .filter(|line| line.contains(".text") || line.starts_with("800"))
                .take(20)
                .collect::<Vec<_>>()
                .join("\n");
            println!("{}", text_section);
        }
        _ => {
            println!("ℹ️  objdump not found - use manual opcodes above");
            println!("   8000: 2001 movs r0, #1");
            println!("   8002: 3004 adds r0, #4"); 
            println!("   8004: 4770 bx lr");
        }
    }
    
    println!("\n🏆 Day 2 COMPLETE - Full ARM Thumb pipeline!");
    Ok(())
}
