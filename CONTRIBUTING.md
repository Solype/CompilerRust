# Contributing

Thanks for your interest in the compiler. This project started as a class project at CAU
(Chung-Ang University); contributions are welcome through pull requests.

## Find something to do

What is left is tracked in the [GitHub issues](https://github.com/Solype/Krc/issues). Each issue
carries the stage it touches:

| Label | Stage |
|---|---|
| `front-end` | Lexer, pre-parser, LR(1) parser, AST (`src/lexer`, `src/hangeul`, `src/parser`) |
| `converter` | Semantic checks and typed IR (`src/converter`) |
| `codegen` | IR to x86-64 instructions (`src/codegen`) |
| `backend` | x86-64 encoder and ELF writer (`src/elf`) |
| `tooling` | Tests, scripts, error messages |
| `later` | Not needed by the compiler today |

New here? Start with an issue labelled `good first issue`. Comment on the issue before you start,
so two people do not work on the same thing.

The language itself is described in [`docs/LANGUAGE_DESIGN.md`](docs/LANGUAGE_DESIGN.md), and the
pipeline in the [README](README.md#how-it-works).

## Build and test

You need a Rust toolchain (edition 2024), `gcc` to link, and Linux x86-64 to run the binaries.

```bash
cargo build
cargo test                    # unit tests
scripts/run_samples.sh        # compiles, links and runs every sample
scripts/run_samples.sh sample/10_gcd.kr   # one sample
scripts/analyze.sh sample/10_gcd.o        # symbols and disassembly of an object file
```

Both `cargo test` and `scripts/run_samples.sh` must pass before a pull request.

## Samples

- `sample/NN_name.kr` must compile and run. Its header comment gives the expected exit code with
  the sentence `` `echo $?` prints N ``; without it, the expected code is 0. If the program
  prints, put its expected stdout in `sample/NN_name.out`.
- `sample/errors/name.kr` must be refused with exit code 1, never with a panic. Its first line
  names the stage that refuses it and why: `// Converter: 가 is declared twice in the same block.`

A new feature comes with a sample that uses it; a new error comes with an error sample.

## Code conventions

- Everything is written in English: code, comments, docs, commit messages. Korean stays in the
  language keywords and the sample programs.
- `mod.rs` holds only `mod` and `pub use` lines; the code goes in named files.
- Import sibling modules with `super::`, not with `crate::` paths.
- A known limit left in the code is marked with a comment in this form:
  `// shortcut: <the limit>, <when to fix it>`.
- Run `cargo clippy` and fix the warnings your change adds.

## Commits and pull requests

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/), with the stage
as scope:

```
feat(codegen): unsigned comparisons with B / BE / A / AE
fix(converter): both sides of a comparison fitted to their common type
refactor(ir): Operand::Float holds only its index
docs: repository layout in the README
```

1. Fork the repository and create a branch from `main`. Branch names always start with a
   standard prefix matching the Conventional Commits type: `feat/`, `fix/`, `docs/`, `refactor/`,
   `test/`, `chore/`, for example `feat/1-unsigned-division`.
2. Keep one subject per pull request, and link its issue (`Closes #12`).
3. Describe what changed and how you tested it.
4. If you used an AI tool to write part of the change, say so in the pull request: the project
   keeps track of it in [`USE_OF_AI.md`](USE_OF_AI.md).

## Reporting a bug

Open an issue with the smallest `.kr` program that shows the problem, the command you ran, and
what you got (the error message, or the panic with `RUST_BACKTRACE=1`).
