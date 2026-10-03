use super::cursor::Lexer;
use super::lexer_error::{LexError, LexErrorKind};
use super::span::Span;
use super::lexer_token::{LexerToken, LexerTokenKind};

/// Cuts `src` into tokens, the last one being `Eof`; stops at the first error
pub fn tokenize(src: &str) -> Result<Vec<LexerToken>, LexError> {
    Lexer::new(src).run()
}

/// Hangul syllables (U+AC00 to U+D7A3), not `is_alphabetic`, which takes any script
fn is_hangul(c: char) -> bool {
    matches!(c, '가'..='힣')
}

fn is_latin(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

impl Lexer<'_> {
    fn run(mut self) -> Result<Vec<LexerToken>, LexError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_trivia()?;
            let start = self.pos;
            let Some(c) = self.bump() else { break };
            let kind = self.token_kind(c, start)?;
            tokens.push(LexerToken::new(kind, Span::new(start, self.pos)));
        }
        let end = self.src.len();
        tokens.push(LexerToken::new(LexerTokenKind::Eof, Span::new(end, end)));
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
    fn token_kind(&mut self, c: char, start: usize) -> Result<LexerTokenKind, LexError> {
        use LexerTokenKind::*;

        if let Some(punctuation) = self.punctuation(c) {
            return Ok(Punctuation(punctuation));
        }
        if let Some(operator) = self.operator(c, start)? {
            return Ok(Operator(operator));
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
            'ㄱ'..='ㆎ' => return Err(self.error(LexErrorKind::LooseJamo(c), start)),
            _ => return Err(self.error(LexErrorKind::UnexpectedChar(c), start)),
        };
        Ok(kind)
    }

    /// After the opening `"` at `start`: the decoded content, up to the closing `"`
    fn string(&mut self, start: usize) -> Result<LexerTokenKind, LexError> {
        let unterminated = LexError::new(
            LexErrorKind::UnterminatedString,
            Span::new(start, start + 1),
        );
        let mut value = String::new();
        loop {
            let escape_start = self.pos;
            match self.bump() {
                None | Some('\n') => return Err(unterminated),
                Some('"') => return Ok(LexerTokenKind::Str(value)),
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
