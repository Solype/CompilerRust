use crate::lexer::lexer_token::{Operator, Punctuation};
use crate::lexer::span::Span;
use crate::lexer::tokenize;

use super::parser_token::ParserTokenKind::{self, *};
use super::parser_token::{Keyword, Particle, Type};
use super::pre_parse;
use super::pre_parse_error::PreParseErrorKind;
use super::words::{KEYWORDS, PARTICULES, has_particule, is_keyword, is_particle};

#[test]
fn every_keyword_is_found() {
    for (text, keyword) in KEYWORDS {
        assert_eq!(is_keyword(text), Some(*keyword), "{text}");
    }
    assert_eq!(is_keyword("주는"), Some(Keyword::Giving));
    assert_eq!(is_keyword("값"), Some(Keyword::Value));
}

#[test]
fn only_whole_words_are_keywords() {
    for word in ["정수", "정수를", "값이에요", "만약에", "주", "", "main"] {
        assert_eq!(is_keyword(word), None, "{word:?}");
    }
}

#[test]
fn every_particle_is_found() {
    for (text, particle) in PARTICULES {
        assert_eq!(is_particle(text), Some(*particle), "{text}");
    }
}

#[test]
fn both_spellings_give_the_same_particle() {
    let pairs = [
        ("을", "를"),
        ("이", "가"),
        ("은", "는"),
        ("과", "와"),
        ("으로", "로"),
        ("이면", "면"),
        ("이에요", "예요"),
    ];
    for (after_batchim, after_vowel) in pairs {
        assert_eq!(
            is_particle(after_batchim),
            is_particle(after_vowel),
            "{after_batchim}"
        );
        assert!(is_particle(after_batchim).is_some(), "{after_batchim}");
    }
    assert_eq!(is_particle("를"), Some(Particle::Object));
    assert_eq!(is_particle("이에요"), Some(Particle::ItIs));
}

#[test]
fn only_whole_particles() {
    for word in ["나를", "정수를", "이에", "에요", "줘요", "", "을를"] {
        assert_eq!(is_particle(word), None, "{word:?}");
    }
}

#[test]
fn particle_at_the_end() {
    use Particle::{In, ItIs, Object, To, Topic};
    let cases = [
        ("합은", ("합", Topic)),
        ("나를", ("나", Object)),
        ("정수를", ("정수", Object)),
        ("값이에요", ("값", ItIs)),
        ("정수예요", ("정수", ItIs)),
        ("서울로", ("서울", To)),
        ("결과에", ("결과", In)),
        ("를", ("", Object)),
    ];
    for (word, expected) in cases {
        assert_eq!(has_particule(word), Some(expected), "{word}");
    }
}

#[test]
fn longest_particle_wins() {
    assert_eq!(has_particule("사이면"), Some(("사", Particle::IfItIs)));
    assert_eq!(has_particule("집으로"), Some(("집", Particle::To)));
}

#[test]
fn no_particle_at_the_end() {
    for word in ["정수", "만약", "줘요", "main", ""] {
        assert_eq!(has_particule(word), None, "{word:?}");
    }
}

/// The parser token kinds of `src`, without the final `Eof`
fn pre(src: &str) -> Vec<ParserTokenKind> {
    let tokens = tokenize(src).unwrap();
    let mut out = pre_parse(&tokens).unwrap_or_else(|err| panic!("{src:?}: {err}"));
    assert_eq!(out.pop().map(|t| t.kind), Some(Eof));
    out.into_iter().map(|t| t.kind).collect()
}

fn pre_error(src: &str) -> PreParseErrorKind {
    let tokens = tokenize(src).unwrap();
    pre_parse(&tokens)
        .err()
        .unwrap_or_else(|| panic!("{src:?} should fail"))
        .kind
}

#[test]
fn function_header() {
    assert_eq!(
        pre("정수를 주는 main()"),
        [
            Type(Type::Int),
            Particle(Particle::Object),
            Keyword(Keyword::Giving),
            Name("main".to_string()),
            ParserTokenKind::Punctuation(Punctuation::LParen),
            ParserTokenKind::Punctuation(Punctuation::RParen),
        ]
    );
}

#[test]
fn particle_after_a_value() {
    assert_eq!(pre("42를"), [Int(42), Particle(Particle::Object)]);
    assert_eq!(pre("0이에요"), [Int(0), Particle(Particle::ItIs)]);
    assert_eq!(
        pre(r#""%ld\n"과"#),
        [Str("%ld\n".to_string()), Particle(Particle::With)]
    );
    assert_eq!(
        pre("(1 + 2)를"),
        [
            ParserTokenKind::Punctuation(Punctuation::LParen),
            Int(1),
            ParserTokenKind::Operator(Operator::Plus),
            Int(2),
            ParserTokenKind::Punctuation(Punctuation::RParen),
            Particle(Particle::Object),
        ]
    );
}

#[test]
fn keywords_are_not_split() {
    assert_eq!(pre("만약"), [Keyword(Keyword::If)]);
    assert_eq!(pre("아니면"), [Keyword(Keyword::Otherwise)]);
    assert_eq!(pre("주는"), [Keyword(Keyword::Giving)]);
}

#[test]
fn keyword_or_type_then_particle() {
    assert_eq!(
        pre("정수의 크기"),
        [
            Type(Type::Int),
            Particle(Particle::Of),
            Keyword(Keyword::Size)
        ]
    );
    assert_eq!(
        pre("값이에요"),
        [Keyword(Keyword::Value), Particle(Particle::ItIs)]
    );
    // 개 after a number is a keyword, not a particle
    assert_eq!(
        pre("10개예요"),
        [Int(10), Keyword(Keyword::Count), Particle(Particle::ItIs)]
    );
}

#[test]
fn split_word_spans() {
    let tokens = pre_parse(&tokenize("정수를").unwrap()).unwrap();
    assert_eq!(tokens[0].span, Span::new(0, 6));
    assert_eq!(tokens[1].span, Span::new(6, 9));
    assert_eq!(tokens[2].span, Span::new(9, 9));
}

#[test]
fn unknown_words() {
    assert_eq!(
        pre_error("사과를"),
        PreParseErrorKind::UnknownWord("사과를".to_string())
    );
    // a particle alone, but not after a value
    assert_eq!(
        pre_error("를"),
        PreParseErrorKind::UnknownWord("를".to_string())
    );
}
