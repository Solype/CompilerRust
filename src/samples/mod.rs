//! Test programs encoded into `output.elf`, until the front-end exists.
//! One function per instruction family, in a file of the same name;
//! each family becomes a global function of the same name in the binary.

pub(crate) mod helpers;
mod start;

pub use start::start;

use crate::elf::instructions::Instruction;

macro_rules! families {
    ($($name:ident),* $(,)?) => {
        $(mod $name;)*

        /// (symbol name, code) for each family
        pub fn families() -> Vec<(&'static str, Vec<Instruction>)> {
            return vec![$((stringify!($name), $name::$name())),*];
        }
    };
}

families!(
    jumps_and_calls,
    syscalls,
    stack_ops,
    mov,
    lea_addressing,
    alu,
    mul_div,
    unary_ops,
    no_operand,
    shift_rotate,
    bit_ops,
    bit_scan,
    conditional_move,
    conditional_set,
    string_ops,
    atomics,
    prefixes,
    sse_float,
    conversions,
);
