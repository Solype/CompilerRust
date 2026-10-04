use crate::hangeul::conjugaison::get_verb_form;
use crate::parser::parser_token::{
    Ending,
    Keyword::{self, *},
    Particle::{self, *},
    Type,
};

/// Every one-word keyword; `부호 없는` (Unsigned) is two lexer tokens, merged elsewhere
pub(super) const KEYWORDS: &[(&str, Keyword)] = &[
    ("외부", Extern),
    ("주는", Giving),
    ("고정된", Const),
    ("짧은", Short),
    ("빈", Void),
    ("만약", If),
    ("아니면", Otherwise),
    ("동안", While),
    ("세면서", Counting),
    ("줘요", Return),
    ("그만해요", Stop),
    ("넘어가요", Skip),
    ("그리고", And),
    ("또는", Or),
    ("값", Value),
    ("크기", Size),
    ("개", Count),
];

/// Every particle, both spellings (after a batchim / after a vowel) giving the same variant
pub(super) const PARTICULES: &[(&str, Particle)] = &[
    ("을", Object),
    ("를", Object),
    ("이", Subject),
    ("가", Subject),
    ("은", Topic),
    ("는", Topic),
    ("과", With),
    ("와", With),
    ("에", In),
    ("으로", To),
    ("로", To),
    ("의", Of),
    ("부터", From),
    ("까지", Until),
    ("이면", IfItIs),
    ("면", IfItIs),
    ("이에요", ItIs),
    ("예요", ItIs),
    ("인", ThatIs),
];

/// Every type word; `&` after a type is also `Address`, handled elsewhere
pub(super) const TYPES: &[(&str, Type)] = &[
    ("정수", Type::Int),
    ("실수", Type::Float),
    ("논리", Type::Bool),
    ("문자", Type::Char),
    ("바이트", Type::Byte),
    ("주소", Type::Address),
];

/// The type `word` is exactly, if any: "정수" → Int, but "정수를" → None
pub(super) fn is_type(word: &str) -> Option<Type> {
    TYPES
        .iter()
        .find(|(text, _)| *text == word)
        .map(|(_, ty)| *ty)
}

/// The keyword `word` is exactly, if any: "주는" → Giving, but "정수를" → None
pub(super) fn is_keyword(word: &str) -> Option<Keyword> {
    KEYWORDS
        .iter()
        .find(|(text, _)| *text == word)
        .map(|(_, keyword)| *keyword)
}

/// The particle `word` is exactly, if any: "를" → Object, "이에요" → ItIs, but "나를" → None
pub(super) fn is_particle(word: &str) -> Option<Particle> {
    PARTICULES
        .iter()
        .find(|(text, _)| *text == word)
        .map(|(_, particle)| *particle)
}

/// Splits the particle off the end of `word`: "합은" → ("합", Topic), "를" → ("", Object);
/// None if it ends with no particle. When two fit (이면 / 면), the longest wins: "사이면" →
/// ("사", IfItIs). Check keywords first: "주는" and "아니면" end like particles
pub(super) fn has_particule(word: &str) -> Option<(&str, Particle)> {
    PARTICULES
        .iter()
        .filter_map(|(text, particle)| word.strip_suffix(text).map(|stem| (stem, *particle)))
        .min_by_key(|(stem, _)| stem.len())
}

/// Built-in verbs used in several forms (넣어요, 넣어서); 주다 and 세다 only appear as the
/// keywords 줘요 and 세면서
pub(super) const BUILTIN_VERBS: &[&str] = &["넣다", "바꾸다"];

/// Every form of `infinitive` the pre-parser recognizes: 더하다 → 더하다, 더해서, 더해요, 더한;
/// None if it is not a verb (no final 다)
pub(super) fn verb_forms(infinitive: &str) -> Option<[(String, Ending); 4]> {
    let form = get_verb_form(infinitive.to_string())?;
    Some([
        (infinitive.to_string(), Ending::Da),
        (form.seo(), Ending::Seo),
        (form.yo(), Ending::Yo),
        (form.adnominal(), Ending::Adnominal),
    ])
}
