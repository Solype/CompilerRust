use super::lexer_error::LexErrorKind::*;
use super::span::Span;
use super::lexer_token::Operator::*;
use super::lexer_token::Punctuation::*;
use super::lexer_token::LexerTokenKind::{self, *};
use super::tokenize;

/// The kinds of `src`, without the final `Eof`
fn kinds(src: &str) -> Vec<LexerTokenKind> {
    let mut tokens = tokenize(src).unwrap_or_else(|err| panic!("{src:?}: {err}"));
    assert_eq!(tokens.pop().map(|t| t.kind), Some(Eof));
    tokens.into_iter().map(|t| t.kind).collect()
}

fn hangul(word: &str) -> LexerTokenKind {
    HangulWord(word.to_string())
}

fn latin(word: &str) -> LexerTokenKind {
    LatinWord(word.to_string())
}

fn error(src: &str) -> (super::lexer_error::LexErrorKind, Span) {
    let err = tokenize(src)
        .err()
        .unwrap_or_else(|| panic!("{src:?} should fail"));
    (err.kind, err.span)
}

#[test]
fn return_with_dot() {
    assert_eq!(
        kinds("42를 줘요."),
        [Int(42), hangul("를"), hangul("줘요"), Punctuation(Dot)]
    );
}

#[test]
fn script_change_splits_words() {
    assert_eq!(kinds("printf해요"), [latin("printf"), hangul("해요")]);
}

#[test]
fn minus_is_its_own_token() {
    assert_eq!(kinds("가-5"), [hangul("가"), Operator(Minus), Int(5)]);
    assert_eq!(kinds("-5를"), [Operator(Minus), Int(5), hangul("를")]);
}

#[test]
fn particle_after_paren() {
    assert_eq!(
        kinds("(가 + 나)를"),
        [
            Punctuation(LParen),
            hangul("가"),
            Operator(Plus),
            hangul("나"),
            Punctuation(RParen),
            hangul("를")
        ]
    );
}

#[test]
fn two_char_operators() {
    assert_eq!(
        kinds("가 <= 나"),
        [hangul("가"), Operator(Le), hangul("나")]
    );
    assert_eq!(
        kinds("1 << 2 >> 3"),
        [Int(1), Operator(Shl), Int(2), Operator(Shr), Int(3)]
    );
    assert_eq!(
        kinds("가 == 나 != 다"),
        [
            hangul("가"),
            Operator(EqEq),
            hangul("나"),
            Operator(NotEq),
            hangul("다")
        ]
    );
    assert_eq!(
        kinds("< > >= %"),
        [Operator(Lt), Operator(Gt), Operator(Ge), Operator(Percent)]
    );
}

#[test]
fn one_char_operators() {
    assert_eq!(
        kinds("+-*/%&|^~"),
        [
            Operator(Plus),
            Operator(Minus),
            Operator(Star),
            Operator(Slash),
            Operator(Percent),
            Operator(Amp),
            Operator(Pipe),
            Operator(Caret),
            Operator(Tilde)
        ]
    );
}

/// `&` after a type is `주소`: the lexer only sees an `Amp`, the parser decides
#[test]
fn pointer_ampersand() {
    assert_eq!(
        kinds("문자& 형식"),
        [hangul("문자"), Operator(Amp), hangul("형식")]
    );
    assert_eq!(
        kinds("정수&&"),
        [hangul("정수"), Operator(Amp), Operator(Amp)]
    );
}

#[test]
fn string_is_decoded() {
    assert_eq!(
        kinds(r#""%ld\n"과"#),
        [Str("%ld\n".to_string()), hangul("과")]
    );
    assert_eq!(kinds(r#""\t\0\\\"""#), [Str("\t\0\\\"".to_string())]);
}

#[test]
fn ellipsis_both_spellings() {
    let expected = [
        Punctuation(LParen),
        hangul("형식"),
        Punctuation(Comma),
        Punctuation(Ellipsis),
        Punctuation(RParen),
    ];
    assert_eq!(kinds("(형식, …)"), expected);
    assert_eq!(kinds("(형식, ...)"), expected);
}

#[test]
fn comments_are_skipped() {
    assert_eq!(kinds("/* a */ 1 // b\n2"), [Int(1), Int(2)]);
    assert_eq!(kinds("1 / 2"), [Int(1), Operator(Slash), Int(2)]);
}

#[test]
fn hangul_span_in_bytes() {
    let tokens = tokenize("정수를").unwrap();
    assert_eq!(tokens[0].span, Span::new(0, 9));
    assert_eq!(tokens[1].span, Span::new(9, 9));
}

#[test]
fn empty_source_is_only_eof() {
    assert_eq!(kinds(""), []);
    assert_eq!(kinds("  // nothing\n"), []);
}

#[test]
fn proto_kr_lexes() {
    let src = include_str!("../../Proto.kr");
    let tokens = kinds(src);
    assert!(tokens.contains(&latin("printf")));
    assert!(tokens.contains(&Punctuation(Ellipsis)));
}

#[test]
fn unterminated_comment() {
    assert_eq!(error("/* abc"), (UnterminatedComment, Span::new(0, 2)));
}

#[test]
fn unterminated_string() {
    assert_eq!(error("\"abc"), (UnterminatedString, Span::new(0, 1)));
    assert_eq!(error("\"a\nb\""), (UnterminatedString, Span::new(0, 1)));
}

#[test]
fn invalid_escape() {
    assert_eq!(error(r#""\q""#), (InvalidEscape('q'), Span::new(1, 3)));
}

#[test]
fn lone_equals_and_bang() {
    assert_eq!(error("가 = 1"), (LoneEquals, Span::new(4, 5)));
    assert_eq!(error("!가"), (LoneBang, Span::new(0, 1)));
}

#[test]
fn integer_overflow() {
    let src = "99999999999999999999";
    assert_eq!(error(src), (IntegerOverflow, Span::new(0, src.len())));
    assert_eq!(kinds("9223372036854775807"), [Int(i64::MAX)]);
}

#[test]
fn unexpected_char_and_loose_jamo() {
    assert_eq!(error("#"), (UnexpectedChar('#'), Span::new(0, 1)));
    assert_eq!(error("ㄱ"), (LooseJamo('ㄱ'), Span::new(0, 3)));
}
