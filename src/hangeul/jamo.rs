//! Indices of every jamo in a Syllable, in Unicode order.
//! A consonant has two different indices as an initial and as a batchim (ㄴ: 2 and 4),
//! hence one module per position: `initial::NIEUN`, `batchim::NIEUN`.

// A complete reference table: most entries are not used (yet)
#![allow(dead_code)]

/// `Syllable::initial` (19)
pub mod initial {
    /// ㄱ
    pub const GIYEOK: u8 = 0;
    /// ㄲ
    pub const SSANGGIYEOK: u8 = 1;
    /// ㄴ
    pub const NIEUN: u8 = 2;
    /// ㄷ
    pub const DIGEUT: u8 = 3;
    /// ㄸ
    pub const SSANGDIGEUT: u8 = 4;
    /// ㄹ
    pub const RIEUL: u8 = 5;
    /// ㅁ
    pub const MIEUM: u8 = 6;
    /// ㅂ
    pub const BIEUP: u8 = 7;
    /// ㅃ
    pub const SSANGBIEUP: u8 = 8;
    /// ㅅ
    pub const SIOS: u8 = 9;
    /// ㅆ
    pub const SSANGSIOS: u8 = 10;
    /// ㅇ (silent as an initial)
    pub const IEUNG: u8 = 11;
    /// ㅈ
    pub const JIEUT: u8 = 12;
    /// ㅉ
    pub const SSANGJIEUT: u8 = 13;
    /// ㅊ
    pub const CHIEUT: u8 = 14;
    /// ㅋ
    pub const KIEUK: u8 = 15;
    /// ㅌ
    pub const TIEUT: u8 = 16;
    /// ㅍ
    pub const PIEUP: u8 = 17;
    /// ㅎ
    pub const HIEUH: u8 = 18;
}

/// `Syllable::vowel` (21)
pub mod vowel {
    /// ㅏ
    pub const A: u8 = 0;
    /// ㅐ
    pub const AE: u8 = 1;
    /// ㅑ
    pub const YA: u8 = 2;
    /// ㅒ
    pub const YAE: u8 = 3;
    /// ㅓ
    pub const EO: u8 = 4;
    /// ㅔ
    pub const E: u8 = 5;
    /// ㅕ
    pub const YEO: u8 = 6;
    /// ㅖ
    pub const YE: u8 = 7;
    /// ㅗ
    pub const O: u8 = 8;
    /// ㅘ
    pub const WA: u8 = 9;
    /// ㅙ
    pub const WAE: u8 = 10;
    /// ㅚ
    pub const OE: u8 = 11;
    /// ㅛ
    pub const YO: u8 = 12;
    /// ㅜ
    pub const U: u8 = 13;
    /// ㅝ
    pub const WO: u8 = 14;
    /// ㅞ
    pub const WE: u8 = 15;
    /// ㅟ
    pub const WI: u8 = 16;
    /// ㅠ
    pub const YU: u8 = 17;
    /// ㅡ
    pub const EU: u8 = 18;
    /// ㅢ
    pub const UI: u8 = 19;
    /// ㅣ
    pub const I: u8 = 20;
}

/// `Syllable::batchim` (27, plus 0 for none)
pub mod batchim {
    /// No batchim
    pub const NONE: u8 = 0;
    /// ㄱ
    pub const GIYEOK: u8 = 1;
    /// ㄲ
    pub const SSANGGIYEOK: u8 = 2;
    /// ㄳ
    pub const GIYEOK_SIOS: u8 = 3;
    /// ㄴ
    pub const NIEUN: u8 = 4;
    /// ㄵ
    pub const NIEUN_JIEUT: u8 = 5;
    /// ㄶ
    pub const NIEUN_HIEUH: u8 = 6;
    /// ㄷ
    pub const DIGEUT: u8 = 7;
    /// ㄹ
    pub const RIEUL: u8 = 8;
    /// ㄺ
    pub const RIEUL_GIYEOK: u8 = 9;
    /// ㄻ
    pub const RIEUL_MIEUM: u8 = 10;
    /// ㄼ
    pub const RIEUL_BIEUP: u8 = 11;
    /// ㄽ
    pub const RIEUL_SIOS: u8 = 12;
    /// ㄾ
    pub const RIEUL_TIEUT: u8 = 13;
    /// ㄿ
    pub const RIEUL_PIEUP: u8 = 14;
    /// ㅀ
    pub const RIEUL_HIEUH: u8 = 15;
    /// ㅁ
    pub const MIEUM: u8 = 16;
    /// ㅂ
    pub const BIEUP: u8 = 17;
    /// ㅄ
    pub const BIEUP_SIOS: u8 = 18;
    /// ㅅ
    pub const SIOS: u8 = 19;
    /// ㅆ
    pub const SSANGSIOS: u8 = 20;
    /// ㅇ
    pub const IEUNG: u8 = 21;
    /// ㅈ
    pub const JIEUT: u8 = 22;
    /// ㅊ
    pub const CHIEUT: u8 = 23;
    /// ㅋ
    pub const KIEUK: u8 = 24;
    /// ㅌ
    pub const TIEUT: u8 = 25;
    /// ㅍ
    pub const PIEUP: u8 = 26;
    /// ㅎ
    pub const HIEUH: u8 = 27;
}
