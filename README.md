# ARM Thumb ISA Visualizer ✨

[![FOSS Hack 2026](https://img.shields.io/badge/FOSS%20Hack-2026-blueviolet)](https://fossunited.org/hack/fosshack26)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-brightorange)](https://rust-lang.org)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue)](./LICENSE-MIT)

**Cycle-accurate ARM Thumb disassembler & pipeline visualizer for embedded developers**

## 🚀 Quickstart

```bash
# Install (Ubuntu/Windows)
winget install LLVM || sudo apt install llvm clang
rustup default stable

# Clone & run
git clone https://github.com/YOUR_USERNAME/thumb-viz
cd thumb-viz
cargo run -- --help
