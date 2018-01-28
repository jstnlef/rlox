use scanner::{Literal, Token};

pub struct AST {
    pub root: Vec<Box<Stmt>>,
}

pub enum Expr {
    Assign(Token, Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(Token),
}

pub trait ExprVisitor<E> {
    fn visit_expr(&mut self, expr: &Box<Expr>) -> E;
}

pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    Expression(Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    Print(Box<Expr>),
    Var(Token, Box<Expr>),
    While(Box<Expr>, Box<Stmt>),
}

pub trait StmtVisitor<E> {
    fn visit_stmt(&mut self, stmt: &Box<Stmt>) -> E;
}
