use super::generation::Context;
use super::instructions::generate_zero;
use crate::converter::ir::structs::VarId;
use crate::elf::instructions::register::{RAX, RCX, RDI};
use crate::elf::instructions::{BinOp, Instruction, Operand as ElfOper, Prefix, Size, StringOp};
use crate::parser::postparser::structs::Type;

/// The whole array is cleared, from its lowest address: rcx holds its size in bytes, not its length
#[test]
fn zero_of_an_array_covers_all_its_bytes() {
    let ctx = Context {
        return_type: None,
        offsets: vec![24],
        data_names: &[],
        float_names: &[],
        floats: &[],
        labels: vec![],
    };
    let element = Type::Int {
        short: false,
        unsigned: false,
    };
    let dest = VarId::new(0, Type::Array(Box::new(element), 3));
    let instrs = generate_zero(&dest, &ctx);

    assert!(
        matches!(&instrs[0], Instruction::Lea { dst, src: ElfOper::MemoryAddress(_), .. } if *dst == RDI),
        "{instrs:?}"
    );
    assert!(
        matches!(&instrs[1], Instruction::Binary { op: BinOp::Mov, dst: ElfOper::Reg(r), src: ElfOper::Imm(24), .. } if *r == RCX),
        "{instrs:?}"
    );
    assert!(
        matches!(&instrs[2], Instruction::Binary { op: BinOp::Mov, dst: ElfOper::Reg(r), src: ElfOper::Imm(0), .. } if *r == RAX),
        "{instrs:?}"
    );
    let Instruction::Prefix { prefix, ins } = &instrs[3] else {
        panic!("{instrs:?}")
    };
    assert!(matches!(prefix[..], [Prefix::Rep]));
    assert!(matches!(
        **ins,
        Instruction::String {
            op: StringOp::Stos,
            size: Some(Size::U8)
        }
    ));
}
