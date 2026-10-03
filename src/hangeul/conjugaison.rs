use super::jamo::batchim;
use super::jamo::vowel::*;
use super::syllable::Syllable;

pub struct VerbForm {
    pub infinitive: String,
    /// The -아/어 base, before 서 or 요: 더해, 줘, 먹어
    pub conjugate: String,
}

impl VerbForm {
    /// The infinitive without its final 다: 더하다 → 더하
    fn stem(&self) -> &str {
        self.infinitive
            .strip_suffix('다')
            .expect("get_verb_form only builds verbs ending in 다")
    }

    /// -아/어요, ends a chain: 더해요
    pub fn yo(&self) -> String {
        format!("{}요", self.conjugate)
    }

    /// -아/어서, links a step of a chain: 더해서
    pub fn seo(&self) -> String {
        format!("{}서", self.conjugate)
    }

    /// -(으)ㄴ, qualifies a noun: 더한 값, 먹은, 만든
    pub fn adnominal(&self) -> String {
        let mut chars: Vec<char> = self.stem().chars().collect();
        let last = chars
            .pop()
            .and_then(Syllable::from_char)
            .expect("get_verb_form checks the last stem syllable");
        match last.batchim {
            // 하 → 한, 가 → 간 ; ㄹ falls and ㄴ replaces it: 만들 → 만든
            batchim::NONE | batchim::RIEUL => chars.push(
                Syllable {
                    batchim: batchim::NIEUN,
                    ..last
                }
                .to_char(),
            ),
            // 먹 → 먹은
            _ => chars.extend([last.to_char(), '은']),
        }
        chars.into_iter().collect()
    }
}

/// The -아/어 form of `syllable`, the last one of the stem (`prev`: the one before it, if any):
/// one syllable when it contracts (주 → 줘), two otherwise (먹 → 먹어)
pub fn add_conjugaison(syllable: Syllable, prev: Option<Syllable>) -> String {
    if syllable.to_char() == '하' {
        return "해".to_string();
    }
    let ending = if matches!(syllable.vowel, A | O) {
        '아'
    } else {
        '어'
    };
    if syllable.has_batchim() {
        return [syllable.to_char(), ending].iter().collect();
    }
    let new_vowel = match syllable.vowel {
        A | EO | AE | E => syllable.vowel,
        O => WA,
        U => WO,
        I => YEO,
        OE => WAE,
        EU => {
            if let Some(prev_syl) = prev
                && matches!(prev_syl.vowel, A | O)
            {
                A
            } else {
                EO
            }
        }
        // no contraction (ㅟ, ㅢ...): 쉬 → 쉬어
        _ => return [syllable.to_char(), ending].iter().collect(),
    };
    Syllable {
        vowel: new_vowel,
        ..syllable
    }
    .to_char()
    .to_string()
}

pub fn get_verb_form(stem: String) -> Option<VerbForm> {
    let mut chars: Vec<char> = stem.chars().collect();
    let n = chars.len();

    if n >= 2
        && let Some(second) = Syllable::from_char(chars[n - 2])
        && let Some(third) = Syllable::from_char(chars[n - 1])
        && third == Syllable::da()
    {
        let first = n.checked_sub(3).and_then(|i| Syllable::from_char(chars[i]));

        // drop 다 and the last stem syllable, replaced by its -아/어 form
        chars.truncate(n - 2);
        let mut conjugate: String = chars.into_iter().collect();
        conjugate.push_str(&add_conjugaison(second, first));

        return Some(VerbForm {
            infinitive: stem,
            conjugate,
        });
    }

    None
}
