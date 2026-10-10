mod elf;
mod hangeul;
mod lexer;
mod parser;
mod samples;

use std::fs::{self, File};
use std::{env, process};

use elf::elfsym::{StBind, StType, StVis, make_st_info};
use elf::file::{ElfFile64, SymbolType, natural_alignment};
use elf::instructions::{Instruction, RelocKind, Relocation, Target};
use elf::shdr::{ElfShdr, SectionName, ShFlags, ShType};
use parser::cfg::parser::Parser;
use parser::cfg::rules::RULES;
use parser::postparser::postparser::convert_to_ast_tree;

const OUTPUT: &str = "output.elf";

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let source = read_source(&args);
    let tokens = lexer::tokenize(&source).unwrap_or_else(|err| {
        let (line, col) = err.span.line_col(&source);
        eprintln!("{}:{line}:{col}: {err}", args.path);
        process::exit(1);
    });
    let pre_parsed = parser::pre_parse(&tokens).unwrap_or_else(|err| {
        let (line, col) = err.span.line_col(&source);
        eprintln!("{}:{line}:{col}: {err}", args.path);
        process::exit(1);
    });
    for warning in &pre_parsed.warnings {
        let (line, col) = warning.span.line_col(&source);
        eprintln!("{}:{line}:{col}: warning: {warning}", args.path);
    }
    let tree = Parser::new(RULES)
        .parse(&pre_parsed.tokens)
        .unwrap_or_else(|token| {
            let (line, col) = token.span.line_col(&source);
            eprintln!("{}:{line}:{col}: syntax error: unexpected {:?}", args.path, token.kind);
            process::exit(1);
        });
    if args.tree {
        println!("{}", tree.show(RULES));
    }
    let ast = convert_to_ast_tree(&tree).unwrap_or_else(|err| {
        let (line, col) = err.span.line_col(&source);
        eprintln!("{}:{line}:{col}: {err}", args.path);
        process::exit(1);
    });
    print!("{ast}");

    let mut elf_file = ElfFile64::default();
    elf_file.declare_non_defined_sym(&"my_exit".to_string(), SymbolType::Function);

    let data = add_section(&mut elf_file, SectionName::Data, ShFlags::Write, 4);
    add_object(&mut elf_file, data, "my_data", b"abcdefg\0");
    add_object(&mut elf_file, data, "my_float", &0.1f64.to_le_bytes());
    add_object(&mut elf_file, data, "my_lock", &0u32.to_le_bytes());
    // pointers resolved by ld: a data symbol and a function defined below
    add_pointers(
        &mut elf_file,
        data,
        "my_ptrs",
        &["my_data", "jumps_and_calls"],
    );

    let text = add_section(&mut elf_file, SectionName::Text, ShFlags::ExecInstr, 16);
    for (name, code) in samples::families() {
        add_function(&mut elf_file, text, name, code);
    }
    add_function(&mut elf_file, text, "_start", samples::start());

    elf_file.write(&mut File::create(OUTPUT)?)?;
    println!("ELF written: {OUTPUT}");
    Ok(())
}

struct Args {
    program: String,
    path: String,
    /// `--tree`: also print the parse tree, before the AST
    tree: bool,
}

/// `compiler [--tree] file.kr`, exits on a wrong usage
fn parse_args() -> Args {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "compiler".to_string());
    let mut tree = false;
    let mut paths = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--tree" => tree = true,
            _ => paths.push(arg),
        }
    }
    let [path] = <[String; 1]>::try_from(paths).unwrap_or_else(|_| {
        eprintln!("usage: {program} [--tree] <file.kr>");
        process::exit(1);
    });
    Args {
        program,
        path,
        tree,
    }
}

/// Reads the source file (UTF-8), exits on error
fn read_source(args: &Args) -> String {
    fs::read_to_string(&args.path).unwrap_or_else(|err| {
        eprintln!("{}: {}: {err}", args.program, args.path);
        process::exit(84);
    })
}

/// Adds an allocated PROGBITS section, with one extra flag (Write, ExecInstr, ...)
fn add_section(elf_file: &mut ElfFile64, name: SectionName, flag: ShFlags, align: u64) -> usize {
    elf_file.add_section(
        name.as_str().to_string(),
        ElfShdr {
            sh_type: ShType::ProgBits as u32,
            sh_flags: ShFlags::Alloc as u64 | flag as u64,
            sh_addralign: align,
            ..Default::default()
        },
    )
}

/// Adds an initialized global variable, at its natural alignment
fn add_object(elf_file: &mut ElfFile64, section: usize, name: &str, bytes: &[u8]) {
    elf_file.add_symbol_to_section_raw(
        section,
        name.to_string(),
        &bytes.to_vec(),
        make_st_info(StBind::Global, StType::Object),
        StVis::Default as u8,
        natural_alignment(bytes.len()),
    );
}

/// Adds a global array of 8-byte pointers to `targets` (`.quad a, b, ...`)
fn add_pointers(elf_file: &mut ElfFile64, section: usize, name: &str, targets: &[&str]) {
    let relocations: Vec<Relocation> = targets
        .iter()
        .enumerate()
        .map(|(i, sym)| Relocation {
            target: Target::Sym(sym.to_string()),
            offset: i * 8,
            size: 8,
            kind: RelocKind::Absolute,
            addend: 0,
        })
        .collect();

    elf_file.add_symbol_to_section_relocated(
        section,
        name.to_string(),
        &vec![0; targets.len() * 8],
        make_st_info(StBind::Global, StType::Object),
        StVis::Default as u8,
        8,
        &relocations,
    );
}

/// Adds a global function whose body is encoded from `code`
fn add_function(elf_file: &mut ElfFile64, section: usize, name: &str, code: Vec<Instruction>) {
    elf_file.add_symbol_to_section(
        section,
        name.to_string(),
        &code,
        make_st_info(StBind::Global, StType::Func),
        StVis::Default as u8,
    );
}
