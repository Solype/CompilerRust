pub mod file;
pub mod section;
pub mod strtab;

pub use file::*;
pub use symbols::natural_alignment;

mod encode;
mod packing;
mod symbols;

#[cfg(test)]
mod symbol_tests;
