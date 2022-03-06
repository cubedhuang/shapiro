#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum Token {
    Number(f64),

    Operation(Operator),

    Eof,
}
