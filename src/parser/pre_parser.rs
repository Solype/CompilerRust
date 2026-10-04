use std::collections::HashMap;

use crate::lexer::lexer_token::{LexerToken, LexerTokenKind, Punctuation};
use crate::lexer::span::Span;

use super::parser_token::{Ending, ParserToken, ParserTokenKind};
use super::pre_parse_error::PreParseError;
use super::pre_parse_warning::{PreParseWarning, PreParseWarningKind};
use super::words::{BUILTIN_VERBS, has_particule, is_keyword, is_particle, is_type, verb_forms};

/// Splits and classifies the lexer tokens (나를 → 나 + 를); `tokens` must end with `Eof`,
/// as `tokenize` returns them
pub fn pre_parse(tokens: &[LexerToken]) -> Result<PreParsed, PreParseError> {
    PreParser::new(tokens).run()
}

/// The pre-parser output: the tokens, and what looked suspicious on the way
pub struct PreParsed {
    pub tokens: Vec<ParserToken>,
    pub warnings: Vec<PreParseWarning>,
}

/// Reading position in the lexer tokens
struct PreParser<'a> {
    tokens: &'a [LexerToken],
    /// Index of the current token, never past `Eof`
    pos: usize,
    /// Every form of the known verbs → (infinitive, ending): 더해서 → (더하다, Seo)
    verbs: HashMap<String, (String, Ending)>,
}

impl<'a> PreParser<'a> {
    fn new(tokens: &'a [LexerToken]) -> Self {
        let mut verbs = HashMap::new();
        for infinitive in BUILTIN_VERBS.iter().copied().chain(declared_verbs(tokens)) {
            for (form, ending) in verb_forms(infinitive).into_iter().flatten() {
                verbs.insert(form, (infinitive.to_string(), ending));
            }
        }
        Self {
            tokens,
            pos: 0,
            verbs,
        }
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
        &self,
        token: &LexerToken,
        prev: Option<&LexerTokenKind>,
    ) -> Result<Vec<ParserToken>, PreParseError> {
        let kind = match &token.kind {
            LexerTokenKind::HangulWord(word) => return self.hangul_word(word, token.span, prev),
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
    /// 1. the whole word is a keyword, a type or a verb form: 주는, 정수, 더해서 (주는 is not 주 + 는)
    /// 2. after a number, a string or `)`, the whole word is a particle: 42를, 0이에요
    /// 3. a keyword or a type followed by a particle: 정수를, 값이에요, 10개예요
    /// 4. anything else is an identifier, left whole: 나를, 결과
    fn hangul_word(
        &self,
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
        if let Some((infinitive, ending)) = self.verbs.get(word) {
            return whole(ParserTokenKind::Verb {
                infinitive: infinitive.clone(),
                ending: *ending,
            });
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

        // not split here: 결과 is a name, not 결 + 과, and only the declared names can tell
        whole(ParserTokenKind::Ident(word.to_string()))
    }

    fn run(mut self) -> Result<PreParsed, PreParseError> {
        let mut tokens = Vec::new();
        let mut warnings = Vec::new();
        let mut prev = None;
        loop {
            let token = self.bump();
            let parsed = self.transform_token(token, prev)?;
            if let Some(warning) = self.check_parentheses(&parsed) {
                warnings.push(warning);
            }
            tokens.extend(parsed);
            if token.kind == LexerTokenKind::Eof {
                return Ok(PreParsed { tokens, warnings });
            }
            prev = Some(&token.kind);
        }
    }

    /// A verb in its dictionary form must be followed by `(`: `더하다(…)` is a definition,
    /// `바다` alone is probably a name ending in 다
    fn check_parentheses(&self, parsed: &[ParserToken]) -> Option<PreParseWarning> {
        let [token] = parsed else { return None };
        let ParserTokenKind::Verb {
            infinitive,
            ending: Ending::Da,
        } = &token.kind
        else {
            return None;
        };
        let followed_by_paren = matches!(
            self.peek().kind,
            LexerTokenKind::Punctuation(Punctuation::LParen)
        );
        (!followed_by_paren).then(|| {
            PreParseWarning::new(
                PreParseWarningKind::VerbWithoutParentheses(infinitive.clone()),
                token.span,
            )
        })
    }
}

/// First pass: every verb of the file, so that a call before the definition is a verb too.
/// 다 marks the functions: any Hangul word ending in 다 is a verb in its dictionary form
fn declared_verbs(tokens: &[LexerToken]) -> impl Iterator<Item = &str> {
    tokens.iter().filter_map(|token| match &token.kind {
        LexerTokenKind::HangulWord(word) if word.ends_with('다') => Some(word.as_str()),
        _ => None,
    })
}
