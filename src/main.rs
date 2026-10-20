mod codegen;
mod converter;
mod elf;
mod hangeul;
mod lexer;
mod parser;
mod samples;

use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::process::ExitCode;
use std::{env, process};

use converter::converter::Converter;
use parser::cfg::parser::Parser;
use parser::cfg::rules::RULES;
use parser::postparser::postparser::convert_to_ast_tree;

const OUTPUT: &str = "output.elf";

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("{err}");
        return ExitCode::FAILURE;
    }
    return ExitCode::SUCCESS;
}

/// The whole run; an error is its message, already formatted
fn run() -> Result<(), Box<dyn Error>> {
    let args = match parse_args()? {
        Command::Demo => {
            samples::write_demo(OUTPUT)?;
            println!("ELF written: {OUTPUT}");
            return Ok(());
        }
        Command::Compile(args) => args,
    };
    let main_parser = Parser::new(RULES);
    let mut objects = Vec::new();
    for path in &args.paths {
        objects.push(compile(&args, path, &main_parser)?);
    }
    if args.executable {
        link(objects)?;
    }

    Ok(())
}

/// Compiles `path` (dir/file.kr) into dir/file.o, and gives the path of the .o
fn compile(args: &Args, path: &str, main_parser: &Parser) -> Result<String, Box<dyn Error>> {
    let source = read_source(&args.program, path)?;
    let tokens = lexer::tokenize(&source).map_err(|err| {
        let (line, col) = err.span.line_col(&source);
        format!("{}:{line}:{col}: {err}", path)
    })?;
    let pre_parsed = parser::pre_parse(&tokens).map_err(|err| {
        let (line, col) = err.span.line_col(&source);
        format!("{}:{line}:{col}: {err}", path)
    })?;
    for warning in &pre_parsed.warnings {
        let (line, col) = warning.span.line_col(&source);
        eprintln!("{}:{line}:{col}: warning: {warning}", path);
    }
    let tree = main_parser.parse(&pre_parsed.tokens).map_err(|token| {
        let (line, col) = token.span.line_col(&source);
        format!(
            "{}:{line}:{col}: syntax error: unexpected {:?}",
            path, token.kind
        )
    })?;
    if args.tree {
        println!("{}", tree.show(RULES));
    }
    let ast = convert_to_ast_tree(&tree).map_err(|err| {
        let (line, col) = err.span.line_col(&source);
        format!("{}:{line}:{col}: {err}", path)
    })?;
    print!("{ast}");
    let ir = Converter::new(&ast).convert().map_err(|err| {
        let (line, col) = err.span.line_col(&source);
        format!("{}:{line}:{col}: {err}", path)
    })?;
    print!("{ir}");
    let object = Path::new(path)
        .with_extension("o")
        .to_string_lossy()
        .to_string();
    codegen::generation::generate(&ir).write(&mut File::create(&object)?)?;
    println!("ELF written: {object}");

    Ok(object)
}

fn link(mut file_names: Vec<String>) -> std::io::Result<()> {
    file_names.extend(["-o".to_string(), "a.out".to_string()]);

    let status = process::Command::new("gcc").args(&file_names).status()?; // Err si gcc est introuvable
    if !status.success() {
        return Err(std::io::Error::other(format!("gcc: {status}")));
    }
    return Ok(());
}

enum Command {
    /// `--demo`: the ELF of the encoder tests, as before the front-end; no source file
    Demo,
    Compile(Args),
}

struct Args {
    program: String,
    paths: Vec<String>,
    /// `--tree`: also print the parse tree, before the AST
    tree: bool,
    /// `-x`: also link the ELF into an executable, with gcc
    executable: bool,
}

/// `compiler [--tree] [-x] file.kr...` or `compiler --demo`; the usage on a wrong one
fn parse_args() -> Result<Command, String> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "compiler".to_string());
    let mut tree = false;
    let mut executable = false;
    let mut demo = false;
    let mut paths = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--tree" => tree = true,
            "-x" => executable = true,
            "--demo" => demo = true,
            _ => paths.push(arg),
        }
    }
    return match (demo, paths.is_empty()) {
        (true, true) if !tree && !executable => Ok(Command::Demo),
        (false, false) => Ok(Command::Compile(Args {
            program,
            paths,
            tree,
            executable,
        })),
        _ => Err(format!(
            "usage: {program} [--tree] [-x] <file.kr>...\n       {program} --demo"
        )),
    };
}

/// Reads the source file (UTF-8)
fn read_source(program: &str, path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| format!("{program}: {path}: {err}"))
}
