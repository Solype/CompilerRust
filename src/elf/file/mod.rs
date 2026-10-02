pub mod file;
pub mod strtab;
pub mod section;

pub use file::*;
pub use symbols::natural_alignment;

mod packing;
mod encode;
mod symbols;

#[cfg(test)]
mod symbol_tests;
