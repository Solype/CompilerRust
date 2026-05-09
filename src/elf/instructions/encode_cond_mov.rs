use crate::elf::instructions::{
    ConditionCode, EncodeInformation, Operand, Register, Size, encode_alu::emit_size_prefix, modrm::{emit_rex, mod_rm_encode}
};

fn get_opcode(cond_mov: ConditionCode,) -> u8
{
    match cond_mov {
        ConditionCode::E  => 0x44,
        ConditionCode::NE => 0x45,
        ConditionCode::L  => 0x4C,
        ConditionCode::GE => 0x4D,
        ConditionCode::LE => 0x4E,
        ConditionCode::G  => 0x4F,
        ConditionCode::A  => 0x47,
        ConditionCode::AE => 0x43,
        ConditionCode::B  => 0x42,
        ConditionCode::BE => 0x46,

        ConditionCode::O  => 0x40,
        ConditionCode::NO => 0x41,

        ConditionCode::S  => 0x48,
        ConditionCode::NS => 0x49,

        ConditionCode::P  => 0x4A,
        ConditionCode::NP => 0x4B,
    }
}

pub(super) fn encode_cmovcc(
    cc: &ConditionCode,
    dst: &Register,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let dst_reg = (*dst) as u8;

    let rm_u8 = match src {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    let mut v = EncodeInformation::new();
    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, Some(dst_reg), Some(rm_u8));
    v.push(0x0F);
    v.push(get_opcode(*cc));

    let modrm = mod_rm_encode(src, &Operand::Reg(*dst));

    v.append(modrm);
    v
}

