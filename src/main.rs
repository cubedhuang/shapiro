use lexer::Lexer;

use crate::token::Token;

mod lexer;
mod parser;
mod token;

fn main() {
    let file = include_str!("../test.shap");

    let lexer = Lexer::new(file);

    loop {
        match lexer.next() {
            Ok(token) => {
                println!("{:?}", token);

                if matches!(token, Token::Eof) {
                    break;
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
}
