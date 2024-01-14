use std::fmt::{Display, Formatter, Result};
use super::*;
use self::{
    identifier::Identifier,
    expression::Expression,
};

#[derive(Debug)]
pub(crate) struct LetStatement<'a> {
    pub token: Token<'a>,
    pub name: Identifier<'a>,
    pub value: Expression<'a>,
}

impl<'a> LetStatement<'a> {
    pub fn new(token: Token<'a>, name: Identifier<'a>, value: Expression<'a>) -> LetStatement<'a> {
        LetStatement { token, name, value }
    }

    pub(crate) fn parse(parser: &mut Parser<'a>) -> anyhow::Result<LetStatement<'a>> {
        let token = parser.current_token;
        parser.next_token();
        let identifier = Identifier::parse(parser)?;
        parser.assert_peek_is(Token::Assign)?;
        // todo skipping the expressions until we encounter a semi colon
        while !parser.current_is(Token::Semicolon) {
            parser.next_token();
        }
        Ok(LetStatement::new(token, identifier, Expression::Identifier(Identifier::new(""))))
    }
}

impl Display for LetStatement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} = {};", self.token_literal(), self.name.value, self.value)
    }
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> String {
        Token::lookup_literal(&self.token)
    }
}

#[cfg(test)]
mod let_statement_tests {
    use super::*;
    use crate::interpreter::{
        ast::{Node, Statement},
        lexer::Lexer
    };
    use crate::interpreter::ast::Program;

    #[test]
    fn test_let_statements() {
        let input = r#"
           let x = 5;
           let y = 10;
           let foobar = 838383; 
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = Program::parse(&mut parser).unwrap();
        parser.check_errors();
        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements. got={}", program.statements.len());
        }
        let tests = [
            "x",
            "y",
            "foobar",
        ];
        for (i, test) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            test_let_statement(stmt, test);
        }
    }

    fn test_let_statement(stmt: &Statement, name: &str) -> bool {
        if let Statement::Let(let_stmt) = stmt {
            if let_stmt.name.value != name {
                panic!("let_stmt.name.value not '{}'. got={}", name, let_stmt.name.value);
            }
            if let_stmt.name.token_literal() != name {
                panic!("let_stmt.name.token_literal() not '{}'. got={}", name, let_stmt.name.token_literal());
            }
        } else {
            panic!("stmt not Statement::Let. got={}", stmt);
        }
        true
    }
}