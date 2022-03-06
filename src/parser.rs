use crate::lexer::Lexer;

pub struct Parser<'src> {
    lexer: Lexer<'src>,
}
