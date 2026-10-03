pub mod lexer_error;
pub mod span;
pub mod token;

mod cursor;
mod scanner;
mod symbols;

#[cfg(test)]
mod lexer_tests;

pub use scanner::tokenize;
