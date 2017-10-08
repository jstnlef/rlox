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

pub trait Visitor<E> {
    fn visit_expr(&mut self, expr: &Box<Expr>) -> E;
}
