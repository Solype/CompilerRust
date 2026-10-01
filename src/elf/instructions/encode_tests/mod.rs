//! Tests unitaires de l'encodeur, une famille d'instructions par fichier.
//!
//! Chaque cas compare `Instruction::encode` aux octets produits par GNU as
//! pour l'instruction donnée en commentaire. Quand plusieurs encodages sont
//! valides, ils sont séparés par `|`.

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

/// Taille par défaut utilisée par `ElfFile64`
const DEFAULT_SIZE: Size = Size::U64;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}

pub(super) fn check(ins: Instruction, expected: &[&[u8]]) {
    let got = ins.encode(DEFAULT_SIZE).data;
    if !expected.iter().any(|e| *e == got.as_slice()) {
        let expected: Vec<String> = expected.iter().map(|e| hex(e)).collect();
        panic!(
            "{ins:?}\n  attendu : {}\n  obtenu  : {}\nGOT=[{}]",
            expected.join(" | "),
            hex(&got),
            hex(&got),
        );
    }
}

/// `nom: instruction => [octets] | [autre encodage valide];`
macro_rules! cases {
    ($( $(#[$attr:meta])* $name:ident : $ins:expr => $([$($b:expr),*])|+ ; )*) => {
        $(
            $(#[$attr])*
            #[test]
            fn $name() {
                $crate::elf::instructions::encode_tests::check($ins, &[$(&[$($b),*]),+]);
            }
        )*
    };
}
pub(super) use cases;
