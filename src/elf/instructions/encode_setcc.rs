use super::{
    enums::*,
    modrm::*,
    register::{Gpr, Register, RegisterClass},
    struct_encode_information::*,
    utils::emit_rex,
};

pub(super) fn encode_setcc(cc: ConditionCode, dest: &Operand) -> EncodeInformation {
    let opcode = match cc {
        ConditionCode::O => 0x90,
        ConditionCode::NO => 0x91,
        ConditionCode::B => 0x92,
        ConditionCode::AE => 0x93,
        ConditionCode::E => 0x94,
        ConditionCode::NE => 0x95,
        ConditionCode::BE => 0x96,
        ConditionCode::A => 0x97,
        ConditionCode::S => 0x98,
        ConditionCode::NS => 0x99,
        ConditionCode::P => 0x9A,
        ConditionCode::NP => 0x9B,
        ConditionCode::L => 0x9C,
        ConditionCode::GE => 0x9D,
        ConditionCode::LE => 0x9E,
        ConditionCode::G => 0x9F,
    };

    let rm: Option<Register> = match dest {
        Operand::Reg(r) => Some(*r),
        Operand::MemoryAddress(_) => None,
        _ => unimplemented!("SETcc destination must be reg or memory"),
    };

    let mut v = EncodeInformation::new();

    // SETcc always writes 8-bit
    emit_rex(&mut v, Size::U8, None, rm);

    v.push(0x0F);
    v.push(opcode);

    // /0 extension
    let reg_field = Operand::Reg(Register {
        class: RegisterClass::Gpr,
        index: Gpr::A as u8,
    });

    let modrm = mod_rm_encode(dest, &reg_field);

    v.append(modrm);

    v
}
