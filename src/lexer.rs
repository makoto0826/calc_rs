use crate::token::Token;

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Option<Vec<Token>> {
        let mut tokens = Vec::<Token>::new();

        while let Some(ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
                continue;
            }

            let token = match ch {
                '+' => {
                    self.input.next();
                    Token::Plus
                }
                '-' => {
                    self.input.next();
                    Token::Minus
                }
                '*' => {
                    self.input.next();
                    Token::Asterisk
                }
                '/' => {
                    self.input.next();
                    Token::Slash
                }
                '%' => {
                    self.input.next();
                    Token::Percent
                }
                '(' => {
                    self.input.next();
                    Token::Lparen
                }
                ')' => {
                    self.input.next();
                    Token::Rparen
                }
                '!' => {
                    self.input.next();
                    Token::Exclamation
                }
                '^' => {
                    self.input.next();
                    Token::Circumflex
                }
                '0'..='9' => self.consume_num()?,
                _ => return None,
            };

            tokens.push(token);
        }

        Some(tokens)
    }

    fn consume_num(&mut self) -> Option<Token> {
        let mut sum: i64 = 0;

        while let Some(ch) = self.input.peek() {
            match ch {
                '0'..='9' => {
                    let temp = sum.checked_mul(10)?;
                    let num = ch.to_digit(10).unwrap_or_else(|| 0) as i64;
                    sum = temp.checked_add(num)?;

                    self.input.next();
                }
                _ => return Some(Token::Num(sum)),
            }
        }

        Some(Token::Num(sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn lexer1_test() {
        let mut lexer = Lexer::new("1 + 2 - (3 * 4 / 5)! % 6^2");
        let tokens = &lexer.tokenize().unwrap()[..];

        assert_eq!(tokens[0], Token::Num(1));
        assert_eq!(tokens[1], Token::Plus);
        assert_eq!(tokens[2], Token::Num(2));
        assert_eq!(tokens[3], Token::Minus);
        assert_eq!(tokens[4], Token::Lparen);
        assert_eq!(tokens[5], Token::Num(3));
        assert_eq!(tokens[6], Token::Asterisk);
        assert_eq!(tokens[7], Token::Num(4));
        assert_eq!(tokens[8], Token::Slash);
        assert_eq!(tokens[9], Token::Num(5));
        assert_eq!(tokens[10], Token::Rparen);
        assert_eq!(tokens[11], Token::Exclamation);
        assert_eq!(tokens[12], Token::Percent);
        assert_eq!(tokens[13], Token::Num(6));
        assert_eq!(tokens[14], Token::Circumflex);
        assert_eq!(tokens[15], Token::Num(2));
    }

    #[test]
    fn lexer2_test() {
        let mut lexer = Lexer::new("9223372036854775808");
        let tokens = &lexer.tokenize();
        assert!(tokens.is_none());
    }
}
