use super::*;
use self::{
    expression::Expression,
    identifier::Identifier
};

#[derive(Debug)]
pub(crate) struct ReturnStatement<'a> {
    pub token: Token<'a>,
    pub value: Expression<'a>
}

impl<'a> ReturnStatement<'a> {
    pub fn new(token: Token<'a>, value: Expression<'a>) -> ReturnStatement<'a> {
        ReturnStatement { token, value }
    }

    pub(crate) fn parse(parser: &mut Parser<'a>) -> anyhow::Result<ReturnStatement<'a>> {
        let token = parser.current_token;
        parser.next_token();
        // todo skipping the expressions until we encounter a semi colon
        while !parser.current_is(Token::Semicolon) {
            parser.next_token();
        }

        // todo: this is a hack to get the parser to work
        let temp_expression = Expression::Identifier(Identifier::new(Token::Identifier(""), ""));
        Ok(ReturnStatement::new(token, temp_expression))
    }
}

impl Display for ReturnStatement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {};", self.token.to_string(), self.value)
    }
}

// impl Node for ReturnStatement<'_> {
//     fn token_literal(&self) -> String {
//         self.token.to_string()
//     }
// }

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
                    assert_eq!(return_statement.token.to_string(), "return");
                    if let Expression::Integer(int) = &return_statement.value {
                        assert_eq!(int.value.to_string(), test.to_string());
                    } else {
                        panic!("expected integer literal");
                    }
                },
                _ => panic!("expected return statement")
            }
        }
    }
}