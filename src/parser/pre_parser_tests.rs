use crate::lexer::lexer_token::{Operator, Punctuation};
use crate::lexer::span::Span;
use crate::lexer::tokenize;

use super::parser_token::ParserTokenKind::{self, *};
use super::parser_token::{Ending, Keyword, Particle, Type};
use super::pre_parse;
use super::pre_parse_error::{PreParseError, PreParseErrorKind};
use super::pre_parse_warning::PreParseWarningKind;
use super::words::{
    KEYWORDS, NAME_PARTICULES, PARTICULES, has_particule, is_keyword, is_name_particle, is_particle,
};

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
fn every_name_particle_is_a_particle() {
    for (text, particle) in NAME_PARTICULES {
        assert_eq!(is_name_particle(text), Some(*particle), "{text}");
        assert_eq!(is_particle(text), Some(*particle), "{text}");
    }
}

#[test]
fn only_types_carry_ro_and_ui() {
    for word in ["으로", "로", "의", "나를", ""] {
        assert_eq!(is_name_particle(word), None, "{word:?}");
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
    let mut out = pre_parse(&tokens)
        .unwrap_or_else(|err| panic!("{src:?}: {err}"))
        .tokens;
    assert_eq!(out.pop().map(|t| t.kind), Some(Eof));
    out.into_iter().map(|t| t.kind).collect()
}

/// The error of `src`, which must fail
fn pre_err(src: &str) -> PreParseError {
    match pre_parse(&tokenize(src).unwrap()) {
        Ok(_) => panic!("{src:?}: no error"),
        Err(err) => err,
    }
}

/// The first word of `src` that is no keyword, type, verb or declared name
fn unknown(src: &str) -> String {
    let PreParseErrorKind::UnknownIdentifier(word) = pre_err(src).kind;
    word
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
    assert_eq!(pre("아닌"), [Keyword(Keyword::IsNot)]);
    assert_eq!(pre("주는"), [Keyword(Keyword::Giving)]);
    assert_eq!(pre("줘요"), [Keyword(Keyword::Return)]);
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
    let tokens = pre_parse(&tokenize("정수를").unwrap()).unwrap().tokens;
    assert_eq!(tokens[0].span, Span::new(0, 6));
    assert_eq!(tokens[1].span, Span::new(6, 9));
    assert_eq!(tokens[2].span, Span::new(9, 9));
}

#[test]
fn undeclared_words_are_errors_left_whole() {
    assert_eq!(unknown("나를"), "나를");
    // not 결 + 과: only the declared names can tell
    assert_eq!(unknown("결과"), "결과");
    // a particle alone, but not after a value: the parameter 가
    assert_eq!(unknown("(가 + 나)"), "가");
}

#[test]
fn return_42() {
    assert_eq!(
        pre("42를 줘요."),
        [
            Int(42),
            Particle(Particle::Object),
            Keyword(Keyword::Return),
            ParserTokenKind::Punctuation(Punctuation::Dot),
        ]
    );
}

fn verb(infinitive: &str, ending: Ending) -> ParserTokenKind {
    Verb {
        infinitive: infinitive.to_string(),
        ending,
    }
}

#[test]
fn declared_verb_in_every_form() {
    let tokens = pre("더하다() { 더해서 더해요 더한 }");
    assert_eq!(
        tokens,
        [
            verb("더하다", Ending::Da),
            ParserTokenKind::Punctuation(Punctuation::LParen),
            ParserTokenKind::Punctuation(Punctuation::RParen),
            ParserTokenKind::Punctuation(Punctuation::LBrace),
            verb("더하다", Ending::Seo),
            verb("더하다", Ending::Yo),
            verb("더하다", Ending::Adnominal),
            ParserTokenKind::Punctuation(Punctuation::RBrace),
        ]
    );
}

#[test]
fn verb_used_before_its_definition() {
    let tokens = pre("main() { 3을 제곱해서 줘요 } 제곱하다() { }");
    assert_eq!(tokens[6], verb("제곱하다", Ending::Seo));
}

#[test]
fn definition_with_or_without_return_type() {
    let tokens = pre("정수를 주는 더하다(정수 가, 정수 나) { }");
    assert_eq!(tokens[3], verb("더하다", Ending::Da));
    // nothing before it: no return value
    assert_eq!(pre("인사하다() { }")[0], verb("인사하다", Ending::Da));
}

#[test]
fn builtin_verbs() {
    assert_eq!(pre("넣어요"), [verb("넣다", Ending::Yo)]);
    assert_eq!(pre("넣어서"), [verb("넣다", Ending::Seo)]);
    assert_eq!(pre("바꿔서"), [verb("바꾸다", Ending::Seo)]);
}

#[test]
fn never_declared_is_not_a_verb() {
    assert_eq!(unknown("더해서"), "더해서");
}

#[test]
fn every_word_ending_in_da_is_a_verb() {
    // wherever it is, with or without parentheses: 다 marks the functions
    assert_eq!(pre("바다"), [verb("바다", Ending::Da)]);
    assert_eq!(pre("main() { 바다 }")[4], verb("바다", Ending::Da));
    // and its forms are verbs too
    assert_eq!(pre("만들다 만든")[1], verb("만들다", Ending::Adnominal));
}

fn warnings(src: &str) -> Vec<PreParseWarningKind> {
    let tokens = tokenize(src).unwrap();
    let warnings = pre_parse(&tokens).unwrap().warnings;
    warnings.into_iter().map(|w| w.kind).collect()
}

#[test]
fn verb_without_parentheses_is_a_warning() {
    assert_eq!(
        warnings("바다() { } 바다"),
        [PreParseWarningKind::VerbWithoutParentheses(
            "바다".to_string()
        )]
    );
    assert!(warnings("정수를 주는 더하다(정수 가) { }").is_empty());
    assert!(warnings("인사하다() { 인사해요 }").is_empty());
}

/// The names declared in `src`, sorted and without duplicates, like `expected`: every
/// declared name is a Hangul `Name` in the output, the Latin ones (main, printf) aside
fn assert_names(src: &str, expected: &[&str]) {
    let tokens = pre_parse(&tokenize(src).unwrap()).unwrap().tokens;
    let mut names: Vec<String> = tokens
        .into_iter()
        .filter_map(|token| match token.kind {
            Name(name) if !name.is_ascii() => Some(name),
            _ => None,
        })
        .collect();
    names.sort();
    names.dedup();
    let mut expected: Vec<String> = expected.iter().map(|name| name.to_string()).collect();
    expected.sort();
    assert_eq!(names, expected, "{src:?}");
}

#[test]
fn variable_with_a_value_or_a_type() {
    assert_names("결과는 0이에요.", &["결과"]);
    assert_names("수는 정수예요.", &["수"]);
    assert_names("가는 1이에요. 결과는 가예요.", &["가", "결과"]);
}

#[test]
fn variable_with_a_longer_value() {
    assert_names("이름은 문자 주소예요.", &["이름"]);
    assert_names("점수는 정수 10개예요.", &["점수"]);
    assert_names("합은 3과 4를 더한 값이에요. 더하다() { }", &["합"]);
}

#[test]
fn variable_at_the_start_of_any_statement() {
    assert_names("main() { 결과는 0이에요. }", &["결과"]);
    assert_names("만약 1 > 0이면 { } 결과는 0이에요.", &["결과"]);
    assert_names("고정된 끝은 10이에요.", &["끝"]);
    assert_names("0을 줘요 결과는 0이에요", &["결과"]);
    assert_names(
        "결과는 0이에요. 결과에 0을 넣어요 수는 1이에요",
        &["결과", "수"],
    );
    // after 이에요 without a `.`
    assert_names("가는 1이에요 나는 2예요", &["가", "나"]);
}

#[test]
fn not_a_variable_declaration() {
    // not at the start of a statement: 없는 is not 없 + 는, so it is never declared
    assert_eq!(unknown("가는 1이에요. 가 없는 정수"), "없는");
    assert_eq!(unknown("3과 결과는"), "결과는");
    // a keyword, and a particle with no name before it
    assert_names("주는", &[]);
    assert_eq!(unknown("는 0이에요"), "는");
}

#[test]
fn every_name_of_the_prototype() {
    assert_names(
        include_str!("../../Proto.kr"),
        &["형식", "가", "나", "끝", "결과", "수", "개수", "숫자"],
    );
}

#[test]
fn declaration_is_split_in_the_output() {
    let tokens = pre_parse(&tokenize("결과는 0이에요. 인사하다(정수 가) { }").unwrap())
        .unwrap()
        .tokens;
    // 결과는 at bytes 0..9 becomes 결과 0..6 and 는 6..9
    assert_eq!(tokens[0].kind, Name("결과".to_string()));
    assert_eq!(tokens[0].span, Span::new(0, 6));
    assert_eq!(tokens[1].kind, Particle(Particle::Topic));
    assert_eq!(tokens[1].span, Span::new(6, 9));
    // the tokens after it moved by one: 0, 이에요, `.`
    assert_eq!(tokens[2].kind, Int(0));
    // a parameter is the whole word: nothing to split
    assert_eq!(tokens[8].kind, Name("가".to_string()));
}

#[test]
fn loop_variable_is_split_in_the_output() {
    let tokens = pre("정수 칸을 0부터 10까지 세면서 { }");
    assert_eq!(
        tokens[..3],
        [
            Type(Type::Int),
            Name("칸".to_string()),
            Particle(Particle::Object)
        ]
    );
    // no type before 결과를: not a declaration, left whole
    assert_eq!(
        unknown("정수 칸을 0부터 10까지 세면서 { } 결과를 줘요"),
        "결과를"
    );
}

#[test]
fn every_split_of_a_file() {
    // split from the last index down, so that the first indices stay right
    let tokens = pre("가는 1이에요 나는 2예요");
    assert_eq!(
        tokens,
        [
            Name("가".to_string()),
            Particle(Particle::Topic),
            Int(1),
            Particle(Particle::ItIs),
            Name("나".to_string()),
            Particle(Particle::Topic),
            Int(2),
            Particle(Particle::ItIs),
        ]
    );
}

fn name(text: &str) -> ParserTokenKind {
    Name(text.to_string())
}

#[test]
fn used_names_are_split() {
    let tokens = pre("인사하다(정수 가, 정수 나) { 가 나를 가에 }");
    assert_eq!(
        tokens[9..],
        [
            name("가"),
            name("나"),
            Particle(Particle::Object),
            name("가"),
            Particle(Particle::In),
            ParserTokenKind::Punctuation(Punctuation::RBrace),
        ]
    );
}

#[test]
fn used_before_its_declaration() {
    assert_eq!(
        pre("결과를 줘요. 결과는 0이에요.")[..2],
        [name("결과"), Particle(Particle::Object)]
    );
}

#[test]
fn longest_declared_name_wins() {
    // 가나 is 가 + 나, but 나 is no particle: both names exist
    let tokens = pre("가는 1이에요. 가나는 2예요. 가나를 가를");
    assert_eq!(tokens[10..12], [name("가나"), Particle(Particle::Object)]);
    assert_eq!(tokens[12..], [name("가"), Particle(Particle::Object)]);
}

#[test]
fn only_name_particles_split_a_name() {
    // 로 and 의 only follow a type; 결과는 not at the start of a statement stays whole too
    assert_eq!(unknown("결과는 0이에요. 결과로"), "결과로");
    assert_eq!(unknown("결과는 0이에요. 결과의"), "결과의");
}

#[test]
fn undeclared_word_is_an_error() {
    let err = pre_err("결과는 0이에요. 모름을");
    assert_eq!(
        err.kind,
        PreParseErrorKind::UnknownIdentifier("모름을".to_string())
    );
    // the whole word: 모름을 at bytes 22..31
    assert_eq!(err.span, Span::new(22, 31));
}

#[test]
fn no_identifier_left_in_the_prototype() {
    let tokens = pre(include_str!("../../Proto.kr"));
    let idents: Vec<_> = tokens
        .iter()
        .filter(|kind| matches!(kind, Ident(_)))
        .collect();
    assert!(idents.is_empty(), "{idents:?}");
}

#[test]
fn particle_after_a_latin_name() {
    assert_eq!(pre("main을"), [name("main"), Particle(Particle::Object)]);
    assert_eq!(
        pre("가는 printf예요")[2..],
        [name("printf"), Particle(Particle::ItIs)]
    );
    // 해요 is a verb form first: a call, not a particle
    assert_eq!(
        pre("printf해요"),
        [name("printf"), verb("하다", Ending::Yo)]
    );
}

#[test]
fn ampersand_after_a_type_is_an_address() {
    assert_eq!(pre("정수&"), [Type(Type::Int), Type(Type::Address)]);
    // repeatable: each `&` is a type, so the next one follows a type too
    assert_eq!(
        pre("정수&&"),
        [Type(Type::Int), Type(Type::Address), Type(Type::Address)]
    );
    // anywhere else, the operator
    assert_eq!(
        pre("1 & 2"),
        [Int(1), ParserTokenKind::Operator(Operator::Amp), Int(2)]
    );
    let tokens = pre_parse(&tokenize("정수&").unwrap()).unwrap().tokens;
    assert_eq!(tokens[1].span, Span::new(6, 7));
}

#[test]
fn parameter_after_an_ampersand() {
    assert_names("인사하다(정수& 가, 문자 && 나) { }", &["가", "나"]);
}

#[test]
fn unsigned_is_merged() {
    assert_eq!(
        pre("부호 없는 정수"),
        [Keyword(Keyword::Unsigned), Type(Type::Int)]
    );
    // one token over both words: 부호 0..6, space, 없는 7..13
    let tokens = pre_parse(&tokenize("부호 없는 정수").unwrap())
        .unwrap()
        .tokens;
    assert_eq!(tokens[0].span, Span::new(0, 13));
    // either word alone is nothing
    assert_eq!(unknown("부호 정수"), "부호");
    assert_eq!(unknown("가는 1이에요. 가 없는 정수"), "없는");
}

#[test]
fn booleans() {
    assert_eq!(pre("참"), [Bool(true)]);
    assert_eq!(
        pre("거짓을 줘요"),
        [
            Bool(false),
            Particle(Particle::Object),
            Keyword(Keyword::Return)
        ]
    );
    assert_eq!(pre("참이면"), [Bool(true), Particle(Particle::IfItIs)]);
}
