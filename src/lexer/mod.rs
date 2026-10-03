pub mod lexer_error;
pub mod lexer_token;
pub mod span;

mod cursor;
mod scanner;
mod symbols;

#[cfg(test)]
mod lexer_tests;

pub use scanner::tokenize;
