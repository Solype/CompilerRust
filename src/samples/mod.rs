//! Programmes de test encodés dans `output.elf`, en attendant le front-end.
//! Une fonction par famille d'instructions, dans un fichier du même nom ;
//! chaque famille devient une fonction globale du même nom dans le binaire.

pub(crate) mod helpers;
mod start;

pub use start::start;

use crate::elf::instructions::Instruction;

macro_rules! families {
    ($($name:ident),* $(,)?) => {
        $(mod $name;)*

        /// (nom du symbole, code) pour chaque famille
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
