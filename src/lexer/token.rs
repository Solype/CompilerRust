use super::lexer_error::LexError;
use super::span::Span;

pub struct Token {
    pub span: Span,
}

pub fn tokenizer(input: &str) -> Result<Vec<Token>, LexError> {
    let tokens: Vec<Token> = Vec::new();

    let _ = input;
    Ok(tokens)
}
