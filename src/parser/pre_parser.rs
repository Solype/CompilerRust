use crate::lexer::lexer_token::{LexerToken, LexerTokenKind};

use super::parser_token::ParserToken;
use super::pre_parse_error::PreParseError;

/// Splits and classifies the lexer tokens (나를 → 나 + 를); `tokens` must end with `Eof`,
/// as `tokenize` returns them
pub fn pre_parse(tokens: &[LexerToken]) -> Result<Vec<ParserToken>, PreParseError> {
    PreParser::new(tokens).run()
}

/// Reading position in the lexer tokens
struct PreParser<'a> {
    tokens: &'a [LexerToken],
    /// Index of the current token, never past `Eof`
    pos: usize,
}

impl<'a> PreParser<'a> {
    fn new(tokens: &'a [LexerToken]) -> Self {
        Self { tokens, pos: 0 }
    }

    /// The current token, without moving
    fn peek(&self) -> &'a LexerToken {
        &self.tokens[self.pos]
    }

    /// The current token, then moves to the next one (stays on `Eof`)
    fn bump(&mut self) -> &'a LexerToken {
        let token = self.peek();
        if token.kind != LexerTokenKind::Eof {
            self.pos += 1;
        }
        token
    }

    fn run(self) -> Result<Vec<ParserToken>, PreParseError> {
        todo!("split and classify the lexer tokens")
    }
}
