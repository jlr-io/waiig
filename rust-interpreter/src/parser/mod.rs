use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Identifier, Program, Statement, Let, Expression};

#[derive(Debug, Clone)]
struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
    errors: Vec<String>
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new()
        };
        parser.next_token();
        parser.next_token();
        parser
    }
    
    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }
    
    fn parse_statement(&mut self) -> Option<Statement<'a>> {
        match self.current_token {
            Token::Let => self.parse_let(),
            _ => None,
        }
    }
    
    fn parse_let(&mut self) -> Option<Statement<'a>> {
        let token = self.current_token;
        let identifier = if let Token::Ident(ident) = self.peek_token {
            self.next_token();
            Identifier::new(ident)
        } else {
            self.errors.push(format!(
                "expected next token to be IDENT, got {:?}",
                self.current_token
            ));
            return None;
        };
        
        if self.peek_token == Token::Assign {
            self.next_token();
        } else {
            self.errors.push(format!(
                "expected next token to be ASSIGN, got {:?}",
                self.current_token
            ));
            return None;
        }
        
        // todo skipping the expressions until we encounter a semi colon
        while self.current_token != Token::Semicolon {
            self.next_token();
        }
        
        Some(Statement::Let(Let::new(token, identifier, Expression::None)))
    }
    
    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();
        let mut current_token = self.current_token;
        while current_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
            current_token = self.current_token;
        };
       Some(program)
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::{Node, Statement};
    use super::*;
    
    #[test]
    fn test_let_statements() {
        // todo: remove used to avoid unused variable warnings
        let x = Statement::None;
        
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
            panic!("s not Statement::Let. got={:?}", stmt);
        }
        true
    }
}
