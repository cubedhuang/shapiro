use std::cell::Cell;

use crate::token::{Keyword, Location, Operator, Separator, Token};

#[derive(Debug)]
pub enum LexError<'src> {
    InvalidCharacter(&'src str, Location),
}

impl<'src> std::fmt::Display for LexError<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::InvalidCharacter(c, _) => {
                write!(f, "Invalid character: '{}'", c)
            }
        }
    }
}

impl<'src> LexError<'src> {
    pub fn loc(&self) -> Location {
        match self {
            LexError::InvalidCharacter(_, loc) => *loc,
        }
    }
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
        Self {
            src: input,
            start: 0.into(),
            cur: 0.into(),
            line: 1.into(),
            col: 1.into(),
        }
    }

    pub fn next(&self) -> Result<Token<'src>, LexError> {
        self.skip_whitespace();

        let c = match self.advance() {
            Some(c) => c,
            None => return Ok(Token::Eof(self.loc())),
        };

        match c {
            c @ _ if Self::is_digit(c) => Ok(self.next_number()),
            c @ _ if Self::is_alpha(c) => Ok(self.next_identifier()),
            "+" => Ok(self.make_operator(Operator::Add)),
            "-" => Ok(self.make_operator(Operator::Sub)),
            "*" => Ok(self.make_operator(Operator::Mul)),
            "/" => Ok(self.make_operator(Operator::Div)),
            "%" => Ok(self.make_operator(Operator::Mod)),
            "(" => Ok(self.make_separator(Separator::LeftParen)),
            ")" => Ok(self.make_separator(Separator::RightParen)),
            ";" => Ok(self.make_separator(Separator::Semicolon)),
            _ => Err(LexError::InvalidCharacter(&c, self.loc())),
        }
    }

    fn next_number(&self) -> Token<'src> {
        self.eat_digits();

        if let Some(".") = self.peek() {
            self.advance();
            self.eat_digits();
        }

        Token::Number(
            self.src[self.start.get()..self.cur.get()].parse().unwrap(),
            self.loc(),
        )
    }

    fn eat_digits(&self) {
        while let Some(c) = self.peek() {
            if Self::is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_identifier(&self) -> Token<'src> {
        while let Some(c) = self.peek() {
            if Self::is_alphanumeric(c) {
                self.advance();
            } else {
                break;
            }
        }

        let value = &self.src[self.start.get()..self.cur.get()];

        match value {
            "is" => self.make_keyword(Keyword::Is),
            "negative" => self.make_keyword(Keyword::Negative),
            _ => Token::Identifier(&self.src[self.start.get()..self.cur.get()], self.loc()),
        }
    }

    fn make_operator(&self, op: Operator) -> Token<'src> {
        Token::Operator(op, self.loc())
    }

    fn make_separator(&self, sep: Separator) -> Token<'src> {
        Token::Separator(sep, self.loc())
    }

    fn make_keyword(&self, keyword: Keyword) -> Token<'src> {
        Token::Keyword(keyword, self.loc())
    }

    fn loc(&self) -> Location {
        let col = self.col.get();
        let len = self.cur.get() - self.start.get();

        Location::new(self.line.get(), if col < len { 0 } else { col - len })
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

    fn is_alpha(value: &'src str) -> bool {
        match value {
            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n"
            | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B"
            | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P"
            | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "_" => true,
            _ => false,
        }
    }

    fn is_alphanumeric(value: &'src str) -> bool {
        Self::is_digit(value) || Self::is_alpha(value) || value == "'"
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
