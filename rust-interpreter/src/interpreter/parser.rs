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
    pub errors: Vec<String>
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

    pub fn current_token_is(&self, token: Token) -> bool {
        self.current_token == token
    }

    pub fn peek_token_is(&self, token: Token) -> bool {
        self.peek_token == token
    }

    pub fn assert_peek_is(&mut self, token: Token) -> bool {
        if self.peek_token_is(token) {
            true
        } else {
            self.errors.push(format!(
                "expected next token to be {:?}, got {:?}",
                token,
                self.peek_token
            ));
            false
        }
    }
}