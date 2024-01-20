use std::fmt::{Debug, Display, Formatter, Result};
pub(crate) mod let_statement;
pub(crate) mod return_statement;
pub(crate) mod expression_statement;
use super::*;
use self::{
    let_statement::LetStatement,
    return_statement::ReturnStatement,
    expression_statement::ExpressionStatement,
    expression::Expression,
};

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Let(LetStatement<'a>),
    Return(ReturnStatement<'a>),
    Expression(ExpressionStatement<'a>)
}

impl<'a> Statement<'a> {
    pub(crate) fn parse(parser: &mut Parser<'a>) -> anyhow::Result<Statement<'a>> {
        let stmt = match parser.current_token {
            Token::Let => Statement::Let(LetStatement::parse(parser)?),
            Token::Return => Statement::Return(ReturnStatement::parse(parser)?),
            _ => Statement::Expression(ExpressionStatement::parse(parser)?)
        };
        Ok(stmt)
    }
}

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Statement::Let(ls) => Display::fmt(&ls, f),
            Statement::Return(rs) => Display::fmt(&rs, f),
            Statement::Expression(es) => Display::fmt(&es, f)
        }
    }
}

#[cfg(test)]
mod statement_tests {
    use super::*;
    
    #[test]
    fn test_to_string() {
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement::new(
                    Token::Let,
                    Identifier::new(Token::Let, "myVar"),
                    Expression::Identifier(Identifier::new(Token::Identifier("someVar"), "someVar"))
                )),
                Statement::Return(ReturnStatement::new(
                    Token::Return,
                    Expression::Identifier(Identifier::new(Token::Return, "someReturn"))
                )),
            ]
        };
        assert_eq!(program.to_string(), "let myVar = someVar;return someReturn;");
    }
}
