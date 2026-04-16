pub mod enums;
pub mod simple_encode;
pub mod modrm;
pub use enums::*;

mod encode_mov;
mod encode_add;
mod opcode_ctrl;