use crate::ast::{Expr, Operator, PostfixOperator, PrefixOperator};

pub fn eval(expr: Expr) -> Option<i64> {
    match expr {
        Expr::UnaryExpr(n) => Some(n),
        Expr::PrefixExpr(op, expr) => {
            if op == PrefixOperator::Minus {
                let n = eval(*expr)?;
                Some(-n)
            } else {
                eval(*expr)
            }
        }
        Expr::PostfixExpr(op, expr) => match op {
            PostfixOperator::Factorial => {
                let n = eval(*expr)?;
                factorial(n)
            }
            PostfixOperator::Exponential(exp) => {
                let n = eval(*expr)?;
                n.checked_pow(exp)
            }
        },
        Expr::BinaryExpr { left, right, op } => {
            let left = eval(*left)?;
            let right = eval(*right)?;

            match op {
                Operator::Add => left.checked_add(right),
                Operator::Sub => left.checked_sub(right),
                Operator::Div => left.checked_div(right),
                Operator::Mul => left.checked_mul(right),
                Operator::Rem => left.checked_rem(right),
            }
        }
    }
}

fn factorial(n: i64) -> Option<i64> {
    let mut sum: i64 = 1;

    for i in 2..=n {
        sum = sum.checked_mul(i)?
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr;
    use crate::executor::eval;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn create_expr(line: &str) -> Expr {
        let mut lexer = Lexer::new(line);
        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn eval1_test() {
        let expr = create_expr("5 + 3 * 6 - 3");
        assert_eq!(eval(expr).unwrap(), 20);
    }

    #[test]
    fn eval2_test() {
        let expr = create_expr("-(3 * 2)! / (11 % 3)");
        assert_eq!(eval(expr).unwrap(), -360);
    }

    #[test]
    fn eval3_test() {
        let expr = create_expr("1! + 0!");
        assert_eq!(eval(expr).unwrap(), 2);
    }

    #[test]
    fn eval4_test() {
        let expr = create_expr("1! + 0!");
        assert_eq!(eval(expr).unwrap(), 2);
    }

    #[test]
    fn eval5_test() {
        let expr = create_expr("2^3 + 10");
        assert_eq!(eval(expr).unwrap(), 18);
    }

    #[test]
    fn eval6_test() {
        let expr = create_expr("1 % 0");
        assert!(eval(expr).is_none());
    }

    #[test]
    fn eval7_test() {
        let expr = create_expr("9223372036854775807 + 1");
        assert!(eval(expr).is_none());
    }
}
