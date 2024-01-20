use super::{
    token::Token,
    lexer::Lexer
};

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7
}

impl From<Token<'_>> for Precedence {
    fn from(token: Token) -> Self {
        match token {
            Token::Equal => Precedence::Equals,
            Token::NotEqual => Precedence::Equals,
            Token::LessThan => Precedence::LessGreater,
            Token::GreaterThan => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Asterisk => Precedence::Product,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
    lexer: Lexer<'a>,
    pub current_token: Token<'a>,
    pub(crate) peek_token: Token<'a>,
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
    
    pub fn assert_peek_is(&mut self, token: Token) -> anyhow::Result<bool> {
        if self.peek_is(token) {
            self.next_token();
            Ok(true)
        } else {
            Err(self.unexpected_token_error(token, self.peek_token))
        }
    }
    
    pub fn unexpected_token_error(&mut self, expected: Token, got: Token) -> anyhow::Error {
        let err = format!("expected {:?}, got {:?}",
                          expected.to_string(),// Token::lookup_literal(&expected),
                          got.to_string());//Token::lookup_literal(&got));
        self.push_error(err);
        anyhow::Error::msg("assert_error")
    }
    
    pub fn unexpected_prefix_error(&mut self, token: Token) -> anyhow::Error {
        let err = format!("no prefix parse function for {:?}", token.to_string());
        self.push_error(err);
        anyhow::Error::msg("assert_error")
    }
    
    // pub fn unexpected_infix_error(&mut self, token: Token) -> anyhow::Error {
    //     let err = format!("no infix parse function for {:?}", token.to_string());
    //     self.push_error(err);
    //     anyhow::Error::msg("assert_error")
    // }
    
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