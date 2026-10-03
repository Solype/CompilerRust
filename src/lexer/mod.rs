pub mod lexer_error;
pub mod span;
pub mod token;

#[cfg(test)]
mod lexer_tests;

use lexer_error::{LexError, LexErrorKind};
use span::Span;
use token::{Punctuation, Token, TokenKind};

/// Cuts `src` into tokens, the last one being `Eof`; stops at the first error
pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    Lexer { src, pos: 0 }.run()
}

struct Lexer<'a> {
    src: &'a str,
    /// In bytes, always on a character boundary
    pos: usize,
}

/// Hangul syllables (U+AC00 to U+D7A3), not `is_alphabetic`, which takes any script
fn is_hangul(c: char) -> bool {
    matches!(c, '가'..='힣')
}

fn is_latin(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

impl Lexer<'_> {
    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn peek_at(&self, n: usize) -> Option<char> {
        self.src[self.pos..].chars().nth(n)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// Consumes `expected` if it comes next
    fn eat(&mut self, expected: char) -> bool {
        let found = self.peek() == Some(expected);
        if found {
            self.bump();
        }
        found
    }

    fn eat_while(&mut self, f: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&f) {
            self.bump();
        }
    }

    fn error(&self, kind: LexErrorKind, start: usize) -> LexError {
        LexError::new(kind, Span::new(start, self.pos))
    }

    fn run(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_trivia()?;
            let start = self.pos;
            let Some(c) = self.bump() else { break };
            let kind = self.token_kind(c, start)?;
            tokens.push(Token::new(kind, Span::new(start, self.pos)));
        }
        let end = self.src.len();
        tokens.push(Token::new(TokenKind::Eof, Span::new(end, end)));
        Ok(tokens)
    }

    /// Skips spaces and comments
    fn skip_trivia(&mut self) -> Result<(), LexError> {
        loop {
            match (self.peek(), self.peek_at(1)) {
                (Some(' ' | '\t' | '\r' | '\n'), _) => {
                    self.bump();
                }
                (Some('/'), Some('/')) => self.eat_while(|c| c != '\n'),
                (Some('/'), Some('*')) => {
                    let start = self.pos;
                    let Some(len) = self.src[start + 2..].find("*/") else {
                        return Err(LexError::new(
                            LexErrorKind::UnterminatedComment,
                            Span::new(start, start + 2),
                        ));
                    };
                    self.pos = start + 2 + len + 2;
                }
                _ => return Ok(()),
            }
        }
    }

    /// `c` (already consumed) starts the token at `start`
    fn token_kind(&mut self, c: char, start: usize) -> Result<TokenKind, LexError> {
        use TokenKind::*;

        if let Some(punctuation) = self.punctuation(c) {
            return Ok(Punctuation(punctuation));
        }

        let kind = match c {
            '가'..='힣' => {
                self.eat_while(is_hangul);
                HangulWord(self.src[start..self.pos].to_string())
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.eat_while(is_latin);
                LatinWord(self.src[start..self.pos].to_string())
            }
            '0'..='9' => {
                self.eat_while(|c| c.is_ascii_digit());
                let value = self.src[start..self.pos].parse::<i64>();
                Int(value.map_err(|_| self.error(LexErrorKind::IntegerOverflow, start))?)
            }
            '"' => self.string(start)?,
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
            'ㄱ'..='ㆎ' => return Err(self.error(LexErrorKind::LooseJamo(c), start)),
            _ => return Err(self.error(LexErrorKind::UnexpectedChar(c), start)),
        };
        Ok(kind)
    }

    /// The punctuation `c` (already consumed) starts, if any
    fn punctuation(&mut self, c: char) -> Option<Punctuation> {
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

    /// After the opening `"` at `start`: the decoded content, up to the closing `"`
    fn string(&mut self, start: usize) -> Result<TokenKind, LexError> {
        let unterminated = LexError::new(
            LexErrorKind::UnterminatedString,
            Span::new(start, start + 1),
        );
        let mut value = String::new();
        loop {
            let escape_start = self.pos;
            match self.bump() {
                None | Some('\n') => return Err(unterminated),
                Some('"') => return Ok(TokenKind::Str(value)),
                Some('\\') => {
                    let decoded = match self.bump() {
                        None | Some('\n') => return Err(unterminated),
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('0') => '\0',
                        Some('\\') => '\\',
                        Some('"') => '"',
                        Some(other) => {
                            return Err(
                                self.error(LexErrorKind::InvalidEscape(other), escape_start)
                            );
                        }
                    };
                    value.push(decoded);
                }
                Some(c) => value.push(c),
            }
        }
    }
}
