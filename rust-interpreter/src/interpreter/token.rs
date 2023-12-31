#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Illegal,
    Eof,
    // Identifiers + literals
    Identifier(&'a str),
    Int(&'a str),
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

impl Token<'_> {
    pub fn lookup_identifier(ident: &str) -> Token {
        match ident {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifier(ident),
        }
    }
    
    pub fn lookup_token(token: Token) -> &str {
        match token {
            Token::Illegal => "illegal",
            Token::Eof => "eof",
            Token::Identifier(_) => "identifier",
            Token::Int(_) => "int",
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Bang => "!",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Lt => "<",
            Token::Gt => ">",
            Token::Eq => "==",
            Token::NotEq => "!=",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LCurly => "{",
            Token::RCurly => "}",
            Token::Function => "fn",
            Token::Let => "let",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
        }
    }
}
