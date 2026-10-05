use super::super::parser_token::ParserTokenKind;
use super::structs::ParserNode;

/// Only the kind of token counts for `Name`, `Int`, `Str`, `Bool` and the verb's infinitive: their
/// value in the rules is a placeholder
fn token_matches_node(token: &ParserTokenKind, node: &ParserNode) -> bool {
    return match (token, node) {
        (ParserTokenKind::Name(_), ParserNode::Term(ParserTokenKind::Name(_)))
        | (ParserTokenKind::Int(_), ParserNode::Term(ParserTokenKind::Int(_)))
        | (ParserTokenKind::Str(_), ParserNode::Term(ParserTokenKind::Str(_)))
        | (ParserTokenKind::Bool(_), ParserNode::Term(ParserTokenKind::Bool(_))) => true,

        (
            ParserTokenKind::Verb { ending, .. },
            ParserNode::Term(ParserTokenKind::Verb {
                ending: node_ending,
                ..
            }),
        ) => ending == node_ending,

        // Keyword, Type, Particle, Punctuation, Operator, Eof: the value is the token
        (token, ParserNode::Term(kind)) => token == kind,

        (_, ParserNode::NTerm(_)) => false,
    };
}
