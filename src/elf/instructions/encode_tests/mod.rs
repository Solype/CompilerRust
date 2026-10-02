//! Encoder unit tests, one instruction family per file.
//!
//! Each case compares `Instruction::encode` with the bytes GNU as produces
//! for the instruction given in the comment. When several encodings are
//! valid, they are separated by `|`.

pub(super) use crate::elf::instructions::{register::*, *};
pub(super) use crate::samples::helpers::*;

mod alu;
mod bit;
mod bitscan;
mod cmov;
mod complex;
mod conversion;
mod ctrl;
mod invalid_forms;
mod lea;
mod mov;
mod no_operand;
mod prefix;
mod relocations;
mod setcc;
mod shift;
mod sse;
mod stack;
mod string;
mod sys;
mod unary;

/// Default size used by `ElfFile64`
const DEFAULT_SIZE: Size = Size::U64;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}

/// Text of a case's `///` comment (the instruction in assembly).
fn asm_from_attrs(attrs: &[&str]) -> Option<String> {
    attrs.iter().find_map(|a| {
        let rest = a.strip_prefix("doc")?.trim_start().strip_prefix('=')?.trim();
        // `stringify!` turns the comment into a raw string: `r" add bl, al"`
        let rest = rest.trim_start_matches('r').trim_matches('#');
        Some(rest.trim_matches('"').trim().to_string())
    })
}

/// Bytes obtained, those differing from `reference` in red, with a line of `^` below.
fn diff_lines(got: &[u8], reference: &[u8]) -> (String, String) {
    let color = std::env::var_os("NO_COLOR").is_none();
    let (mut bytes, mut marks) = (Vec::new(), Vec::new());
    for (i, b) in got.iter().enumerate() {
        let same = reference.get(i) == Some(b);
        bytes.push(if same || !color { format!("{b:02X}") } else { format!("\x1b[1;31m{b:02X}\x1b[0m") });
        marks.push(if same { "  " } else { "^^" });
    }
    // expected bytes that are missing
    for _ in got.len()..reference.len() {
        bytes.push(if color { "\x1b[2m--\x1b[0m".into() } else { "--".into() });
        marks.push("^^");
    }
    (bytes.join(" "), marks.join(" ").trim_end().to_string())
}

pub(super) fn check(ins: Instruction, expected: &[&[u8]], attrs: &[&str]) {
    let got = ins.encode(DEFAULT_SIZE).data;
    if !expected.iter().any(|e| *e == got.as_slice()) {
        let title = asm_from_attrs(attrs).unwrap_or_else(|| format!("{ins:?}"));
        let (got_str, marks) = diff_lines(&got, expected[0]);
        let expected: Vec<String> = expected.iter().map(|e| hex(e)).collect();
        panic!(
            "{title}\n  expected: {}\n  got     : {got_str}\n            {marks}\n  detail  : {ins:?}",
            expected.join(" | "),
        );
    }
}

/// `name: instruction => [bytes] | [other valid encoding];`
macro_rules! cases {
    ($( $(#[$attr:meta])* $name:ident : $ins:expr => $([$($b:expr),*])|+ ; )*) => {
        $(
            $(#[$attr])*
            #[test]
            fn $name() {
                $crate::elf::instructions::encode_tests::check($ins, &[$(&[$($b),*]),+], &[$(stringify!($attr)),*]);
            }
        )*
    };
}
pub(super) use cases;
