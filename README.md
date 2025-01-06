# CHIP-8 Interpreter

A CHIP-8 interpreter/emulator written in Rust using SDL2 for graphics and input handling.

## Overview

This project implements a CHIP-8 interpreter, which can run CHIP-8 ROMs. CHIP-8 is an interpreted programming language developed in the 1970s, primarily used for creating simple video games on microcomputers.

## Features

- Complete CHIP-8 CPU emulation
- Graphics display (64x32 pixels, scaled for modern displays)
- Keyboard input handling
- Built-in font set support
- Memory management
- SDL2-based rendering

## Prerequisites

- Rust (2021 edition or later)
- SDL2 development libraries

### Installing SDL2

#### Ubuntu/Debian

```bash
sudo apt-get install libsdl2-dev
```

#### macOS

```bash
brew install sdl2
```

#### Windows

Download SDL2 development libraries from [SDL's website](https://www.libsdl.org/) or install via MSYS2.

## Building

1. Clone the repository:

```bash
git clone https://github.com/yourusername/chip8-interpreter.git
cd chip8-interpreter
```

2. Build the project:

```bash
cargo build --release
```

## Running

To run the interpreter, load one of the roms in the `roms` folder:

```bash
cargo run --release path/to/rom
```

## Project Structure

- `src/main.rs`: Main entry point and SDL2 initialization
- `src/chip8/`: Core emulator components
  - `cpu.rs`: CPU implementation and instruction handling
  - `memory.rs`: Memory management
  - `display.rs`: Display handling
  - `keyboard.rs`: Keyboard input processing
  - `constants.rs`: System constants and font data

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [SDL2 Rust bindings](https://github.com/Rust-SDL2/rust-sdl2)
