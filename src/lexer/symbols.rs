use super::cursor::Lexer;
use super::lexer_error::{LexError, LexErrorKind};
use super::lexer_token::{Operator, Punctuation};

impl Lexer<'_> {
    /// The punctuation `c` (already consumed) starts, if any
    pub(super) fn punctuation(&mut self, c: char) -> Option<Punctuation> {
        use Punctuation::*;

        let punctuation = match c {
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            ',' => Comma,
            '.' if self.peek() == Some('.') && self.peek_at(1) == Some('.') => {
                self.bump();
                self.bump();
                Ellipsis
            }
            '.' => Dot,
            '…' => Ellipsis,
            _ => return None,
        };
        Some(punctuation)
    }

    /// The operator `c` (already consumed) starts at `start`, if any; `=` and `!` alone are errors
    pub(super) fn operator(&mut self, c: char, start: usize) -> Result<Option<Operator>, LexError> {
        use Operator::*;

        let operator = match c {
            '<' if self.eat('<') => Shl,
            '<' if self.eat('=') => Le,
            '<' => Lt,
            '>' if self.eat('>') => Shr,
            '>' if self.eat('=') => Ge,
            '>' => Gt,
            '=' if self.eat('=') => EqEq,
            '=' => return Err(self.error(LexErrorKind::LoneEquals, start)),
            '!' if self.eat('=') => NotEq,
            '!' => return Err(self.error(LexErrorKind::LoneBang, start)),
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            // `//` and `/*` are already skipped as comments
            '/' => Slash,
            '%' => Percent,
            '&' => Amp,
            '|' => Pipe,
            '^' => Caret,
            '~' => Tilde,
            _ => return Ok(None),
        };
        Ok(Some(operator))
    }
}
