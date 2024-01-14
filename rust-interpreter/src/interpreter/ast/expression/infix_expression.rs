use anyhow::anyhow;
use crate::interpreter::ast::expression::Expression;
use super::*;

#[derive(Debug)]
pub(crate) enum InfixOperator {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

impl<'a> From<&InfixOperator> for Token<'a> {
    fn from(operator: &InfixOperator) -> Token<'a> {
        match operator {
            InfixOperator::Plus => Token::Plus,
            InfixOperator::Minus => Token::Minus,
            InfixOperator::Divide => Token::Slash,
            InfixOperator::Multiply => Token::Asterisk,
            InfixOperator::Equal => Token::Equal,
            InfixOperator::NotEqual => Token::NotEqual,
            InfixOperator::LessThan => Token::LessThan,
            InfixOperator::GreaterThan => Token::GreaterThan,
        }
    }
}

impl TryFrom<&Token<'_>> for InfixOperator {
    type Error = anyhow::Error;
    fn try_from(value: &Token) -> anyhow::Result<InfixOperator> {
        let operator = match value {
            Token::Plus => InfixOperator::Plus,
            Token::Minus => InfixOperator::Minus,
            Token::Slash => InfixOperator::Divide,
            Token::Asterisk => InfixOperator::Multiply,
            Token::Equal => InfixOperator::Equal,
            Token::NotEqual => InfixOperator::NotEqual,
            Token::LessThan => InfixOperator::LessThan,
            Token::GreaterThan => InfixOperator::GreaterThan,
            _ => return Err(anyhow!("Expected InfixOperator, got {:?}", value))
        };
        Ok(operator)
    }
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", Token::from(self))
    }
}

impl Node for InfixOperator {
    fn token_literal(&self) -> String {
        Token::from(self).to_string()
    }
}

#[derive(Debug)]
pub(crate) struct InfixExpression<'a> {
    token: Token<'a>,
    left: Box<Expression<'a>>,
    operator: InfixOperator,
    right: Box<Expression<'a>>,
}

impl Display for InfixExpression<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Node for InfixExpression<'_> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl InfixExpression<'_> {
    pub fn new<'a>(token: Token<'a>, left: Expression<'a>, operator: InfixOperator, right: Expression<'a>) -> InfixExpression<'a> {
        InfixExpression { token, left: Box::new(left), operator, right: Box::new(right) }
    }
    
    pub fn parse<'a>(parser: &mut Parser<'a>, left: Expression<'a>) -> anyhow::Result<InfixExpression<'a>> {
        let token = parser.current_token;
        let operator = InfixOperator::try_from(&token)?;
        let precedence = Precedence::from(parser.current_token);
        parser.next_token();
        let right = Expression::parse(parser, precedence)?;
        Ok(InfixExpression::new(token, left, operator, right))
    }
}

#[cfg(test)]
mod infix_expression_tests {
    use crate::interpreter::{
        parser::Parser,
        lexer::Lexer,
    };
    use super::*;

    #[test]
    fn test_parse_infix_expression() {
        let tests = vec![
            ("5 + 5;", "5", "+", "5"),
            ("5 - 5;", "5", "-", "5"),
            ("5 * 5;", "5", "*", "5"),
            ("5 / 5;", "5", "/", "5"),
            ("5 == 5;", "5", "==", "5"),
            ("5 != 5;", "5", "!=", "5"),
            ("5 < 5;", "5", "<", "5"),
            ("5 > 5;", "5", ">", "5"),
        ];
        
        for test in tests {
            let lexer = Lexer::new(test.0);
            let mut parser = Parser::new(lexer);
            let program = Program::parse(&mut parser).unwrap();
            parser.check_errors();
            assert_eq!(program.statements.len(), 1);
            let stmt = &program.statements[0];
            match stmt {
                Statement::Expression(es) => {
                    match &es.expression {
                        Expression::Infix(ie) => {
                            assert_eq!(ie.left.to_string(), test.1);
                            assert_eq!(ie.operator.to_string(), test.2);
                            assert_eq!(ie.right.to_string(), test.3);
                        },
                        _ => panic!("Expected InfixExpression, got {}", es.expression.token_literal())
                    }
                },
                _ => panic!("Expected ExpressionStatement, got {}", stmt)
            }
        }
        
    }
}