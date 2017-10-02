use scanner::{Literal, Token};

pub struct AST {
    root: Box<Expr>,
}

pub enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
}
