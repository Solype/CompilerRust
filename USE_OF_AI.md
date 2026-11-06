# Use of AI in Krc

This document separates what I wrote myself from what was produced with an AI assistant (Claude,
through Claude Code).

## How this document was put together

- **The commits**: every commit made with the AI between October 2 and 5, and some of those of
  October 9 and 10 (`d198bad`), carry the line `Co-Authored-By: Claude`.
- **The AI sessions still on record** (October 5, 9 and 15): they give the detail, function by
  function, of who wrote what in the grammar, the LR table, the parser, the AST and the
  converter.
- **Exception**: the commits `fc1447c`, `efaad85` and `d1c847c` (October 9) were made by the AI,
  but without the `Co-Authored-By` line, at my request. Their content is detailed below. The
  same goes for the commits of October 15 (`996c222` to `41000ea`), October 20 (from `7c222ba`)
  and November 6 (from `f2adc0a`).
- **Limits**:
  - the AI sessions of October 2 to 4 are no longer available: for that period, only the
    `Co-Authored-By` line counts;
  - the AI sessions of October 10 to 12 are no longer available either: see "To be confirmed";
  - the commits of October 1 and 2 without that line (encoder tests, reports, encoding edge
    cases) were made with the AI: I state it myself, no record shows it;
  - the commits of September 30 are still to be confirmed: see "To be confirmed".

## Overview

| Part | Who |
|---|---|
| ELF backend and x86-64 encoder (`src/elf`), March to September | Me; once the complex functions were in place, the AI completed it instruction by instruction (opcodes), then added the PLT32/32S relocations, the SSE instructions and the ELF labels (October 2) |
| Design of the Korean language, TODO, translation to English | Me and AI |
| Lexer, hangeul conjugation, pre-parser (October 3 to 5) | Me for 20-30 % of the pre-parser, AI for the rest |
| Grammar, FIRST, LR(1) table, parser, tree, AST (October 5 to 9) | Me and AI: the grammar and FIRST by the AI, on my `ParserProduction` struct; detail below |
| Converter, the semantic analysis (`src/converter`, October 12 to 15) | Shared: detail below |
| IR and code generation (`src/converter/ir`, `src/codegen`, October 16 to 31) | Mostly me; the AI helped complete the structs of the IR: detail below |
| Sample programs (`src/samples`), README | Me |
| Every unit test and the encoding edge cases they revealed, every macro (`elf_derive`, `rule!` and `term!`), the command line (`-x`, several files, errors) | AI |

## Overall share (estimate, October 20)

An estimate from the line count of each part (17,560 lines of Rust) and the sections of this
document, not an exact measure.

| Measure | Me | AI |
|---|---|---|
| The whole project, in lines | ~40 % | ~60 % |
| Without the tests (~12,300 lines) | ~55 % | ~45 % |

- **Mostly the AI**: the tests (5,275 lines, 30 % of the project), the lexer, the hangeul
  conjugation, most of the pre-parser, the conversion to the AST, the macros.
- **Mostly me**: the backend (`src/elf`, 5,302 lines: architecture, ModRM/SIB, ELF format), the
  samples, the IR and the codegen.
- **Shared**: the LR(1) table and parser (`src/parser/cfg`), the converter (`src/converter`).
- **Uncertainty**: the share of the opcodes completed by the AI in `src/elf` is not counted
  anywhere; it can move the line without the tests by about 10 points.
- **In design**, the share is more clearly mine: the encoder, the ELF format, `ParserProduction`,
  the loop of `parse`, the IR and the codegen.

## The backend: written by me, completed with the AI

From March 20 to September 30, 2026 (about 100 commits).

**Written by me:**

- **The ELF format**: header, sections, program headers, string tables, local and global symbols,
  relocations, writing the file, 32-bit version of the header.
- **The x86-64 encoder**: ModRM/SIB, prefixes, registers r8-r15, and the instruction encoding
  mechanism.
- **The refactorings** of that code (`EncodeInformation`, memory addresses, file layout), the
  README and moving the test programs into `src/samples`.

**Completed with the AI:** once the complex encoding functions were in place, I used the AI
instruction by instruction: for each new instruction, it found its opcodes and variants (instead
of me looking them up in the Intel manual) and completed its encoding within the existing
mechanism. The encoder architecture, ModRM/SIB, the prefixes and the ELF format are mine.

**Written by the AI:** the serialization macro (`elf_derive`), like every macro of the project.

## The tests: all written by the AI

I wrote no test by hand: they were all written by the AI.

- **Backend**: `src/elf/instructions/encode_tests/` (one file per instruction family),
  `src/elf/file/symbol_tests.rs`, the report `docs/TEST_REPORT.md` and the `nextest.toml`
  configuration.
- **Frontend**: `src/hangeul/hangul_tests.rs`, `src/lexer/lexer_tests.rs`,
  `src/parser/preparser/pre_parser_tests.rs`, `src/parser/cfg/first_tests.rs`,
  `src/parser/cfg/parser_tests.rs` and `src/parser/postparser/postparser_tests.rs`.

These tests revealed encoding edge cases (jumps, REX.B/REX.X, 64-bit immediates, ModRM r12/r13),
handled with the AI on October 1 and 2.

## Made with the AI, October 2 to 5

Commits signed `Co-Authored-By: Claude`. I gave the request and reviewed, the AI wrote the code,
except for the documentation, written together.

- **Backend**:
  - PLT32 and 32S relocations, resolving the relocation symbols, `iretq`;
  - SSE instructions: movd/movq, xorpd/xorps, andpd/andps, sqrtsd/sqrtss, min/max,
    movapd/movaps;
  - object alignment, relocations in data, labels separated from symbols (`LabelId`).
- **Documentation**, written together: design of the Korean source language, split of the TODO,
  translation of the comments and messages to English, backend gap report.
- **Frontend**:
  - reading the source file given as argument;
  - the lexer (`src/lexer`): `Span`, `LexError`, the tokens;
  - Hangul syllables and regular conjugation (`src/hangeul`);
  - the pre-parser (`src/parser/preparser`): keywords, types, particles, verbs, declared names,
    errors on the identifiers left. I wrote 20-30 % of it, the AI 70-80 %.
- **Grammar** (`src/parser/cfg`):
  - the `ParserProduction` struct is mine (I asked to keep it as I wrote it);
  - the rules of `rules.rs`, the `rule!` and `term!` macros, and the computation of the nullable
    non-terminals and FIRST sets (`first.rs`) were written by the AI;
  - `docs/LR_TABLE_EXAMPLE.txt`, the example table worked out by hand, was written by the AI.

## October 9: LR(1) table, parser, tree and AST

On this part, I wrote some of the code myself, guided by the explanations of the AI.

### LR(1) table (`parsing_table.rs`)

| Element | Who |
|---|---|
| `Item`, numbering of the terminals (`get_terms`) | Me |
| `TerminalKey` and its `Hash`, `ParsingEntry` | AI |
| Errors on the `StartSymbol` rule (`get_first_rule`) | AI, at my request |
| Closure loop (`closure`) | Me, debugged with the AI |
| `get_new_items` | Me (the body), AI (signature and steps as comments) |
| `goto` | Me (the body), AI (signature and steps) |
| Loop over the states | Me (draft), finished by AI |
| Filling the table (S, R, Goto, Accept) and detecting conflicts (`fill`) | Me (first `R` line), finished by AI |
| Reorganizing `new()`, comments of `Item` and `First` | AI |
| `get` and `get_goto` | Me (start of `get`), finished by AI |
| Step-by-step debug mode (`LR_DEBUG`) | AI, removed since |

### Parser (`parser.rs`)

| Element | Who |
|---|---|
| `Parser` struct and `new` | Me |
| Signature of `parse` and steps as comments, `rules` field | AI |
| Loop of `parse` (shift, reduce, goto, accept, error) | Me |
| `TreeNode` | Me; the AI adjusted the enum (lifetime, `Vec` of children, rule number) |
| Building the tree in `parse`, `TreeNode::show` | AI |
| Wiring into main, syntax error message, tests (`parser_tests.rs`) | AI |

### AST (`src/parser/postparser`)

| Element | Who |
|---|---|
| Start of `Type` and `Function`, signature of `convert_to_ast_tree` | Me |
| The AST structs (`structs.rs`) | Me (start), completed by AI |
| The body of `convert_to_ast_tree` (`postparser.rs`) | AI |

## October 12 to 15: the converter (`src/converter`)

The converter checks the AST before the code generation: names and scopes, types, calls, returns,
constants and loops.

### October 12

The session is no longer available. What is known of it: the AI wrote a whole `convert_block`,
which I asked it to empty, because I had not written it myself. From then on, I wrote the checks of
the converter, and the AI only the narrow pieces I asked for (imports, a struct, a fix, the simple
arms of `get_type`).

### October 15

The session is on record. After each commit, the AI gave a to-do list of the checks still missing;
I chose the points and asked for them. On that day, the AI wrote most of the code.

| Element | Who |
|---|---|
| `PartialEq` of `BlockKind` | Me (a `derive` that did not compile), AI wrote the impl by hand |
| The integer check of the `세면서` bounds | Me (start), completed by AI |
| Fix of `convert_break` (`BreakOutOfLoop` instead of `ContinueOutOfLoop`) | Me |
| `WrongAssignType`, in the place I had left for it in `convert_assign` | AI |
| Variable of `세면서` (declared by the loop or existing, not `고정된`), `Scope::in_loop`, removing the `loops` field and the unused imports | AI |
| Type names in the messages (`type_name`) | AI |
| Literals taking the expected type (`fits`), `AmbiguousCall`, operand checks (`WrongOperands`) | AI |
| Calls without a value (`NoValue`), missing return (`always_returns`, `MissingReturn`), `check_main` | AI |
| Splitting `converter.rs` into `scope.rs`, `types.rs`, `expressions.rs`, `statements.rs` and `functions.rs` (`ebd3cc9`) | AI, at my request |
| Error samples in `sample/errors/` | AI |
| To-do lists, C equivalent of each type, explanation of the typed IR the codegen needs | AI, explanations only |

## October 16 to 19: the IR (`src/converter/ir`)

The converter now gives an IR, the three address code the codegen will turn into x86-64
instructions. The AI helped me complete the IR structs; I wrote the rest by hand.

## October 20 to 31: the code generation (`src/codegen`)

`sample/01_return.kr` was the first program compiled into a binary that runs (October 20); every
`sample/*.kr` runs since October 31.

| Element | Who |
|---|---|
| The codegen architecture: `generate`, one global function per IR function, `Context`, prologue and epilogue, `convert_operand`, `var_size` | Me |
| Calls (System V): argument counters, `al`, float arguments in `xmm`, float parameters stored by the prologue, return and call result in `xmm0` | Me |
| The pieces of the frame and the calls written for me: `normal_param`, `var_offsets`, storing the call result and the parameters, `Context` instead of the offsets | AI, at my request |
| Copies, strings and float constants in `.rodata`, `load_operand` | Me, finished by AI (end of `generate_copy`, the `.rodata` strings) |
| Integer operators: `simple_operation`, `multiplication`, `division` | Me, from the instructions the AI gave me |
| The other operators: shifts, end of `generate_unary`, `+ - * /` and `-x` on `소수`, integer `%` | AI, at my request |
| Sizes and types in the IR: `VarId` with its `Type`, `Type::size`, the size and the sign of the constants, `Type::is_unsigned`, `Size::from` for 1 and 2 bytes | Me; the AI adapted every use |
| `바꾸다` (`generate_cast`) | Me (start), finished by AI |
| Commit `d7235e8` (cast, float call results, 짧은 소수 constants, `VarId` with its `Type`) | Three hands: I added the functions and changed the return type, the AI carried the change through the project, the formatter and linter (`rustfmt`, `clippy`) changed a few lines, and the AI fixed the bugs before the commit |
| Literals at the size of their expected type (`fit_operand`, `fit_args`), the common type of a comparison | Me for `common_type`, from AI code; AI for the rest |
| Comparisons, labels and jumps (`comparison`, `Label`, `Jump`, `generate_jmp_if_not`) | Me, AI autocomplete |
| `==` / `!=` of a NaN (`comparison` reads PF, issue #3, pull request #32) | ShahadatTest: written with Cursor (AI), reviewed by them; the tests moved to `arithmetic_tests.rs`, the NaN case of `sample/15_floats.kr` and this row by AI, at my request |
| Control flow in the converter: `만약` / `아니면` and `동안` | Me, reviewed by AI |
| Control flow in the converter: `convert_cond`, `세면서`, end of the `그리고` / `또는` short-circuit | AI, at my request |
| The rest of the converter: `convert_init` and `convert_assign` through `convert_expr`, the `Operand` variants for addresses | Me |
| The rest of the converter: `convert_expr` split in functions, a variable from `빈 주소` typed `Address(Null)` | AI, at my request |
| Command line (`-x`, several files, errors), `scripts/run_samples.sh`, the samples `12` to `15` | AI, at my request |
| Imports, compile errors, warnings, doc comments, refactorings (number of arguments) | AI, at my request |
| Review of my code, explanations (stack frame, x86 constraints, division, casts, jumps) | AI, explanations only |
| README and TODO updates, the TODO turned into GitHub issues, the repository layout (`docs/`, `scripts/`) | AI |

## November 6: unsigned operations, documentation and release

| Element | Who |
|---|---|
| Unsigned `/`, `%` and `>>` (issue #1, pull request #35) | Me, reviewed by AI |
| Documentation, tests and deployment (README, website, release `v0.1.0`) | AI, at my request |

## How I used the AI

Over the whole project, the AI was mostly used to **explain**: how a format, an encoding or an
algorithm works, why my code did not work, and what to do next. The code it wrote is listed in the
sections above; the rest of the exchanges were explanations.

- **To understand**: most of the exchanges of October 9 are explanations of how to build an LR(1)
  table (items, closure, lookaheads, FIRST, goto), often without any code.
- **To get started**: the AI wrote the signature of a function and its steps as comments, I wrote
  the body, then it reviewed it.
- **To review and fix**: checking my code, finding the bugs, running it on `Proto.kr`.
- **To write directly**: the repetitive parts or the ones I delegated (all the tests, the grammar
  rules, the lexer, the pre-parser, the conversion to the AST, the converter checks of October
  15, the command line options of October 20).

## To be confirmed

These commits have no `Co-Authored-By` line, and no session tells whether they were made with or
without the AI. I still have to state their status:

- `6773546` to `2c33af8` (September 30): moving the programs into `src/samples`, a fix for
  rust-analyzer, TODO.
- `b04d848` to `7b6e27c` (October 10 to 12), except `d198bad`: printing the AST, float literals,
  sentence declarations, spans in the AST, samples, the start of the converter. The sessions are
  no longer available.
