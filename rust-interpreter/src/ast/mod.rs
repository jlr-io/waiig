use crate::token::Token;

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

impl ExpressionNode for Expression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub(crate) struct Identifier<'a> {
    pub value: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn new(value: &str) -> Identifier {
        Identifier { value }
    }
}

impl<'a> Node for Identifier<'a> {
    fn token_literal(&self) -> &str {
        self.value
    }
}

impl<'a> ExpressionNode for Identifier<'a> {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Let(Let<'a>),
    None
}

#[derive(Debug)]
pub(crate) struct Let<'a> {
    pub token: Token<'a>,
    pub name: Identifier<'a>,
    pub value: Expression,
}

impl<'a> Let<'a> {
    pub fn new(token: Token<'a>, name: Identifier<'a>, value: Expression) -> Let<'a> {
        Let { token, name, value }
    }
}

impl<'a> Node for Let<'a> {
    fn token_literal(&self) -> &str {
        match self.token {
            Token::Ident(ident) => ident,
            _ => "" // illegal?
        }
    }
}

impl<'a> StatementNode for Let<'a> {
    fn statement_node(&self) {}
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
