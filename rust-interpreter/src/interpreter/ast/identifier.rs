use std::fmt::{Result};
use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Identifier<'a> {
    pub(crate) token: Token<'a>,
    pub(crate) value: &'a str,
}

impl<'a> From<&Identifier<'a>> for Token<'a> {
    fn from(identifier: &Identifier<'a>) -> Token<'a> {
        Token::Identifier(identifier.value)
    }
}

impl<'a> Identifier<'a> {
    pub fn new(token: Token<'a>, value: &'a str) -> Identifier<'a> {
        Identifier { token, value }
    }

    pub(crate) fn parse(parser: &mut Parser<'a>) -> anyhow::Result<Identifier<'a>> {
        match parser.current_token {
            Token::Identifier(identifier) => Ok(Identifier::new(parser.current_token, identifier)),
            _ => Err(parser.unexpected_token_error(Token::Identifier(""), parser.current_token))
        }
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod identifier_tests {
    use crate::interpreter::{
        parser::Parser,
        lexer::Lexer
    };
    use super::*;

    #[test]
    fn test_identifier() {
        let input = Lexer::new("foobar");
        let mut parser = Parser::new(input);
        let identifier = Identifier::parse(&mut parser).unwrap();
        parser.check_errors();
        assert_eq!(identifier.value, "foobar");
        assert_eq!(identifier.token.to_string(), "foobar");
    }
}