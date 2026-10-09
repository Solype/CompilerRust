use std::collections::{BTreeMap, HashMap, HashSet};

use crate::lexer::lexer_token::{LexerToken, LexerTokenKind, Operator, Punctuation};
use crate::lexer::span::Span;
use crate::parser::parser_token::{Ending, Keyword, ParserToken, ParserTokenKind, Particle, Type};

use super::pre_parse_error::{PreParseError, PreParseErrorKind};
use super::pre_parse_warning::{PreParseWarning, PreParseWarningKind};
use super::words::{
    BUILTIN_VERBS, has_particule, is_bool, is_keyword, is_name_particle, is_particle, is_type,
    verb_forms,
};

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
        for infinitive in BUILTIN_VERBS
            .iter()
            .copied()
            .chain(Self::declared_verbs(tokens))
        {
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
            LexerTokenKind::Float(value) => ParserTokenKind::Float(*value),
            LexerTokenKind::Int(value) => ParserTokenKind::Int(*value),
            LexerTokenKind::Str(value) => ParserTokenKind::Str(value.clone()),
            LexerTokenKind::Punctuation(punctuation) => ParserTokenKind::Punctuation(*punctuation),
            LexerTokenKind::Operator(operator) => ParserTokenKind::Operator(*operator),
            LexerTokenKind::Eof => ParserTokenKind::Eof,
        };
        Ok(vec![ParserToken::new(kind, token.span)])
    }

    /// Classifies a Hangul word, in this order:
    /// 1. the whole word is a keyword, a type, a boolean or a verb form: 주는, 정수, 참, 더해서
    ///    (주는 is not 주 + 는)
    /// 2. after a number, a string, `)` or a Latin name, the whole word is a particle: 42를,
    ///    0이에요, printf를
    /// 3. a keyword, a type or a boolean followed by a particle: 정수를, 값이에요, 10개예요, 참이면
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
        if let Some(value) = is_bool(word) {
            return whole(ParserTokenKind::Bool(value));
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
                    | LexerTokenKind::Float(_)
                    | LexerTokenKind::Str(_)
                    | LexerTokenKind::Punctuation(Punctuation::RParen)
                    | LexerTokenKind::LatinWord(_)
            )
        );
        if after_a_value && let Some(particle) = is_particle(word) {
            return whole(ParserTokenKind::Particle(particle));
        }

        if let Some((stem, particle)) = has_particule(word) {
            let stem_kind = is_keyword(stem)
                .map(ParserTokenKind::Keyword)
                .or_else(|| is_type(stem).map(ParserTokenKind::Type))
                .or_else(|| is_bool(stem).map(ParserTokenKind::Bool));
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
            let parsed = match self.merge_unsigned(token) {
                Some(unsigned) => vec![unsigned],
                None => self.transform_token(token, prev)?,
            };
            let parsed = Self::amp_after_a_type(parsed, tokens.last());
            if let Some(warning) = self.check_parentheses(&parsed) {
                warnings.push(warning);
            }
            tokens.extend(parsed);
            if token.kind == LexerTokenKind::Eof {
                break;
            }
            // the last lexer token read: 없는 after a merged 부호 없는
            prev = Some(&self.tokens[self.pos - 1].kind);
        }
        let names = Self::declared_names(&tokens);
        for (i, replacement) in names.splits.into_iter().rev() {
            tokens.splice(i..=i, replacement);
        }
        let tokens = Self::tokenize_names(tokens, &names.names);
        // every word is classified by now: an `Ident` left is a name never declared
        if let Some(err) = tokens.iter().find_map(|token| match &token.kind {
            ParserTokenKind::Ident(word) => Some(PreParseError::new(
                PreParseErrorKind::UnknownIdentifier(word.clone()),
                token.span,
            )),
            _ => None,
        }) {
            return Err(err);
        }

        Ok(PreParsed { tokens, warnings })
    }

    /// `부호 없는` is two lexer tokens for one keyword: reads both, gives one `Unsigned` spanning
    /// them; None, reading nothing more, for anything else
    fn merge_unsigned(&mut self, token: &LexerToken) -> Option<ParserToken> {
        let LexerTokenKind::HangulWord(first) = &token.kind else {
            return None;
        };
        let LexerTokenKind::HangulWord(second) = &self.peek().kind else {
            return None;
        };
        if first != "부호" || second != "없는" {
            return None;
        }
        let end = self.bump().span.end;
        Some(ParserToken::new(
            ParserTokenKind::Keyword(Keyword::Unsigned),
            Span::new(token.span.start, end),
        ))
    }

    /// `&` right after a type is `주소`: `정수&` → Type(Int) + Type(Address); since it becomes a
    /// type itself, `정수&&` is two addresses. Before the declared names, so that `정수& 가)`
    /// declares 가; anywhere else `&` stays the operator
    fn amp_after_a_type(parsed: Vec<ParserToken>, last: Option<&ParserToken>) -> Vec<ParserToken> {
        match (parsed.as_slice(), last.map(|token| &token.kind)) {
            (
                [
                    ParserToken {
                        kind: ParserTokenKind::Operator(Operator::Amp),
                        span,
                    },
                ],
                Some(ParserTokenKind::Type(_)),
            ) => vec![ParserToken::new(
                ParserTokenKind::Type(Type::Address),
                *span,
            )],
            _ => parsed,
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

    /// First pass: every verb of the file, so that a call before the definition is a verb too.
    /// 다 marks the functions: any Hangul word ending in 다 is a verb in its dictionary form
    fn declared_verbs(tokens: &[LexerToken]) -> impl Iterator<Item = &str> {
        tokens.iter().filter_map(|token| match &token.kind {
            LexerTokenKind::HangulWord(word) if word.ends_with('다') => Some(word.as_str()),
            _ => None,
        })
    }

    /// Replaces every `Ident` left that is a declared name, alone or followed by a particle a name
    /// can carry: 나 → Name(나), 나를 → Name(나) + Particle(Object). When several names fit, the
    /// longest wins (가나를 → 가나 + 를, not 가 + 나를); any other `Ident` stays
    fn tokenize_names(tokens: Vec<ParserToken>, names: &[String]) -> Vec<ParserToken> {
        let names: HashSet<&str> = names.iter().map(String::as_str).collect();
        let mut out = Vec::with_capacity(tokens.len());
        for token in tokens {
            let ParserTokenKind::Ident(word) = &token.kind else {
                out.push(token);
                continue;
            };
            if names.contains(word.as_str()) {
                out.push(ParserToken::new(
                    ParserTokenKind::Name(word.clone()),
                    token.span,
                ));
                continue;
            }
            // from the longest name down: 가나를 tries 가나 before 가
            let split = word
                .char_indices()
                .rev()
                .filter(|(cut, _)| *cut > 0)
                .map(|(cut, _)| word.split_at(cut))
                .find_map(|(name, rest)| {
                    let particle = is_name_particle(rest)?;
                    names.contains(name).then_some((name, particle))
                });
            match split {
                Some((name, particle)) => out.extend(Self::split_name(name, particle, token.span)),
                None => out.push(token),
            }
        }
        out
    }

    /// Every declared variable and parameter name, from the pre-parsed tokens (names still `Ident`)
    fn declared_names(tokens: &[ParserToken]) -> DeclaredNames {
        let mut names = Vec::new();
        let mut splits = BTreeMap::new();
        for (i, token) in tokens.iter().enumerate() {
            let ParserTokenKind::Ident(word) = &token.kind else {
                continue;
            };
            let prev = i.checked_sub(1).map(|p| &tokens[p].kind);
            let next = tokens.get(i + 1).map(|t| &t.kind);

            // parameter: `정수 가,` or `정수 나)`, the whole word is the name
            let after_a_type = matches!(prev, Some(ParserTokenKind::Type(_)));
            let before_comma_or_paren = matches!(
                next,
                Some(ParserTokenKind::Punctuation(
                    Punctuation::Comma | Punctuation::RParen
                ))
            );
            if after_a_type && before_comma_or_paren {
                names.push(word.clone());
                continue;
            }

            // variable: `결과는 0이에요`, 은/는 at the start of a statement;
            // loop: `정수 칸을 0부터 10까지 세면서`, 을/를 after a type (`칸을` alone uses 칸)
            let Some((name, particle)) = has_particule(word) else {
                continue;
            };
            let declares = match particle {
                Particle::Topic => Self::starts_a_statement(prev),
                Particle::Object => after_a_type,
                _ => false,
            };
            if declares && !name.is_empty() {
                names.push(name.to_string());
                splits.insert(i, Self::split_name(name, particle, token.span));
            }
        }
        DeclaredNames { names, splits }
    }

    /// A declaring word cut after its name, each part with its own span
    /// (byte offsets, like Span: 결과는 0..9 → 결과 0..6, 는 6..9)
    fn split_name(name: &str, particle: Particle, span: Span) -> [ParserToken; 2] {
        let cut = span.start + name.len();
        [
            ParserToken::new(
                ParserTokenKind::Name(name.to_string()),
                Span::new(span.start, cut),
            ),
            ParserToken::new(
                ParserTokenKind::Particle(particle),
                Span::new(cut, span.end),
            ),
        ]
    }

    /// Whether the word after `prev` starts a statement: first word, after `{`, `}`, `.`, 고정된,
    /// or after a statement that ended in 요 (줘요, 넣어요, 이에요…)
    fn starts_a_statement(prev: Option<&ParserTokenKind>) -> bool {
        matches!(
            prev,
            None | Some(
                ParserTokenKind::Punctuation(
                    Punctuation::LBrace | Punctuation::RBrace | Punctuation::Dot
                ) | ParserTokenKind::Keyword(
                    Keyword::Const | Keyword::Return | Keyword::Stop | Keyword::Skip
                ) | ParserTokenKind::Verb {
                    ending: Ending::Yo,
                    ..
                } | ParserTokenKind::Particle(Particle::ItIs)
            )
        )
    }
}

/// The names declared by the `Ident`s of the pre-parsed tokens
#[derive(Debug)]
struct DeclaredNames {
    /// In the order of the file; a name declared twice appears twice
    names: Vec<String>,
    /// Index of a declaring `Ident` → the tokens that replace it, only when it is split:
    /// 결과는 → [Name(결과), Particle(Topic)]; nothing for a parameter (가)
    splits: BTreeMap<usize, [ParserToken; 2]>,
}
