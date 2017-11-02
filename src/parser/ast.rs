use scanner::{Literal, Token};

pub struct AST {
    pub root: Vec<Box<Stmt>>,
}

pub enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(Token),
}

pub trait ExprVisitor<E> {
    fn visit_expr(&mut self, expr: &Box<Expr>) -> E;
}

pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Box<Expr>),
}

pub trait StmtVisitor<E> {
    fn visit_stmt(&mut self, stmt: &Box<Stmt>) -> E;
}
