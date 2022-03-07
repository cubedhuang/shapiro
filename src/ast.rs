use crate::token::Token;

#[derive(Debug)]
pub enum Expr<'src> {
    Literal {
        token: Token<'src>,
        value: f64,
    },
    Grouping {
        paren: Token<'src>,
        expr: Box<Expr<'src>>,
    },
    Unary {
        op: Token<'src>,
        right: Box<Expr<'src>>,
    },
    Binary {
        left: Box<Expr<'src>>,
        op: Token<'src>,
        right: Box<Expr<'src>>,
    },
}

#[derive(Debug)]
pub enum Stmt<'src> {
    Expr(Expr<'src>),
}
