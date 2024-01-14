use super::*;
use self::{
    expression::Expression,
    identifier::Identifier
};

#[derive(Debug)]
pub(crate) struct ReturnStatement<'a> {
    pub token: Token<'a>,
    pub return_value: Expression<'a>
}

impl<'a> ReturnStatement<'a> {
    pub fn new(token: Token<'a>, return_value: Expression<'a>) -> ReturnStatement<'a> {
        ReturnStatement { token, return_value }
    }

    pub(crate) fn parse(parser: &mut Parser<'a>) -> anyhow::Result<ReturnStatement<'a>> {
        let token = parser.current_token;
        parser.next_token();
        // todo skipping the expressions until we encounter a semi colon
        while !parser.current_is(Token::Semicolon) {
            parser.next_token();
        }
        Ok(ReturnStatement::new(token, Expression::Identifier(Identifier::new(""))))
    }
}

impl Display for ReturnStatement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.return_value)
    }
}

impl Node for ReturnStatement<'_> {
    fn token_literal(&self) -> String {
        Token::lookup_literal(&self.token)
    }
}

#[cfg(test)]
mod return_statement_tests {
    use crate::interpreter::lexer::Lexer;
    use super::*;
    
    #[test]
    fn test_return_statements() {
        let input = r#"
           return 5;
           return 10;
           return 993322; 
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = Program::parse(&mut parser).unwrap();
        parser.check_errors();

        assert_eq!(program.statements.len(), 3);

        let tests = vec![5, 10, 993322];

        for (i, test) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            match stmt {
                Statement::Return(return_statement) => {
                    assert_eq!(return_statement.token_literal(), "return");
                    assert_eq!(return_statement.return_value.token_literal(), test.to_string());
                },
                _ => panic!("expected return statement")
            }
        }
    }
}