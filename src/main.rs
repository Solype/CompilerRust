mod elf;
mod hangeul;
mod lexer;
mod parser;
mod samples;

use std::fs;
use std::{env, process};

use parser::cfg::parser::Parser;
use parser::cfg::rules::RULES;
use parser::postparser::postparser::convert_to_ast_tree;

const OUTPUT: &str = "output.elf";

fn main() -> std::io::Result<()> {
    let args = match parse_args() {
        Command::Demo => {
            samples::write_demo(OUTPUT)?;
            println!("ELF written: {OUTPUT}");
            return Ok(());
        }
        Command::Compile(args) => args,
    };
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
            eprintln!(
                "{}:{line}:{col}: syntax error: unexpected {:?}",
                args.path, token.kind
            );
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

    Ok(())
}

enum Command {
    /// `--demo`: the ELF of the encoder tests, as before the front-end; no source file
    Demo,
    Compile(Args),
}

struct Args {
    program: String,
    path: String,
    /// `--tree`: also print the parse tree, before the AST
    tree: bool,
}

/// `compiler [--tree] file.kr` or `compiler --demo`, exits on a wrong usage
fn parse_args() -> Command {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "compiler".to_string());
    let mut tree = false;
    let mut demo = false;
    let mut paths = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--tree" => tree = true,
            "--demo" => demo = true,
            _ => paths.push(arg),
        }
    }
    return match (demo, <[String; 1]>::try_from(paths)) {
        (true, Err(paths)) if paths.is_empty() && !tree => Command::Demo,
        (false, Ok([path])) => Command::Compile(Args {
            program,
            path,
            tree,
        }),
        _ => {
            eprintln!("usage: {program} [--tree] <file.kr>\n       {program} --demo");
            process::exit(1);
        }
    };
}

/// Reads the source file (UTF-8), exits on error
fn read_source(args: &Args) -> String {
    fs::read_to_string(&args.path).unwrap_or_else(|err| {
        eprintln!("{}: {}: {err}", args.program, args.path);
        process::exit(84);
    })
}
