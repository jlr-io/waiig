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
    Int(Integer),
    Prefix(PrefixExpression<'a>),
    Infix(InfixExpression<'a>),
}

impl<'a> Expression<'a> {
    pub(crate) fn parse(parser: &mut Parser<'a>, precedence: Precedence) -> anyhow::Result<Expression<'a>> {
        let token = parser.current_token;
        let mut exp = match token {
            Token::Identifier(_) => Expression::Identifier(Identifier::parse(parser)?),
            Token::Int(_) => Expression::Int(Integer::parse(parser)?),
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

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Identifier(i) => Display::fmt(&i, f),
            Expression::Int(i) => Display::fmt(&i, f),
            Expression::Prefix(p) => Display::fmt(&p, f),
            Expression::Infix(i) => Display::fmt(&i, f),
        }
    }
}

impl Node for Expression<'_> {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token_literal(),
            Expression::Int(i) => i.token_literal(),
            Expression::Prefix(p) => p.token_literal(),
            Expression::Infix(i) => i.token_literal(),
        }
    }
}