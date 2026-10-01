//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// sete cl
    sete_cl: setcc(ConditionCode::E, reg(RCX)) => [0x0F, 0x94, 0xC1];
    /// setne cl
    setne_cl: setcc(ConditionCode::NE, reg(RCX)) => [0x0F, 0x95, 0xC1];
    /// setg cl
    setg_cl: setcc(ConditionCode::G, reg(RCX)) => [0x0F, 0x9F, 0xC1];
    /// setge cl
    setge_cl: setcc(ConditionCode::GE, reg(RCX)) => [0x0F, 0x9D, 0xC1];
    /// setl cl
    setl_cl: setcc(ConditionCode::L, reg(RCX)) => [0x0F, 0x9C, 0xC1];
    /// setle cl
    setle_cl: setcc(ConditionCode::LE, reg(RCX)) => [0x0F, 0x9E, 0xC1];
    /// seta cl
    seta_cl: setcc(ConditionCode::A, reg(RCX)) => [0x0F, 0x97, 0xC1];
    /// setae cl
    setae_cl: setcc(ConditionCode::AE, reg(RCX)) => [0x0F, 0x93, 0xC1];
    /// setb cl
    setb_cl: setcc(ConditionCode::B, reg(RCX)) => [0x0F, 0x92, 0xC1];
    /// setbe cl
    setbe_cl: setcc(ConditionCode::BE, reg(RCX)) => [0x0F, 0x96, 0xC1];
    /// sets cl
    sets_cl: setcc(ConditionCode::S, reg(RCX)) => [0x0F, 0x98, 0xC1];
    /// setns cl
    setns_cl: setcc(ConditionCode::NS, reg(RCX)) => [0x0F, 0x99, 0xC1];
    /// seto cl
    seto_cl: setcc(ConditionCode::O, reg(RCX)) => [0x0F, 0x90, 0xC1];
    /// setno cl
    setno_cl: setcc(ConditionCode::NO, reg(RCX)) => [0x0F, 0x91, 0xC1];
    /// setp cl
    setp_cl: setcc(ConditionCode::P, reg(RCX)) => [0x0F, 0x9A, 0xC1];
    /// setnp cl
    setnp_cl: setcc(ConditionCode::NP, reg(RCX)) => [0x0F, 0x9B, 0xC1];
    /// sete sil
    #[ignore = "BUG: produit `sete dh`"]
    sete_sil: setcc(ConditionCode::E, reg(RSI)) => [0x40, 0x0F, 0x94, 0xC6];
    /// sete r10b
    sete_r10b: setcc(ConditionCode::E, reg(R10)) => [0x41, 0x0F, 0x94, 0xC2];
    /// setne byte ptr [rbx+8]
    setne_byte_rbx_plus_8: setcc(ConditionCode::NE, mem(at(RBX).disp(8))) => [0x0F, 0x95, 0x43, 0x08];
    /// setne byte ptr [r9]
    setne_byte_r9: setcc(ConditionCode::NE, mem(at(R9))) => [0x41, 0x0F, 0x95, 0x01];
}
