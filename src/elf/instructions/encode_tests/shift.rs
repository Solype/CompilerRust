//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// shl dl, 1
    shl_dl_1: shift(ShiftOp::Shl, reg(RDX), imm(1), BYTE) => [0xD0, 0xE2];
    /// shl dl, 3
    shl_dl_3: shift(ShiftOp::Shl, reg(RDX), imm(3), BYTE) => [0xC0, 0xE2, 0x03];
    /// shl dl, cl
    shl_dl_cl: shift(ShiftOp::Shl, reg(RDX), reg(RCX), BYTE) => [0xD2, 0xE2];
    /// shl dx, 1
    shl_dx_1: shift(ShiftOp::Shl, reg(RDX), imm(1), WORD) => [0x66, 0xD1, 0xE2];
    /// shl dx, 3
    shl_dx_3: shift(ShiftOp::Shl, reg(RDX), imm(3), WORD) => [0x66, 0xC1, 0xE2, 0x03];
    /// shl dx, cl
    shl_dx_cl: shift(ShiftOp::Shl, reg(RDX), reg(RCX), WORD) => [0x66, 0xD3, 0xE2];
    /// shl edx, 1
    shl_edx_1: shift(ShiftOp::Shl, reg(RDX), imm(1), DWORD) => [0xD1, 0xE2];
    /// shl edx, 3
    shl_edx_3: shift(ShiftOp::Shl, reg(RDX), imm(3), DWORD) => [0xC1, 0xE2, 0x03];
    /// shl edx, cl
    shl_edx_cl: shift(ShiftOp::Shl, reg(RDX), reg(RCX), DWORD) => [0xD3, 0xE2];
    /// shl rdx, 1
    shl_rdx_1: shift(ShiftOp::Shl, reg(RDX), imm(1), QWORD) => [0x48, 0xD1, 0xE2];
    /// shl rdx, 3
    shl_rdx_3: shift(ShiftOp::Shl, reg(RDX), imm(3), QWORD) => [0x48, 0xC1, 0xE2, 0x03];
    /// shl rdx, cl
    shl_rdx_cl: shift(ShiftOp::Shl, reg(RDX), reg(RCX), QWORD) => [0x48, 0xD3, 0xE2];
    /// shl r10, 7
    #[ignore = "BUG: produit `shl rdx,0x7`"]
    shl_r10_7: shift(ShiftOp::Shl, reg(R10), imm(7), QWORD) => [0x49, 0xC1, 0xE2, 0x07];
    /// shl dword ptr [rbx], 2
    shl_dword_rbx_2: shift(ShiftOp::Shl, mem(at(RBX)), imm(2), DWORD) => [0xC1, 0x23, 0x02];
    /// shl qword ptr [rbx+8], cl
    shl_qword_rbx_plus_8_cl: shift(ShiftOp::Shl, mem(at(RBX).disp(8)), reg(RCX), QWORD) => [0x48, 0xD3, 0x63, 0x08];
    /// shr dl, 1
    shr_dl_1: shift(ShiftOp::Shr, reg(RDX), imm(1), BYTE) => [0xD0, 0xEA];
    /// shr dl, 3
    shr_dl_3: shift(ShiftOp::Shr, reg(RDX), imm(3), BYTE) => [0xC0, 0xEA, 0x03];
    /// shr dl, cl
    shr_dl_cl: shift(ShiftOp::Shr, reg(RDX), reg(RCX), BYTE) => [0xD2, 0xEA];
    /// shr dx, 1
    shr_dx_1: shift(ShiftOp::Shr, reg(RDX), imm(1), WORD) => [0x66, 0xD1, 0xEA];
    /// shr dx, 3
    shr_dx_3: shift(ShiftOp::Shr, reg(RDX), imm(3), WORD) => [0x66, 0xC1, 0xEA, 0x03];
    /// shr dx, cl
    shr_dx_cl: shift(ShiftOp::Shr, reg(RDX), reg(RCX), WORD) => [0x66, 0xD3, 0xEA];
    /// shr edx, 1
    shr_edx_1: shift(ShiftOp::Shr, reg(RDX), imm(1), DWORD) => [0xD1, 0xEA];
    /// shr edx, 3
    shr_edx_3: shift(ShiftOp::Shr, reg(RDX), imm(3), DWORD) => [0xC1, 0xEA, 0x03];
    /// shr edx, cl
    shr_edx_cl: shift(ShiftOp::Shr, reg(RDX), reg(RCX), DWORD) => [0xD3, 0xEA];
    /// shr rdx, 1
    shr_rdx_1: shift(ShiftOp::Shr, reg(RDX), imm(1), QWORD) => [0x48, 0xD1, 0xEA];
    /// shr rdx, 3
    shr_rdx_3: shift(ShiftOp::Shr, reg(RDX), imm(3), QWORD) => [0x48, 0xC1, 0xEA, 0x03];
    /// shr rdx, cl
    shr_rdx_cl: shift(ShiftOp::Shr, reg(RDX), reg(RCX), QWORD) => [0x48, 0xD3, 0xEA];
    /// shr r10, 7
    #[ignore = "BUG: produit `shr rdx,0x7`"]
    shr_r10_7: shift(ShiftOp::Shr, reg(R10), imm(7), QWORD) => [0x49, 0xC1, 0xEA, 0x07];
    /// shr dword ptr [rbx], 2
    shr_dword_rbx_2: shift(ShiftOp::Shr, mem(at(RBX)), imm(2), DWORD) => [0xC1, 0x2B, 0x02];
    /// shr qword ptr [rbx+8], cl
    shr_qword_rbx_plus_8_cl: shift(ShiftOp::Shr, mem(at(RBX).disp(8)), reg(RCX), QWORD) => [0x48, 0xD3, 0x6B, 0x08];
    /// sar dl, 1
    sar_dl_1: shift(ShiftOp::Sar, reg(RDX), imm(1), BYTE) => [0xD0, 0xFA];
    /// sar dl, 3
    sar_dl_3: shift(ShiftOp::Sar, reg(RDX), imm(3), BYTE) => [0xC0, 0xFA, 0x03];
    /// sar dl, cl
    sar_dl_cl: shift(ShiftOp::Sar, reg(RDX), reg(RCX), BYTE) => [0xD2, 0xFA];
    /// sar dx, 1
    sar_dx_1: shift(ShiftOp::Sar, reg(RDX), imm(1), WORD) => [0x66, 0xD1, 0xFA];
    /// sar dx, 3
    sar_dx_3: shift(ShiftOp::Sar, reg(RDX), imm(3), WORD) => [0x66, 0xC1, 0xFA, 0x03];
    /// sar dx, cl
    sar_dx_cl: shift(ShiftOp::Sar, reg(RDX), reg(RCX), WORD) => [0x66, 0xD3, 0xFA];
    /// sar edx, 1
    sar_edx_1: shift(ShiftOp::Sar, reg(RDX), imm(1), DWORD) => [0xD1, 0xFA];
    /// sar edx, 3
    sar_edx_3: shift(ShiftOp::Sar, reg(RDX), imm(3), DWORD) => [0xC1, 0xFA, 0x03];
    /// sar edx, cl
    sar_edx_cl: shift(ShiftOp::Sar, reg(RDX), reg(RCX), DWORD) => [0xD3, 0xFA];
    /// sar rdx, 1
    sar_rdx_1: shift(ShiftOp::Sar, reg(RDX), imm(1), QWORD) => [0x48, 0xD1, 0xFA];
    /// sar rdx, 3
    sar_rdx_3: shift(ShiftOp::Sar, reg(RDX), imm(3), QWORD) => [0x48, 0xC1, 0xFA, 0x03];
    /// sar rdx, cl
    sar_rdx_cl: shift(ShiftOp::Sar, reg(RDX), reg(RCX), QWORD) => [0x48, 0xD3, 0xFA];
    /// sar r10, 7
    #[ignore = "BUG: produit `sar rdx,0x7`"]
    sar_r10_7: shift(ShiftOp::Sar, reg(R10), imm(7), QWORD) => [0x49, 0xC1, 0xFA, 0x07];
    /// sar dword ptr [rbx], 2
    sar_dword_rbx_2: shift(ShiftOp::Sar, mem(at(RBX)), imm(2), DWORD) => [0xC1, 0x3B, 0x02];
    /// sar qword ptr [rbx+8], cl
    sar_qword_rbx_plus_8_cl: shift(ShiftOp::Sar, mem(at(RBX).disp(8)), reg(RCX), QWORD) => [0x48, 0xD3, 0x7B, 0x08];
    /// rol dl, 1
    rol_dl_1: shift(ShiftOp::Rol, reg(RDX), imm(1), BYTE) => [0xD0, 0xC2];
    /// rol dl, 3
    rol_dl_3: shift(ShiftOp::Rol, reg(RDX), imm(3), BYTE) => [0xC0, 0xC2, 0x03];
    /// rol dl, cl
    rol_dl_cl: shift(ShiftOp::Rol, reg(RDX), reg(RCX), BYTE) => [0xD2, 0xC2];
    /// rol dx, 1
    rol_dx_1: shift(ShiftOp::Rol, reg(RDX), imm(1), WORD) => [0x66, 0xD1, 0xC2];
    /// rol dx, 3
    rol_dx_3: shift(ShiftOp::Rol, reg(RDX), imm(3), WORD) => [0x66, 0xC1, 0xC2, 0x03];
    /// rol dx, cl
    rol_dx_cl: shift(ShiftOp::Rol, reg(RDX), reg(RCX), WORD) => [0x66, 0xD3, 0xC2];
    /// rol edx, 1
    rol_edx_1: shift(ShiftOp::Rol, reg(RDX), imm(1), DWORD) => [0xD1, 0xC2];
    /// rol edx, 3
    rol_edx_3: shift(ShiftOp::Rol, reg(RDX), imm(3), DWORD) => [0xC1, 0xC2, 0x03];
    /// rol edx, cl
    rol_edx_cl: shift(ShiftOp::Rol, reg(RDX), reg(RCX), DWORD) => [0xD3, 0xC2];
    /// rol rdx, 1
    rol_rdx_1: shift(ShiftOp::Rol, reg(RDX), imm(1), QWORD) => [0x48, 0xD1, 0xC2];
    /// rol rdx, 3
    rol_rdx_3: shift(ShiftOp::Rol, reg(RDX), imm(3), QWORD) => [0x48, 0xC1, 0xC2, 0x03];
    /// rol rdx, cl
    rol_rdx_cl: shift(ShiftOp::Rol, reg(RDX), reg(RCX), QWORD) => [0x48, 0xD3, 0xC2];
    /// rol r10, 7
    #[ignore = "BUG: produit `rol rdx,0x7`"]
    rol_r10_7: shift(ShiftOp::Rol, reg(R10), imm(7), QWORD) => [0x49, 0xC1, 0xC2, 0x07];
    /// rol dword ptr [rbx], 2
    rol_dword_rbx_2: shift(ShiftOp::Rol, mem(at(RBX)), imm(2), DWORD) => [0xC1, 0x03, 0x02];
    /// rol qword ptr [rbx+8], cl
    rol_qword_rbx_plus_8_cl: shift(ShiftOp::Rol, mem(at(RBX).disp(8)), reg(RCX), QWORD) => [0x48, 0xD3, 0x43, 0x08];
    /// ror dl, 1
    ror_dl_1: shift(ShiftOp::Ror, reg(RDX), imm(1), BYTE) => [0xD0, 0xCA];
    /// ror dl, 3
    ror_dl_3: shift(ShiftOp::Ror, reg(RDX), imm(3), BYTE) => [0xC0, 0xCA, 0x03];
    /// ror dl, cl
    ror_dl_cl: shift(ShiftOp::Ror, reg(RDX), reg(RCX), BYTE) => [0xD2, 0xCA];
    /// ror dx, 1
    ror_dx_1: shift(ShiftOp::Ror, reg(RDX), imm(1), WORD) => [0x66, 0xD1, 0xCA];
    /// ror dx, 3
    ror_dx_3: shift(ShiftOp::Ror, reg(RDX), imm(3), WORD) => [0x66, 0xC1, 0xCA, 0x03];
    /// ror dx, cl
    ror_dx_cl: shift(ShiftOp::Ror, reg(RDX), reg(RCX), WORD) => [0x66, 0xD3, 0xCA];
    /// ror edx, 1
    ror_edx_1: shift(ShiftOp::Ror, reg(RDX), imm(1), DWORD) => [0xD1, 0xCA];
    /// ror edx, 3
    ror_edx_3: shift(ShiftOp::Ror, reg(RDX), imm(3), DWORD) => [0xC1, 0xCA, 0x03];
    /// ror edx, cl
    ror_edx_cl: shift(ShiftOp::Ror, reg(RDX), reg(RCX), DWORD) => [0xD3, 0xCA];
    /// ror rdx, 1
    ror_rdx_1: shift(ShiftOp::Ror, reg(RDX), imm(1), QWORD) => [0x48, 0xD1, 0xCA];
    /// ror rdx, 3
    ror_rdx_3: shift(ShiftOp::Ror, reg(RDX), imm(3), QWORD) => [0x48, 0xC1, 0xCA, 0x03];
    /// ror rdx, cl
    ror_rdx_cl: shift(ShiftOp::Ror, reg(RDX), reg(RCX), QWORD) => [0x48, 0xD3, 0xCA];
    /// ror r10, 7
    #[ignore = "BUG: produit `ror rdx,0x7`"]
    ror_r10_7: shift(ShiftOp::Ror, reg(R10), imm(7), QWORD) => [0x49, 0xC1, 0xCA, 0x07];
    /// ror dword ptr [rbx], 2
    ror_dword_rbx_2: shift(ShiftOp::Ror, mem(at(RBX)), imm(2), DWORD) => [0xC1, 0x0B, 0x02];
    /// ror qword ptr [rbx+8], cl
    ror_qword_rbx_plus_8_cl: shift(ShiftOp::Ror, mem(at(RBX).disp(8)), reg(RCX), QWORD) => [0x48, 0xD3, 0x4B, 0x08];
}
