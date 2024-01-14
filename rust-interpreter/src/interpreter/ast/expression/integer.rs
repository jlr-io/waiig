use std::fmt::Result;
use anyhow::anyhow;
use super::*;

#[derive(Debug)]
pub(crate) struct Integer {
    value: i64
}

impl TryFrom<&str> for Integer {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> anyhow::Result<Integer> {
        Ok(Integer { value: value.parse::<i64>()? })
    }
}

impl TryFrom<Token<'_>> for Integer {
    type Error = anyhow::Error;
    fn try_from(value: Token) -> anyhow::Result<Integer> {
        match value {
            Token::Int(i) => Ok(Integer::try_from(i)?),
            _ => Err(anyhow!("Expected Integer, got {:?}", value))
        }
    }
}

impl Integer {
    pub fn new(value: i64) -> anyhow::Result<Self> {
        Ok(Integer { value })
    }
    
    pub fn parse(parser: &mut Parser) -> anyhow::Result<Integer> {
        Integer::try_from(parser.current_token)
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Node for Integer {
    fn token_literal(&self) -> String {
        self.value.to_string()
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
        assert_eq!(integer.token_literal(), "5");
    }
}