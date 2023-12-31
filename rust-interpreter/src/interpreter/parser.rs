use crate::interpreter::{
    token::Token,
    lexer::Lexer,
};

pub trait Parse<'a> where Self: Sized {
    fn parse(parser: &mut Parser<'a>) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
    lexer: Lexer<'a>,
    pub current_token: Token<'a>,
    peek_token: Token<'a>,
    errors: Vec<String>
}

impl<'a> Parser<'a> {
    pub(crate) fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new()
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    pub fn current_is(&self, token: Token) -> bool {
        self.current_token == token
    }

    pub fn peek_is(&self, token: Token) -> bool {
        self.peek_token == token
    }
    
    pub fn assert_peek_is(&mut self, token: Token) -> bool {
        if self.peek_is(token) {
            self.next_token();
            true
        } else {
            self.assert_error(token, self.peek_token);
            false
        }
    }
    
    pub fn assert_error(&mut self, expected: Token, got: Token) {
        self.push_error(format!("expected {:?}, got {:?}", 
                                Token::lookup_token(expected), 
                                Token::lookup_token(got)));
    }
    
    pub fn push_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    pub fn check_errors(&self) {
        if self.errors.len() == 0 {
            return;
        }
        println!("parser has {} errors", self.errors.len());
        for error in &self.errors {
            println!("parser error: {}", error);
        }
        panic!("parser has {} errors", self.errors.len());
    }
}