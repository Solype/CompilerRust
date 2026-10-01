//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// syscall
    syscall: sys(SysOp::Syscall) => [0x0F, 0x05];
    /// sysenter
    sysenter: sys(SysOp::Sysenter) => [0x0F, 0x34];
    /// int 0x80
    int_0x80: sys(SysOp::Int(0x80)) => [0xCD, 0x80];
    /// int 3
    int_3: sys(SysOp::Int(3)) => [0xCC] | [0xCD, 0x03];
}
