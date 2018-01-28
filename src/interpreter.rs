use std::rc::Rc;

use environment::Environment;
use parser::ast::{Expr, ExprVisitor, Stmt, StmtVisitor, AST};
use scanner::{Literal, Token, TokenType};

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(Environment::new())
        }
    }

    pub fn interpret(&mut self, ast: &AST) -> RuntimeResult<()> {
        for statement in ast.root.iter() {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Box<Stmt>) -> RuntimeResult<()> {
        self.visit_stmt(stmt)
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> RuntimeResult<Literal> {
        self.visit_expr(expr)
    }

    fn execute_block(&mut self, statements: &[Box<Stmt>], env: Environment) -> RuntimeResult<()> {
        let previous = Rc::clone(&self.environment);
        self.environment = Rc::new(env);
        for statement in statements {
            self.execute(statement)?;
        }
        self.environment = previous;
        Ok(())
    }
}

impl StmtVisitor<RuntimeResult<()>> for Interpreter {
    fn visit_stmt(&mut self, stmt: &Box<Stmt>) -> RuntimeResult<()> {
        match **stmt {
            Stmt::Block(ref statements) => {
                let enclosed_env = Environment::new_enclosed(Rc::clone(&self.environment));
                self.execute_block(statements, enclosed_env)?;
                Ok(())
            }
            Stmt::Expression(ref expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::If(ref condition, ref then_clause, ref maybe_else_clause) => {
                if is_truthy(self.evaluate(condition)?) {
                    self.execute(then_clause)?
                } else if let Some(ref else_clause) = *maybe_else_clause {
                    self.execute(else_clause)?
                }
                Ok(())
            }
            Stmt::Print(ref expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var(ref name, ref initializer) => {
                let value = self.evaluate(initializer)?;
                Rc::clone(&self.environment).define(&name.lexeme, &value);
                Ok(())
            }
        }
    }
}

impl ExprVisitor<RuntimeResult<Literal>> for Interpreter {
    fn visit_expr(&mut self, expr: &Box<Expr>) -> RuntimeResult<Literal> {
        match **expr {
            Expr::Literal(ref literal) => Ok(literal.clone()),

            Expr::Binary(ref lhs, ref token, ref rhs) => {
                let left = self.evaluate(lhs)?;
                let right = self.evaluate(rhs)?;

                match token.token_type {
                    TokenType::MINUS => minus(&left, &right, &token),
                    TokenType::SLASH => slash(&left, &right, &token),
                    TokenType::STAR => star(&left, &right, &token),
                    TokenType::PLUS => plus(&left, &right, &token),
                    TokenType::GREATER => greater(&left, &right, &token),
                    TokenType::GREATER_EQUAL => greater_equal(&left, &right, &token),
                    TokenType::LESS => less(&left, &right, &token),
                    TokenType::LESS_EQUAL => less_equal(&left, &right, &token),
                    TokenType::BANG_EQUAL => Ok(Literal::Boolean(!is_equal(&left, &right))),
                    TokenType::EQUAL_EQUAL => Ok(Literal::Boolean(is_equal(&left, &right))),
                    _ => Err(RuntimeError::new(
                        token,
                        "Unrecognized token for Binary operation.",
                    )),
                }
            }

            Expr::Unary(ref token, ref e) => {
                let right = self.evaluate(e)?;
                match token.token_type {
                    TokenType::BANG => Ok(Literal::Boolean(!is_truthy(right))),
                    TokenType::MINUS => {
                        match right {
                            Literal::Number(n) => Ok(Literal::Number(-n)),
                            _ => Err(RuntimeError::new(token, "Operand must be a number.")),
                        }
                    }
                    _ => Err(RuntimeError::new(
                        token,
                        "Unrecognized token for Unary operation.",
                    )),
                }
            }

            Expr::Grouping(ref e) => self.evaluate(e),

            Expr::Variable(ref name) => self.environment.get(name),

            Expr::Assign(ref name, ref value) => {
                let value = self.evaluate(value)?;
                self.environment.assign(name, &value)?;
                Ok(value)
            }
        }
    }
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> Self {
        RuntimeError {
            token: token.clone(),
            message: message.to_owned(),
        }
    }
}

fn is_truthy(literal: Literal) -> bool {
    match literal {
        Literal::Nil => false,
        Literal::Boolean(b) => b,
        _ => true,
    }
}

fn is_equal(left: &Literal, right: &Literal) -> bool {
    left == right
}

fn get_number_operands(
    left: &Literal,
    right: &Literal,
    token: &Token,
) -> RuntimeResult<(f64, f64)> {
    match (left, right) {
        (&Literal::Number(l), &Literal::Number(r)) => Ok((l, r)),
        _ => Err(RuntimeError::new(token, "Operands must be numbers.")),
    }
}


fn minus(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Number(l - r))
}

fn slash(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    let (l, r) = get_number_operands(left, right, token)?;
    if r == 0.0 {
        return Err(RuntimeError::new(token, "Divide by zero error."));
    }
    Ok(Literal::Number(l / r))
}

fn star(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Number(l * r))
}

fn plus(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    match (left, right) {
        (&Literal::Number(l), &Literal::Number(r)) => Ok(Literal::Number(l + r)),
        (&Literal::String(ref l), &Literal::String(ref r)) => {
            Ok(Literal::String(format!("{}{}", l, r)))
        }
        _ => Err(RuntimeError::new(
            token,
            "Operands must be two numbers or two strings.",
        )),
    }
}

fn greater(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Boolean(l > r))
}

fn greater_equal(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Boolean(l >= r))
}

fn less(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Boolean(l < r))
}

fn less_equal(left: &Literal, right: &Literal, token: &Token) -> RuntimeResult<Literal> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Boolean(l <= r))
}
