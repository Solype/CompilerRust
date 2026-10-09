use crate::lexer::lexer_token::{Operator, Punctuation};
use crate::parser::parser_token::{Ending, Keyword, ParserTokenKind, Particle, Type};

use super::structs::NTerm::*;
use super::structs::ParserNode::{NTerm, Term};
use super::structs::ParserProduction;

/// `rule!(Left => a, b, c)` is the production `Left → a b c`, `rule!(Left =>)` derives ε
macro_rules! rule {
    ($left:ident => $($node:expr),* $(,)?) => {
        ParserProduction {
            left: $left,
            right: &[$($node),*],
        }
    };
}

/// `term!(Keyword::If)`, `term!(Punctuation::LParen)`… is that token; `term!(Verb::Seo)` is any
/// verb with that ending; `term!(Name)`, `term!(Int)`… is any token of that kind (the value is a
/// placeholder, only the kind of token counts)
macro_rules! term {
    (Verb::$ending:ident) => {
        Term(ParserTokenKind::Verb {
            infinitive: String::new(),
            ending: Ending::$ending,
        })
    };
    (Name) => {
        Term(ParserTokenKind::Name(String::new()))
    };
    (Int) => {
        Term(ParserTokenKind::Int(0))
    };
    (Str) => {
        Term(ParserTokenKind::Str(String::new()))
    };
    (Float) => {
        Term(ParserTokenKind::Float(0.0))
    };
    (Bool) => {
        Term(ParserTokenKind::Bool(false))
    };
    (Eof) => {
        Term(ParserTokenKind::Eof)
    };
    ($kind:ident::$value:ident) => {
        Term(ParserTokenKind::$kind($kind::$value))
    };
}

pub const RULES: &[ParserProduction] = &[
    rule!(StartSymbol => NTerm(Items), term!(Eof)),
    rule!(Items => NTerm(Items), NTerm(Item)),
    rule!(Items =>),
    rule!(Item => NTerm(Function)),
    rule!(Item => NTerm(ExternFunction)),
    // Functions
    rule!(Function =>
        NTerm(FunctionHead),
        term!(Punctuation::LParen),
        NTerm(Parameters),
        term!(Punctuation::RParen),
        NTerm(Block),
    ),
    rule!(Function =>
        NTerm(ReturnType),
        NTerm(SentenceParameters),
        term!(Verb::Da),
        NTerm(Block),
    ),
    rule!(Function => NTerm(SentenceParameters), term!(Verb::Da), NTerm(Block)),
    rule!(ExternFunction =>
        term!(Keyword::Extern),
        NTerm(FunctionHead),
        term!(Punctuation::LParen),
        NTerm(Parameters),
        term!(Punctuation::RParen),
    ),
    rule!(FunctionHead => NTerm(ReturnType), NTerm(FunctionName)),
    rule!(FunctionHead => NTerm(FunctionName)),
    rule!(ReturnType => NTerm(TypeSpec), term!(Particle::Object), term!(Keyword::Giving)),
    rule!(FunctionName => term!(Verb::Da)),
    rule!(FunctionName => term!(Name)),
    rule!(Parameters =>),
    rule!(Parameters => NTerm(ParameterList)),
    rule!(Parameters =>
        NTerm(ParameterList),
        term!(Punctuation::Comma),
        term!(Punctuation::Ellipsis),
    ),
    rule!(ParameterList => NTerm(Parameter)),
    rule!(ParameterList => NTerm(ParameterList), term!(Punctuation::Comma), NTerm(Parameter)),
    rule!(Parameter => NTerm(TypeSpec), term!(Name)),
    // Not empty: an empty list would have to be reduced before a type, where the return type
    // can start too
    rule!(SentenceParameters => NTerm(Parameter), term!(Particle::Object)),
    rule!(SentenceParameters => NTerm(SentenceLinks), NTerm(Parameter), term!(Particle::Object)),
    rule!(SentenceLinks => NTerm(Parameter), term!(Particle::With)),
    rule!(SentenceLinks => NTerm(SentenceLinks), NTerm(Parameter), term!(Particle::With)),
    // Types
    rule!(TypeSpec => NTerm(BaseType)),
    rule!(TypeSpec => term!(Keyword::Short), NTerm(BaseType)),
    rule!(TypeSpec => term!(Keyword::Unsigned), NTerm(BaseType)),
    rule!(TypeSpec => NTerm(TypeSpec), term!(Type::Address)),
    rule!(TypeSpec => NTerm(TypeSpec), term!(Int), term!(Keyword::Count)),
    rule!(BaseType => term!(Type::Int)),
    rule!(BaseType => term!(Type::Float)),
    rule!(BaseType => term!(Type::Bool)),
    rule!(BaseType => term!(Type::Char)),
    rule!(BaseType => term!(Type::Byte)),
    // Statements
    rule!(Block => term!(Punctuation::LBrace), NTerm(Statements), term!(Punctuation::RBrace)),
    rule!(Statements => NTerm(Statements), NTerm(Statement)),
    rule!(Statements =>),
    rule!(Statement => NTerm(VariableDeclaration)),
    rule!(Statement => NTerm(Chain), NTerm(OptionalDot)),
    rule!(Statement => NTerm(IfStatement)),
    rule!(Statement => NTerm(WhileLoop)),
    rule!(Statement => NTerm(ForLoop)),
    rule!(Statement => term!(Keyword::Stop), NTerm(OptionalDot)),
    rule!(Statement => term!(Keyword::Skip), NTerm(OptionalDot)),
    rule!(OptionalDot => term!(Punctuation::Dot)),
    rule!(OptionalDot =>),
    // Variables
    rule!(VariableDeclaration =>
        term!(Name),
        term!(Particle::Topic),
        NTerm(DeclarationValue),
        term!(Particle::ItIs),
        NTerm(OptionalDot),
    ),
    rule!(VariableDeclaration =>
        term!(Keyword::Const),
        term!(Name),
        term!(Particle::Topic),
        NTerm(DeclarationValue),
        term!(Particle::ItIs),
        NTerm(OptionalDot),
    ),
    rule!(DeclarationValue => NTerm(Operand)),
    rule!(DeclarationValue => NTerm(TypeSpec)),
    rule!(DeclarationValue => NTerm(Arguments), term!(Verb::Adnominal), term!(Keyword::Value)),
    rule!(DeclarationValue => term!(Verb::Adnominal), term!(Keyword::Value)),
    // Call chains
    rule!(Chain => NTerm(Steps), NTerm(EndStep)),
    rule!(Chain => NTerm(EndStep)),
    rule!(Steps => NTerm(Step)),
    rule!(Steps => NTerm(Steps), NTerm(Step)),
    rule!(Step => NTerm(Arguments), NTerm(LinkVerb)),
    rule!(Step => NTerm(LinkVerb)),
    rule!(EndStep => NTerm(Arguments), NTerm(EndVerb)),
    rule!(EndStep => NTerm(EndVerb)),
    rule!(LinkVerb => term!(Verb::Seo)),
    rule!(LinkVerb => term!(Name), term!(Verb::Seo)),
    rule!(EndVerb => term!(Verb::Yo)),
    rule!(EndVerb => term!(Name), term!(Verb::Yo)),
    rule!(EndVerb => term!(Keyword::Return)),
    rule!(Arguments => NTerm(Argument)),
    rule!(Arguments => NTerm(Arguments), NTerm(Argument)),
    rule!(Argument => NTerm(Operand), NTerm(ArgumentParticle)),
    rule!(Argument => NTerm(TypeSpec), term!(Particle::To)),
    rule!(Argument => NTerm(TypeSpec), term!(Name), term!(Particle::Object)),
    rule!(ArgumentParticle => term!(Particle::Object)),
    rule!(ArgumentParticle => term!(Particle::With)),
    rule!(ArgumentParticle => term!(Particle::In)),
    rule!(ArgumentParticle => term!(Particle::From)),
    rule!(ArgumentParticle => term!(Particle::Until)),
    rule!(Operand => NTerm(Primary)),
    rule!(Operand => term!(Operator::Minus), term!(Int)),
    rule!(Operand => term!(Operator::Minus), term!(Float)),
    // Control flow
    rule!(IfStatement => term!(Keyword::If), NTerm(Condition), NTerm(Block)),
    rule!(IfStatement =>
        term!(Keyword::If),
        NTerm(Condition),
        NTerm(Block),
        term!(Keyword::Otherwise),
        NTerm(Block),
    ),
    rule!(IfStatement =>
        term!(Keyword::If),
        NTerm(Condition),
        NTerm(Block),
        term!(Keyword::Otherwise),
        NTerm(IfStatement),
    ),
    rule!(Condition => NTerm(Expression), term!(Particle::IfItIs)),
    rule!(Condition => NTerm(Expression), term!(Particle::Subject), term!(Keyword::Otherwise)),
    rule!(WhileLoop =>
        NTerm(Expression),
        term!(Particle::ThatIs),
        term!(Keyword::While),
        NTerm(Block),
    ),
    rule!(WhileLoop =>
        NTerm(Expression),
        term!(Particle::Subject),
        term!(Keyword::IsNot),
        term!(Keyword::While),
        NTerm(Block),
    ),
    rule!(ForLoop => NTerm(Arguments), term!(Keyword::Counting), NTerm(Block)),
    // Expressions, C precedence
    rule!(Expression => NTerm(Expression), term!(Keyword::Or), NTerm(AndExpression)),
    rule!(Expression => NTerm(AndExpression)),
    rule!(AndExpression => NTerm(AndExpression), term!(Keyword::And), NTerm(BitOrExpression)),
    rule!(AndExpression => NTerm(BitOrExpression)),
    rule!(BitOrExpression =>
        NTerm(BitOrExpression),
        term!(Operator::Pipe),
        NTerm(BitXorExpression),
    ),
    rule!(BitOrExpression => NTerm(BitXorExpression)),
    rule!(BitXorExpression =>
        NTerm(BitXorExpression),
        term!(Operator::Caret),
        NTerm(BitAndExpression),
    ),
    rule!(BitXorExpression => NTerm(BitAndExpression)),
    rule!(BitAndExpression =>
        NTerm(BitAndExpression),
        term!(Operator::Amp),
        NTerm(EqualityExpression),
    ),
    rule!(BitAndExpression => NTerm(EqualityExpression)),
    rule!(EqualityExpression =>
        NTerm(EqualityExpression),
        term!(Operator::EqEq),
        NTerm(RelationalExpression),
    ),
    rule!(EqualityExpression =>
        NTerm(EqualityExpression),
        term!(Operator::NotEq),
        NTerm(RelationalExpression),
    ),
    rule!(EqualityExpression => NTerm(RelationalExpression)),
    rule!(RelationalExpression =>
        NTerm(RelationalExpression),
        term!(Operator::Lt),
        NTerm(ShiftExpression),
    ),
    rule!(RelationalExpression =>
        NTerm(RelationalExpression),
        term!(Operator::Le),
        NTerm(ShiftExpression),
    ),
    rule!(RelationalExpression =>
        NTerm(RelationalExpression),
        term!(Operator::Gt),
        NTerm(ShiftExpression),
    ),
    rule!(RelationalExpression =>
        NTerm(RelationalExpression),
        term!(Operator::Ge),
        NTerm(ShiftExpression),
    ),
    rule!(RelationalExpression => NTerm(ShiftExpression)),
    rule!(ShiftExpression =>
        NTerm(ShiftExpression),
        term!(Operator::Shl),
        NTerm(AdditiveExpression),
    ),
    rule!(ShiftExpression =>
        NTerm(ShiftExpression),
        term!(Operator::Shr),
        NTerm(AdditiveExpression),
    ),
    rule!(ShiftExpression => NTerm(AdditiveExpression)),
    rule!(AdditiveExpression =>
        NTerm(AdditiveExpression),
        term!(Operator::Plus),
        NTerm(MultiplicativeExpression),
    ),
    rule!(AdditiveExpression =>
        NTerm(AdditiveExpression),
        term!(Operator::Minus),
        NTerm(MultiplicativeExpression),
    ),
    rule!(AdditiveExpression => NTerm(MultiplicativeExpression)),
    rule!(MultiplicativeExpression =>
        NTerm(MultiplicativeExpression),
        term!(Operator::Star),
        NTerm(UnaryExpression),
    ),
    rule!(MultiplicativeExpression =>
        NTerm(MultiplicativeExpression),
        term!(Operator::Slash),
        NTerm(UnaryExpression),
    ),
    rule!(MultiplicativeExpression =>
        NTerm(MultiplicativeExpression),
        term!(Operator::Percent),
        NTerm(UnaryExpression),
    ),
    rule!(MultiplicativeExpression => NTerm(UnaryExpression)),
    rule!(UnaryExpression => term!(Operator::Minus), NTerm(UnaryExpression)),
    rule!(UnaryExpression => term!(Operator::Tilde), NTerm(UnaryExpression)),
    rule!(UnaryExpression => NTerm(Primary)),
    rule!(Primary => term!(Int)),
    rule!(Primary => term!(Float)),
    rule!(Primary => term!(Str)),
    rule!(Primary => term!(Bool)),
    rule!(Primary => term!(Name)),
    rule!(Primary => term!(Keyword::Void), term!(Type::Address)),
    rule!(Primary => NTerm(TypeSpec), term!(Particle::Of), term!(Keyword::Size)),
    rule!(Primary => term!(Punctuation::LParen), NTerm(Expression), term!(Punctuation::RParen)),
];
