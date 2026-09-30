# CompilerRust
Compiler made in Rust for the Compiler class of CAU (Chung-Ang University).

## About
This project is an early-stage compiler written in Rust, developed as part of the Compiler course at CAU. It currently focuses on the backend: generating a valid ELF object file (`.o` equivalent) that can be linked using `ld`.

## Current State
⚠️ This project is a work in progress.

- No command-line arguments are supported yet — everything is currently hardcoded.
- The instructions to encode are hardcoded test programs in `src/samples/`; `main.rs` only builds the ELF file.
- Running the program generates an ELF object file, which can then be linked with `ld` to produce an executable.

## How it works
1. Instructions are represented as Rust enum variants (hardcoded in `src/samples/`).
2. These instructions are encoded into machine code.
3. The encoded bytes are written into a properly structured ELF object file.
4. The resulting `.elf` (equivalent to `.o`) file can be linked with standard tools (e.g. `ld`) to produce a runnable binary.

## Usage
```bash
cargo build --release
./target/release/compiler_rust
ld -o output output.elf
./output
```
*(exact commands may vary depending on the current output filename)*

## Roadmap
- [ ] Finish all instructions (currently on x86-64 instructions on SIMD registers)
- [ ] Add a proper lexer/tokenizer for source input
- [ ] Add a parser to build an AST
- [ ] Replace hardcoded instruction enums with dynamic instruction generation
- [ ] Support command-line arguments (input file, output file, options)
- [ ] Expand instruction set coverage

## Context
This project was built during my second semester studying in Korea, as part of the Compiler class at CAU, with the goal of exploring how a compiler translates source-level instructions down into machine code and executable binaries.
