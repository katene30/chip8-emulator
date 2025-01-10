# CHIP-8 Emulator

## Overview
This is a CHIP-8 emulator in Rust with two components:

1. **Core Emulator Logic (`chip8_core`)**: Handles CPU emulation, memory, and instruction set. Designed as a reusable crate.
2. **Desktop Frontend (`desktop`)**: Provides a graphical interface for running CHIP-8 programs on desktop.

The project supports future frontends like web-based UIs.

---

## Project Structure

```
chip8-emulator/
├── chip8_core/        # Core emulator logic
├── desktop/           # Desktop frontend
├── Cargo.toml         # Workspace config
└── README.md          # Documentation
```

---

## Features
- **64x32 Display**: Emulates CHIP-8 sprites.
- **CPU Implementation**: Supports 35 instructions.
- **Keyboard Input**: Maps CHIP-8's 16-key input to modern keyboards.
- **Timers**: Includes delay and sound timers.
- **Extensible**: Easy to add more frontends.

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- CHIP-8 ROM

---

## Usage
1. Place a CHIP-8 ROM in the `desktop` folder.
2. Run the emulator:
   ```bash
   cargo run --release -- path/to/rom.ch8
   ```

---

## Development

### Workspace
Use the Rust workspace to build all crates:
```bash
cargo build
```

### Dependencies
Add dependencies in `Cargo.toml` for the respective crate.

---

## Roadmap
- [ ] Sound support
- [ ] Web-based frontend
- [ ] Debugging tools

---

## License
MIT License. See [LICENSE](LICENSE).

---

## Acknowledgements
- [Aquova's CHIP-8 Book](https://github.com/aquova/chip8-book)
