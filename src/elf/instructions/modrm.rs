use super::enums::*;

///////////////////////////////////////////////////////////////////

fn encode_modrm_field_reg(reg_op: &Operand) -> u8 {
    match reg_op {
        Operand::Reg(r) => *r as u8,
        Operand::NoOperand => 0, // si instruction utilise /digit au lieu de reg
        _ => panic!("ModRM register field must be a register or noop!"),
    }
}

fn encode_sib(scale: Scale, index: Option<Register>, base: Option<Register>) -> u8 {
    let scale_bits = match scale {
        Scale::One => 0b00,
        Scale::Two => 0b01,
        Scale::Four => 0b10,
        Scale::Eight => 0b11,
    };

    let index_bits = match index {
        Some(r) if r != Register::Esp => r as u8,
        _ => 0b100, // no index
    };

    let base_bits = match base {
        Some(r) => r as u8,
        None => 0b101, // disp32
    };

    (scale_bits << 6) | (index_bits << 3) | base_bits
}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// 
///////////////////////////////////////////////////////////////////

fn get_disp(
    disp: &MemDisplacement,
    data_len: usize,
    relocs: &mut Vec<Relocation>
) -> i32 {
    match disp {
        MemDisplacement::Imm(val) => *val,
        MemDisplacement::Sym(sym) => {
            relocs.push(Relocation {
                sym: sym.clone(),
                offset: data_len,
                size: 4,
                ..Default::default()
            });
            0
        }
    }
}

fn encode_direct(disp: &MemDisplacement, data: &mut Vec<u8>, relocs: &mut Vec<Relocation>) {
    data[0] = (MEMNODISP << 6) | 0b101;

    let val = get_disp(disp, data.len(), relocs);
    data.extend(&(val as u32).to_le_bytes());
}

fn encode_base(base: Register, data: &mut Vec<u8>) {
    if base == Register::Esp {
        data[0] = (MEMNODISP << 6) | 0b100;
        data.push(encode_sib(Scale::One, None, Some(base)));
    } else if base == Register::Ebp {
        data[0] = (MEMDISP8 << 6) | (base as u8);
        data.push(0);
    } else {
        data[0] = (MEMNODISP << 6) | (base as u8);
    }
}

fn encode_base_disp(
    base: Register,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    let val = get_disp(disp, data.len(), relocs);
    let (mod_bits, disp_bytes) = encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    if base == Register::Esp {
        data[0] = (mod_bits << 6) | 0b100;
        data.push(encode_sib(Scale::One, None, Some(base)));
    } else {
        data[0] = (mod_bits << 6) | (base as u8);
    }

    data.extend(disp_bytes);
}

fn encode_base_index(base: Register, index: Register, data: &mut Vec<u8>) {
    data[0] = (MEMNODISP << 6) | 0b100;
    data.push(encode_sib(Scale::One, Some(index), Some(base)));
}

fn encode_index_disp(
    index: Register,
    scale: Scale,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    assert!(index != Register::Esp); // interdit

    let val = get_disp(disp, data.len(), relocs);
    let (mod_bits, disp_bytes) = encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    data[0] = (mod_bits << 6) | 0b100; // rm = 100 → SIB

    // base = none → 101
    data.push(encode_sib(scale, Some(index), None));

    data.extend(disp_bytes);
}

fn encode_base_index_scale(
    base: Register,
    index: Register,
    scale: Scale,
    data: &mut Vec<u8>,
) {
    assert!(index != Register::Esp);

    data[0] = (MEMNODISP << 6) | 0b100;

    data.push(encode_sib(scale, Some(index), Some(base)));
}

fn encode_base_index_scale_disp(
    base: Register,
    index: Register,
    scale: Scale,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    assert!(index != Register::Esp);

    let val = get_disp(disp, data.len(), relocs);
    let (mod_bits, disp_bytes) = encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    data[0] = (mod_bits << 6) | 0b100;

    data.push(encode_sib(scale, Some(index), Some(base)));

    data.extend(disp_bytes);
}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// 
///////////////////////////////////////////////////////////////////

fn encode_disp(disp: i32, is_placeholder : bool) -> (u8, Vec<u8>) {
    if is_placeholder {
        return (0b10, disp.to_le_bytes().to_vec());
    }
    if disp == 0 {
        (0b00, vec![])
    } else if (-128..=127).contains(&disp) {
        (0b01, vec![disp as u8])
    } else {
        (0b10, disp.to_le_bytes().to_vec())
    }
}

fn encode_reg(data: &mut Vec<u8>, rm: Register) {
    data[0] = (REG << 6) | (rm as u8);
}

fn encode_symbol(sym: &String, data: &mut Vec<u8>, relocs: &mut Vec<Relocation>) {
    data[0] = (MEMNODISP << 6) | 0b101;

    relocs.push(Relocation {
        sym: sym.clone(),
        offset: 1,
        size: 4,
        ..Default::default()
    });

    data.extend(&0u32.to_le_bytes());
}


fn encode_memory(mem: &MemAddress, data: &mut Vec<u8>, relocs: &mut Vec<Relocation>) {
    match mem {
        MemAddress::Direct { disp } => encode_direct(disp, data, relocs),

        MemAddress::Base { base } => encode_base(*base, data),

        MemAddress::BaseDisp { base, disp } =>
            encode_base_disp(*base, disp, data, relocs),

        MemAddress::IndexScaleDisp { index, scale, disp } =>
            encode_index_disp(*index, *scale, disp, data, relocs),

        MemAddress::BaseIndex { base, index } =>
            encode_base_index(*base, *index, data),

        MemAddress::BaseIndexScale { base, index, scale } =>
            encode_base_index_scale(*base, *index, *scale, data),

        MemAddress::BaseIndexScaleDisp { base, index, scale, disp } =>
            encode_base_index_scale_disp(*base, *index, *scale, disp, data, relocs),
    }
}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// 
///////////////////////////////////////////////////////////////////

pub(super) fn mod_rm_encode(op1: &Operand, op2: &Operand) -> EncodeInformation {
    let mut data = vec![0u8];
    let mut relocs = vec![];

    let reg_field = encode_modrm_field_reg(op2);

    match op1 {
        Operand::Reg(rm) => encode_reg(&mut data, *rm),

        Operand::MemoryAddress(mem) => {
            encode_memory(mem, &mut data, &mut relocs);
        }

        Operand::Sym(sym) => {
            encode_symbol(sym, &mut data, &mut relocs);
        }

        _ => panic!("Invalid operand for ModRM"),
    }

    // inject reg field
    data[0] = (data[0] & 0b11000111) | ((reg_field & 0b111) << 3);

    EncodeInformation { data, relocations: relocs }
}