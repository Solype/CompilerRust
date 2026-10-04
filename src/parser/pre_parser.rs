use crate::lexer::lexer_token::{LexerToken, LexerTokenKind, Punctuation};
use crate::lexer::span::Span;

use super::parser_token::{ParserToken, ParserTokenKind};
use super::pre_parse_error::{PreParseError, PreParseErrorKind};
use super::words::{has_particule, is_keyword, is_particle, is_type};

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

    /// One lexer token gives one parser token, or two for a split word (정수를 → 정수 + 를);
    /// `prev` is the lexer token before it, if any
    fn transform_token(
        token: &LexerToken,
        prev: Option<&LexerTokenKind>,
    ) -> Result<Vec<ParserToken>, PreParseError> {
        let kind = match &token.kind {
            LexerTokenKind::HangulWord(word) => return Self::hangul_word(word, token.span, prev),
            LexerTokenKind::LatinWord(name) => ParserTokenKind::Name(name.clone()),
            LexerTokenKind::Int(value) => ParserTokenKind::Int(*value),
            LexerTokenKind::Str(value) => ParserTokenKind::Str(value.clone()),
            LexerTokenKind::Punctuation(punctuation) => ParserTokenKind::Punctuation(*punctuation),
            LexerTokenKind::Operator(operator) => ParserTokenKind::Operator(*operator),
            LexerTokenKind::Eof => ParserTokenKind::Eof,
        };
        Ok(vec![ParserToken::new(kind, token.span)])
    }

    /// Classifies a Hangul word, in this order:
    /// 1. the whole word is a keyword or a type: 주는, 정수 (주는 is not 주 + 는)
    /// 2. after a number, a string or `)`, the whole word is a particle: 42를, 0이에요
    /// 3. a keyword or a type followed by a particle: 정수를, 값이에요, 10개예요
    fn hangul_word(
        word: &str,
        span: Span,
        prev: Option<&LexerTokenKind>,
    ) -> Result<Vec<ParserToken>, PreParseError> {
        let whole = |kind| Ok(vec![ParserToken::new(kind, span)]);

        if let Some(keyword) = is_keyword(word) {
            return whole(ParserTokenKind::Keyword(keyword));
        }
        if let Some(ty) = is_type(word) {
            return whole(ParserTokenKind::Type(ty));
        }

        let after_a_value = matches!(
            prev,
            Some(
                LexerTokenKind::Int(_)
                    | LexerTokenKind::Str(_)
                    | LexerTokenKind::Punctuation(Punctuation::RParen)
            )
        );
        if after_a_value && let Some(particle) = is_particle(word) {
            return whole(ParserTokenKind::Particle(particle));
        }

        if let Some((stem, particle)) = has_particule(word) {
            let stem_kind = is_keyword(stem)
                .map(ParserTokenKind::Keyword)
                .or_else(|| is_type(stem).map(ParserTokenKind::Type));
            if let Some(stem_kind) = stem_kind {
                // byte offsets, like Span: 정수를 0..9 → 정수 0..6, 를 6..9
                let cut = span.start + stem.len();
                return Ok(vec![
                    ParserToken::new(stem_kind, Span::new(span.start, cut)),
                    ParserToken::new(
                        ParserTokenKind::Particle(particle),
                        Span::new(cut, span.end),
                    ),
                ]);
            }
        }

        Err(PreParseError::new(
            PreParseErrorKind::UnknownWord(word.to_string()),
            span,
        ))
    }

    fn run(mut self) -> Result<Vec<ParserToken>, PreParseError> {
        let mut tokens = Vec::new();
        let mut prev = None;
        loop {
            let token = self.bump();
            tokens.extend(Self::transform_token(token, prev)?);
            if token.kind == LexerTokenKind::Eof {
                return Ok(tokens);
            }
            prev = Some(&token.kind);
        }
    }
}
