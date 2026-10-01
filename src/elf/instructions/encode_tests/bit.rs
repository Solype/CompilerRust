//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// bt ax, 3
    bt_ax_3: bit(BitOp::Bt, reg(RAX), imm(3), WORD) => [0x66, 0x0F, 0xBA, 0xE0, 0x03];
    /// bt ax, bx
    bt_ax_bx: bit(BitOp::Bt, reg(RAX), reg(RBX), WORD) => [0x66, 0x0F, 0xA3, 0xD8];
    /// bt eax, 3
    bt_eax_3: bit(BitOp::Bt, reg(RAX), imm(3), DWORD) => [0x0F, 0xBA, 0xE0, 0x03];
    /// bt eax, ebx
    bt_eax_ebx: bit(BitOp::Bt, reg(RAX), reg(RBX), DWORD) => [0x0F, 0xA3, 0xD8];
    /// bt rax, 3
    bt_rax_3: bit(BitOp::Bt, reg(RAX), imm(3), QWORD) => [0x48, 0x0F, 0xBA, 0xE0, 0x03];
    /// bt rax, rbx
    bt_rax_rbx: bit(BitOp::Bt, reg(RAX), reg(RBX), QWORD) => [0x48, 0x0F, 0xA3, 0xD8];
    /// bt r11, r12
    bt_r11_r12: bit(BitOp::Bt, reg(R11), reg(R12), QWORD) => [0x4D, 0x0F, 0xA3, 0xE3];
    /// bt r11, 40
    bt_r11_40: bit(BitOp::Bt, reg(R11), imm(40), QWORD) => [0x49, 0x0F, 0xBA, 0xE3, 0x28];
    /// bt dword ptr [rbx+8], 2
    bt_dword_rbx_plus_8_2: bit(BitOp::Bt, mem(at(RBX).disp(8)), imm(2), DWORD) => [0x0F, 0xBA, 0x63, 0x08, 0x02];
    /// bt dword ptr [rbx+8], eax
    bt_dword_rbx_plus_8_eax: bit(BitOp::Bt, mem(at(RBX).disp(8)), reg(RAX), DWORD) => [0x0F, 0xA3, 0x43, 0x08];
    /// bts ax, 3
    bts_ax_3: bit(BitOp::Bts, reg(RAX), imm(3), WORD) => [0x66, 0x0F, 0xBA, 0xE8, 0x03];
    /// bts ax, bx
    bts_ax_bx: bit(BitOp::Bts, reg(RAX), reg(RBX), WORD) => [0x66, 0x0F, 0xAB, 0xD8];
    /// bts eax, 3
    bts_eax_3: bit(BitOp::Bts, reg(RAX), imm(3), DWORD) => [0x0F, 0xBA, 0xE8, 0x03];
    /// bts eax, ebx
    bts_eax_ebx: bit(BitOp::Bts, reg(RAX), reg(RBX), DWORD) => [0x0F, 0xAB, 0xD8];
    /// bts rax, 3
    bts_rax_3: bit(BitOp::Bts, reg(RAX), imm(3), QWORD) => [0x48, 0x0F, 0xBA, 0xE8, 0x03];
    /// bts rax, rbx
    bts_rax_rbx: bit(BitOp::Bts, reg(RAX), reg(RBX), QWORD) => [0x48, 0x0F, 0xAB, 0xD8];
    /// bts r11, r12
    bts_r11_r12: bit(BitOp::Bts, reg(R11), reg(R12), QWORD) => [0x4D, 0x0F, 0xAB, 0xE3];
    /// bts r11, 40
    bts_r11_40: bit(BitOp::Bts, reg(R11), imm(40), QWORD) => [0x49, 0x0F, 0xBA, 0xEB, 0x28];
    /// bts dword ptr [rbx+8], 2
    bts_dword_rbx_plus_8_2: bit(BitOp::Bts, mem(at(RBX).disp(8)), imm(2), DWORD) => [0x0F, 0xBA, 0x6B, 0x08, 0x02];
    /// bts dword ptr [rbx+8], eax
    bts_dword_rbx_plus_8_eax: bit(BitOp::Bts, mem(at(RBX).disp(8)), reg(RAX), DWORD) => [0x0F, 0xAB, 0x43, 0x08];
    /// btr ax, 3
    btr_ax_3: bit(BitOp::Btr, reg(RAX), imm(3), WORD) => [0x66, 0x0F, 0xBA, 0xF0, 0x03];
    /// btr ax, bx
    btr_ax_bx: bit(BitOp::Btr, reg(RAX), reg(RBX), WORD) => [0x66, 0x0F, 0xB3, 0xD8];
    /// btr eax, 3
    btr_eax_3: bit(BitOp::Btr, reg(RAX), imm(3), DWORD) => [0x0F, 0xBA, 0xF0, 0x03];
    /// btr eax, ebx
    btr_eax_ebx: bit(BitOp::Btr, reg(RAX), reg(RBX), DWORD) => [0x0F, 0xB3, 0xD8];
    /// btr rax, 3
    btr_rax_3: bit(BitOp::Btr, reg(RAX), imm(3), QWORD) => [0x48, 0x0F, 0xBA, 0xF0, 0x03];
    /// btr rax, rbx
    btr_rax_rbx: bit(BitOp::Btr, reg(RAX), reg(RBX), QWORD) => [0x48, 0x0F, 0xB3, 0xD8];
    /// btr r11, r12
    btr_r11_r12: bit(BitOp::Btr, reg(R11), reg(R12), QWORD) => [0x4D, 0x0F, 0xB3, 0xE3];
    /// btr r11, 40
    btr_r11_40: bit(BitOp::Btr, reg(R11), imm(40), QWORD) => [0x49, 0x0F, 0xBA, 0xF3, 0x28];
    /// btr dword ptr [rbx+8], 2
    btr_dword_rbx_plus_8_2: bit(BitOp::Btr, mem(at(RBX).disp(8)), imm(2), DWORD) => [0x0F, 0xBA, 0x73, 0x08, 0x02];
    /// btr dword ptr [rbx+8], eax
    btr_dword_rbx_plus_8_eax: bit(BitOp::Btr, mem(at(RBX).disp(8)), reg(RAX), DWORD) => [0x0F, 0xB3, 0x43, 0x08];
    /// btc ax, 3
    btc_ax_3: bit(BitOp::Btc, reg(RAX), imm(3), WORD) => [0x66, 0x0F, 0xBA, 0xF8, 0x03];
    /// btc ax, bx
    btc_ax_bx: bit(BitOp::Btc, reg(RAX), reg(RBX), WORD) => [0x66, 0x0F, 0xBB, 0xD8];
    /// btc eax, 3
    btc_eax_3: bit(BitOp::Btc, reg(RAX), imm(3), DWORD) => [0x0F, 0xBA, 0xF8, 0x03];
    /// btc eax, ebx
    btc_eax_ebx: bit(BitOp::Btc, reg(RAX), reg(RBX), DWORD) => [0x0F, 0xBB, 0xD8];
    /// btc rax, 3
    btc_rax_3: bit(BitOp::Btc, reg(RAX), imm(3), QWORD) => [0x48, 0x0F, 0xBA, 0xF8, 0x03];
    /// btc rax, rbx
    btc_rax_rbx: bit(BitOp::Btc, reg(RAX), reg(RBX), QWORD) => [0x48, 0x0F, 0xBB, 0xD8];
    /// btc r11, r12
    btc_r11_r12: bit(BitOp::Btc, reg(R11), reg(R12), QWORD) => [0x4D, 0x0F, 0xBB, 0xE3];
    /// btc r11, 40
    btc_r11_40: bit(BitOp::Btc, reg(R11), imm(40), QWORD) => [0x49, 0x0F, 0xBA, 0xFB, 0x28];
    /// btc dword ptr [rbx+8], 2
    btc_dword_rbx_plus_8_2: bit(BitOp::Btc, mem(at(RBX).disp(8)), imm(2), DWORD) => [0x0F, 0xBA, 0x7B, 0x08, 0x02];
    /// btc dword ptr [rbx+8], eax
    btc_dword_rbx_plus_8_eax: bit(BitOp::Btc, mem(at(RBX).disp(8)), reg(RAX), DWORD) => [0x0F, 0xBB, 0x43, 0x08];
    /// bt qword ptr [r9], 2
    bt_qword_r9_2: bit(BitOp::Bt, mem(at(R9)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x21, 0x02];
    /// bt qword ptr [rbx+r8*2], 2
    bt_qword_rbx_plus_r8_x_2_2: bit(BitOp::Bt, mem(at(RBX).index_scale(R8, Scale::Two)), imm(2), QWORD) => [0x4A, 0x0F, 0xBA, 0x24, 0x43, 0x02];
    /// bt qword ptr [r10+rax], 2
    bt_qword_r10_plus_rax_2: bit(BitOp::Bt, mem(at(R10).index(RAX)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x24, 0x02, 0x02];
    /// bt qword ptr [r9], r10
    bt_qword_r9_r10: bit(BitOp::Bt, mem(at(R9)), reg(R10), QWORD) => [0x4D, 0x0F, 0xA3, 0x11];
    /// bts qword ptr [r9], 2
    bts_qword_r9_2: bit(BitOp::Bts, mem(at(R9)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x29, 0x02];
    /// bts qword ptr [rbx+r8*2], 2
    bts_qword_rbx_plus_r8_x_2_2: bit(BitOp::Bts, mem(at(RBX).index_scale(R8, Scale::Two)), imm(2), QWORD) => [0x4A, 0x0F, 0xBA, 0x2C, 0x43, 0x02];
    /// bts qword ptr [r10+rax], 2
    bts_qword_r10_plus_rax_2: bit(BitOp::Bts, mem(at(R10).index(RAX)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x2C, 0x02, 0x02];
    /// bts qword ptr [r9], r10
    bts_qword_r9_r10: bit(BitOp::Bts, mem(at(R9)), reg(R10), QWORD) => [0x4D, 0x0F, 0xAB, 0x11];
    /// btr qword ptr [r9], 2
    btr_qword_r9_2: bit(BitOp::Btr, mem(at(R9)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x31, 0x02];
    /// btr qword ptr [rbx+r8*2], 2
    btr_qword_rbx_plus_r8_x_2_2: bit(BitOp::Btr, mem(at(RBX).index_scale(R8, Scale::Two)), imm(2), QWORD) => [0x4A, 0x0F, 0xBA, 0x34, 0x43, 0x02];
    /// btr qword ptr [r10+rax], 2
    btr_qword_r10_plus_rax_2: bit(BitOp::Btr, mem(at(R10).index(RAX)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x34, 0x02, 0x02];
    /// btr qword ptr [r9], r10
    btr_qword_r9_r10: bit(BitOp::Btr, mem(at(R9)), reg(R10), QWORD) => [0x4D, 0x0F, 0xB3, 0x11];
    /// btc qword ptr [r9], 2
    btc_qword_r9_2: bit(BitOp::Btc, mem(at(R9)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x39, 0x02];
    /// btc qword ptr [rbx+r8*2], 2
    btc_qword_rbx_plus_r8_x_2_2: bit(BitOp::Btc, mem(at(RBX).index_scale(R8, Scale::Two)), imm(2), QWORD) => [0x4A, 0x0F, 0xBA, 0x3C, 0x43, 0x02];
    /// btc qword ptr [r10+rax], 2
    btc_qword_r10_plus_rax_2: bit(BitOp::Btc, mem(at(R10).index(RAX)), imm(2), QWORD) => [0x49, 0x0F, 0xBA, 0x3C, 0x02, 0x02];
    /// btc qword ptr [r9], r10
    btc_qword_r9_r10: bit(BitOp::Btc, mem(at(R9)), reg(R10), QWORD) => [0x4D, 0x0F, 0xBB, 0x11];
}
