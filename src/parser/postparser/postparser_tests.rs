use std::sync::OnceLock;

use crate::lexer::span::Span;
use crate::lexer::tokenize;
use crate::parser::pre_parse;

use super::super::cfg::parser::Parser;
use super::super::cfg::rules::RULES;
use super::post_parse_error::PostParseErrorKind;
use super::postparser::convert_to_ast_tree;
use super::structs::{
    Argument, Assign, Expr, ExprKind, If, Program, Statement, StatementKind, Type,
};

/// Building the table takes seconds in debug: once for all the tests
fn parser() -> &'static Parser {
    static PARSER: OnceLock<Parser> = OnceLock::new();
    PARSER.get_or_init(|| Parser::new(RULES))
}

fn ast(source: &str) -> Program {
    let tokens = pre_parse(&tokenize(source).unwrap()).unwrap().tokens;
    let tree = parser().parse(&tokens).ok().unwrap();
    convert_to_ast_tree(&tree).unwrap()
}

/// The error of a program the parser accepts but the AST cannot hold, and the text it points at
fn error(source: &str) -> (PostParseErrorKind, &str) {
    let tokens = pre_parse(&tokenize(source).unwrap()).unwrap().tokens;
    let tree = parser().parse(&tokens).ok().unwrap();
    let Err(err) = convert_to_ast_tree(&tree) else {
        panic!("an error was expected");
    };
    (err.kind, text(source, err.span))
}

fn text(source: &str, span: Span) -> &str {
    &source[span.start..span.end]
}

/// The statements of the last function: the functions it calls are declared before it
fn body(program: &Program) -> &[Statement] {
    program.functions.last().unwrap().body.as_deref().unwrap()
}

#[test]
fn function_and_parameters_span_their_names() {
    let source = "정수를 반환하는 더하다(정수 가, 정수 나) {\n    (가 + 나)를 반환해요.\n}\n";
    let program = ast(source);
    let function = &program.functions[0];
    assert_eq!(text(source, function.span), "더하다");
    assert_eq!(text(source, function.params[0].span), "가");
    assert_eq!(text(source, function.params[1].span), "나");
}

#[test]
fn sentence_function_spans_its_verb() {
    let source = "정수 가와 정수 나를 받아 더하고 정수를 반환 {\n    (가 + 나)를 반환해요.\n}\n";
    let program = ast(source);
    let function = &program.functions[0];
    assert_eq!(function.name, "더하다");
    assert_eq!(text(source, function.span), "더하고");
    assert_eq!(text(source, function.params[0].span), "가");
    assert_eq!(text(source, function.params[1].span), "나");
}

#[test]
fn statement_spans_up_to_its_dot() {
    let source = "정수를 반환하는 main() {\n    결과는 0이에요.\n    결과를 반환해요.\n}\n";
    let program = ast(source);
    let [init, ret] = body(&program) else {
        panic!("two statements were expected");
    };
    assert_eq!(text(source, init.span), "결과는 0이에요.");
    assert_eq!(text(source, ret.span), "결과를 반환해요.");
}

#[test]
fn parentheses_are_in_the_span() {
    let source = "정수를 반환하는 더하다(정수 가, 정수 나) {\n    (가 + 나)를 반환해요.\n}\n";
    let program = ast(source);
    let StatementKind::Return(value) = &body(&program)[0].kind else {
        panic!("a return was expected");
    };
    assert_eq!(text(source, value.span), "(가 + 나)");
    let ExprKind::Binary { left, right, .. } = &value.kind else {
        panic!("a binary operation was expected");
    };
    assert_eq!(text(source, left.span), "가");
    assert_eq!(text(source, right.span), "나");
}

#[test]
fn negative_literal_spans_its_minus() {
    let source = "정수를 반환하는 main() {\n    -1을 반환해요.\n}\n";
    let program = ast(source);
    let StatementKind::Return(value) = &body(&program)[0].kind else {
        panic!("a return was expected");
    };
    assert!(matches!(value.kind, ExprKind::Int(-1)));
    assert_eq!(text(source, value.span), "-1");
}

/// The value of `Previous`, the first argument of a call
fn previous(call: &Expr) -> &Expr {
    let ExprKind::Call { args, .. } = &call.kind else {
        panic!("a call was expected");
    };
    let Argument::Previous(previous) = &args[0] else {
        panic!("the result of the step before was expected");
    };
    previous
}

#[test]
fn call_spans_from_the_start_of_its_chain() {
    let source = "외부 정수를 반환하는 더하다(정수 가, 정수 나)\n외부 정수를 반환하는 곱하다(정수 가, 정수 나)\n정수를 반환하는 main() {\n    결과는 0이에요.\n    3과 4를 더해서 5를 곱해서 결과에 넣어요.\n}\n";
    let program = ast(source);
    let assign = &body(&program)[1];
    assert_eq!(
        text(source, assign.span),
        "3과 4를 더해서 5를 곱해서 결과에 넣어요."
    );
    let StatementKind::Assign(Assign { value, .. }) = &assign.kind else {
        panic!("an assignment was expected");
    };
    assert_eq!(text(source, value.span), "3과 4를 더해서 5를 곱해서");
    assert_eq!(text(source, previous(value).span), "3과 4를 더해서");
}

#[test]
fn not_spans_its_operand() {
    let source = "정수를 반환하는 main() {\n    가는 0이에요.\n    만약 (가 > 0)이 아니면 {\n        1을 반환해요.\n    }\n    가를 반환해요.\n}\n";
    let program = ast(source);
    let StatementKind::If(If { condition, .. }) = &body(&program)[1].kind else {
        panic!("an if was expected");
    };
    let ExprKind::Unary { operand, .. } = &condition.kind else {
        panic!("a negation was expected");
    };
    assert_eq!(text(source, condition.span), "(가 > 0)");
    assert_eq!(text(source, operand.span), "(가 > 0)");
}

#[test]
fn else_if_spans_from_its_if() {
    let source = "정수를 반환하는 main() {\n    가는 0이에요.\n    만약 가 > 0면 {\n        1을 반환해요.\n    } 아니면 만약 가 == 0면 {\n        0을 반환해요.\n    }\n    가를 반환해요.\n}\n";
    let program = ast(source);
    let StatementKind::If(If { otherwise, .. }) = &body(&program)[1].kind else {
        panic!("an if was expected");
    };
    let [inner] = otherwise.as_deref().unwrap() else {
        panic!("one statement was expected in 아니면");
    };
    assert!(matches!(inner.kind, StatementKind::If(_)));
    assert_eq!(
        text(source, inner.span),
        "만약 가 == 0면 {\n        0을 반환해요.\n    }"
    );
}

/// `body` inside a `main` that declares 가
fn in_main(body: &str) -> String {
    format!("정수를 반환하는 main() {{\n    가는 0이에요.\n{body}\n    가를 반환해요.\n}}\n")
}

#[test]
fn unsigned_float_is_an_error_on_the_type() {
    let source = "정수를 반환하는 main(부호 없는 소수 가) {\n    0을 반환해요.\n}\n";
    assert_eq!(
        error(source),
        (PostParseErrorKind::UnsignedType, "부호 없는 소수")
    );
}

#[test]
fn short_bool_is_an_error_on_the_type() {
    let source = in_main("    나는 짧은 논리예요.");
    assert_eq!(error(&source), (PostParseErrorKind::ShortType, "짧은 논리"));
}

#[test]
fn constant_without_value_is_an_error_on_the_declaration() {
    let source = in_main("    고정된 나는 정수예요.");
    let kind = PostParseErrorKind::ConstantWithoutValue("나".to_string());
    assert_eq!(error(&source), (kind, "고정된 나는 정수예요."));
}

#[test]
fn second_value_of_give_is_the_error() {
    let source = in_main("    가를 2를 반환해요.");
    assert_eq!(error(&source), (PostParseErrorKind::GiveArguments, "2를"));
}

#[test]
fn give_without_value_is_an_error_on_the_chain() {
    let source = in_main("    반환해요.");
    assert_eq!(
        error(&source),
        (PostParseErrorKind::GiveArguments, "반환해요")
    );
}

#[test]
fn give_without_object_particle_is_the_error() {
    let source = in_main("    가에 반환해요.");
    assert_eq!(error(&source), (PostParseErrorKind::GiveArguments, "가에"));
}

#[test]
fn put_without_target_is_an_error_on_the_chain() {
    let source = in_main("    1을 넣어요.");
    assert_eq!(
        error(&source),
        (PostParseErrorKind::PutArguments, "1을 넣어요")
    );
}

#[test]
fn second_target_of_put_is_the_error() {
    let source = in_main("    1을 가에 가에 넣어요.");
    assert_eq!(error(&source), (PostParseErrorKind::PutArguments, "가에"));
}

#[test]
fn count_without_until_is_an_error_on_its_arguments() {
    let source = in_main("    가를 1부터 세면서 {\n    }");
    assert_eq!(
        error(&source),
        (PostParseErrorKind::CountArguments, "가를 1부터")
    );
}

#[test]
fn second_from_of_count_is_the_error() {
    let source = in_main("    가를 1부터 2부터 10까지 세면서 {\n    }");
    assert_eq!(
        error(&source),
        (PostParseErrorKind::CountArguments, "2부터")
    );
}

#[test]
fn declaration_out_of_count_is_the_error() {
    let source = format!(
        "외부 정수를 반환하는 더하다(정수 가)\n{}",
        in_main("    정수 나를 더해요.")
    );
    let kind = PostParseErrorKind::DeclaredOutOfCount("나".to_string());
    assert_eq!(error(&source), (kind, "정수 나를"));
}

#[test]
fn counted_group_gives_its_type_to_each_name() {
    let source =
        "두 짧은 정수 가와 나와 소수 라를 받아 섞고 소수를 반환 {\n    라를 반환해요.\n}\n";
    let program = ast(source);
    let function = &program.functions[0];
    assert_eq!(function.name, "섞다");
    assert!(matches!(
        function.return_type,
        Some(Type::Float { short: false })
    ));
    let params: Vec<(&str, &str)> = function
        .params
        .iter()
        .map(|param| (param.name.as_str(), text(source, param.span)))
        .collect();
    assert_eq!(params, [("가", "가"), ("나", "나"), ("라", "라")]);
    let short_int = |ty: &Type| {
        matches!(
            ty,
            Type::Int {
                short: true,
                unsigned: false
            }
        )
    };
    assert!(short_int(&function.params[0].ty) && short_int(&function.params[1].ty));
    assert!(matches!(
        function.params[2].ty,
        Type::Float { short: false }
    ));
}

#[test]
fn name_without_type_nor_counter_is_the_error() {
    let source = "정수 가와 나를 받아 더하고 정수를 반환 {\n    가를 반환해요.\n}\n";
    let kind = PostParseErrorKind::ParameterWithoutType("나".to_string());
    assert_eq!(error(source), (kind, "나"));
}

#[test]
fn name_past_the_counter_is_the_error() {
    let source = "두 정수 가와 나와 라를 받아 더하고 정수를 반환 {\n    가를 반환해요.\n}\n";
    let kind = PostParseErrorKind::CounterMismatch {
        expected: 2,
        found: 3,
    };
    assert_eq!(error(source), (kind, "두 정수 가와 나와 라"));
}

#[test]
fn counter_with_too_few_names_is_the_error() {
    let source = "세 정수 가와 나와 소수 라를 받아 더하고 정수를 반환 {\n    가를 반환해요.\n}\n";
    let kind = PostParseErrorKind::CounterMismatch {
        expected: 3,
        found: 2,
    };
    assert_eq!(error(source), (kind, "세 정수 가와 나"));
}
