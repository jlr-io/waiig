pub(crate) mod identifier;
pub(crate) mod statement;
pub(crate) mod expression;
use std::fmt::{Display, Formatter};
use super::{
    token::Token,
    parser::{Parser, Precedence}
};
use self::{
    identifier::Identifier,
    statement::Statement
};

pub trait Node : Display {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub(crate) struct Program<'a> {
    pub statements: Vec<Statement<'a>>
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Program { statements: Vec::new() }
    }

    fn parse(parser: &mut Parser<'a>) -> anyhow::Result<Program<'a>> {
        let mut program = Program::new();
        while !parser.current_is(Token::Eof) {
            if let Ok(stmt) = Statement::parse(parser) {
                program.statements.push(stmt);
            }
            parser.next_token();
        };
        Ok(program)
    }
}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod program_tests {
    use super::*;
    use crate::interpreter::{
        parser::Parser,
        lexer::Lexer
    };

    #[test]
    fn test_to_string() {
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
        assert_eq!(program.to_string(), "let x = 5;let y = 10;let foobar = 838383;");
    }
}