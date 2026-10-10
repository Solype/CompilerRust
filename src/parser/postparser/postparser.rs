use crate::lexer::lexer_token::Operator;
use crate::lexer::span::Span;

use super::super::cfg::parser::TreeNode;
use super::super::cfg::rules::RULES;
use super::super::cfg::structs::NTerm;
use super::super::parser_token::{Keyword, ParserTokenKind, Particle, Type as TokenType};

use super::post_parse_error::{PostParseError, PostParseErrorKind};
use super::structs::{
    Argument, BinaryOp, Expr, ExprKind, Function, Param, Program, Statement, StatementKind, Type,
    UnaryOp,
};

/// The parse tree of `Parser::parse` (its `Items` root) as an AST. Each function below converts
/// one non-terminal: the rules it handles are in its comment, and the children it gets follow
/// their right side. A program the grammar accepts but the AST cannot hold (`부호 없는 소수`,
/// `줘요` with two values…) is an error at the first such place; a tree that does not follow
/// the rules panics
pub fn convert_to_ast_tree(tree: &TreeNode) -> Result<Program, PostParseError> {
    let mut prog = Program::default();
    prog.functions = items(tree)?;
    return Ok(prog);
}

/// The left side of the rule of a branch, and its children
fn branch<'t, 'a>(node: &'t TreeNode<'a>) -> (NTerm, &'t [TreeNode<'a>]) {
    return match node {
        TreeNode::Branch { rule, children } => (RULES[*rule].left, children),
        TreeNode::Leaf(token) => panic!("a rule was expected, found the token {:?}", token.kind),
    };
}

/// The token of a leaf
fn token<'a>(node: &TreeNode<'a>) -> &'a ParserTokenKind {
    return match node {
        TreeNode::Leaf(token) => &token.kind,
        TreeNode::Branch { rule, .. } => {
            panic!("a token was expected, found {:?}", RULES[*rule].left)
        }
    };
}

/// The source of a node that is not empty
fn span(node: &TreeNode) -> Span {
    return node.span().expect("an empty rule has no span");
}

fn name(node: &TreeNode) -> String {
    return match token(node) {
        ParserTokenKind::Name(name) => name.clone(),
        kind => panic!("a name was expected, found {kind:?}"),
    };
}

fn verb(node: &TreeNode) -> String {
    return match token(node) {
        ParserTokenKind::Verb { infinitive, .. } => infinitive.clone(),
        kind => panic!("a verb was expected, found {kind:?}"),
    };
}

////////////////////////////////
// Functions
////////////////////////////////

/// Items → Items Item | ε
fn items(node: &TreeNode) -> Result<Vec<Function>, PostParseError> {
    return match branch(node).1 {
        [] => Ok(Vec::new()),
        [before, item] => {
            let mut functions = items(before)?;
            functions.push(function(item)?);
            Ok(functions)
        }
        _ => unreachable!(),
    };
}

/// Item → Function | ExternFunction
fn function(node: &TreeNode) -> Result<Function, PostParseError> {
    let [inner] = branch(node).1 else {
        unreachable!()
    };
    let (head, params, body) = match branch(inner) {
        // Function → FunctionHead ( Parameters ) Block
        (NTerm::Function, [head, _, params, _, body]) => (head, params, Some(block(body)?)),
        // ExternFunction → 외부 FunctionHead ( Parameters )
        (NTerm::ExternFunction, [_, head, _, params, _]) => (head, params, None),
        // Function → ReturnType SentenceParameters VERB-다 Block
        (NTerm::Function, [return_type, params, name, body]) => {
            return Ok(Function {
                name: verb(name),
                span: span(name),
                return_type: Some(type_spec(&branch(return_type).1[0])?),
                params: sentence_parameters(params)?,
                variadic: false,
                body: Some(block(body)?),
            });
        }
        // Function → SentenceParameters VERB-다 Block
        (NTerm::Function, [params, name, body]) => {
            return Ok(Function {
                name: verb(name),
                span: span(name),
                return_type: None,
                params: sentence_parameters(params)?,
                variadic: false,
                body: Some(block(body)?),
            });
        }
        _ => unreachable!(),
    };
    let (name, name_span, return_type) = function_head(head)?;
    let (params, variadic) = parameters(params)?;
    return Ok(Function {
        name,
        span: name_span,
        return_type,
        params,
        variadic,
        body,
    });
}

/// FunctionHead → ReturnType FunctionName | FunctionName
/// ReturnType → TypeSpec 를 주는
/// FunctionName → VERB-다 | Name
fn function_head(node: &TreeNode) -> Result<(String, Span, Option<Type>), PostParseError> {
    let (return_type, function_name) = match branch(node).1 {
        [return_type, function_name] => {
            (Some(type_spec(&branch(return_type).1[0])?), function_name)
        }
        [function_name] => (None, function_name),
        _ => unreachable!(),
    };
    let [leaf] = branch(function_name).1 else {
        unreachable!()
    };
    let name = match token(leaf) {
        ParserTokenKind::Verb { infinitive, .. } => infinitive.clone(),
        _ => name(leaf),
    };
    return Ok((name, span(leaf), return_type));
}

/// Parameters → ε | ParameterList | ParameterList , …
/// ParameterList → Parameter | ParameterList , Parameter
/// Parameter → TypeSpec Name
fn parameters(node: &TreeNode) -> Result<(Vec<Param>, bool), PostParseError> {
    return match branch(node).1 {
        [] => Ok((Vec::new(), false)),
        [list] => Ok((parameter_list(list)?, false)),
        [list, _, _] => Ok((parameter_list(list)?, true)),
        _ => unreachable!(),
    };
}

fn parameter_list(node: &TreeNode) -> Result<Vec<Param>, PostParseError> {
    let (mut params, last) = match branch(node).1 {
        [last] => (Vec::new(), last),
        [before, _, last] => (parameter_list(before)?, last),
        _ => unreachable!(),
    };
    params.push(parameter(last)?);
    return Ok(params);
}

/// Parameter → TypeSpec Name
fn parameter(node: &TreeNode) -> Result<Param, PostParseError> {
    let [ty, param_name] = branch(node).1 else {
        unreachable!()
    };
    return Ok(Param {
        ty: type_spec(ty)?,
        name: name(param_name),
        span: span(param_name),
    });
}

/// SentenceParameters → Parameter 를 | SentenceLinks Parameter 를
/// SentenceLinks → Parameter 와 | SentenceLinks Parameter 와
fn sentence_parameters(node: &TreeNode) -> Result<Vec<Param>, PostParseError> {
    let (mut params, last) = match branch(node).1 {
        [last, _] => (Vec::new(), last),
        [before, last, _] => (sentence_parameters(before)?, last),
        _ => unreachable!(),
    };
    params.push(parameter(last)?);
    return Ok(params);
}

////////////////////////////////
// Types
////////////////////////////////

/// TypeSpec → BaseType | 짧은 BaseType | 부호 없는 BaseType | TypeSpec & | TypeSpec N 개
fn type_spec(node: &TreeNode) -> Result<Type, PostParseError> {
    return match branch(node).1 {
        [base] => base_type(base, None, span(node)),
        [TreeNode::Leaf(modifier), base] => base_type(base, Some(&modifier.kind), span(node)),
        [inner, _] => Ok(Type::Address(Box::new(type_spec(inner)?))),
        [inner, count, _] => match token(count) {
            ParserTokenKind::Int(count) => {
                Ok(Type::Array(Box::new(type_spec(inner)?), *count as usize))
            }
            kind => panic!("a count was expected, found {kind:?}"),
        },
        _ => unreachable!(),
    };
}

/// BaseType → 정수 | 소수 | 논리 | 문자 | 바이트, with its `짧은` or `부호 없는`; `type_span` is the
/// source of the whole type, modifier included
fn base_type(
    node: &TreeNode,
    modifier: Option<&ParserTokenKind>,
    type_span: Span,
) -> Result<Type, PostParseError> {
    let [leaf] = branch(node).1 else {
        unreachable!()
    };
    let short = matches!(modifier, Some(ParserTokenKind::Keyword(Keyword::Short)));
    let unsigned = matches!(modifier, Some(ParserTokenKind::Keyword(Keyword::Unsigned)));
    let error = |kind| Err(PostParseError::new(kind, type_span));
    return match (token(leaf), short, unsigned) {
        (ParserTokenKind::Type(TokenType::Int), _, _) => Ok(Type::Int { short, unsigned }),
        (ParserTokenKind::Type(TokenType::Float), _, false) => Ok(Type::Float { short }),
        (ParserTokenKind::Type(TokenType::Bool), false, false) => Ok(Type::Bool),
        (ParserTokenKind::Type(TokenType::Char), false, false) => Ok(Type::Char),
        (ParserTokenKind::Type(TokenType::Byte), false, false) => Ok(Type::Byte),
        (ParserTokenKind::Type(_), true, _) => error(PostParseErrorKind::ShortType),
        (ParserTokenKind::Type(_), _, true) => error(PostParseErrorKind::UnsignedType),
        (kind, _, _) => panic!("a base type was expected, found {kind:?}"),
    };
}

////////////////////////////////
// Statements
////////////////////////////////

/// Block → { Statements }
/// Statements → Statements Statement | ε
fn block(node: &TreeNode) -> Result<Vec<Statement>, PostParseError> {
    let [_, statements, _] = branch(node).1 else {
        unreachable!()
    };
    return statement_list(statements);
}

fn statement_list(node: &TreeNode) -> Result<Vec<Statement>, PostParseError> {
    return match branch(node).1 {
        [] => Ok(Vec::new()),
        [before, last] => {
            let mut statements = statement_list(before)?;
            statements.push(statement(last)?);
            Ok(statements)
        }
        _ => unreachable!(),
    };
}

/// Statement → VariableDeclaration | Chain OptionalDot | IfStatement | WhileLoop | ForLoop
///           | 그만해요 OptionalDot | 넘어가요 OptionalDot
fn statement(node: &TreeNode) -> Result<Statement, PostParseError> {
    let kind = match branch(node).1 {
        [TreeNode::Leaf(word), _] => match word.kind {
            ParserTokenKind::Keyword(Keyword::Stop) => StatementKind::Break,
            ParserTokenKind::Keyword(Keyword::Skip) => StatementKind::Continue,
            _ => unreachable!(),
        },
        [inner, _] => chain(inner)?,
        [inner] => match branch(inner).0 {
            NTerm::VariableDeclaration => variable_declaration(inner)?,
            NTerm::IfStatement => if_statement(inner)?,
            NTerm::WhileLoop => while_loop(inner)?,
            NTerm::ForLoop => for_loop(inner)?,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    return Ok(Statement {
        kind,
        span: span(node),
    });
}

/// VariableDeclaration → [고정된] Name 는 DeclarationValue 이에요 OptionalDot
/// DeclarationValue → Operand | TypeSpec | Arguments VERB-ㄴ 값 | VERB-ㄴ 값
fn variable_declaration(node: &TreeNode) -> Result<StatementKind, PostParseError> {
    let children = branch(node).1;
    let constant = children.len() == 6;
    let [variable, _, declared, _, _] = &children[children.len() - 5..] else {
        unreachable!()
    };
    let name = name(variable);
    let value = match branch(declared).1 {
        [inner] if branch(inner).0 == NTerm::TypeSpec => {
            if constant {
                return Err(PostParseError::new(
                    PostParseErrorKind::ConstantWithoutValue(name),
                    span(node),
                ));
            }
            return Ok(StatementKind::Declare {
                name,
                ty: type_spec(inner)?,
            });
        }
        [operand] => expression(operand)?,
        [args, adnominal, _] => Expr {
            kind: ExprKind::Call {
                verb: verb(adnominal),
                args: arguments(args)?
                    .into_iter()
                    .map(to_argument)
                    .collect::<Result<_, _>>()?,
            },
            span: span(declared),
        },
        [adnominal, _] => Expr {
            kind: ExprKind::Call {
                verb: verb(adnominal),
                args: Vec::new(),
            },
            span: span(declared),
        },
        _ => unreachable!(),
    };
    return Ok(StatementKind::Init {
        name,
        value,
        constant,
    });
}

/// IfStatement → 만약 Condition Block | 만약 Condition Block 아니면 Block
///             | 만약 Condition Block 아니면 IfStatement
/// Condition → Expression 면 | Expression 이 아니면
fn if_statement(node: &TreeNode) -> Result<StatementKind, PostParseError> {
    let (condition, then, otherwise) = match branch(node).1 {
        [_, condition, then] => (condition, then, None),
        [_, condition, then, _, otherwise] => (condition, then, Some(otherwise)),
        _ => unreachable!(),
    };
    let condition = match branch(condition).1 {
        [expr, _] => expression(expr)?,
        [expr, _, _] => not(expression(expr)?),
        _ => unreachable!(),
    };
    let otherwise = match otherwise {
        None => None,
        Some(otherwise) => Some(match branch(otherwise).0 {
            NTerm::Block => block(otherwise)?,
            _ => vec![Statement {
                kind: if_statement(otherwise)?,
                span: span(otherwise),
            }],
        }),
    };
    return Ok(StatementKind::If {
        condition,
        then: block(then)?,
        otherwise,
    });
}

/// WhileLoop → Expression 인 동안 Block | Expression 이 아닌 동안 Block
fn while_loop(node: &TreeNode) -> Result<StatementKind, PostParseError> {
    let (condition, body) = match branch(node).1 {
        [expr, _, _, body] => (expression(expr)?, body),
        [expr, _, _, _, body] => (not(expression(expr)?), body),
        _ => unreachable!(),
    };
    return Ok(StatementKind::While {
        condition,
        body: block(body)?,
    });
}

/// ForLoop → Arguments 세면서 Block: `수를 1부터 끝까지`, or `정수 수를 …` to declare 수
/// An argument that is none of them, or one of them twice, is the error; a missing one makes
/// all the arguments the error
fn for_loop(node: &TreeNode) -> Result<StatementKind, PostParseError> {
    let [args, _, body] = branch(node).1 else {
        unreachable!()
    };
    let error = |span| PostParseError::new(PostParseErrorKind::CountArguments, span);
    let mut variable = None;
    let mut from = None;
    let mut until = None;
    for (arg, arg_span) in arguments(args)? {
        match arg {
            Arg::Declared(ty, name) if variable.is_none() => variable = Some((name, Some(ty))),
            Arg::Value(
                Expr {
                    kind: ExprKind::Name(name),
                    ..
                },
                Particle::Object,
            ) if variable.is_none() => variable = Some((name, None)),
            Arg::Value(value, Particle::From) if from.is_none() => from = Some(value),
            Arg::Value(value, Particle::Until) if until.is_none() => until = Some(value),
            _ => return Err(error(arg_span)),
        }
    }
    let (Some((variable, ty)), Some(from), Some(until)) = (variable, from, until) else {
        return Err(error(span(args)));
    };
    return Ok(StatementKind::For {
        variable,
        ty,
        from,
        until,
        body: block(body)?,
    });
}

////////////////////////////////
// Call chains
////////////////////////////////

/// An argument before it gets its place in a call: `정수 수를` only declares a loop variable.
/// It goes with its source, particle included (`결과에`)
enum Arg {
    Value(Expr, Particle),
    ToType(Type),
    Declared(Type, String),
}

fn to_argument((arg, arg_span): (Arg, Span)) -> Result<Argument, PostParseError> {
    return match arg {
        Arg::Value(value, particle) => Ok(Argument::Value { value, particle }),
        Arg::ToType(ty) => Ok(Argument::ToType(ty)),
        Arg::Declared(_, name) => Err(PostParseError::new(
            PostParseErrorKind::DeclaredOutOfCount(name),
            arg_span,
        )),
    };
}

/// Chain → Steps EndStep | EndStep
/// Steps → Step | Steps Step
/// Step → Arguments LinkVerb | LinkVerb, and EndStep the same with EndVerb
/// The result of each step is the first argument of the next one; the end verb makes the
/// statement: 줘요 a `Return`, 넣어요 an `Assign`, any other verb a `Call`
fn chain(node: &TreeNode) -> Result<StatementKind, PostParseError> {
    let (steps, end) = match branch(node).1 {
        [steps, end] => (step_list(steps)?, end),
        [end] => (Vec::new(), end),
        _ => unreachable!(),
    };
    let mut previous = None;
    for (args, verb, step_span) in steps {
        previous = Some(call(verb.unwrap(), previous, args, step_span)?);
    }
    let (args, end_verb, end_span) = step(end)?;
    return match end_verb.as_deref() {
        None => give(previous, args, span(node)),
        Some("넣다") => put(previous, args, span(node)),
        Some(_) => Ok(StatementKind::Call(call(
            end_verb.unwrap(),
            previous,
            args,
            end_span,
        )?)),
    };
}

/// The arguments, the verb and the source of each step
type Step = (Vec<(Arg, Span)>, Option<String>, Span);

fn step_list(node: &TreeNode) -> Result<Vec<Step>, PostParseError> {
    let (mut steps, last) = match branch(node).1 {
        [last] => (Vec::new(), last),
        [before, last] => (step_list(before)?, last),
        _ => unreachable!(),
    };
    steps.push(step(last)?);
    return Ok(steps);
}

/// The arguments, the verb and the source of a step; the verb is `None` for 줘요
/// LinkVerb → VERB-서 | Name VERB-서 (`printf해서`), EndVerb → VERB-요 | Name VERB-요 | 줘요
fn step(node: &TreeNode) -> Result<Step, PostParseError> {
    let (args, verb_node) = match branch(node).1 {
        [args, verb_node] => (arguments(args)?, verb_node),
        [verb_node] => (Vec::new(), verb_node),
        _ => unreachable!(),
    };
    let verb_name = match branch(verb_node).1 {
        [TreeNode::Leaf(word)] if word.kind == ParserTokenKind::Keyword(Keyword::Return) => None,
        [word] => Some(verb(word)),
        [function_name, _] => Some(name(function_name)),
        _ => unreachable!(),
    };
    return Ok((args, verb_name, span(node)));
}

/// The call of a step: its source starts with the step before, if any (`수를 제곱해서 결과를
/// 더해서` for 더하다) and ends with `step_span`, the source of the step
fn call(
    verb: String,
    previous: Option<Expr>,
    args: Vec<(Arg, Span)>,
    step_span: Span,
) -> Result<Expr, PostParseError> {
    let start = previous.as_ref().map_or(step_span.start, |previous| previous.span.start);
    let mut all = Vec::new();
    if let Some(previous) = previous {
        all.push(Argument::Previous(previous));
    }
    for arg in args {
        all.push(to_argument(arg)?);
    }
    return Ok(Expr {
        kind: ExprKind::Call { verb, args: all },
        span: Span::new(start, step_span.end),
    });
}

/// `X를 줘요`, or `… 해서 줘요` that gives the result of the chain. The error is on the argument
/// that is too many or without 을/를, or on the whole chain (`chain_span`) without a value
fn give(
    previous: Option<Expr>,
    args: Vec<(Arg, Span)>,
    chain_span: Span,
) -> Result<StatementKind, PostParseError> {
    let error = |span| PostParseError::new(PostParseErrorKind::GiveArguments, span);
    let mut args = args.into_iter();
    let value = match previous {
        Some(value) => value,
        None => match args.next() {
            Some((Arg::Value(value, Particle::Object), _)) => value,
            Some((_, arg_span)) => return Err(error(arg_span)),
            None => return Err(error(chain_span)),
        },
    };
    if let Some((_, arg_span)) = args.next() {
        return Err(error(arg_span));
    }
    return Ok(StatementKind::Return(value));
}

/// `X를 Y에 넣어요`, or `… 해서 Y에 넣어요` that puts the result of the chain. The error is on
/// the argument that is none of them, or one of them twice; a missing one makes the whole chain
/// (`chain_span`) the error
fn put(
    previous: Option<Expr>,
    args: Vec<(Arg, Span)>,
    chain_span: Span,
) -> Result<StatementKind, PostParseError> {
    let error = |span| PostParseError::new(PostParseErrorKind::PutArguments, span);
    let mut target = None;
    let mut value = previous;
    for (arg, arg_span) in args {
        match arg {
            Arg::Value(
                Expr {
                    kind: ExprKind::Name(name),
                    ..
                },
                Particle::In,
            ) if target.is_none() => target = Some(name),
            Arg::Value(object, Particle::Object) if value.is_none() => value = Some(object),
            _ => return Err(error(arg_span)),
        }
    }
    let (Some(target), Some(value)) = (target, value) else {
        return Err(error(chain_span));
    };
    return Ok(StatementKind::Assign { target, value });
}

/// Arguments → Argument | Arguments Argument
/// Argument → Operand ArgumentParticle | TypeSpec 로 | TypeSpec Name 를
fn arguments(node: &TreeNode) -> Result<Vec<(Arg, Span)>, PostParseError> {
    let (mut args, last) = match branch(node).1 {
        [last] => (Vec::new(), last),
        [before, last] => (arguments(before)?, last),
        _ => unreachable!(),
    };
    let arg = match branch(last).1 {
        [ty, _] if branch(ty).0 == NTerm::TypeSpec => Arg::ToType(type_spec(ty)?),
        [operand, particle] => {
            let [leaf] = branch(particle).1 else {
                unreachable!()
            };
            let ParserTokenKind::Particle(particle) = token(leaf) else {
                unreachable!()
            };
            Arg::Value(expression(operand)?, *particle)
        }
        [ty, variable, _] => Arg::Declared(type_spec(ty)?, name(variable)),
        _ => unreachable!(),
    };
    args.push((arg, span(last)));
    return Ok(args);
}

////////////////////////////////
// Expressions
////////////////////////////////

/// Every level from Expression down to Primary, and Operand:
/// - one child: the same value one level lower, the node is crossed
/// - `left OP right`: a `Binary`, `OP right` in UnaryExpression: a `Unary`
/// - Operand → - N: the negative literal
fn expression(node: &TreeNode) -> Result<Expr, PostParseError> {
    let (left_side, children) = branch(node);
    let kind = match (left_side, children) {
        (NTerm::Primary, _) => primary(children)?,
        (_, [inner]) => return expression(inner),
        (NTerm::Operand, [_, number]) => match token(number) {
            ParserTokenKind::Int(number) => ExprKind::Int(-number),
            ParserTokenKind::Float(number) => ExprKind::Float(-number),
            kind => panic!("a number was expected, found {kind:?}"),
        },
        (_, [op, operand]) => ExprKind::Unary {
            op: unary_op(token(op)),
            operand: Box::new(expression(operand)?),
        },
        (_, [left, op, right]) => ExprKind::Binary {
            op: binary_op(token(op)),
            left: Box::new(expression(left)?),
            right: Box::new(expression(right)?),
        },
        _ => unreachable!(),
    };
    return Ok(Expr {
        kind,
        span: span(node),
    });
}

/// Primary → Int | Float | Str | Bool | Name | 빈 주소 | TypeSpec 의 크기 | ( Expression )
/// `( Expression )` gives the expression, its span set to the parentheses by `expression`
fn primary(children: &[TreeNode]) -> Result<ExprKind, PostParseError> {
    return Ok(match children {
        [leaf] => match token(leaf) {
            ParserTokenKind::Int(number) => ExprKind::Int(*number),
            ParserTokenKind::Float(number) => ExprKind::Float(*number),
            ParserTokenKind::Str(text) => ExprKind::Str(text.clone()),
            ParserTokenKind::Bool(value) => ExprKind::Bool(*value),
            ParserTokenKind::Name(name) => ExprKind::Name(name.clone()),
            kind => panic!("a value was expected, found {kind:?}"),
        },
        [_, _] => ExprKind::Null,
        [TreeNode::Leaf(_), inner, _] => expression(inner)?.kind,
        [ty, _, _] => ExprKind::SizeOf(type_spec(ty)?),
        _ => unreachable!(),
    });
}

/// `Not` has no token of its own (`아니면`, `아닌 동안`): its span is the one of its operand
fn not(expr: Expr) -> Expr {
    let span = expr.span;
    return Expr {
        kind: ExprKind::Unary {
            op: UnaryOp::Not,
            operand: Box::new(expr),
        },
        span,
    };
}

fn unary_op(kind: &ParserTokenKind) -> UnaryOp {
    return match kind {
        ParserTokenKind::Operator(Operator::Minus) => UnaryOp::Neg,
        ParserTokenKind::Operator(Operator::Tilde) => UnaryOp::BitNot,
        kind => panic!("a unary operator was expected, found {kind:?}"),
    };
}

fn binary_op(kind: &ParserTokenKind) -> BinaryOp {
    return match kind {
        ParserTokenKind::Keyword(Keyword::Or) => BinaryOp::Or,
        ParserTokenKind::Keyword(Keyword::And) => BinaryOp::And,
        ParserTokenKind::Operator(op) => match op {
            Operator::Pipe => BinaryOp::BitOr,
            Operator::Caret => BinaryOp::BitXor,
            Operator::Amp => BinaryOp::BitAnd,
            Operator::EqEq => BinaryOp::Eq,
            Operator::NotEq => BinaryOp::NotEq,
            Operator::Lt => BinaryOp::Lt,
            Operator::Le => BinaryOp::Le,
            Operator::Gt => BinaryOp::Gt,
            Operator::Ge => BinaryOp::Ge,
            Operator::Shl => BinaryOp::Shl,
            Operator::Shr => BinaryOp::Shr,
            Operator::Plus => BinaryOp::Add,
            Operator::Minus => BinaryOp::Sub,
            Operator::Star => BinaryOp::Mul,
            Operator::Slash => BinaryOp::Div,
            Operator::Percent => BinaryOp::Rem,
            Operator::Tilde => panic!("~ is no binary operator"),
        },
        kind => panic!("a binary operator was expected, found {kind:?}"),
    };
}
