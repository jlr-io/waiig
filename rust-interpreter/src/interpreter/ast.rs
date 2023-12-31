pub mod identifier;
pub mod let_stmt;

use crate::interpreter::{
    ast::{
        let_stmt::LetStmt
    },
    parser::{Parser, Parse},
    token::Token,
};

pub trait Node {
    fn token_literal(&self) -> &str;
}

trait StatementNode: Node {
    fn statement_node(&self);
}

trait ExpressionNode: Node {
    fn expression_node(&self);
}

#[derive(Debug)]
pub(crate) enum Expression {
    None
    // todo
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            _ => todo!()
        }
    }
}

#[derive(Debug)] 
#[allow(dead_code)]
pub(crate) enum Statement<'a> {
    Let(LetStmt<'a>),
    None
}

impl Node for Statement<'_> {
    fn token_literal(&self) -> &str {
        match self {
            Statement::Let(let_statement) => let_statement.token_literal(),
            Statement::None => ""
        }
    }
}

impl StatementNode for Statement<'_> {
    fn statement_node(&self) {}
}

impl<'a> Parse<'a> for Statement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Option<Statement<'a>> {
        match parser.current_token {
            Token::Let => Some(Statement::Let(LetStmt::parse(parser)?)),
            _ => None
        }
    }
}

#[derive(Debug)]
pub(crate) struct Program<'a> {
    pub statements: Vec<Statement<'a>>
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Program { statements: Vec::new() }
    }
}

impl<'a> Parse<'a> for Program<'a> {
    fn parse(parser: &mut Parser<'a>) -> Option<Program<'a>> {
        let mut program = Program::new();
        while !parser.current_token_is(Token::Eof) {
            if let Some(stmt) = Statement::parse(parser) {
                program.statements.push(stmt);
            }
            parser.next_token();
        };
        Some(program)
    }
}