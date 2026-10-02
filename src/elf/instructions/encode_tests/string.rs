//! Generated encoding tests: reference bytes produced by GNU as.
//! `#[ignore = "BUG: ..."]` marks a wrong encoding (see `cargo test -- --ignored`).

use super::*;

cases! {
    /// movsb
    movsb: string(StringOp::Movs, BYTE) => [0xA4];
    /// movsw
    movsw: string(StringOp::Movs, WORD) => [0x66, 0xA5];
    /// movsd
    movsd: string(StringOp::Movs, DWORD) => [0xA5];
    /// movsq
    movsq: string(StringOp::Movs, QWORD) => [0x48, 0xA5];
    /// movsq
    movsq_2: string(StringOp::Movs, None) => [0x48, 0xA5];
    /// cmpsb
    cmpsb: string(StringOp::Cmps, BYTE) => [0xA6];
    /// cmpsw
    cmpsw: string(StringOp::Cmps, WORD) => [0x66, 0xA7];
    /// cmpsd
    cmpsd: string(StringOp::Cmps, DWORD) => [0xA7];
    /// cmpsq
    cmpsq: string(StringOp::Cmps, QWORD) => [0x48, 0xA7];
    /// cmpsq
    cmpsq_2: string(StringOp::Cmps, None) => [0x48, 0xA7];
    /// scasb
    scasb: string(StringOp::Scas, BYTE) => [0xAE];
    /// scasw
    scasw: string(StringOp::Scas, WORD) => [0x66, 0xAF];
    /// scasd
    scasd: string(StringOp::Scas, DWORD) => [0xAF];
    /// scasq
    scasq: string(StringOp::Scas, QWORD) => [0x48, 0xAF];
    /// scasq
    scasq_2: string(StringOp::Scas, None) => [0x48, 0xAF];
    /// lodsb
    lodsb: string(StringOp::Lods, BYTE) => [0xAC];
    /// lodsw
    lodsw: string(StringOp::Lods, WORD) => [0x66, 0xAD];
    /// lodsd
    lodsd: string(StringOp::Lods, DWORD) => [0xAD];
    /// lodsq
    lodsq: string(StringOp::Lods, QWORD) => [0x48, 0xAD];
    /// lodsq
    lodsq_2: string(StringOp::Lods, None) => [0x48, 0xAD];
    /// stosb
    stosb: string(StringOp::Stos, BYTE) => [0xAA];
    /// stosw
    stosw: string(StringOp::Stos, WORD) => [0x66, 0xAB];
    /// stosd
    stosd: string(StringOp::Stos, DWORD) => [0xAB];
    /// stosq
    stosq: string(StringOp::Stos, QWORD) => [0x48, 0xAB];
    /// stosq
    stosq_2: string(StringOp::Stos, None) => [0x48, 0xAB];
    /// rep movsb
    rep_movsb: prefixed(vec![Prefix::Rep], string(StringOp::Movs, BYTE)) => [0xF3, 0xA4];
    /// rep stosq
    rep_stosq: prefixed(vec![Prefix::Rep], string(StringOp::Stos, QWORD)) => [0xF3, 0x48, 0xAB];
    /// repe cmpsb
    repe_cmpsb: prefixed(vec![Prefix::Repe], string(StringOp::Cmps, BYTE)) => [0xF3, 0xA6];
    /// repne scasd
    repne_scasd: prefixed(vec![Prefix::Repne], string(StringOp::Scas, DWORD)) => [0xF2, 0xAF];
}
