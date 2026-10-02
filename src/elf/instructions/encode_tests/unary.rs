//! Generated encoding tests: reference bytes produced by GNU as.
//! `#[ignore = "BUG: ..."]` marks a wrong encoding (see `cargo test -- --ignored`).

use super::*;

cases! {
    /// inc cl
    inc_cl: unary(UnaryOp::Inc, reg(RCX), BYTE) => [0xFE, 0xC1];
    /// inc byte ptr [rbx+8]
    inc_byte_rbx_plus_8: unary(UnaryOp::Inc, mem(at(RBX).disp(8)), BYTE) => [0xFE, 0x43, 0x08];
    /// inc cx
    inc_cx: unary(UnaryOp::Inc, reg(RCX), WORD) => [0x66, 0xFF, 0xC1];
    /// inc word ptr [rbx+8]
    inc_word_rbx_plus_8: unary(UnaryOp::Inc, mem(at(RBX).disp(8)), WORD) => [0x66, 0xFF, 0x43, 0x08];
    /// inc ecx
    inc_ecx: unary(UnaryOp::Inc, reg(RCX), DWORD) => [0xFF, 0xC1];
    /// inc dword ptr [rbx+8]
    inc_dword_rbx_plus_8: unary(UnaryOp::Inc, mem(at(RBX).disp(8)), DWORD) => [0xFF, 0x43, 0x08];
    /// inc rcx
    inc_rcx: unary(UnaryOp::Inc, reg(RCX), QWORD) => [0x48, 0xFF, 0xC1];
    /// inc qword ptr [rbx+8]
    inc_qword_rbx_plus_8: unary(UnaryOp::Inc, mem(at(RBX).disp(8)), QWORD) => [0x48, 0xFF, 0x43, 0x08];
    /// inc r13
    inc_r13: unary(UnaryOp::Inc, reg(R13), QWORD) => [0x49, 0xFF, 0xC5];
    /// inc r13b
    inc_r13b: unary(UnaryOp::Inc, reg(R13), BYTE) => [0x41, 0xFE, 0xC5];
    /// inc sil
    inc_sil: unary(UnaryOp::Inc, reg(RSI), BYTE) => [0x40, 0xFE, 0xC6];
    /// dec cl
    dec_cl: unary(UnaryOp::Dec, reg(RCX), BYTE) => [0xFE, 0xC9];
    /// dec byte ptr [rbx+8]
    dec_byte_rbx_plus_8: unary(UnaryOp::Dec, mem(at(RBX).disp(8)), BYTE) => [0xFE, 0x4B, 0x08];
    /// dec cx
    dec_cx: unary(UnaryOp::Dec, reg(RCX), WORD) => [0x66, 0xFF, 0xC9];
    /// dec word ptr [rbx+8]
    dec_word_rbx_plus_8: unary(UnaryOp::Dec, mem(at(RBX).disp(8)), WORD) => [0x66, 0xFF, 0x4B, 0x08];
    /// dec ecx
    dec_ecx: unary(UnaryOp::Dec, reg(RCX), DWORD) => [0xFF, 0xC9];
    /// dec dword ptr [rbx+8]
    dec_dword_rbx_plus_8: unary(UnaryOp::Dec, mem(at(RBX).disp(8)), DWORD) => [0xFF, 0x4B, 0x08];
    /// dec rcx
    dec_rcx: unary(UnaryOp::Dec, reg(RCX), QWORD) => [0x48, 0xFF, 0xC9];
    /// dec qword ptr [rbx+8]
    dec_qword_rbx_plus_8: unary(UnaryOp::Dec, mem(at(RBX).disp(8)), QWORD) => [0x48, 0xFF, 0x4B, 0x08];
    /// dec r13
    dec_r13: unary(UnaryOp::Dec, reg(R13), QWORD) => [0x49, 0xFF, 0xCD];
    /// dec r13b
    dec_r13b: unary(UnaryOp::Dec, reg(R13), BYTE) => [0x41, 0xFE, 0xCD];
    /// dec sil
    dec_sil: unary(UnaryOp::Dec, reg(RSI), BYTE) => [0x40, 0xFE, 0xCE];
    /// neg cl
    neg_cl: unary(UnaryOp::Neg, reg(RCX), BYTE) => [0xF6, 0xD9];
    /// neg byte ptr [rbx+8]
    neg_byte_rbx_plus_8: unary(UnaryOp::Neg, mem(at(RBX).disp(8)), BYTE) => [0xF6, 0x5B, 0x08];
    /// neg cx
    neg_cx: unary(UnaryOp::Neg, reg(RCX), WORD) => [0x66, 0xF7, 0xD9];
    /// neg word ptr [rbx+8]
    neg_word_rbx_plus_8: unary(UnaryOp::Neg, mem(at(RBX).disp(8)), WORD) => [0x66, 0xF7, 0x5B, 0x08];
    /// neg ecx
    neg_ecx: unary(UnaryOp::Neg, reg(RCX), DWORD) => [0xF7, 0xD9];
    /// neg dword ptr [rbx+8]
    neg_dword_rbx_plus_8: unary(UnaryOp::Neg, mem(at(RBX).disp(8)), DWORD) => [0xF7, 0x5B, 0x08];
    /// neg rcx
    neg_rcx: unary(UnaryOp::Neg, reg(RCX), QWORD) => [0x48, 0xF7, 0xD9];
    /// neg qword ptr [rbx+8]
    neg_qword_rbx_plus_8: unary(UnaryOp::Neg, mem(at(RBX).disp(8)), QWORD) => [0x48, 0xF7, 0x5B, 0x08];
    /// neg r13
    neg_r13: unary(UnaryOp::Neg, reg(R13), QWORD) => [0x49, 0xF7, 0xDD];
    /// neg r13b
    neg_r13b: unary(UnaryOp::Neg, reg(R13), BYTE) => [0x41, 0xF6, 0xDD];
    /// neg sil
    neg_sil: unary(UnaryOp::Neg, reg(RSI), BYTE) => [0x40, 0xF6, 0xDE];
    /// not cl
    not_cl: unary(UnaryOp::Not, reg(RCX), BYTE) => [0xF6, 0xD1];
    /// not byte ptr [rbx+8]
    not_byte_rbx_plus_8: unary(UnaryOp::Not, mem(at(RBX).disp(8)), BYTE) => [0xF6, 0x53, 0x08];
    /// not cx
    not_cx: unary(UnaryOp::Not, reg(RCX), WORD) => [0x66, 0xF7, 0xD1];
    /// not word ptr [rbx+8]
    not_word_rbx_plus_8: unary(UnaryOp::Not, mem(at(RBX).disp(8)), WORD) => [0x66, 0xF7, 0x53, 0x08];
    /// not ecx
    not_ecx: unary(UnaryOp::Not, reg(RCX), DWORD) => [0xF7, 0xD1];
    /// not dword ptr [rbx+8]
    not_dword_rbx_plus_8: unary(UnaryOp::Not, mem(at(RBX).disp(8)), DWORD) => [0xF7, 0x53, 0x08];
    /// not rcx
    not_rcx: unary(UnaryOp::Not, reg(RCX), QWORD) => [0x48, 0xF7, 0xD1];
    /// not qword ptr [rbx+8]
    not_qword_rbx_plus_8: unary(UnaryOp::Not, mem(at(RBX).disp(8)), QWORD) => [0x48, 0xF7, 0x53, 0x08];
    /// not r13
    not_r13: unary(UnaryOp::Not, reg(R13), QWORD) => [0x49, 0xF7, 0xD5];
    /// not r13b
    not_r13b: unary(UnaryOp::Not, reg(R13), BYTE) => [0x41, 0xF6, 0xD5];
    /// not sil
    not_sil: unary(UnaryOp::Not, reg(RSI), BYTE) => [0x40, 0xF6, 0xD6];
    /// inc qword ptr [r9]
    inc_qword_r9: unary(UnaryOp::Inc, mem(at(R9)), QWORD) => [0x49, 0xFF, 0x01];
    /// inc qword ptr [rbx+r8*2]
    inc_qword_rbx_plus_r8_x_2: unary(UnaryOp::Inc, mem(at(RBX).index_scale(R8, Scale::Two)), QWORD) => [0x4A, 0xFF, 0x04, 0x43];
    /// inc qword ptr [r10+rax]
    inc_qword_r10_plus_rax: unary(UnaryOp::Inc, mem(at(R10).index(RAX)), QWORD) => [0x49, 0xFF, 0x04, 0x02];
    /// inc byte ptr [r9]
    inc_byte_r9: unary(UnaryOp::Inc, mem(at(R9)), BYTE) => [0x41, 0xFE, 0x01];
    /// dec qword ptr [r9]
    dec_qword_r9: unary(UnaryOp::Dec, mem(at(R9)), QWORD) => [0x49, 0xFF, 0x09];
    /// dec qword ptr [rbx+r8*2]
    dec_qword_rbx_plus_r8_x_2: unary(UnaryOp::Dec, mem(at(RBX).index_scale(R8, Scale::Two)), QWORD) => [0x4A, 0xFF, 0x0C, 0x43];
    /// dec qword ptr [r10+rax]
    dec_qword_r10_plus_rax: unary(UnaryOp::Dec, mem(at(R10).index(RAX)), QWORD) => [0x49, 0xFF, 0x0C, 0x02];
    /// dec byte ptr [r9]
    dec_byte_r9: unary(UnaryOp::Dec, mem(at(R9)), BYTE) => [0x41, 0xFE, 0x09];
    /// neg qword ptr [r9]
    neg_qword_r9: unary(UnaryOp::Neg, mem(at(R9)), QWORD) => [0x49, 0xF7, 0x19];
    /// neg qword ptr [rbx+r8*2]
    neg_qword_rbx_plus_r8_x_2: unary(UnaryOp::Neg, mem(at(RBX).index_scale(R8, Scale::Two)), QWORD) => [0x4A, 0xF7, 0x1C, 0x43];
    /// neg qword ptr [r10+rax]
    neg_qword_r10_plus_rax: unary(UnaryOp::Neg, mem(at(R10).index(RAX)), QWORD) => [0x49, 0xF7, 0x1C, 0x02];
    /// neg byte ptr [r9]
    neg_byte_r9: unary(UnaryOp::Neg, mem(at(R9)), BYTE) => [0x41, 0xF6, 0x19];
    /// not qword ptr [r9]
    not_qword_r9: unary(UnaryOp::Not, mem(at(R9)), QWORD) => [0x49, 0xF7, 0x11];
    /// not qword ptr [rbx+r8*2]
    not_qword_rbx_plus_r8_x_2: unary(UnaryOp::Not, mem(at(RBX).index_scale(R8, Scale::Two)), QWORD) => [0x4A, 0xF7, 0x14, 0x43];
    /// not qword ptr [r10+rax]
    not_qword_r10_plus_rax: unary(UnaryOp::Not, mem(at(R10).index(RAX)), QWORD) => [0x49, 0xF7, 0x14, 0x02];
    /// not byte ptr [r9]
    not_byte_r9: unary(UnaryOp::Not, mem(at(R9)), BYTE) => [0x41, 0xF6, 0x11];
}
