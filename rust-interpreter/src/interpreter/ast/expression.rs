pub(crate) mod integer;
pub(crate) mod prefix_expression;
pub(crate)mod infix_expression;
use std::fmt::{Debug, Display, Result};
use self::{
    super::*,
    prefix_expression::PrefixExpression,
    infix_expression::InfixExpression,
    Precedence,
    identifier::Identifier,
    integer::Integer,
};

#[derive(Debug)]
pub(crate) enum Expression<'a> {
    Identifier(Identifier<'a>),
    Integer(Integer<'a>),
    Prefix(PrefixExpression<'a>),
    Infix(InfixExpression<'a>),
}

impl<'a> From<&Box<Expression<'a>>> for Token<'a> {
    fn from(value: &Box<Expression<'a>>) -> Token<'a> {
        match value.as_ref() {
            Expression::Identifier(identifier) => identifier.token,
            Expression::Integer(integer) => integer.token,
            Expression::Prefix(prefix) => prefix.token,
            Expression::Infix(infix) => infix.token,
        }
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Identifier(i) => Display::fmt(&i, f),
            Expression::Integer(i) => Display::fmt(&i, f),
            Expression::Prefix(p) => Display::fmt(&p, f),
            Expression::Infix(i) => Display::fmt(&i, f),
        }
    }
}

impl<'a> Expression<'a> {
    pub(crate) fn parse(parser: &mut Parser<'a>, precedence: Precedence) -> anyhow::Result<Expression<'a>> {
        let token = parser.current_token;
        let mut exp = match token {
            Token::Identifier(_) => Expression::Identifier(Identifier::parse(parser)?),
            Token::Integer(_) => Expression::Integer(Integer::parse(parser)?),
            token if token.is_prefix() => Expression::Prefix(PrefixExpression::parse(parser, &precedence)?),
            _ => return Err(parser.unexpected_prefix_error(token)),
        };
        
        while !parser.peek_is(Token::Semicolon) && precedence < Precedence::from(parser.peek_token) {
            if !parser.peek_token.is_infix() {
                return Ok(exp)
            }
            parser.next_token();
            exp = Expression::Infix(InfixExpression::parse(parser, exp)?);
        }
        
        return Ok(exp)
    }
}

#[cfg(test)]
mod expression_tests {
    use crate::interpreter::ast::Program;
    use crate::interpreter::lexer::Lexer;
    use crate::interpreter::parser::Parser;

    #[test]
    fn walk_parse_expression() {
        let tests = vec![
            "5 + 5 * 5;",
        ];

        for test in tests {
            let lexer = Lexer::new(test);
            let mut parser = Parser::new(lexer);
            let program = Program::parse(&mut parser).unwrap();
            parser.check_errors();
            assert_eq!(program.statements.len(), 1);
            // let stmt = &program.statements[0];
            println!("{}", program);
        }
    }
}