use crate::interpreter::{
    token::Token,
    ast::{identifier::Identifier, Expression, Node},
    parser::{Parser, Parse}
};
use crate::interpreter::ast::StatementNode;

#[derive(Debug)]
pub(crate) struct ReturnStatement<'a> {
    pub token: Token<'a>,
    pub return_value: Expression
}

impl ReturnStatement<'_> {
    pub fn new(token: Token, return_value: Expression) -> ReturnStatement {
        ReturnStatement { token, return_value }
    }
}

impl Node for ReturnStatement<'_> {
    fn token_literal(&self) -> &str {
        Token::lookup_token(self.token)
    }
}

impl StatementNode for ReturnStatement<'_> {
    fn statement_node(&self) {}
}

impl<'a> Parse<'a> for ReturnStatement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Option<ReturnStatement<'a>> {
        let token = parser.current_token;
        parser.next_token();
        // todo skipping the expressions until we encounter a semi colon
        while !parser.current_is(Token::Semicolon) {
            parser.next_token();
        }
        Some(ReturnStatement::new(token, Expression::None))
    }
}

#[cfg(test)]
mod return_statement_tests {
    use super::*;
    use crate::interpreter::{
        ast::{Node, Statement},
        lexer::Lexer
    };
    use crate::interpreter::ast::Program;

    #[test]
    fn test_return_statements() {
        let input = r#"
           return 5;
           return 10;
           return 993322; 
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = match Program::parse(&mut parser) {
            Some(program) => program,
            None => panic!("failed to parse program")
        };
        parser.check_errors();

        assert_eq!(program.statements.len(), 3);

        let tests = vec![5, 10, 993322];

        for (i, test) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            match stmt {
                Statement::Return(return_statement) => {
                    assert_eq!(return_statement.token_literal(), "return");
                    // assert_eq!(return_statement.return_value.token_literal(), test.to_string());
                },
                _ => panic!("expected return statement")
            }
        }
    }
}