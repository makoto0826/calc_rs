use crate::ast::{Expr, Operator, PostfixOperator, PrefixOperator};
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Sum,
    Product,
    Prefix,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_expr(Precedence::Lowest)
    }

    fn parse_expr(&mut self, p: Precedence) -> Option<Expr> {
        let mut lhs = self.parse_prefix_expr()?;
        lhs = self.parse_postfix_expr(lhs)?;

        while self.peek_token().is_some() && p < self.peek_token_to_precedence() {
            self.next();
            lhs = self.parse_infix_expr(lhs)?;
        }

        Some(lhs)
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        match self.current_token() {
            Some(token) => match *token {
                Token::Plus => {
                    self.next();
                    let rhs = self.parse_expr(Precedence::Prefix)?;
                    Some(Expr::PrefixExpr(PrefixOperator::Plus, Box::new(rhs)))
                }
                Token::Minus => {
                    self.next();
                    let rhs = self.parse_expr(Precedence::Prefix)?;
                    Some(Expr::PrefixExpr(PrefixOperator::Minus, Box::new(rhs)))
                }
                Token::Num(num) => Some(Expr::UnaryExpr(num)),
                Token::Lparen => self.parse_grouped_expr(),
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_infix_expr(&mut self, lhs: Expr) -> Option<Expr> {
        Some(match self.current_token() {
            Some(token) => match *token {
                Token::Plus => {
                    self.next();
                    let rhs = self.parse_expr(Precedence::Sum)?;
                    Expr::BinaryExpr {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Operator::Add,
                    }
                }
                Token::Minus => {
                    self.next();
                    let rhs = self.parse_expr(Precedence::Sum)?;
                    Expr::BinaryExpr {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Operator::Sub,
                    }
                }
                Token::Slash => {
                    self.next();
                    let rhs = self.parse_expr(Precedence::Product)?;
                    Expr::BinaryExpr {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Operator::Div,
                    }
                }
                Token::Asterisk => {
                    self.next();
                    let rhs = self.parse_expr(Precedence::Product)?;
                    Expr::BinaryExpr {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Operator::Mul,
                    }
                }
                Token::Percent => {
                    self.next();
                    let rhs = self.parse_expr(Precedence::Product)?;
                    Expr::BinaryExpr {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                        op: Operator::Rem,
                    }
                }

                _ => return None,
            },
            _ => return None,
        })
    }

    fn parse_postfix_expr(&mut self, lhs: Expr) -> Option<Expr> {
        Some(match self.peek_token() {
            Some(token) => match *token {
                Token::Exclamation => {
                    self.next();
                    Expr::PostfixExpr(PostfixOperator::Factorial, Box::new(lhs))
                }
                Token::Circumflex => {
                    self.next();
                    self.next();

                    if let Some(Token::Num(n)) = self.current_token() {
                        Expr::PostfixExpr(
                            PostfixOperator::Exponential(n.clone() as u32),
                            Box::new(lhs),
                        )
                    } else {
                        return None;
                    }
                }
                _ => lhs,
            },
            _ => lhs,
        })
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.next();
        let lhs = self.parse_expr(Precedence::Lowest);

        if self.peek_token_is(&Token::Rparen) {
            self.next();
            lhs
        } else {
            None
        }
    }

    fn next(&mut self) {
        self.index += 1;
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    fn peek_token_to_precedence(&self) -> Precedence {
        match self.peek_token() {
            Some(token) => match *token {
                Token::Minus | Token::Plus => Precedence::Sum,
                Token::Slash | Token::Asterisk | Token::Percent => Precedence::Product,
                _ => Precedence::Lowest,
            },
            _ => Precedence::Lowest,
        }
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        if let Some(t) = self.peek_token() {
            t == token
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Operator, PostfixOperator, PrefixOperator};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn create_expr(line: &str) -> Option<Expr> {
        let mut lexer = Lexer::new(line);
        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn parse1_test() {
        let expr = create_expr("1 + 2");

        assert_eq!(
            expr.unwrap(),
            Expr::BinaryExpr {
                left: Box::new(Expr::UnaryExpr(1)),
                right: Box::new(Expr::UnaryExpr(2)),
                op: Operator::Add
            }
        );
    }

    #[test]
    fn parse2_test() {
        let expr = create_expr("(3 - 1) * -5");

        assert_eq!(
            expr.unwrap(),
            Expr::BinaryExpr {
                left: Box::new(Expr::BinaryExpr {
                    left: Box::new(Expr::UnaryExpr(3)),
                    right: Box::new(Expr::UnaryExpr(1)),
                    op: Operator::Sub
                }),
                right: Box::new(Expr::PrefixExpr(
                    PrefixOperator::Minus,
                    Box::new(Expr::UnaryExpr(5))
                )),
                op: Operator::Mul
            }
        );
    }

    #[test]
    fn parse3_test() {
        let expr = create_expr("(3 - 1) * (-(3 + 3) / -2)");

        assert_eq!(
            expr.unwrap(),
            Expr::BinaryExpr {
                left: Box::new(Expr::BinaryExpr {
                    left: Box::new(Expr::UnaryExpr(3)),
                    right: Box::new(Expr::UnaryExpr(1)),
                    op: Operator::Sub
                }),
                right: Box::new(Expr::BinaryExpr {
                    left: Box::new(Expr::PrefixExpr(
                        PrefixOperator::Minus,
                        Box::new(Expr::BinaryExpr {
                            left: Box::new(Expr::UnaryExpr(3)),
                            right: Box::new(Expr::UnaryExpr(3)),
                            op: Operator::Add
                        })
                    )),
                    right: Box::new(Expr::PrefixExpr(
                        PrefixOperator::Minus,
                        Box::new(Expr::UnaryExpr(2))
                    )),
                    op: Operator::Div
                }),
                op: Operator::Mul
            }
        );
    }

    #[test]
    fn parse4_test() {
        let expr = create_expr("3! - 2!");

        assert_eq!(
            expr.unwrap(),
            Expr::BinaryExpr {
                left: Box::new(Expr::PostfixExpr(
                    PostfixOperator::Factorial,
                    Box::new(Expr::UnaryExpr(3)),
                )),
                right: Box::new(Expr::PostfixExpr(
                    PostfixOperator::Factorial,
                    Box::new(Expr::UnaryExpr(2))
                )),
                op: Operator::Sub
            }
        );
    }

    #[test]
    fn parse5_test() {
        let expr = create_expr("(3 - 1) * (-(3 + 3) / -2)!");

        assert_eq!(
            expr.unwrap(),
            Expr::BinaryExpr {
                left: Box::new(Expr::BinaryExpr {
                    left: Box::new(Expr::UnaryExpr(3)),
                    right: Box::new(Expr::UnaryExpr(1)),
                    op: Operator::Sub
                }),
                right: Box::new(Expr::PostfixExpr(
                    PostfixOperator::Factorial,
                    Box::new(Expr::BinaryExpr {
                        left: Box::new(Expr::PrefixExpr(
                            PrefixOperator::Minus,
                            Box::new(Expr::BinaryExpr {
                                left: Box::new(Expr::UnaryExpr(3)),
                                right: Box::new(Expr::UnaryExpr(3)),
                                op: Operator::Add
                            })
                        )),
                        right: Box::new(Expr::PrefixExpr(
                            PrefixOperator::Minus,
                            Box::new(Expr::UnaryExpr(2))
                        )),
                        op: Operator::Div
                    })
                )),
                op: Operator::Mul
            }
        );
    }

    #[test]
    fn parse6_test() {
        let expr = create_expr("3^2 - 2");

        assert_eq!(
            expr.unwrap(),
            Expr::BinaryExpr {
                left: Box::new(Expr::PostfixExpr(
                    PostfixOperator::Exponential(2),
                    Box::new(Expr::UnaryExpr(3)),
                )),
                right: Box::new(Expr::UnaryExpr(2)),
                op: Operator::Sub
            }
        );
    }
}
