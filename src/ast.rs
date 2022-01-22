#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostfixOperator {
    Factorial,
    Exponential(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    PrefixExpr(PrefixOperator, Box<Expr>),
    PostfixExpr(PostfixOperator, Box<Expr>),
    UnaryExpr(i64),
    BinaryExpr {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Operator,
    },
}
