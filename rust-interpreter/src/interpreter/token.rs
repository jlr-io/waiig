use std::fmt::Display;
use crate::interpreter::parser::Precedence;

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
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
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

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => write!(f, "illegal"),
            Token::Eof => write!(f, "eof"),
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Int(int) => write!(f, "{}", int),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
}

impl<'a> Token<'a> {
    pub fn lookup_token(ident: &str) -> Token {
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
    
    pub fn lookup_literal(token: &Token) -> String {
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
            Token::LessThan => "<",
            Token::GreaterThan => ">",
            Token::Equal => "==",
            Token::NotEqual => "!=",
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
        }.to_string()
    }
    
    pub fn is_prefix(&self) -> bool {
        match self {
            Token::Bang => true,
            Token::Minus => true,
            _ => false,
        }
    }
    
    pub fn is_infix(&self) -> bool {
        match self {
            Token::Plus => true,
            Token::Minus => true,
            Token::Bang => true,
            Token::Asterisk => true,
            Token::Slash => true,
            Token::Equal => true,
            Token::NotEqual => true,
            Token::LessThan => true,
            Token::GreaterThan => true,
            _ => false,
        }
    }
}
