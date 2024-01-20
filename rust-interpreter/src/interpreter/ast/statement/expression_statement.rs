use crate::interpreter::parser::Precedence;
use super::*;
use self::{expression::Expression};

#[derive(Debug)]
pub(crate) struct ExpressionStatement<'a> {
    pub(crate) token: Token<'a>,
    pub(crate) expression: Expression<'a>
}

impl<'a> ExpressionStatement<'a> {
    pub fn new(token: Token<'a>, expression: Expression<'a>) -> ExpressionStatement<'a> {
        ExpressionStatement { token, expression }
    }

    pub(crate) fn parse(parser: &mut Parser<'a>) -> anyhow::Result<ExpressionStatement<'a>> {
        let token = parser.current_token;
        let expression = Expression::parse(parser, Precedence::Lowest)?;
        if parser.peek_is(Token::Semicolon) {
            parser.next_token();
        }
        return Ok(ExpressionStatement::new(token, expression));
    }
}

impl Display for ExpressionStatement<'_> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.expression)
    }
}