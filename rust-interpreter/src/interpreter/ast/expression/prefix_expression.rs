use super::*;

#[derive(Debug)]
pub(crate) enum PrefixOperator {
    Not,
    Minus,
}

impl From<&PrefixOperator> for Token<'_> {
    fn from(value: &PrefixOperator) -> Self {
        match value {
            PrefixOperator::Not => Token::Bang,
            PrefixOperator::Minus => Token::Minus,
        }
    }
}

#[derive(Debug)]
pub(crate) struct PrefixExpression<'a> {
    pub token: Token<'a>,
    pub operator: PrefixOperator,
    pub right: Box<Expression<'a>>,
}

impl<'a> PrefixExpression<'a> {
    pub fn new(token: Token<'a>, operator: PrefixOperator, right: Expression<'a>) -> PrefixExpression<'a> {
        PrefixExpression {
            token,
            operator,
            right: Box::new(right)
        }
    }

    pub(crate) fn parse(parser: &mut Parser<'a>, _precedence: &Precedence) -> anyhow::Result<PrefixExpression<'a>> {
        let token = parser.current_token;
        let operator = match token {
            Token::Bang => PrefixOperator::Not,
            Token::Minus => PrefixOperator::Minus,
            _ => return Err(parser.unexpected_prefix_error(token))
        };
        parser.next_token();
        let right = Expression::parse(parser, Precedence::Prefix)?;
        return Ok(PrefixExpression::new(token, operator, right));
    }
}

impl Display for PrefixExpression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}{})", self.token, self.right)
    }
}

#[cfg(test)]
mod prefix_tests {
    use super::*;
    use crate::interpreter::{
        parser::Parser,
        lexer::Lexer,
    };

    #[test]
    fn test_parse_prefix() {
        let tests = vec![
            ("-15;", "-", "15"),
            ("!5;", "!", "5"),
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
                    // assert_eq!(es.expression.token_literal(), test.1);
                    match &es.expression {
                        Expression::Prefix(pe) => {
                            let operator_token = Token::from(&pe.operator);
                            assert_eq!(operator_token.to_string(), test.1);
                            let right_token = Token::from(&pe.right);
                            assert_eq!(right_token.to_string(), test.2);
                        },
                        _ => panic!("Expected prefix expression, got {:?}", es.expression)
                    }
                },
                _ => panic!("Expected ExpressionStatement, got {:?}", stmt)
            }
        }
        
    }
}