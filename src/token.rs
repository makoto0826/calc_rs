#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Lparen,
    Rparen,
    Exclamation,
    Circumflex,
}
