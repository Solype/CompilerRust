use std::collections::HashSet;

use crate::lexer::lexer_token::{Operator, Punctuation};
use crate::parser::parser_token::{Keyword, ParserTokenKind, Particle, Type};

use super::first::First;
use super::rules::RULES;
use super::structs::NTerm;

fn first_of(first: &First, non_terminal: NTerm) -> Vec<ParserTokenKind> {
    first.first[&non_terminal]
        .iter()
        .map(|kind| (*kind).clone())
        .collect()
}

#[test]
fn nullable_non_terminals() {
    let first = First::new(RULES);
    let expected = HashSet::from([
        NTerm::Items,
        NTerm::Parameters,
        NTerm::Statements,
        NTerm::OptionalDot,
    ]);
    assert_eq!(first.nullable, expected);
}

#[test]
fn every_non_terminal_starts_with_something() {
    let first = First::new(RULES);
    for rule in RULES {
        assert!(!first.first[&rule.left].is_empty(), "{:?}", rule.left);
    }
}

#[test]
fn first_of_optional_dot() {
    let first = First::new(RULES);
    assert_eq!(
        first_of(&first, NTerm::OptionalDot),
        [ParserTokenKind::Punctuation(Punctuation::Dot)]
    );
}

#[test]
fn first_of_statement() {
    let first = First::new(RULES);
    let set = first_of(&first, NTerm::Statement);
    for kind in [
        ParserTokenKind::Name(String::new()),
        ParserTokenKind::Int(0),
        ParserTokenKind::Keyword(Keyword::If),
        ParserTokenKind::Keyword(Keyword::Const),
        ParserTokenKind::Keyword(Keyword::Stop),
        ParserTokenKind::Keyword(Keyword::Return),
        ParserTokenKind::Type(Type::Int),
        ParserTokenKind::Operator(Operator::Minus),
        ParserTokenKind::Punctuation(Punctuation::LParen),
    ] {
        assert!(set.contains(&kind), "{kind:?}");
    }
    assert!(!set.contains(&ParserTokenKind::Punctuation(Punctuation::RBrace)));
    assert!(!set.contains(&ParserTokenKind::Particle(Particle::Object)));
}

#[test]
fn first_of_items_is_a_function_start() {
    let first = First::new(RULES);
    let set = first_of(&first, NTerm::Items);
    assert!(set.contains(&ParserTokenKind::Keyword(Keyword::Extern)));
    assert!(set.contains(&ParserTokenKind::Name(String::new())));
    assert!(set.contains(&ParserTokenKind::Type(Type::Int)));
    assert!(!set.contains(&ParserTokenKind::Eof));
}

#[test]
fn sequence_after_a_nullable_non_terminal() {
    let first = First::new(RULES);
    // StartSymbol → Items Eof: Items is nullable, so Eof is in FIRST of the sequence
    let start = RULES
        .iter()
        .find(|rule| rule.left == NTerm::StartSymbol)
        .unwrap();
    let (set, nullable) = first.of_sequence(start.right);
    assert!(set.contains(&&ParserTokenKind::Eof));
    assert!(!nullable);
}
