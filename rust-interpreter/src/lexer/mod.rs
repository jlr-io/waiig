#![allow(dead_code)]
use crate::token::{lookup_ident, Token};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
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

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.read_position).unwrap())
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            None => Token::Eof,
            Some(ch) => match ch {
                '=' => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        Token::Eq
                    } else {
                        Token::Assign
                    }
                }
                '+' => Token::Plus,
                '-' => Token::Minus,
                '!' => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        Token::NotEq
                    } else {
                        Token::Bang
                    }
                }
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                '<' => Token::Lt,
                '>' => Token::Gt,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                ch if ch.is_alphabetic() => {
                    let literal = self.read_identifier();
                    return lookup_ident(&literal);
                }
                ch if ch.is_numeric() => {
                    let literal = self.read_number();
                    return Token::Int(literal);
                }
                _ => Token::Illegal,
            },
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

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if !ch.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::Eof {
            None
        } else {
            Some(token)
        }
    }
}

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

            !-/*5;
            5 < 10 > 5;
            

            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            
            10 == 10;
            10 != 9;
            "#;

        let mut lexer = Lexer::new(input.into());
        let tests = [
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RParen,
            Token::LCurly,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::RCurly,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::LParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::Gt,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::RParen,
            Token::LCurly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RCurly,
            Token::Else,
            Token::LCurly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RCurly,
            Token::Int("10".into()),
            Token::Eq,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("10".into()),
            Token::NotEq,
            Token::Int("9".into()),
            Token::Semicolon,
            Token::Eof,
        ];

        for (i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(
                &tok, tt,
                "tests[{}] - token type wrong. expected={:#?}, got={:#?}",
                i, tt, tok
            );
        }
    }
}
