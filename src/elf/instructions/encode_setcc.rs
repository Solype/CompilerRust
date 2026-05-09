use super::{
    modrm::*,
    enums::*,
    struct_encode_information::*,
};

pub(super) fn encode_setcc(
    cc: ConditionCode,
    dest: &Operand,
) -> EncodeInformation {
    let opcode = match cc {
        ConditionCode::O   => 0x90,
        ConditionCode::NO  => 0x91,
        ConditionCode::B   => 0x92, // CF=1   (setc/setnae/setb)
        ConditionCode::AE  => 0x93, // CF=0   (setnc/setae/setnb)
        ConditionCode::E   => 0x94, // ZF=1
        ConditionCode::NE  => 0x95, // ZF=0
        ConditionCode::BE  => 0x96, // CF=1 || ZF=1
        ConditionCode::A   => 0x97, // CF=0 && ZF=0
        ConditionCode::S   => 0x98, // SF=1
        ConditionCode::NS  => 0x99, // SF=0
        ConditionCode::P   => 0x9A, // PF=1
        ConditionCode::NP  => 0x9B, // PF=0
        ConditionCode::L   => 0x9C, // SF!=OF
        ConditionCode::GE  => 0x9D, // SF==OF
        ConditionCode::LE  => 0x9E, // ZF=1 || SF!=OF
        ConditionCode::G   => 0x9F, // ZF=0 && SF==OF
    };

    let rm_u8 = match dest {
        Operand::Reg(r) => *r as u8,
        Operand::MemoryAddress(_) => 0,
        _ => unimplemented!("SETcc destination must be reg or memory"),
    };

    let mut v = Vec::new();

    // SETcc writes 8-bit destination
    emit_rex(&mut v, Size::U8, None, Some(rm_u8));

    v.push(0x0F);
    v.push(opcode);

    let base = v.len();

    // reg field ignored for SETcc, use /0 convention
    let reg_field = Operand::Reg(Register::A);

    let modrm = mod_rm_encode(dest, &reg_field);

    v.extend(modrm.data);

    let relocations = modrm
        .relocations
        .into_iter()
        .map(|mut r| {
            r.offset += base;
            r
        })
        .collect();

    EncodeInformation {
        data: v,
        relocations,
    }
}
