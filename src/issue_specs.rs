//! Specs of open issues that only the internals can show: see `scripts/missing.sh`.
//!
//! Each `mod iN` belongs to issue N. Its tests are `#[ignore]` so that `cargo test` stays green
//! until the issue is done: move a test next to the code it covers, without `#[ignore]`, in the
//! pull request that closes the issue.

use crate::codegen::generation::Context;
use crate::converter::ir::structs::{Operand as IrOp, VarId};
use crate::elf::instructions::{Instruction, Size};
use crate::parser::postparser::structs::Type;

/// Variables 0, 1 and 2 at `[rbp - 8]`, `[rbp - 16]` and `[rbp - 24]`
fn context(return_type: Option<Type>) -> Context<'static> {
    Context {
        return_type,
        offsets: vec![8, 16, 24],
        data_names: &[],
        float_names: &[],
        floats: &[],
        labels: vec![],
    }
}

const SHORT_INT: Type = Type::Int {
    short: true,
    unsigned: false,
};

fn var(id: usize, ty: Type) -> IrOp {
    IrOp::Var(VarId::new(id, ty))
}

/// True if an instruction reads memory on exactly `want`
fn reads_on(instrs: &[Instruction], want: Size) -> bool {
    instrs.iter().any(|instr| match instr {
        Instruction::Binary { size, .. } => *size == Some(want),
        Instruction::Extend { src_size, .. } => *src_size == Some(want),
        _ => false,
    })
}

mod i2 {
    use super::*;
    use crate::codegen::instructions::generate_cast;
    use crate::elf::instructions::ExtendOp;

    /// No program can declare this type before #20: `부호 없는 짧은 정수` to `정수`
    #[test]
    #[ignore = "issue #2"]
    fn four_byte_unsigned_is_zero_extended() {
        let unsigned = Type::Int {
            short: true,
            unsigned: true,
        };
        let long = Type::Int {
            short: false,
            unsigned: false,
        };
        let instrs = generate_cast(&VarId::new(0, long), &var(1, unsigned), &context(None));
        let signed = instrs.iter().any(|instr| {
            matches!(
                instr,
                Instruction::Extend {
                    op: ExtendOp::Movsxd,
                    ..
                }
            )
        });
        assert!(!signed, "movsxd extends the sign: {instrs:?}");
    }
}

mod i4 {
    use super::*;
    use crate::codegen::instructions::{generate_return, normal_param};

    #[test]
    // #[ignore = "issue #4"]
    fn return_of_short_int_reads_4_bytes() {
        let instrs = generate_return(&var(0, SHORT_INT), &context(Some(SHORT_INT)));
        assert!(reads_on(&instrs, Size::U32), "{instrs:?}");
    }

    #[test]
    // #[ignore = "issue #4"]
    fn return_of_bool_reads_1_byte() {
        let instrs = generate_return(&var(0, Type::Bool), &context(Some(Type::Bool)));
        assert!(reads_on(&instrs, Size::U8), "{instrs:?}");
    }

    #[test]
    // #[ignore = "issue #4"]
    fn return_of_short_float_reads_4_bytes() {
        let ty = Type::Float { short: true };
        let instrs = generate_return(&var(0, ty.clone()), &context(Some(ty)));
        assert!(reads_on(&instrs, Size::U32), "{instrs:?}");
    }

    #[test]
    // #[ignore = "issue #4"]
    fn argument_short_int_reads_4_bytes() {
        let instr = normal_param(&var(0, SHORT_INT), &mut 0, &context(None));
        assert!(
            reads_on(std::slice::from_ref(&instr), Size::U32),
            "{instr:?}"
        );
    }
}

mod i27 {
    use crate::elf::instructions::register::{RBX, XMM10};
    use crate::elf::instructions::{BinOp, Size};
    use crate::samples::helpers::*;

    /// `movsd [rbx], xmm10`: the memory is the destination, the register the source
    #[test]
    #[ignore = "issue #27"]
    fn store_float_has_its_register_in_src() {
        let got = bin(BinOp::StoreF, mem(at(RBX)), reg(XMM10), None).encode(Size::U64);
        assert_eq!(got.data, [0xF2, 0x44, 0x0F, 0x11, 0x13]);
    }

    /// The trap of the issue: the register in `dst` must not encode a store anymore
    #[test]
    #[ignore = "issue #27"]
    #[should_panic]
    fn store_float_refuses_its_register_in_dst() {
        bin(BinOp::StoreF, reg(XMM10), mem(at(RBX)), None).encode(Size::U64);
    }
}

mod i30 {
    use crate::elf::instructions::register::{R12, RAX, RBX};
    use crate::elf::instructions::{BinOp, CtrlOp, Instruction, Operand, Size, StackOp, SysOp};
    use crate::samples::helpers::*;

    fn bytes(instr: Instruction) -> Vec<u8> {
        instr.encode(Size::U64).data
    }

    #[test]
    #[ignore = "issue #30"]
    fn push_has_no_rex_w() {
        assert_eq!(bytes(stack(StackOp::Push, reg(RAX), None)), [0x50]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn pop_has_no_rex_w() {
        assert_eq!(bytes(stack(StackOp::Pop, reg(RAX), None)), [0x58]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn push_r12_keeps_only_rex_b() {
        assert_eq!(bytes(stack(StackOp::Push, reg(R12), None)), [0x41, 0x54]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn push_memory_has_no_rex_w() {
        let got = bytes(stack(StackOp::Push, mem(at(RBX).disp(8)), None));
        assert_eq!(got, [0xFF, 0x73, 0x08]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn add_eax_uses_the_accumulator_form() {
        let got = bytes(bin(BinOp::Add, reg(RAX), imm(0x1000), DWORD));
        assert_eq!(got, [0x05, 0x00, 0x10, 0x00, 0x00]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn add_rax_uses_the_accumulator_form() {
        let got = bytes(bin(BinOp::Add, reg(RAX), imm(0x1000), QWORD));
        assert_eq!(got, [0x48, 0x05, 0x00, 0x10, 0x00, 0x00]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn test_eax_uses_the_accumulator_form() {
        let got = bytes(bin(BinOp::Test, reg(RAX), imm(0x1000), DWORD));
        assert_eq!(got, [0xA9, 0x00, 0x10, 0x00, 0x00]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn int3_is_one_byte() {
        assert_eq!(bytes(sys(SysOp::Int(3))), [0xCC]);
    }

    #[test]
    #[ignore = "issue #30"]
    fn ret_with_an_immediate() {
        let got = bytes(Instruction::Ctrl {
            op: CtrlOp::Ret,
            target: Operand::Imm(8),
        });
        assert_eq!(got, [0xC2, 0x08, 0x00]);
    }
}
