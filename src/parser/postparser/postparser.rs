use crate::lexer::lexer_token::Operator;

use super::super::cfg::parser::TreeNode;
use super::super::cfg::rules::RULES;
use super::super::cfg::structs::NTerm;
use super::super::parser_token::{Keyword, ParserTokenKind, Particle, Type as TokenType};

use super::structs::{
    Argument, BinaryOp, Expr, Function, Param, Program, Statement, Type, UnaryOp,
};

/// The parse tree of `Parser::parse` (its `Items` root) as an AST. Each function below converts
/// one non-terminal: the rules it handles are in its comment, and the children it gets follow
/// their right side. A program the grammar accepts but the AST cannot hold (`부호 없는 실수`,
/// `줘요` with two values…) panics: those checks will become real errors later
pub fn convert_to_ast_tree(tree: &TreeNode) -> Program {
    let mut prog = Program::default();
    prog.functions = items(tree);
    return prog;
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
fn items(node: &TreeNode) -> Vec<Function> {
    return match branch(node).1 {
        [] => Vec::new(),
        [before, item] => {
            let mut functions = items(before);
            functions.push(function(item));
            functions
        }
        _ => unreachable!(),
    };
}

/// Item → Function | ExternFunction
fn function(node: &TreeNode) -> Function {
    let [inner] = branch(node).1 else {
        unreachable!()
    };
    let (head, params, body) = match branch(inner) {
        // Function → FunctionHead ( Parameters ) Block
        (NTerm::Function, [head, _, params, _, body]) => (head, params, Some(block(body))),
        // ExternFunction → 외부 FunctionHead ( Parameters )
        (NTerm::ExternFunction, [_, head, _, params, _]) => (head, params, None),
        // Function → ReturnType SentenceParameters VERB-다 Block
        (NTerm::Function, [return_type, params, name, body]) => {
            return Function {
                name: verb(name),
                return_type: Some(type_spec(&branch(return_type).1[0])),
                params: sentence_parameters(params),
                variadic: false,
                body: Some(block(body)),
            };
        }
        // Function → SentenceParameters VERB-다 Block
        (NTerm::Function, [params, name, body]) => {
            return Function {
                name: verb(name),
                return_type: None,
                params: sentence_parameters(params),
                variadic: false,
                body: Some(block(body)),
            };
        }
        _ => unreachable!(),
    };
    let (name, return_type) = function_head(head);
    let (params, variadic) = parameters(params);
    return Function {
        name,
        return_type,
        params,
        variadic,
        body,
    };
}

/// FunctionHead → ReturnType FunctionName | FunctionName
/// ReturnType → TypeSpec 를 주는
/// FunctionName → VERB-다 | Name
fn function_head(node: &TreeNode) -> (String, Option<Type>) {
    let (return_type, function_name) = match branch(node).1 {
        [return_type, function_name] => (Some(type_spec(&branch(return_type).1[0])), function_name),
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
    return (name, return_type);
}

/// Parameters → ε | ParameterList | ParameterList , …
/// ParameterList → Parameter | ParameterList , Parameter
/// Parameter → TypeSpec Name
fn parameters(node: &TreeNode) -> (Vec<Param>, bool) {
    return match branch(node).1 {
        [] => (Vec::new(), false),
        [list] => (parameter_list(list), false),
        [list, _, _] => (parameter_list(list), true),
        _ => unreachable!(),
    };
}

fn parameter_list(node: &TreeNode) -> Vec<Param> {
    let (mut params, last) = match branch(node).1 {
        [last] => (Vec::new(), last),
        [before, _, last] => (parameter_list(before), last),
        _ => unreachable!(),
    };
    params.push(parameter(last));
    return params;
}

/// Parameter → TypeSpec Name
fn parameter(node: &TreeNode) -> Param {
    let [ty, param_name] = branch(node).1 else {
        unreachable!()
    };
    return Param {
        ty: type_spec(ty),
        name: name(param_name),
    };
}

/// SentenceParameters → Parameter 를 | SentenceLinks Parameter 를
/// SentenceLinks → Parameter 와 | SentenceLinks Parameter 와
fn sentence_parameters(node: &TreeNode) -> Vec<Param> {
    let (mut params, last) = match branch(node).1 {
        [last, _] => (Vec::new(), last),
        [before, last, _] => (sentence_parameters(before), last),
        _ => unreachable!(),
    };
    params.push(parameter(last));
    return params;
}

////////////////////////////////
// Types
////////////////////////////////

/// TypeSpec → BaseType | 짧은 BaseType | 부호 없는 BaseType | TypeSpec & | TypeSpec N 개
fn type_spec(node: &TreeNode) -> Type {
    return match branch(node).1 {
        [base] => base_type(base, None),
        [TreeNode::Leaf(modifier), base] => base_type(base, Some(&modifier.kind)),
        [inner, _] => Type::Address(Box::new(type_spec(inner))),
        [inner, count, _] => match token(count) {
            ParserTokenKind::Int(count) => Type::Array(Box::new(type_spec(inner)), *count as usize),
            kind => panic!("a count was expected, found {kind:?}"),
        },
        _ => unreachable!(),
    };
}

/// BaseType → 정수 | 실수 | 논리 | 문자 | 바이트, with its `짧은` or `부호 없는`
fn base_type(node: &TreeNode, modifier: Option<&ParserTokenKind>) -> Type {
    let [leaf] = branch(node).1 else {
        unreachable!()
    };
    let short = matches!(modifier, Some(ParserTokenKind::Keyword(Keyword::Short)));
    let unsigned = matches!(modifier, Some(ParserTokenKind::Keyword(Keyword::Unsigned)));
    return match (token(leaf), short, unsigned) {
        (ParserTokenKind::Type(TokenType::Int), _, _) => Type::Int { short, unsigned },
        (ParserTokenKind::Type(TokenType::Float), _, false) => Type::Float { short },
        (ParserTokenKind::Type(TokenType::Bool), false, false) => Type::Bool,
        (ParserTokenKind::Type(TokenType::Char), false, false) => Type::Char,
        (ParserTokenKind::Type(TokenType::Byte), false, false) => Type::Byte,
        (kind, _, _) => panic!("{kind:?} cannot be 짧은 or 부호 없는"),
    };
}

////////////////////////////////
// Statements
////////////////////////////////

/// Block → { Statements }
/// Statements → Statements Statement | ε
fn block(node: &TreeNode) -> Vec<Statement> {
    let [_, statements, _] = branch(node).1 else {
        unreachable!()
    };
    return statement_list(statements);
}

fn statement_list(node: &TreeNode) -> Vec<Statement> {
    return match branch(node).1 {
        [] => Vec::new(),
        [before, last] => {
            let mut statements = statement_list(before);
            statements.push(statement(last));
            statements
        }
        _ => unreachable!(),
    };
}

/// Statement → VariableDeclaration | Chain OptionalDot | IfStatement | WhileLoop | ForLoop
///           | 그만해요 OptionalDot | 넘어가요 OptionalDot
fn statement(node: &TreeNode) -> Statement {
    return match branch(node).1 {
        [TreeNode::Leaf(word), _] => match word.kind {
            ParserTokenKind::Keyword(Keyword::Stop) => Statement::Break,
            ParserTokenKind::Keyword(Keyword::Skip) => Statement::Continue,
            _ => unreachable!(),
        },
        [inner, _] => chain(inner),
        [inner] => match branch(inner).0 {
            NTerm::VariableDeclaration => variable_declaration(inner),
            NTerm::IfStatement => if_statement(inner),
            NTerm::WhileLoop => while_loop(inner),
            NTerm::ForLoop => for_loop(inner),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
}

/// VariableDeclaration → [고정된] Name 는 DeclarationValue 이에요 OptionalDot
/// DeclarationValue → Operand | TypeSpec | Arguments VERB-ㄴ 값 | VERB-ㄴ 값
fn variable_declaration(node: &TreeNode) -> Statement {
    let children = branch(node).1;
    let constant = children.len() == 6;
    let [variable, _, declared, _, _] = &children[children.len() - 5..] else {
        unreachable!()
    };
    let name = name(variable);
    let value = match branch(declared).1 {
        [inner] if branch(inner).0 == NTerm::TypeSpec => {
            if constant {
                panic!("the constant {name} has no value");
            }
            return Statement::Declare {
                name,
                ty: type_spec(inner),
            };
        }
        [operand] => expression(operand),
        [args, adnominal, _] => Expr::Call {
            verb: verb(adnominal),
            args: arguments(args).into_iter().map(to_argument).collect(),
        },
        [adnominal, _] => Expr::Call {
            verb: verb(adnominal),
            args: Vec::new(),
        },
        _ => unreachable!(),
    };
    return Statement::Init {
        name,
        value,
        constant,
    };
}

/// IfStatement → 만약 Condition Block | 만약 Condition Block 아니면 Block
///             | 만약 Condition Block 아니면 IfStatement
/// Condition → Expression 면 | Expression 이 아니면
fn if_statement(node: &TreeNode) -> Statement {
    let (condition, then, otherwise) = match branch(node).1 {
        [_, condition, then] => (condition, then, None),
        [_, condition, then, _, otherwise] => (condition, then, Some(otherwise)),
        _ => unreachable!(),
    };
    let condition = match branch(condition).1 {
        [expr, _] => expression(expr),
        [expr, _, _] => not(expression(expr)),
        _ => unreachable!(),
    };
    let otherwise = otherwise.map(|otherwise| match branch(otherwise).0 {
        NTerm::Block => block(otherwise),
        _ => vec![if_statement(otherwise)],
    });
    return Statement::If {
        condition,
        then: block(then),
        otherwise,
    };
}

/// WhileLoop → Expression 인 동안 Block | Expression 이 아닌 동안 Block
fn while_loop(node: &TreeNode) -> Statement {
    let (condition, body) = match branch(node).1 {
        [expr, _, _, body] => (expression(expr), body),
        [expr, _, _, _, body] => (not(expression(expr)), body),
        _ => unreachable!(),
    };
    return Statement::While {
        condition,
        body: block(body),
    };
}

/// ForLoop → Arguments 세면서 Block: `수를 1부터 끝까지`, or `정수 수를 …` to declare 수
fn for_loop(node: &TreeNode) -> Statement {
    let [args, _, body] = branch(node).1 else {
        unreachable!()
    };
    let mut variable = None;
    let mut from = None;
    let mut until = None;
    for arg in arguments(args) {
        match arg {
            Arg::Declared(ty, name) => variable = Some((name, Some(ty))),
            Arg::Value(Expr::Name(name), Particle::Object) => variable = Some((name, None)),
            Arg::Value(value, Particle::From) => from = Some(value),
            Arg::Value(value, Particle::Until) => until = Some(value),
            _ => panic!("세면서 takes a variable, a 부터 and a 까지"),
        }
    }
    let (Some((variable, ty)), Some(from), Some(until)) = (variable, from, until) else {
        panic!("세면서 takes a variable, a 부터 and a 까지");
    };
    return Statement::For {
        variable,
        ty,
        from,
        until,
        body: block(body),
    };
}

////////////////////////////////
// Call chains
////////////////////////////////

/// An argument before it gets its place in a call: `정수 수를` only declares a loop variable
enum Arg {
    Value(Expr, Particle),
    ToType(Type),
    Declared(Type, String),
}

fn to_argument(arg: Arg) -> Argument {
    return match arg {
        Arg::Value(value, particle) => Argument::Value { value, particle },
        Arg::ToType(ty) => Argument::ToType(ty),
        Arg::Declared(_, name) => panic!("{name} can only be declared by a 세면서 loop"),
    };
}

/// Chain → Steps EndStep | EndStep
/// Steps → Step | Steps Step
/// Step → Arguments LinkVerb | LinkVerb, and EndStep the same with EndVerb
/// The result of each step is the first argument of the next one; the end verb makes the
/// statement: 줘요 a `Return`, 넣어요 an `Assign`, any other verb a `Call`
fn chain(node: &TreeNode) -> Statement {
    let (steps, end) = match branch(node).1 {
        [steps, end] => (step_list(steps), end),
        [end] => (Vec::new(), end),
        _ => unreachable!(),
    };
    let mut previous = None;
    for (args, verb) in steps {
        previous = Some(call(verb.unwrap(), previous, args));
    }
    let (args, end_verb) = step(end);
    return match end_verb.as_deref() {
        None => give(previous, args),
        Some("넣다") => put(previous, args),
        Some(_) => Statement::Call(call(end_verb.unwrap(), previous, args)),
    };
}

fn step_list(node: &TreeNode) -> Vec<(Vec<Arg>, Option<String>)> {
    let (mut steps, last) = match branch(node).1 {
        [last] => (Vec::new(), last),
        [before, last] => (step_list(before), last),
        _ => unreachable!(),
    };
    steps.push(step(last));
    return steps;
}

/// The arguments and the verb of a step; the verb is `None` for 줘요
/// LinkVerb → VERB-서 | Name VERB-서 (`printf해서`), EndVerb → VERB-요 | Name VERB-요 | 줘요
fn step(node: &TreeNode) -> (Vec<Arg>, Option<String>) {
    let (args, verb_node) = match branch(node).1 {
        [args, verb_node] => (arguments(args), verb_node),
        [verb_node] => (Vec::new(), verb_node),
        _ => unreachable!(),
    };
    let verb_name = match branch(verb_node).1 {
        [TreeNode::Leaf(word)] if word.kind == ParserTokenKind::Keyword(Keyword::Return) => None,
        [word] => Some(verb(word)),
        [function_name, _] => Some(name(function_name)),
        _ => unreachable!(),
    };
    return (args, verb_name);
}

fn call(verb: String, previous: Option<Expr>, args: Vec<Arg>) -> Expr {
    let mut all = Vec::new();
    if let Some(previous) = previous {
        all.push(Argument::Previous(previous));
    }
    all.extend(args.into_iter().map(to_argument));
    return Expr::Call { verb, args: all };
}

/// `X를 줘요`, or `… 해서 줘요` that gives the result of the chain
fn give(previous: Option<Expr>, args: Vec<Arg>) -> Statement {
    let mut args = args.into_iter();
    return match (previous, args.next(), args.next()) {
        (None, Some(Arg::Value(value, Particle::Object)), None) => Statement::Return(value),
        (Some(value), None, None) => Statement::Return(value),
        _ => panic!("줘요 takes one value"),
    };
}

/// `X를 Y에 넣어요`, or `… 해서 Y에 넣어요` that puts the result of the chain
fn put(previous: Option<Expr>, args: Vec<Arg>) -> Statement {
    let mut target = None;
    let mut value = previous;
    for arg in args {
        match arg {
            Arg::Value(Expr::Name(name), Particle::In) => target = Some(name),
            Arg::Value(object, Particle::Object) if value.is_none() => value = Some(object),
            _ => panic!("넣어요 takes a value and a variable with 에"),
        }
    }
    let (Some(target), Some(value)) = (target, value) else {
        panic!("넣어요 takes a value and a variable with 에");
    };
    return Statement::Assign { target, value };
}

/// Arguments → Argument | Arguments Argument
/// Argument → Operand ArgumentParticle | TypeSpec 로 | TypeSpec Name 를
fn arguments(node: &TreeNode) -> Vec<Arg> {
    let (mut args, last) = match branch(node).1 {
        [last] => (Vec::new(), last),
        [before, last] => (arguments(before), last),
        _ => unreachable!(),
    };
    let arg = match branch(last).1 {
        [ty, _] if branch(ty).0 == NTerm::TypeSpec => Arg::ToType(type_spec(ty)),
        [operand, particle] => {
            let [leaf] = branch(particle).1 else {
                unreachable!()
            };
            let ParserTokenKind::Particle(particle) = token(leaf) else {
                unreachable!()
            };
            Arg::Value(expression(operand), *particle)
        }
        [ty, variable, _] => Arg::Declared(type_spec(ty), name(variable)),
        _ => unreachable!(),
    };
    args.push(arg);
    return args;
}

////////////////////////////////
// Expressions
////////////////////////////////

/// Every level from Expression down to Primary, and Operand:
/// - one child: the same value one level lower, the node is crossed
/// - `left OP right`: a `Binary`, `OP right` in UnaryExpression: a `Unary`
/// - Operand → - N: the negative literal
fn expression(node: &TreeNode) -> Expr {
    let (left_side, children) = branch(node);
    return match (left_side, children) {
        (NTerm::Primary, _) => primary(children),
        (_, [inner]) => expression(inner),
        (NTerm::Operand, [_, number]) => match token(number) {
            ParserTokenKind::Int(number) => Expr::Int(-number),
            ParserTokenKind::Float(number) => Expr::Float(-number),
            kind => panic!("a number was expected, found {kind:?}"),
        },
        (_, [op, operand]) => Expr::Unary {
            op: unary_op(token(op)),
            operand: Box::new(expression(operand)),
        },
        (_, [left, op, right]) => Expr::Binary {
            op: binary_op(token(op)),
            left: Box::new(expression(left)),
            right: Box::new(expression(right)),
        },
        _ => unreachable!(),
    };
}

/// Primary → Int | Float | Str | Bool | Name | 빈 주소 | TypeSpec 의 크기 | ( Expression )
fn primary(children: &[TreeNode]) -> Expr {
    return match children {
        [leaf] => match token(leaf) {
            ParserTokenKind::Int(number) => Expr::Int(*number),
            ParserTokenKind::Float(number) => Expr::Float(*number),
            ParserTokenKind::Str(text) => Expr::Str(text.clone()),
            ParserTokenKind::Bool(value) => Expr::Bool(*value),
            ParserTokenKind::Name(name) => Expr::Name(name.clone()),
            kind => panic!("a value was expected, found {kind:?}"),
        },
        [_, _] => Expr::Null,
        [TreeNode::Leaf(_), inner, _] => expression(inner),
        [ty, _, _] => Expr::SizeOf(type_spec(ty)),
        _ => unreachable!(),
    };
}

fn not(expr: Expr) -> Expr {
    return Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(expr),
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
