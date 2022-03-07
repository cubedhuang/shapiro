use crate::{
    ast::{Expr, Stmt},
    token::{Location, Operator, Token},
};

#[derive(Debug)]
pub enum WalkError {
    Wtf(Location),
}

impl std::fmt::Display for WalkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkError::Wtf(_) => write!(f, "Wtf"),
        }
    }
}

impl WalkError {
    pub fn loc(&self) -> Location {
        match self {
            WalkError::Wtf(loc) => *loc,
        }
    }
}

type WalkResult<T> = Result<T, WalkError>;

pub struct Walker {}

impl Walker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&self, stmt: Stmt) -> WalkResult<()> {
        match stmt {
            Stmt::Expr(expr) => {
                dbg!(self.eval_expr(expr)?);
                Ok(())
            }
        }
    }

    pub fn eval_expr(&self, expr: Expr) -> WalkResult<f64> {
        match expr {
            Expr::Literal { value, .. } => Ok(value),
            Expr::Grouping { expr, .. } => Ok(self.eval_expr(*expr)?),
            Expr::Unary { right, .. } => Ok(-self.eval_expr(*right)?),
            Expr::Binary { left, op, right } => {
                let op = match op {
                    Token::Operator(op, _) => op,
                    token => return Err(WalkError::Wtf(*token.loc())),
                };

                let left = self.eval_expr(*left)?;
                let right = self.eval_expr(*right)?;

                match op {
                    Operator::Add => Ok(left + right),
                    Operator::Sub => Ok(left - right),
                    Operator::Mul => Ok(left * right),
                    Operator::Div => Ok(left / right),
                    Operator::Mod => Ok(left % right),
                }
            }
        }
    }
}
