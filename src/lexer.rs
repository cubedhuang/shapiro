use std::cell::Cell;

use crate::token::{Operator, Token};

#[derive(Debug)]
pub enum LexError<'src> {
    InvalidCharacter(&'src str),
}

#[derive(Debug)]
pub struct Lexer<'src> {
    src: &'src str,
    start: Cell<usize>,
    cur: Cell<usize>,
    line: Cell<usize>,
    col: Cell<usize>,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Lexer {
            src: input,
            start: 0.into(),
            cur: 0.into(),
            line: 1.into(),
            col: 0.into(),
        }
    }

    pub fn next(&self) -> Result<Token, LexError> {
        self.skip_whitespace();

        let c = match self.advance() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        match c {
            num @ _ if Self::is_digit(num) => Ok(self.next_number()),
            "+" => Ok(Token::Operation(Operator::Add)),
            "-" => Ok(Token::Operation(Operator::Sub)),
            "*" => Ok(Token::Operation(Operator::Mul)),
            "/" => Ok(Token::Operation(Operator::Div)),
            "%" => Ok(Token::Operation(Operator::Mod)),
            _ => Err(LexError::InvalidCharacter(&c)),
        }
    }

    fn next_number(&self) -> Token {
        while let Some(c) = self.peek() {
            if Self::is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }

        Token::Number(self.src[self.start.get()..self.cur.get()].parse().unwrap())
    }

    fn advance(&self) -> Option<&'src str> {
        let c = self.peek();
        self.cur.set(self.cur.get() + 1);
        self.col.set(self.col.get() + 1);
        c
    }

    fn peek(&self) -> Option<&'src str> {
        self.src.get(self.cur.get()..=self.cur.get())
    }

    fn is_digit(value: &'src str) -> bool {
        match value {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
            _ => false,
        }
    }

    fn skip_whitespace(&self) {
        while let Some(c) = self.peek() {
            match c {
                "\n" => {
                    self.line.set(self.line.get() + 1);
                    self.col.set(0);
                    self.advance();
                }
                "\t" => {
                    self.col.set(self.col.get() + 3);
                    self.advance();
                }
                " " | "\r" => {
                    self.advance();
                }
                _ => break,
            }
        }

        self.start.set(self.cur.get());
    }
}
