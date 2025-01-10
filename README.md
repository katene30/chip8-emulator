# CHIP-8 Emulator

A simple and efficient CHIP-8 emulator written in Rust. It runs locally, allowing you to load and execute CHIP-8 ROM files seamlessly. The emulator supports most standard CHIP-8 opcodes and provides a foundation for further extensions like WebAssembly integration.

## Features
- **Opcode Support**: Implements most of the CHIP-8 instruction set.
- **Local Execution**: Load and run CHIP-8 ROMs on your desktop.
- **Dynamic Input Handling**: Supports key presses and releases.
- **Graphical Output**: Renders the CHIP-8 display to a virtual screen.

## Requirements
- Rust (Stable version)

## Getting Started

### Clone the Repository
```bash
git clone https://github.com/your-username/chip8-emulator.git
cd chip8-emulator
```

### Build the Emulator
```bash
cargo build --release
```

### Run a ROM
```bash
cargo run --release path/to/rom
```

Replace `path/to/rom` with the file path of the CHIP-8 ROM you want to run.

## ROM Format
CHIP-8 ROMs are binary files consisting of opcodes. These files are loaded into memory starting at address `0x200`. 

## Next Steps
The next step is integrating **WebAssembly (Wasm)** to enable running the emulator in a web browser, extending its accessibility and usability.

Stay tuned for updates!

---

## License
MIT License. See [LICENSE](LICENSE).

---

## Acknowledgements
- [Aquova's CHIP-8 Book](https://github.com/aquova/chip8-book)
