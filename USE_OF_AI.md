# Use of AI in CompilerRust

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
  same goes for the commits of October 15 (`996c222` to `41000ea`).
- **Limits**:
  - the AI sessions of October 2 to 4 are no longer available: for that period, only the
    `Co-Authored-By` line counts;
  - the AI sessions of October 10 to 12 are no longer available either: see "To be confirmed";
  - the commits of October 1 and 2 without that line (encoder tests, reports, REX/ModRM fixes)
    were made with the AI: I state it myself, no record shows it;
  - the commits of September 30 are still to be confirmed: see "To be confirmed".

## Overview

| Part | Who |
|---|---|
| ELF backend and x86-64 encoder (`src/elf`), March to September | Me; once the complex functions were in place, the AI completed it instruction by instruction (opcodes) |
| Every macro of the project (`elf_derive`, `rule!` and `term!` of the grammar) | AI |
| Sample programs (`src/samples`), README | Me |
| Encoder tests and REX/ModRM fixes (October 1 and 2) | AI |
| Every unit test of the project | AI |
| Relocation fixes, SSE instructions, ELF labels (October 2) | AI, at my request |
| Design of the Korean language, TODO, translation to English | AI and me |
| Lexer, hangeul conjugation (October 3) | AI, at my request |
| Pre-parser (October 4 and 5) | AI for 70-80 %, me for the rest |
| Grammar and FIRST (October 5) | AI, on my `ParserProduction` struct |
| LR(1) table, parser, tree, AST (October 9) | Shared: detail below |
| Converter, the semantic analysis (`src/converter`, October 12 to 15) | Shared: detail below |
| IR, the output of the converter (`src/converter/ir`, October 16 to 19) | Me; the AI helped complete the structs |

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
  `src/elf/file/symbol_tests.rs`, the report `TEST_REPORT.md` and the `nextest.toml`
  configuration.
- **Frontend**: `src/hangeul/hangul_tests.rs`, `src/lexer/lexer_tests.rs`,
  `src/parser/preparser/pre_parser_tests.rs`, `src/parser/cfg/first_tests.rs`,
  `src/parser/cfg/parser_tests.rs` and `src/parser/postparser/postparser_tests.rs`.

The backend fixes of October 1 and 2 (jumps, REX.B/REX.X, 64-bit immediates, ModRM r12/r13) were
found thanks to these tests, and made with the AI.

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
    errors on the identifiers left. 70-80 % of it was written by the AI, the rest by me.
- **Grammar** (`src/parser/cfg`):
  - the `ParserProduction` struct is mine (I asked to keep it as I wrote it);
  - the rules of `rules.rs`, the `rule!` and `term!` macros, and the computation of the nullable
    non-terminals and FIRST sets (`first.rs`) were written by the AI;
  - `LR_TABLE_EXAMPLE.txt`, the example table worked out by hand, was written by the AI.

## October 9: LR(1) table, parser, tree and AST

On this part, I wrote some of the code myself, guided by the explanations of the AI.

### LR(1) table (`parsing_table.rs`)

| Element | Who |
|---|---|
| `Item`, numbering of the terminals (`get_terms`) | Me |
| `TerminalKey` and its `Hash`, `ParsingEntry` | AI |
| Errors on the `StartSymbol` rule (`get_first_rule`) | AI, at my request |
| Closure loop (`closure`) | Me; the AI fixed my first version |
| `get_new_items` | AI: signature and steps as comments; me: the body. I fixed the `point + 1` bug after an explanation |
| `goto` | AI: signature and steps; me: the body. I fixed the filter after an explanation |
| Loop over the states | AI, from my draft |
| Filling the table (S, R, Goto, Accept) and detecting conflicts (`fill`) | AI, from my first `R` line |
| Reorganizing `new()`, comments of `Item` and `First` | AI |
| `get` and `get_goto` | AI, from my start of `get` |
| Step-by-step debug mode (`LR_DEBUG`) | AI, removed since |

### Parser (`parser.rs`)

| Element | Who |
|---|---|
| `Parser` struct and `new` | Me |
| Signature of `parse` and steps as comments, `rules` field | AI |
| Loop of `parse` (shift, reduce, goto, accept, error) | Me |
| `TreeNode` | Me; the AI fixed the enum (lifetime, `Vec` of children, rule number) |
| Building the tree in `parse`, `TreeNode::show` | AI |
| Wiring into main, syntax error message, tests (`parser_tests.rs`) | AI |

### AST (`src/parser/postparser`)

| Element | Who |
|---|---|
| Start of `Type` and `Function`, signature of `convert_to_ast_tree` | Me |
| The AST structs (`structs.rs`) | I started, AI completed |
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
| `PartialEq` of `BlockKind` | AI: my `derive` did not compile, it wrote the impl by hand |
| The integer check of the `세면서` bounds | I started it, the AI completed it |
| Fix of `convert_break` (`BreakOutOfLoop` instead of `ContinueOutOfLoop`) | Me |
| `WrongAssignType`, in the place I had left for it in `convert_assign` | AI |
| Variable of `세면서` (declared by the loop or existing, not `고정된`), `Scope::in_loop`, removing the `loops` field and the unused imports | AI |
| Type names in the messages (`type_name`) | AI |
| Literals taking the expected type (`fits`), `AmbiguousCall`, operand checks (`WrongOperands`) | AI |
| Calls without a value (`NoValue`), missing return (`always_returns`, `MissingReturn`), `check_main` | AI |
| Splitting `converter.rs` into `scope.rs`, `types.rs`, `expressions.rs`, `statements.rs` and `functions.rs` (`ebd3cc9`) | AI, at my request |
| Error samples in `sample/erreurs/` | AI |
| To-do lists, C equivalent of each type, explanation of the typed IR the codegen needs | AI, explanations only |

## October 16 to 19: the IR (`src/converter/ir`)

The converter now gives an IR, the three address code the codegen will turn into x86-64
instructions. The AI helped me complete the IR structs; I wrote the rest by hand.

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
  15).

## To be confirmed

These commits have no `Co-Authored-By` line, and no session tells whether they were made with or
without the AI. I still have to state their status:

- `6773546` to `2c33af8` (September 30): moving the programs into `src/samples`, a fix for
  rust-analyzer, TODO.
- `b04d848` to `7b6e27c` (October 10 to 12), except `d198bad`: printing the AST, float literals,
  sentence declarations, spans in the AST, samples, the start of the converter. The sessions are
  no longer available.
