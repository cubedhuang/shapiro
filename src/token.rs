#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Is,
    Negative,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Is => write!(f, "is"),
            Keyword::Negative => write!(f, "negative"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Separator {
    LeftParen,
    RightParen,
    Semicolon,
}

impl std::fmt::Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Separator::LeftParen => write!(f, "("),
            Separator::RightParen => write!(f, ")"),
            Separator::Semicolon => write!(f, ";"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Mod => write!(f, "%"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token<'src> {
    Number(f64, Location),
    Identifier(&'src str, Location),

    Keyword(Keyword, Location),
    Separator(Separator, Location),
    Operator(Operator, Location),

    Eof(Location),
}

impl<'src> std::fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n, _) => write!(f, "{}", n),
            Token::Identifier(s, _) => write!(f, "{}", s),
            Token::Keyword(k, _) => write!(f, "{}", k),
            Token::Separator(s, _) => write!(f, "{}", s),
            Token::Operator(s, _) => write!(f, "{}", s),
            Token::Eof(_) => write!(f, "EOF"),
        }
    }
}

impl<'src> Token<'src> {
    pub fn loc(&self) -> &Location {
        match self {
            Token::Number(_, loc) => loc,
            Token::Identifier(_, loc) => loc,
            Token::Keyword(_, loc) => loc,
            Token::Separator(_, loc) => loc,
            Token::Operator(_, loc) => loc,
            Token::Eof(loc) => loc,
        }
    }
}
