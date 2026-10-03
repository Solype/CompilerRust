use crate::hangeul::jamo::{batchim, initial, vowel};

/// Code of 가, the first syllable: syllables are numbered from there
const FIRST: u32 = '가' as u32;

/// A Hangul syllable split into indices: initial 0..19, vowel 0..21, batchim 0..28 (0 = none)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Syllable {
    pub initial: u8,
    pub vowel: u8,
    pub batchim: u8,
}

impl Syllable {
    /// `None` if `c` is not a syllable (가..=힣)
    pub fn from_char(c: char) -> Option<Syllable> {
        if !('가'..='힣').contains(&c) {
            return None;
        }
        let code = c as u32 - FIRST;
        Some(Self {
            initial: (code / 588) as u8,
            vowel: (code % 588 / 28) as u8,
            batchim: (code % 28) as u8,
        })
    }

    pub fn to_char(self) -> char {
        let code = 588 * self.initial as u32 + 28 * self.vowel as u32 + self.batchim as u32;
        char::from_u32(FIRST + code).expect("jamo index out of range")
    }

    pub fn has_batchim(self) -> bool {
        self.batchim != 0
    }

    pub fn da() -> Syllable {
        return Syllable {
            initial: initial::DIGEUT,
            vowel: vowel::A,
            batchim: batchim::NONE,
        };
    }
}
