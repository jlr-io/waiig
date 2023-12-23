#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String),
    Int(String),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LCurly,
    RCurly,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    }
}
