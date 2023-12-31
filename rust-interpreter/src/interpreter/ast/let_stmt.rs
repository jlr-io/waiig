use crate::interpreter::{
    token::Token,
    ast::{identifier::Identifier, Expression, Node},
    parser::{Parser, Parse}
};

#[derive(Debug)]
pub(crate) struct LetStmt<'a> {
    pub token: Token<'a>,
    pub name: Identifier<'a>,
    pub value: Expression,
}

impl<'a> LetStmt<'a> {
    pub fn new(token: Token<'a>, name: Identifier<'a>, value: Expression) -> LetStmt<'a> {
        LetStmt { token, name, value }
    }
}

impl<'a> Node for LetStmt<'a> {
    fn token_literal(&self) -> &str {
        match self.token {
            Token::Ident(ident) => ident,
            _ => "" // illegal?
        }
    }
}

impl<'a> Parse<'a> for LetStmt<'a> {
    fn parse(parser: &mut Parser<'a>) -> Option<LetStmt<'a>> {
        let token = parser.current_token;
        parser.next_token();
        let identifier = Identifier::parse(parser)?;
        if !parser.assert_peek_is(Token::Assign) { return None; }
        parser.next_token();
        // todo skipping the expressions until we encounter a semi colon
        while !parser.current_token_is(Token::Semicolon) {
            parser.next_token();
        }
        Some(LetStmt::new(token, identifier, Expression::None))
    }
}

#[cfg(test)]
mod let_statement_tests {
    use super::*;
    use crate::interpreter::{
        ast::{Node, Statement},
        lexer::Lexer
    };
    
    #[test]
    fn test_let_statements() {
        let input = r#"
           let x = 5;
           let y = 10;
           let foobar = 838383; 
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = match parser.parse_program() {
            Some(program) => program,
            None => panic!("parse_program() returned None"),
        };
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
            panic!("stmt not Statement::Let. got={:?}", stmt);
        }
        true
    }
}