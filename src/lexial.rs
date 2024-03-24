use crate::token::TokenType;


#[derive(Debug)]
struct Lexer {
    token: TokenType,
    line: u32,
    column: u32,
}

