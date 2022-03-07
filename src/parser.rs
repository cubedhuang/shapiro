use std::cell::{Cell, RefCell};

use crate::{
    ast::*,
    lexer::{LexError, Lexer},
    token::{Location, Operator::*, Separator, Token},
};

#[derive(Debug)]
pub enum ParseError<'src> {
    LexError(LexError<'src>),
    UnexpectedToken(Token<'src>),
    Wtf,
}

impl<'src> std::fmt::Display for ParseError<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::LexError(err) => err.fmt(f),
            ParseError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: '{}'.", token)
            }
            ParseError::Wtf => write!(f, "Wtf"),
        }
    }
}

impl<'src> ParseError<'src> {
    pub fn loc(&self) -> Location {
        match self {
            ParseError::LexError(err) => err.loc(),
            ParseError::UnexpectedToken(token) => *token.loc(),
            ParseError::Wtf => Location { line: 0, col: 0 },
        }
    }
}

type ParseResult<'src, T> = Result<T, ParseError<'src>>;

impl<'src> From<LexError<'src>> for ParseError<'src> {
    fn from(e: LexError<'src>) -> Self {
        ParseError::LexError(e)
    }
}

#[derive(Debug)]
pub struct Parser<'src> {
    lexer: Lexer<'src>,
    pos: Cell<usize>,
    tokens: RefCell<Vec<Token<'src>>>,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            lexer: Lexer::new(input),
            pos: 0.into(),
            tokens: vec![].into(),
        }
    }

    pub fn parse(&self) -> ParseResult<Vec<Stmt>> {
        self.program()
    }

    fn program(&self) -> ParseResult<Vec<Stmt>> {
        let mut stmts = vec![];

        while !matches!(self.peek()?, Token::Eof(_)) {
            stmts.push(self.stmt()?);
        }

        Ok(stmts)
    }

    fn stmt(&self) -> ParseResult<Stmt> {
        let expr = self.expr()?;

        if let Token::Separator(Separator::Semicolon, _) = self.next()? {
            return Ok(Stmt::Expr(expr));
        }

        Err(ParseError::UnexpectedToken(self.peek()?))
    }

    fn expr(&self) -> ParseResult<Expr> {
        let mut left = self.term()?;

        while let op @ Token::Operator(Add | Sub, _) = self.peek()? {
            self.next()?;

            left = Expr::Binary {
                left: left.into(),
                op,
                right: self.term()?.into(),
            };
        }

        Ok(left)
    }

    fn term(&self) -> ParseResult<Expr> {
        let mut left = self.factor()?;

        while let op @ Token::Operator(Mul | Div | Mod, _) = self.peek()? {
            self.next()?;

            left = Expr::Binary {
                left: left.into(),
                op,
                right: self.factor()?.into(),
            };
        }

        Ok(left)
    }

    fn factor(&self) -> ParseResult<Expr> {
        let op = match self.peek()? {
            op @ Token::Operator(Sub, _) => op,
            _ => return self.atom(),
        };

        self.next()?;

        Ok(Expr::Unary {
            op,
            right: self.factor()?.into(),
        })
    }

    fn atom(&self) -> ParseResult<Expr> {
        match self.next()? {
            ref token @ Token::Number(value, _) => Ok(Expr::Literal {
                token: token.clone(),
                value,
            }),
            ref token @ Token::Separator(Separator::LeftParen, _) => {
                let expr = self.expr()?;

                if let Token::Separator(Separator::RightParen, _) = self.next()? {
                    return Ok(Expr::Grouping {
                        paren: token.clone(),
                        expr: Box::new(expr),
                    });
                }

                Err(ParseError::UnexpectedToken(self.peek()?))
            }
            _ => Err(ParseError::UnexpectedToken(self.prev()?)),
        }
    }

    fn next(&self) -> ParseResult<Token<'src>> {
        let token = self.peek();

        self.pos.set(self.pos.get() + 1);

        token
    }

    fn prev(&self) -> ParseResult<Token<'src>> {
        let pos = self.pos.get();

        if pos == 0 {
            return Err(ParseError::Wtf);
        }

        self.pos.set(pos - 1);

        self.peek()
    }

    fn peek(&self) -> ParseResult<Token<'src>> {
        if self.tokens.borrow().len() <= self.pos.get() {
            let mut tokens = self.tokens.borrow_mut();

            tokens.push(self.lexer.next()?);
        }

        self.tokens
            .borrow()
            .get(self.pos.get())
            .cloned()
            .ok_or(ParseError::Wtf)
    }
}
