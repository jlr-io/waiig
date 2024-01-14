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

impl Node for Statement<'_> {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(ls) => ls.token_literal(),
            Statement::Return(rs) => rs.token_literal(),
            Statement::Expression(es) => es.token_literal(),
        }
    }
}
// 
// impl<'a> Parse<'a> for Statement<'a> {
//     fn parse(parser: &mut Parser<'a>, _: Option<Precedence>) -> anyhow::Result<Statement<'a>> {
//         let stmt = match parser.current_token {
//             Token::Let => Statement::Let(LetStatement::parse(parser)?),
//             Token::Return => Statement::Return(ReturnStatement::parse(parser)?),
//             _ => Statement::Expression(ExpressionStatement::parse(parser)?)
//         };
//         Ok(stmt)
//     }
// }

#[cfg(test)]
mod statement_tests {
    use super::*;
    
    #[test]
    fn test_to_string() {
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement::new(
                    Token::Let,
                    Identifier::new("myVar"),
                    Expression::Identifier(Identifier::new("someVar"))
                )),
                Statement::Return(ReturnStatement::new(
                    Token::Return,
                    Expression::Identifier(Identifier::new("someReturn"))
                )),
            ]
        };
        assert_eq!(program.to_string(), "let myVar = someVar;return someReturn;");
    }
}
