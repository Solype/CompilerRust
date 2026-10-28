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
  same goes for the commits of October 15 (`996c222` to `41000ea`) and October 20 (from `7c222ba`).
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
| ELF backend and x86-64 encoder (`src/elf`), March to September | Me; once the complex functions were in place, the AI completed it instruction by instruction (opcodes) |
| Every macro of the project (`elf_derive`, `rule!` and `term!` of the grammar) | AI |
| Sample programs (`src/samples`), README | Me |
| Every unit test of the project, and the encoding edge cases they revealed (October 1 and 2) | AI |
| PLT32/32S relocations, SSE instructions, ELF labels (October 2) | AI, at my request |
| Design of the Korean language, TODO, translation to English | AI and me |
| Lexer, hangeul conjugation (October 3) | AI, at my request |
| Pre-parser (October 4 and 5) | AI for 70-80 %, me for the rest |
| Grammar and FIRST (October 5) | AI, on my `ParserProduction` struct |
| LR(1) table, parser, tree, AST (October 9) | Shared: detail below |
| Converter, the semantic analysis (`src/converter`, October 12 to 15) | Shared: detail below |
| IR, the output of the converter (`src/converter/ir`, October 16 to 19) | Me; the AI helped complete the structs |
| First code generation (`src/codegen`, October 20) | Me |
| Command line: `-x`, several files, errors (October 20) | AI, at my request |

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
  `src/elf/file/symbol_tests.rs`, the report `TEST_REPORT.md` and the `nextest.toml`
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
| Closure loop (`closure`) | Me, debugged with the AI |
| `get_new_items` | AI: signature and steps as comments; me: the body |
| `goto` | AI: signature and steps; me: the body |
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
| `TreeNode` | Me; the AI adjusted the enum (lifetime, `Vec` of children, rule number) |
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

## October 20: the first code generation (`src/codegen`) and the command line

`sample/01_retour.kr` is the first program compiled into a binary that runs, then
`sample/12_addition.kr`, a call with two arguments.

| Element | Who |
|---|---|
| `get_size_of`, `Operand::Float` as an index in `Program.floats`, the literal arms of `convert_expr` | Me |
| The codegen: `generate`, one global function per IR function, `convert_operand`, `generate_return` | Me |
| Prologue and epilogue, `generate_call` (argument counters, `al`), first version of `generate_binary`, the size in `VarId`, filling the IR params | Me |
| Small corrections in my code (the `SizeOf` arm, floats in `operand()`, `Return`, `link`, `generate_binary` through `rax`) | AI |
| `normal_param` (System V registers), exact offset of each variable (`var_offsets`), storing the call result and the parameters | AI, at my request |
| Wiring the codegen into `main.rs`; the `-x` option; errors returned instead of `process::exit`; several files, each `file.kr` giving `file.o` | AI, at my request |
| `get_size_of` as a free function, `unimplemented!()` instead of silent cases in the converter, warnings and style fixes, `sample/12_addition.kr` | AI, at my request |
| Review of my code, guidelines for the codegen (stack frame, operands, x86 constraints, System V calling convention, later optimizations) | AI, explanations only |
| README and TODO updates | AI |

## October 21: `Copy`, and a script that runs the samples

| Element | Who |
|---|---|
| `generate_copy` started, its arm in `generate_instruction`; `Type::Bool` in `get_size_of` | Me |
| End of `generate_copy` (the value through `rax`) | AI, at my request |
| `run_samples.sh` (compile, link, run each sample, compare the exit code and stdout; the error samples must give code 1), `sample/02_bonjour.out` | AI, at my request |
| `convert_expr` split into `convert_binary`, `convert_unary`, `convert_call_value` and `convert_args` | AI, at my request |
| Plan of the next steps, in `TODO_Frontend.md` | AI |
| README and TODO updates | AI |

## October 22: the operators, `짧은 정수` on the stack, calls with arguments

| Element | Who |
|---|---|
| `convert_init` and `convert_assign` through `convert_expr` | Me |
| The `ir_func` parameter of `convert_assign` | AI, at my request |
| `generate_binary` moved into `arithmetic.rs`, `simple_operation` (Sub, And, Or, Xor) | Me |
| `multiplication` and `division` from the instructions the AI gave me (`imul`, `cqo` / `idiv`) | Me, from AI code |
| `shift` (`shl` / `sar` through `cl`), missing imports | AI, at my request |
| `var_size` and the size of the `mov` in Copy, Binary, the call result, the prologue; `cdq` on 4 bytes | Me |
| `sample/13_operateurs.kr`, `sample/14_court.kr`, which found the `cqo` and prologue bugs | AI, at my request |
| `convert_call` converting the arguments of a call alone | Me |
| `Context` passed instead of the offsets | AI, at my request |
| Review of my code, explanations (x86 division, `mov` versus `lea` for a string, an address operand in the IR) | AI, explanations only |

## October 23: strings in `.rodata`

| Element | Who |
|---|---|
| `load_operand` (a string through `lea`), from code the AI gave me | Me, from AI code |
| The imports of `utils.rs`, every `mov` that loads an operand replaced by `load_operand` | AI, at my request |
| The strings in `.rodata` in `generate`: started | Me |
| End of the `.rodata` strings (one local symbol `.str.N` per string, ended by 0) | AI, at my request |
| Doc comments of the codegen functions | AI, at my request |
| README and TODO updates | AI |

## October 26 and 27: unary operators, `소수` arithmetic and float arguments

| Element | Who |
|---|---|
| `generate_unary`: started | Me |
| End of `generate_unary` (the store of the result, `xor al, 1` for the logical not) | AI, at my request |
| `Operand::Str` renamed `Address`, then split into `Address(VarId)` and `DataAddress` | Me |
| Float constants in `.rodata`, `float_names` in `Context` | Me |
| `sample/15_flottants.kr` | AI, at my request |
| `+ - * /` on `소수`, `-x` of a `소수`, integer `%`, `VarId` with named fields | AI, at my request |
| Float arguments in `xmm`, float parameters stored by the prologue, return in `xmm0` | Me |
| The `Type` import and the `Type` compile errors in that code | AI, at my request |
| Review of my code (a symbol defined twice, `movapd` versus `movsd`, a float variable passed in an integer register) | AI, explanations only |

## October 28: `바꾸다`, `짧은 소수` and comparisons

| Element | Who |
|---|---|
| `VarId` carries its `Type`, `Type::size` and `Type::is_float` replace `get_size_of` | Me |
| Every use adapted to the new `VarId` and `Type::size` | AI, at my request |
| `Operand::Int` / `Float` carry their size, `짧은 소수` constants written on 4 bytes | Me |
| `generate_cast` (`cvtss2sd`, `cvtsi2sd`, `cvttsd2si`, `movsxd`, `movzx`) | AI, at my request, after my start |
| The result of a float call read from `xmm0` | Me |
| `fit_operand` / `fit_args`: a literal takes the size of its expected type | AI, at my request |
| `common_type`, the sides of a comparison fitted to it | Me, from AI code |
| The type of the result of a comparison (`ty`, not the type of its sides) | AI |
| Explanations (the cast table, the comparison problem) and a review of the architecture | AI, explanations only |
| README and TODO updates | AI |

## October 28 (continued): comparisons and jumps

| Element | Who |
|---|---|
| `Size::from` for 1 and 2 bytes | Me |
| The declaration and the calls of `comparison`, `그리고` / `또는` as `and` / `or` | AI, at my request |
| `comparison` (`cmp` / `ucomisd`, `setcc`, sides swapped for `<` on `소수`) | AI, at my request, after my start |
| `Instr::Label` to `Instruction::Label` | Me |
| The `LabelId` table in `Context`, a new `LabelId` per label of the function | AI, at my request |
| `Instr::Jump` to `jmp` | Me, finished by AI |
| `generate_jmp_if_not` | Me |
| Explanations (the label collisions, how `JumpIfNot` works, the priorities) | AI, explanations only |
| README and TODO updates | AI |

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
