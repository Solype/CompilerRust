use super::{
    AddressingMode,
    MemDisplacement,
    struct_encode_information::*,
    Scale,
    MemAddress,
    enums::*,
    register::{Register, RegisterClass, Gpr},
};

///////////////////////////////////////////////////////////////////

fn encode_modrm_field_reg(reg_op: &Operand) -> u8 {
    match reg_op {
        Operand::Reg(r) => r.low3(),
        Operand::NoOperand => 0, // when the instruction uses /digit instead of reg
        _ => panic!("ModRM register field must be a register or noop!"),
    }
}

fn encode_sib(scale: Scale, index: Option<Register>, base: Option<Register>) -> u8 {
    let scale_bits = scale as u8;

    let index_bits = match index {
        Some(r)
            if r.class == RegisterClass::Gpr
            && r.index != Gpr::Sp as u8 =>
        {
            r.low3()
        }

        _ => 0b100, // no index
    };

    let base_bits = match base {
        Some(r) => r.low3(),
        None => 0b101, // disp32
    };

    (scale_bits << 6) | (index_bits << 3) | base_bits
}

///////////////////////////////////////////////////////////////////

fn get_disp(
    disp: &MemDisplacement,
    data_len: usize,
    relocs: &mut Vec<Relocation>,
) -> i32 {
    match disp {
        MemDisplacement::Imm(val) => *val,

        MemDisplacement::Sym(sym) => {
            relocs.push(Relocation {
                sym: sym.clone(),
                offset: data_len,
                size: 4,
                kind: RelocKind::AbsoluteSigned,
                addend: 0,
            });

            0
        }
    }
}

fn encode_rip_relative(
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    data[0] = (MEMNODISP << 6) | 0b101;

    let offset = data.len();

    match disp {
        MemDisplacement::Sym(sym) => {
            data.extend(&0u32.to_le_bytes());

            relocs.push(Relocation {
                sym: sym.clone(),
                offset,
                size: 4,
                kind: RelocKind::Relative,
                addend: -4,
            });
        }

        MemDisplacement::Imm(val) => {
            data.extend(&(*val as u32).to_le_bytes());
        }
    }
}

fn encode_absolute(
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    // In 64 bits, mod = 00 / rm = 101 means [rip + disp32]:
    // an absolute address goes through a SIB with no base and no index (0x25)
    data[0] = (MEMNODISP << 6) | 0b100;
    data.push(encode_sib(Scale::One, None, None));

    let offset = data.len();

    match disp {
        MemDisplacement::Sym(sym) => {
            data.extend(&0u32.to_le_bytes());

            relocs.push(Relocation {
                sym: sym.clone(),
                offset,
                size: 4,
                kind: RelocKind::AbsoluteSigned,
                addend: 0,
            });
        }

        MemDisplacement::Imm(val) => {
            data.extend(&(*val as u32).to_le_bytes());
        }
    }
}

fn encode_base(base: Register, data: &mut Vec<u8>) {
    assert!(base.class == RegisterClass::Gpr);

    // ModRM only sees the low 3 bits: r12 behaves like rsp (SIB required)
    // and r13 like rbp (no mode without displacement)
    if base.low3() == Gpr::Sp as u8 {
        data[0] = (MEMNODISP << 6) | 0b100;
        data.push(encode_sib(Scale::One, None, Some(base)));

    } else if base.low3() == Gpr::Bp as u8 {
        data[0] = (MEMDISP8 << 6) | base.low3();
        data.push(0);

    } else {
        data[0] = (MEMNODISP << 6) | base.low3();
    }
}

fn encode_base_disp(
    base: Register,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    assert!(base.class == RegisterClass::Gpr);

    // The SIB goes before get_disp: the relocation points at the disp, after it
    let rm = if base.low3() == Gpr::Sp as u8 {
        data.push(encode_sib(Scale::One, None, Some(base)));
        0b100
    } else {
        base.low3()
    };

    let val = get_disp(disp, data.len(), relocs);

    let (mod_bits, disp_bytes) =
        encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    data[0] = (mod_bits << 6) | rm;

    data.extend(disp_bytes);
}

fn encode_index_disp(
    index: Register,
    scale: Scale,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    assert!(index.class == RegisterClass::Gpr);
    assert!(index.index != Gpr::Sp as u8);

    // Without a base, the SIB requires base = 101 with mod = 00 and a disp32
    // always present (mod = 01 or 10 would mean [rbp + index*scale + disp])
    data[0] = (MEMNODISP << 6) | 0b100;

    data.push(encode_sib(scale, Some(index), None));

    let val = get_disp(disp, data.len(), relocs);

    data.extend(val.to_le_bytes());
}

fn encode_base_index_scale(
    base: Register,
    index: Register,
    scale: Scale,
    data: &mut Vec<u8>,
) {
    assert!(base.class == RegisterClass::Gpr);
    assert!(index.class == RegisterClass::Gpr);

    assert!(index.index != Gpr::Sp as u8);

    // With a SIB, base = 101 and mod = 00 means "no base + disp32":
    // rbp / r13 as base therefore need a zero disp8
    if base.low3() == Gpr::Bp as u8 {
        data[0] = (MEMDISP8 << 6) | 0b100;
        data.push(encode_sib(scale, Some(index), Some(base)));
        data.push(0);

    } else {
        data[0] = (MEMNODISP << 6) | 0b100;
        data.push(encode_sib(scale, Some(index), Some(base)));
    }
}

fn encode_base_index_scale_disp(
    base: Register,
    index: Register,
    scale: Scale,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    assert!(base.class == RegisterClass::Gpr);
    assert!(index.class == RegisterClass::Gpr);

    assert!(index.index != Gpr::Sp as u8);

    // The SIB goes before get_disp: the relocation points at the disp, after it
    data.push(encode_sib(scale, Some(index), Some(base)));

    let val = get_disp(disp, data.len(), relocs);

    let (mod_bits, disp_bytes) =
        encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    data[0] = (mod_bits << 6) | 0b100;

    data.extend(disp_bytes);
}

///////////////////////////////////////////////////////////////////

fn encode_disp(disp: i32, is_placeholder: bool) -> (u8, Vec<u8>) {
    if is_placeholder {
        return (0b10, disp.to_le_bytes().to_vec());
    }

    if disp == 0 {
        (MEMNODISP, vec![])

    } else if (-128..=127).contains(&disp) {
        (MEMDISP8, vec![disp as u8])

    } else {
        (MEMDISP32, disp.to_le_bytes().to_vec())
    }
}

fn encode_memory(
    mem: &MemAddress,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    match (mem.base, mem.index) {

        // [disp]
        (None, None) => {
            match mem.mode {
                AddressingMode::RipRelative
                | AddressingMode::Default => encode_rip_relative(&mem.disp, data, relocs),

                AddressingMode::Absolute => encode_absolute(&mem.disp, data, relocs)
            }
        }

        // [base]
        // [base + disp]
        (Some(base), None) => {
            match &mem.disp {
                MemDisplacement::Imm(0) =>encode_base(base, data),
                _ => encode_base_disp( base, &mem.disp, data, relocs, ),
            }
        }

        // [index * scale + disp]
        (None, Some(index)) => {
            encode_index_disp( index, mem.scale, &mem.disp, data, relocs, )
        }

        // [base + index * scale]
        // [base + index * scale + disp]
        (Some(base), Some(index)) => {
            match &mem.disp {
                MemDisplacement::Imm(0) => encode_base_index_scale( base, index, mem.scale, data, ),
                _ => encode_base_index_scale_disp( base, index, mem.scale, &mem.disp, data, relocs, ),
            }
        }
    }
}

///////////////////////////////////////////////////////////////////

fn encode_reg(data: &mut Vec<u8>, rm: Register)
{
    data[0] = (REG << 6) | rm.low3();
}

pub(super) fn mod_rm_encode(
    op1: &Operand,
    op2: &Operand,
) -> EncodeInformation {
    let mut data = vec![0u8];
    let mut relocs = vec![];

    let reg_field = encode_modrm_field_reg(op2);

    match op1 {
        Operand::Reg(rm) => encode_reg(&mut data, *rm),

        Operand::MemoryAddress(mem) => encode_memory(mem, &mut data, &mut relocs),

        _ => panic!("Invalid operand for ModRM"),
    }

    data[0] =
        (data[0] & 0b11000111)
        | ((reg_field & 0b111) << 3);

    EncodeInformation {
        data,
        relocations: relocs,
    }
}
