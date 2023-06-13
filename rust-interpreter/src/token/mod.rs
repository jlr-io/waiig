#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LCURLY,
    RCURLY,

    // Keywords
    FUNCTION,
    LET,
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::FUNCTION,
        "let" => Token::LET,
        _ => Token::IDENT(ident.to_string()),
    }
}
