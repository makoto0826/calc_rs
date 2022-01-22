mod ast;
mod executor;
mod lexer;
mod parser;
mod token;

use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self};

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line);

        if line.starts_with("q") {
            break;
        }

        let mut lexer = Lexer::new(&line);

        if let Some(tokens) = lexer.tokenize() {
            let mut parser = Parser::new(tokens);

            if let Some(expr) = parser.parse() {
                if let Some(n) = executor::eval(expr) {
                    println!("{}", n);
                }
            }
        }
    }
}
