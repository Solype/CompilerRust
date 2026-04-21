use super::modrm::*;
use super::enums::*;

pub(super) fn encode_setcc(
    op: SetCC,
    dest: &Operand,
) -> EncodeInformation {
    let opcode = match op {
        SetCC::Seto   => 0x90,
        SetCC::Setno  => 0x91,
        SetCC::Setb   => 0x92, // CF=1   (setc/setnae/setb)
        SetCC::Setae  => 0x93, // CF=0   (setnc/setae/setnb)
        SetCC::Sete   => 0x94, // ZF=1
        SetCC::Setne  => 0x95, // ZF=0
        SetCC::Setbe  => 0x96, // CF=1 || ZF=1
        SetCC::Seta   => 0x97, // CF=0 && ZF=0
        SetCC::Sets   => 0x98, // SF=1
        SetCC::Setns  => 0x99, // SF=0
        SetCC::Setp   => 0x9A, // PF=1
        SetCC::Setnp  => 0x9B, // PF=0
        SetCC::Setl   => 0x9C, // SF!=OF
        SetCC::Setge  => 0x9D, // SF==OF
        SetCC::Setle  => 0x9E, // ZF=1 || SF!=OF
        SetCC::Setg   => 0x9F, // ZF=0 && SF==OF
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
