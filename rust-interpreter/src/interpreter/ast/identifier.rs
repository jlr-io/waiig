use crate::interpreter::{
    token::Token,
    ast::{ExpressionNode, Node},
    parser::{Parser, Parse}
};

#[derive(Debug)]
pub(crate) struct Identifier<'a> {
    pub value: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn new(value: &'a str) -> Identifier<'a> {
        Identifier { value }
    }
}

impl<'a> Node for Identifier<'a> {
    fn token_literal(&self) -> &str {
        self.value
    }
}

impl<'a> ExpressionNode for Identifier<'a> {
    fn expression_node(&self) {}
}

impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(parser: &mut Parser<'a>) -> Option<Identifier<'a>> {
        match parser.current_token {
            Token::Ident(ident) => Some(Identifier::new(ident)),
            _ => {
                parser.errors.push(format!("expected identifier, got {:?}", parser.current_token));
                None
            }
        }
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
        assert_eq!(identifier.value, "foobar");
    }
}