use super::conjugaison::get_verb_form;
use super::jamo::{batchim, initial, vowel};
use super::syllable::Syllable;

fn syllable(initial: u8, vowel: u8, batchim: u8) -> Syllable {
    Syllable {
        initial,
        vowel,
        batchim,
    }
}

#[test]
fn decompose() {
    // 한 = ㅎ (18) + ㅏ (0) + ㄴ (4)
    assert_eq!(Syllable::from_char('한'), Some(syllable(18, 0, 4)));
    assert_eq!(Syllable::from_char('가'), Some(syllable(0, 0, 0)));
    assert_eq!(Syllable::from_char('힣'), Some(syllable(18, 20, 27)));
}

#[test]
fn not_a_syllable() {
    for c in ['a', 'ㄱ', 'ㅏ', '1', ' '] {
        assert_eq!(Syllable::from_char(c), None, "{c:?}");
    }
}

#[test]
fn round_trip_on_every_syllable() {
    for c in '가'..='힣' {
        assert_eq!(Syllable::from_char(c).unwrap().to_char(), c);
    }
}

#[test]
fn batchim() {
    let has = |c| Syllable::from_char(c).unwrap().has_batchim();
    assert!(has('밥'));
    assert!(has('값'));
    assert!(!has('나'));
}

/// Each constant of jamo.rs against the real syllables: 가 까 나…, 아 애 야…, 각 갂 갃…
#[test]
fn jamo_constants_match_unicode() {
    use batchim::*;
    let initials = "가까나다따라마바빠사싸아자짜차카타파하";
    let vowels = "아애야얘어에여예오와왜외요우워웨위유으의이";
    let batchims = "가각갂갃간갅갆갇갈갉갊갋갌갍갎갏감갑값갓갔강갖갗갘같갚갛";
    let of = |s: &str, f: fn(Syllable) -> u8| -> Vec<u8> {
        s.chars()
            .map(|c| f(Syllable::from_char(c).unwrap()))
            .collect()
    };
    {
        use initial::*;
        let expected = [
            GIYEOK,
            SSANGGIYEOK,
            NIEUN,
            DIGEUT,
            SSANGDIGEUT,
            RIEUL,
            MIEUM,
            BIEUP,
            SSANGBIEUP,
            SIOS,
            SSANGSIOS,
            IEUNG,
            JIEUT,
            SSANGJIEUT,
            CHIEUT,
            KIEUK,
            TIEUT,
            PIEUP,
            HIEUH,
        ];
        assert_eq!(of(initials, |s| s.initial), expected);
    }
    {
        use vowel::*;
        let expected = [
            A, AE, YA, YAE, EO, E, YEO, YE, O, WA, WAE, OE, YO, U, WO, WE, WI, YU, EU, UI, I,
        ];
        assert_eq!(of(vowels, |s| s.vowel), expected);
    }
    let expected = [
        NONE,
        GIYEOK,
        SSANGGIYEOK,
        GIYEOK_SIOS,
        NIEUN,
        NIEUN_JIEUT,
        NIEUN_HIEUH,
        DIGEUT,
        RIEUL,
        RIEUL_GIYEOK,
        RIEUL_MIEUM,
        RIEUL_BIEUP,
        RIEUL_SIOS,
        RIEUL_TIEUT,
        RIEUL_PIEUP,
        RIEUL_HIEUH,
        MIEUM,
        BIEUP,
        BIEUP_SIOS,
        SIOS,
        SSANGSIOS,
        IEUNG,
        JIEUT,
        CHIEUT,
        KIEUK,
        TIEUT,
        PIEUP,
        HIEUH,
    ];
    assert_eq!(of(batchims, |s| s.batchim), expected);
}

/// The -아/어 base of `verb`
fn conjugate(verb: &str) -> String {
    get_verb_form(verb.to_string())
        .unwrap_or_else(|| panic!("{verb} should conjugate"))
        .conjugate
}

#[test]
fn conjugate_contractions() {
    let cases = [
        ("가다", "가"),
        ("서다", "서"),
        ("보내다", "보내"),
        ("세다", "세"),
        ("보다", "봐"),
        ("주다", "줘"),
        ("마시다", "마셔"),
        ("되다", "돼"),
        ("빼다", "빼"),
    ];
    for (verb, expected) in cases {
        assert_eq!(conjugate(verb), expected, "{verb}");
    }
}

#[test]
fn conjugate_eu_follows_previous_vowel() {
    assert_eq!(conjugate("쓰다"), "써");
    assert_eq!(conjugate("바쁘다"), "바빠");
    assert_eq!(conjugate("기쁘다"), "기뻐");
}

#[test]
fn conjugate_ha() {
    assert_eq!(conjugate("더하다"), "더해");
    assert_eq!(conjugate("하다"), "해");
    assert_eq!(conjugate("제곱하다"), "제곱해");
}

#[test]
fn conjugate_batchim_adds_a_syllable() {
    assert_eq!(conjugate("먹다"), "먹어");
    assert_eq!(conjugate("받다"), "받아");
    assert_eq!(conjugate("만들다"), "만들어");
}

#[test]
fn conjugate_without_contraction() {
    assert_eq!(conjugate("쉬다"), "쉬어");
}

#[test]
fn not_a_verb() {
    assert!(get_verb_form("먹어".to_string()).is_none());
    assert!(get_verb_form("다".to_string()).is_none());
}

fn verb(infinitive: &str) -> super::conjugaison::VerbForm {
    get_verb_form(infinitive.to_string()).unwrap()
}

#[test]
fn yo_and_seo() {
    assert_eq!(verb("더하다").yo(), "더해요");
    assert_eq!(verb("더하다").seo(), "더해서");
    assert_eq!(verb("빼다").yo(), "빼요");
    assert_eq!(verb("빼다").seo(), "빼서");
    assert_eq!(verb("주다").yo(), "줘요");
    assert_eq!(verb("먹다").seo(), "먹어서");
}

#[test]
fn several_forms_of_one_verb() {
    let form = verb("제곱하다");
    assert_eq!(form.seo(), "제곱해서");
    assert_eq!(form.yo(), "제곱해요");
    assert_eq!(form.adnominal(), "제곱한");
}

#[test]
fn adnominal() {
    let cases = [
        ("더하다", "더한"),
        ("빼다", "뺀"),
        ("가다", "간"),
        ("만들다", "만든"),
        ("먹다", "먹은"),
        ("받다", "받은"),
    ];
    for (infinitive, expected) in cases {
        assert_eq!(verb(infinitive).adnominal(), expected, "{infinitive}");
    }
}
