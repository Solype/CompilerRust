use std::hash::{Hash, Hasher};
use std::mem::discriminant;

use crate::parser::parser_token::ParserTokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NTerm {
    /// Items then Eof
    StartSymbol,
    Items,
    /// A function or an extern declaration
    Item,
    /// `정수를 주는 더하다(정수 가, 정수 나) { … }`
    Function,
    /// `외부 정수를 주는 printf(문자 주소 형식, …)`
    ExternFunction,
    /// `정수를 주는 더하다`, or `인사하다` without a return type
    FunctionHead,
    /// `정수를 주는`
    ReturnType,
    /// A verb in -다, or a latin name (`main`, `printf`)
    FunctionName,
    /// What is between the parentheses, maybe empty, maybe ending with `…`
    Parameters,
    ParameterList,
    /// `정수 가`
    Parameter,
    /// `정수`, `짧은 정수`, `부호 없는 정수`, `문자 주소`, `정수&&`, `정수 10개`
    TypeSpec,
    /// `정수`, `실수`, `논리`, `문자`, `바이트`
    BaseType,
    /// `{ … }`
    Block,
    Statements,
    Statement,
    /// The `.` accepted after a final `요`
    OptionalDot,
    /// `개수는 0이에요`, `고정된 합은 3과 4를 더한 값이에요`
    VariableDeclaration,
    /// What is before `이에요/예요`: a value, a type, or `… 더한 값`
    DeclarationValue,
    /// `3과 4를 더해서 5를 곱해서 줘요`
    Chain,
    /// The `-아/어서` steps of a chain
    Steps,
    /// `3과 4를 더해서`
    Step,
    /// `5를 곱해요`, `42를 줘요`
    EndStep,
    /// `더해서`, `printf해서`
    LinkVerb,
    /// `더해요`, `printf해요`, `줘요`
    EndVerb,
    Arguments,
    /// `3과`, `4를`, `결과에`, `1부터`, `10까지`, `실수로`, `정수 칸을`
    Argument,
    ArgumentParticle,
    /// A simple term, `-5`, or `(EXPR)`: what can carry a particle
    Operand,
    /// `만약 … 이면 { … } 아니면 { … }`
    IfStatement,
    /// `가 > 나면`, `(가 > 0)이 아니면`
    Condition,
    /// `가 < 10인 동안 { … }`, `COND이 아닌 동안 { … }`
    WhileLoop,
    /// `수를 1부터 10까지 세면서 { … }`
    ForLoop,
    /// `또는`, the weakest level
    Expression,
    /// `그리고`
    AndExpression,
    /// `|`
    BitOrExpression,
    /// `^`
    BitXorExpression,
    /// `&`
    BitAndExpression,
    /// `==` `!=`
    EqualityExpression,
    /// `<` `<=` `>` `>=`
    RelationalExpression,
    /// `<<` `>>`
    ShiftExpression,
    /// `+` `-`
    AdditiveExpression,
    /// `*` `/` `%`
    MultiplicativeExpression,
    /// `-` `~`
    UnaryExpression,
    /// A literal, a name, `빈 주소`, `정수의 크기`, `(EXPR)`
    Primary,
}

/// A terminal is a token category, without the token's value: `Name` matches any name
pub enum ParserNode {
    Term(ParserTokenKind),
    NTerm(NTerm),
}

pub struct ParserProduction {
    pub left: NTerm,
    pub right: &'static [ParserNode],
}

/// Only the kind of token counts for `Name`, `Int`, `Str`, `Bool` and the verb's infinitive: their
/// value in the rules is a placeholder
pub(super) fn same_terminal(a: &ParserTokenKind, b: &ParserTokenKind) -> bool {
    return match (a, b) {
        (ParserTokenKind::Name(_), ParserTokenKind::Name(_))
        | (ParserTokenKind::Int(_), ParserTokenKind::Int(_))
        | (ParserTokenKind::Str(_), ParserTokenKind::Str(_))
        | (ParserTokenKind::Bool(_), ParserTokenKind::Bool(_)) => true,

        (
            ParserTokenKind::Verb { ending, .. },
            ParserTokenKind::Verb {
                ending: other_ending,
                ..
            },
        ) => ending == other_ending,

        // Keyword, Type, Particle, Punctuation, Operator, Eof: the value is the token
        (a, b) => a == b,
    };
}

/// Same rule as `TerminalKey`: a terminal is compared on its kind
impl PartialEq for ParserNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParserNode::Term(a), ParserNode::Term(b)) => same_terminal(a, b),
            (ParserNode::NTerm(a), ParserNode::NTerm(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for ParserNode {}

impl Hash for ParserNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
        match self {
            ParserNode::Term(kind) => discriminant(kind).hash(state),
            ParserNode::NTerm(non_terminal) => non_terminal.hash(state),
        }
    }
}
