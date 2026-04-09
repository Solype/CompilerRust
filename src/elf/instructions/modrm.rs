use super::enums::*;

fn encode_modrm_field_reg(reg_op: &Operand) -> u8 {
    match reg_op {
        Operand::Reg(r) | Operand::RegMemory(r) => *r as u8,
        Operand::NoOperand => 0, // si instruction utilise /digit au lieu de reg
        _ => panic!("ModRM reg field must be a register or noop!"),
    }
}

pub(super) fn mod_rm_encode(op1: &Operand, op2: &Operand) -> EncodeInformation {
    let mut data = vec![0u8];

    // 1️⃣ Encode le champ reg
    let reg_field = encode_modrm_field_reg(op2);

    match op1 {
        Operand::Reg(rm) => {
            data[0] = ((ModRMModField::Reg as u8) << 6) | (*rm as u8);
        }

        Operand::RegMemory(rm) => {
            if *rm == Register::Ebp {
                data[0] = ((ModRMModField::MemDisp8 as u8) << 6) | (*rm as u8);
                data.push(0u8);
            } else {
                data[0] = ((ModRMModField::MemNoDisp as u8) << 6) | (*rm as u8);
            }
        }

        Operand::RegMemDisp(rm, disp) => {
            if (-128..=127).contains(disp) {
                data[0] = ((ModRMModField::MemDisp8 as u8) << 6) | (*rm as u8);
                data.push(*disp as u8);
            } else {
                data[0] = ((ModRMModField::MemDisp32 as u8) << 6) | (*rm as u8);
                data.extend(&(*disp as u32).to_le_bytes());
            }
        }

        _ => panic!("Error in the OP1 of mod rm encode function"),
    }

    data[0] = (data[0] & 0b11000111) | ((reg_field & 0b111) << 3);

    EncodeInformation {
        data,
        relocations: vec![],
    }
}

