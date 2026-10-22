use crate::{converter::ir::structs::VarId, elf::instructions::Size};

pub fn var_size(var: &VarId) -> Option<Size> {
    return Some(Size::from(var.1));
}
