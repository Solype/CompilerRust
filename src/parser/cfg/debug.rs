use std::io::{BufRead, Write};

use super::super::parser_token::ParserTokenKind;

use super::structs::{ParserNode, ParserProduction};

/// `LR_DEBUG=1`: only the transitions between states
/// `LR_DEBUG=closure`: also each closure, step by step, with Enter to go on
/// `LR_DEBUG=closure cargo test closure_step_by_step -- --ignored --nocapture`
pub fn transitions() -> bool {
    std::env::var_os("LR_DEBUG").is_some()
}

pub fn closure() -> bool {
    std::env::var("LR_DEBUG").is_ok_and(|value| value == "closure")
}

/// Waits for Enter, like a debugger's "next"
pub fn pause() {
    print!("        [Entrée pour continuer]");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
}

/// `ReturnType → 정수 · 를 주는   [Name]`
pub fn item(rule: &ParserProduction, point: usize, lookahead: &ParserTokenKind) -> String {
    let mut right: Vec<String> = rule.right.iter().map(node).collect();
    right.insert(point, "·".to_string());
    format!(
        "{:?} → {}   [{}]",
        rule.left,
        right.join(" "),
        term(lookahead)
    )
}

pub fn nodes(nodes: &[ParserNode]) -> String {
    let shown: Vec<String> = nodes.iter().map(node).collect();
    format!("[{}]", shown.join(", "))
}

pub fn terms(kinds: &[&ParserTokenKind]) -> String {
    let shown: Vec<String> = kinds.iter().map(|kind| term(kind)).collect();
    format!("{{{}}}", shown.join(", "))
}

pub fn node(node: &ParserNode) -> String {
    match node {
        ParserNode::Term(kind) => term(kind),
        ParserNode::NTerm(non_terminal) => format!("{non_terminal:?}"),
    }
}

/// The kind of terminal only: the values in the rules are placeholders
pub fn term(kind: &ParserTokenKind) -> String {
    match kind {
        ParserTokenKind::Keyword(keyword) => format!("{keyword:?}"),
        ParserTokenKind::Type(ty) => format!("Type({ty:?})"),
        ParserTokenKind::Name(_) => "Name".to_string(),
        ParserTokenKind::Ident(_) => "Ident".to_string(),
        ParserTokenKind::Verb { ending, .. } => format!("Verb({ending:?})"),
        ParserTokenKind::Particle(particle) => format!("Particle({particle:?})"),
        ParserTokenKind::Int(_) => "Int".to_string(),
        ParserTokenKind::Str(_) => "Str".to_string(),
        ParserTokenKind::Bool(_) => "Bool".to_string(),
        ParserTokenKind::Punctuation(punctuation) => format!("{punctuation:?}"),
        ParserTokenKind::Operator(operator) => format!("{operator:?}"),
        ParserTokenKind::Eof => "Eof".to_string(),
    }
}
