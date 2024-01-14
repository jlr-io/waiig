use std::fmt::{Result};
use super::*;

#[derive(Debug)]
pub(crate) struct Identifier<'a> {
    pub value: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn new(value: &str) -> Identifier {
        Identifier { value }
    }

    pub(crate) fn parse(parser: &mut Parser<'a>) -> anyhow::Result<Identifier<'a>> {
        match parser.current_token {
            Token::Identifier(ident) => Ok(Identifier::new(ident)),
            _ => Err(parser.unexpected_token_error(Token::Identifier(""), parser.current_token))
        }
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> String {
        self.value.to_string()
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
        assert_eq!(identifier.token_literal(), "foobar");
    }
}