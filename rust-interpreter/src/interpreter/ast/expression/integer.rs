use std::fmt::Result;
use std::str::FromStr;
use super::*;

#[derive(Debug)]
pub(crate) struct Integer<'a> {
    pub(crate) token: Token<'a>,
    pub(crate) value: i64
}

impl<'a> Integer<'a> {
    pub fn new(token: Token<'a>, value: i64) -> anyhow::Result<Integer<'a>> {
        Ok(Integer { token, value })
    }
    
    pub fn parse(parser: &mut Parser<'a>) -> anyhow::Result<Integer<'a>> {
        match parser.current_token {
            Token::Integer(i) => Ok(Integer::new(Token::Integer(i), i64::from_str(i)?).unwrap()),
            _ => Err(parser.unexpected_token_error(Token::Integer(""), parser.current_token))
        }
    }
}

impl Display for Integer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod integer_tests {
    use crate::interpreter::lexer::Lexer;
    use super::*;

    #[test]
    fn test_integer() {
        let input = Lexer::new("5");
        let mut parser = Parser::new(input);
        let integer = Integer::parse(&mut parser).unwrap();
        parser.check_errors();
        assert_eq!(integer.value, 5);
        assert_eq!(integer.token.to_string(), "5");
    }
}