//! Tests de cvtsi2sd : octets de référence produits par GNU as.

use super::*;
use crate::elf::instructions::encode_conversion::encode_conversion;

fn encode(dst: Register, src: Operand, size: Size) -> Vec<u8> {
    encode_conversion(ConvOp::Cvtsi2sd, &dst, &src, size).data
}

// ==========================================
// Source registre
// ==========================================

#[test]
fn reg64() {
    assert_eq!(encode(XMM0, Operand::Reg(RAX), Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn reg32_has_no_rex() {
    assert_eq!(encode(XMM0, Operand::Reg(RAX), Size::U32), [0xF2, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn extended_xmm_sets_rex_r() {
    assert_eq!(encode(XMM15, Operand::Reg(RDI), Size::U64), [0xF2, 0x4C, 0x0F, 0x2A, 0xFF]);
}

#[test]
fn extended_gpr_sets_rex_b() {
    assert_eq!(encode(XMM3, Operand::Reg(R8), Size::U64), [0xF2, 0x49, 0x0F, 0x2A, 0xD8]);
    assert_eq!(encode(XMM3, Operand::Reg(R8), Size::U32), [0xF2, 0x41, 0x0F, 0x2A, 0xD8]);
}

#[test]
fn extended_both() {
    assert_eq!(encode(XMM8, Operand::Reg(R15), Size::U64), [0xF2, 0x4D, 0x0F, 0x2A, 0xC7]);
}

#[test]
fn prefix_comes_before_rex() {
    let bytes = encode(XMM9, Operand::Reg(R12), Size::U64);
    assert_eq!(bytes[0], 0xF2);
    assert_eq!(bytes[1] & 0xF0, 0x40);
}

// ==========================================
// Source mémoire
// ==========================================

#[test]
fn mem64() {
    assert_eq!(encode(XMM1, mem(at(RBX)), Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0x0B]);
}

#[test]
fn mem32() {
    assert_eq!(encode(XMM1, mem(at(RBX)), Size::U32), [0xF2, 0x0F, 0x2A, 0x0B]);
}

#[test]
fn mem_with_disp8() {
    let src = Operand::MemoryAddress(MemAddress::new().base(RBP).disp(8));
    assert_eq!(encode(XMM1, src, Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0x4D, 0x08]);
}

#[test]
fn mem_rsp_base_needs_sib() {
    assert_eq!(encode(XMM2, mem(at(RSP)), Size::U64), [0xF2, 0x48, 0x0F, 0x2A, 0x14, 0x24]);
}

#[test]
fn mem_extended_xmm() {
    assert_eq!(encode(XMM9, mem(at(RAX)), Size::U64), [0xF2, 0x4C, 0x0F, 0x2A, 0x08]);
}

#[test]
fn mem_extended_base() {
    assert_eq!(encode(XMM1, mem(at(R12)), Size::U64), [0xF2, 0x49, 0x0F, 0x2A, 0x0C, 0x24]);
}

#[test]
fn rip_relative_symbol_emits_relocation() {
    let src = Operand::MemoryAddress(MemAddress::symbol("my_float"));
    let info = encode_conversion(ConvOp::Cvtsi2sd, &XMM2, &src, Size::U64);

    assert_eq!(info.data, [0xF2, 0x48, 0x0F, 0x2A, 0x15, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(info.relocations.len(), 1);
    assert_eq!(info.relocations[0].sym, "my_float");
    assert_eq!(info.relocations[0].offset, 5);
}

// ==========================================
// Dispatch via Instruction::encode
// ==========================================

#[test]
fn instruction_uses_default_size() {
    let ins = Instruction::Convert { op: ConvOp::Cvtsi2sd, dst: XMM0, src: Operand::Reg(RAX), size: None };
    assert_eq!(ins.encode(Size::U64).data, [0xF2, 0x48, 0x0F, 0x2A, 0xC0]);
    assert_eq!(ins.encode(Size::U32).data, [0xF2, 0x0F, 0x2A, 0xC0]);
}

#[test]
fn instruction_explicit_size_wins() {
    let ins = Instruction::Convert { op: ConvOp::Cvtsi2sd, dst: XMM0, src: Operand::Reg(RAX), size: Some(Size::U32) };
    assert_eq!(ins.encode(Size::U64).data, [0xF2, 0x0F, 0x2A, 0xC0]);
}

// ==========================================
// Opérandes invalides
// ==========================================

#[test]
#[should_panic(expected = "destination must be an XMM register")]
fn gpr_destination_panics() {
    encode(RAX, Operand::Reg(RBX), Size::U64);
}

#[test]
#[should_panic(expected = "source must be a GPR or a memory address")]
fn xmm_source_panics() {
    encode(XMM0, Operand::Reg(XMM1), Size::U64);
}

#[test]
#[should_panic(expected = "source must be a GPR or a memory address")]
fn immediate_source_panics() {
    encode(XMM0, Operand::Imm(42), Size::U64);
}

#[test]
#[should_panic(expected = "integer source must be 32 or 64 bits")]
fn size_u16_panics() {
    encode(XMM0, Operand::Reg(RAX), Size::U16);
}

#[test]
#[should_panic(expected = "integer source must be 32 or 64 bits")]
fn size_u8_panics() {
    encode(XMM0, Operand::Reg(RAX), Size::U8);
}
