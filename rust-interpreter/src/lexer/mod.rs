
#![allow(dead_code)]
use crate::token::{Token, lookup_ident};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position).unwrap());
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('=') => Token::ASSIGN,
            Some(';') => Token::SEMICOLON,
            Some('(') => Token::LPAREN,
            Some(')') => Token::RPAREN,
            Some(',') => Token::COMMA,
            Some('+') => Token::PLUS,
            Some('{') => Token::LCURLY,
            Some('}') => Token::RCURLY,
            Some(ch) if ch.is_alphabetic() => {
                let literal = self.read_identifier();
                return lookup_ident(&literal);
            }
            Some(ch) if ch.is_numeric() => {
                let literal = self.read_number();
                return Token::INT(literal);
            }
            Some(ch) if ch.is_whitespace() => {
                self.read_char();
                return self.next_token();
            }
            Some(ch) => Token::ILLEGAL,
            None => Token::EOF,
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let Some(ch) = self.ch {
            if !ch.is_alphabetic() {
                break;
            }
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while let Some(ch) = self.ch {
            if !ch.is_numeric() {
                break;
            }
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
}

// tests
#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
            x + y;
            
            };
    
            let result = add(five, ten);
            "#;

        let mut lexer = Lexer::new(input.into());
        let tests = [
            Token::LET, Token::IDENT("five".into()), Token::ASSIGN, Token::INT("5".into()), Token::SEMICOLON,
            Token::LET, Token::IDENT("ten".into()), Token::ASSIGN, Token::INT("10".into()), Token::SEMICOLON,
            Token::LET, Token::IDENT("add".into()), Token::ASSIGN, Token::FUNCTION, Token::LPAREN, Token::IDENT("x".into()), Token::COMMA, Token::IDENT("y".into()), Token::RPAREN, Token::LCURLY,
            Token::IDENT("x".into()), Token::PLUS, Token::IDENT("y".into()), Token::SEMICOLON, 
            Token::RCURLY, Token::SEMICOLON,
            Token::LET, Token::IDENT("result".into()), Token::ASSIGN, Token::IDENT("add".into()), Token::LPAREN, Token::IDENT("five".into()), Token::COMMA, Token::IDENT("ten".into()), Token::RPAREN, Token::SEMICOLON,
            Token::EOF,
        ];

        for (i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(&tok, tt, "tests[{}] - tokentype wrong. expected={:#?}, got={:#?}", i, tt, tok);
        }
    }
}
