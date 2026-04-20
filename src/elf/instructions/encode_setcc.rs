use super::modrm::*;
use super::enums::*;


pub(super) fn encode_setcc(
    op: SetCC,
    dest: &Operand,
) -> EncodeInformation {
    let opcode = match op {
        SetCC::Sete  => 0x94,
        SetCC::Setne => 0x95,
        SetCC::Setb  => 0x92,
        SetCC::Seta  => 0x97,
        SetCC::Setl  => 0x9C,
        SetCC::Setge => 0x9D,
        SetCC::Setle => 0x9E,
        SetCC::Setg  => 0x9F,
    };

    let rm_u8 = match dest {
        Operand::Reg(r) => *r as u8,
        Operand::MemoryAddress(_) => 0,
        _ => unimplemented!("SETcc destination must be reg or memory"),
    };

    let mut v = Vec::new();

    // SETcc always writes byte
    emit_rex(&mut v, Size::U8, None, Some(rm_u8));

    v.push(0x0F);
    v.push(opcode);

    let base = v.len();

    // /0 ignored by opcode form, real dst goes in r/m
    let reg_field = Operand::Reg(Register::Eax);

    let modrm = mod_rm_encode(dest, &reg_field);

    v.extend(modrm.data);

    let relocations = modrm.relocations.into_iter().map(|mut r| {
            r.offset += base;
            r
        }).collect();

    EncodeInformation {
        data: v,
        relocations,
    }
}

