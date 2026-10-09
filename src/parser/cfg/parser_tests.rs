use std::sync::OnceLock;

use crate::lexer::tokenize;
use crate::parser::parser_token::{ParserToken, ParserTokenKind};
use crate::parser::pre_parse;

use super::parser::{Parser, TreeNode};
use super::rules::RULES;
use super::structs::NTerm;

/// Building the table takes seconds in debug: once for all the tests
fn parser() -> &'static Parser {
    static PARSER: OnceLock<Parser> = OnceLock::new();
    PARSER.get_or_init(|| Parser::new(RULES))
}

/// The tokens of `source` after the lexer and the pre-parser
fn tokens(source: &str) -> Vec<ParserToken> {
    pre_parse(&tokenize(source).unwrap()).unwrap().tokens
}

/// The leaves from left to right
fn leaves<'a>(node: &TreeNode<'a>, out: &mut Vec<&'a ParserToken>) {
    match node {
        TreeNode::Leaf(token) => out.push(token),
        TreeNode::Branch { children, .. } => {
            for child in children {
                leaves(child, out);
            }
        }
    }
}

/// `cargo test tree_of_main -- --nocapture` to see the tree
#[test]
fn tree_of_main() {
    let tokens = tokens("정수를 주는 main() {\n    42를 줘요.\n}\n");
    let tree = parser().parse(&tokens).ok().unwrap();
    println!("{}", tree.show(RULES));

    let TreeNode::Branch { rule, .. } = &tree else {
        panic!("the root is a leaf");
    };
    assert_eq!(RULES[*rule].left, NTerm::Items);
    // Every token but Eof, in order: Eof is never shifted
    let mut found = Vec::new();
    leaves(&tree, &mut found);
    let kinds: Vec<&ParserTokenKind> = found.iter().map(|token| &token.kind).collect();
    let expected: Vec<&ParserTokenKind> = tokens[..tokens.len() - 1].iter().map(|token| &token.kind).collect();
    assert_eq!(kinds, expected);
}

#[test]
fn empty_file() {
    let tokens = tokens("");
    let Ok(TreeNode::Branch { children, .. }) = parser().parse(&tokens) else {
        panic!("an empty file is a valid program");
    };
    assert!(children.is_empty());
}

#[test]
fn proto_kr() {
    let source = std::fs::read_to_string("Proto.kr").unwrap();
    assert!(parser().parse(&tokens(&source)).is_ok());
}

#[test]
fn missing_brace() {
    let tokens = tokens("정수를 주는 main()\n    42를 줘요.\n}\n");
    let Err(token) = parser().parse(&tokens) else {
        panic!("a body without its {{ is accepted");
    };
    assert!(matches!(token.kind, ParserTokenKind::Int(42)));
}

#[test]
fn extra_brace() {
    let tokens = tokens("정수를 주는 main() {\n    42를 줘요.\n}\n}\n");
    let Err(token) = parser().parse(&tokens) else {
        panic!("a second }} is accepted");
    };
    assert_eq!(token.span.line_col("정수를 주는 main() {\n    42를 줘요.\n}\n}\n"), (4, 1));
}

#[test]
fn float_literals() {
    let source = "실수를 주는 main() {\n    가는 0.5예요.\n    -1.25를 줘요.\n}\n";
    let tokens = tokens(source);
    assert!(parser().parse(&tokens).is_ok());
    assert!(tokens.iter().any(|token| matches!(token.kind, ParserTokenKind::Float(0.5))));
}

#[test]
fn sentence_declarations() {
    let source = "정수를 주는 정수 가를 제곱하다 {\n    (가 * 가)를 줘요.\n}\n\
                  정수 하나와 실수 둘과 정수 셋을 인사하다 {\n}\n";
    assert!(parser().parse(&tokens(source)).is_ok());
}
