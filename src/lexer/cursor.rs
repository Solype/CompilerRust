use super::lexer_error::{LexError, LexErrorKind};
use super::span::Span;

/// Reading position in the source
pub(super) struct Lexer<'a> {
    pub(super) src: &'a str,
    /// In bytes, always on a character boundary
    pub(super) pos: usize,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    pub(super) fn peek_at(&self, n: usize) -> Option<char> {
        self.src[self.pos..].chars().nth(n)
    }

    pub(super) fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// Consumes `expected` if it comes next
    pub(super) fn eat(&mut self, expected: char) -> bool {
        let found = self.peek() == Some(expected);
        if found {
            self.bump();
        }
        found
    }

    pub(super) fn eat_while(&mut self, f: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&f) {
            self.bump();
        }
    }

    /// An error spanning from `start` to the current position
    pub(super) fn error(&self, kind: LexErrorKind, start: usize) -> LexError {
        LexError::new(kind, Span::new(start, self.pos))
    }
}
