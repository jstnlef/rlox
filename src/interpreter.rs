use std::rc::Rc;

use environment::Environment;
use parser::ast::{Expr, ExprVisitor, Stmt, StmtVisitor, AST};
use lox_object::{LoxObject, Callable};
use native_functions::Clock;
use scanner::{Literal, Token, TokenType};

pub struct Interpreter {
    globals: Rc<Environment>,
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(Environment::new());

        globals.define("clock", &LoxObject::Function(Rc::new(Clock)));

        Interpreter {
            globals: globals.clone(),
            environment: globals
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

    fn evaluate(&mut self, expr: &Box<Expr>) -> RuntimeResult<LoxObject> {
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
                if self.evaluate(condition)?.is_truthy() {
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

            Stmt::While(ref condition, ref body) => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?
                }
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

impl ExprVisitor<RuntimeResult<LoxObject>> for Interpreter {
    fn visit_expr(&mut self, expr: &Box<Expr>) -> RuntimeResult<LoxObject> {
        match **expr {
            Expr::Literal(ref literal) => Ok(literal.clone().to_lox_object()),

            Expr::Logical(ref lhs, ref token, ref rhs) => {
                let left = self.evaluate(lhs)?;

                if token.token_type == TokenType::OR {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                } else {
                    if !left.is_truthy() {
                        return Ok(left);
                    }
                }

                self.evaluate(rhs)
            }

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
                    TokenType::BANG_EQUAL => Ok(Literal::Boolean(!is_equal(&left, &right)).to_lox_object()),
                    TokenType::EQUAL_EQUAL => Ok(Literal::Boolean(is_equal(&left, &right)).to_lox_object()),
                    _ => Err(RuntimeError::new(
                        token,
                        "Unrecognized token for Binary operation.",
                    )),
                }
            }

            Expr::Call(ref callee, ref paren, ref arguments) => {
                let function = match self.evaluate(callee)? {
                    LoxObject::Function(c) => c,
                    _ => return Err(RuntimeError::new(paren, "Can only call functions and classes."))
                };

                if arguments.len() != function.arity() {
                    return Err(RuntimeError::new(
                        paren,
                        &format!("Expected {} arguments but got {}.", function.arity(), arguments.len())
                    ))
                }

                let mut evaluated_args = Vec::with_capacity(arguments.len());
                for argument in arguments.iter() {
                    evaluated_args.push(self.evaluate(argument)?);
                }

                function.call(self, &evaluated_args)
            }

            Expr::Unary(ref token, ref e) => {
                let right = self.evaluate(e)?;
                match token.token_type {
                    TokenType::BANG => Ok(Literal::Boolean(!right.is_truthy()).to_lox_object()),
                    TokenType::MINUS => {
                        match right {
                            LoxObject::Literal(Literal::Number(n)) => Ok(Literal::Number(-n).to_lox_object()),
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

fn is_equal(left: &LoxObject, right: &LoxObject) -> bool {
    left == right
}

fn get_number_operands(
    left: &LoxObject,
    right: &LoxObject,
    token: &Token,
) -> RuntimeResult<(f64, f64)> {
    match (left, right) {
        (&LoxObject::Literal(Literal::Number(l)), &LoxObject::Literal(Literal::Number(r))) => Ok((l, r)),
        _ => Err(RuntimeError::new(token, "Operands must be numbers.")),
    }
}


fn minus(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Number(l - r).to_lox_object())
}

fn slash(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    let (l, r) = get_number_operands(left, right, token)?;
    if r == 0.0 {
        return Err(RuntimeError::new(token, "Divide by zero error."));
    }
    Ok(Literal::Number(l / r).to_lox_object())
}

fn star(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(Literal::Number(l * r).to_lox_object())
}

fn plus(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    match (left, right) {
        (&LoxObject::Literal(Literal::Number(l)), &LoxObject::Literal(Literal::Number(r))) => Ok(Literal::Number(l + r).to_lox_object()),
        (&LoxObject::Literal(Literal::String(ref l)), &LoxObject::Literal(Literal::String(ref r))) => {
            Ok(Literal::String(format!("{}{}", l, r)).to_lox_object())
        }
        _ => Err(RuntimeError::new(
            token,
            "Operands must be two numbers or two strings.",
        )),
    }
}

fn greater(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(LoxObject::Literal(Literal::Boolean(l > r)))
}

fn greater_equal(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(LoxObject::Literal(Literal::Boolean(l >= r)))
}

fn less(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(LoxObject::Literal(Literal::Boolean(l < r)))
}

fn less_equal(left: &LoxObject, right: &LoxObject, token: &Token) -> RuntimeResult<LoxObject> {
    let (l, r) = get_number_operands(left, right, token)?;
    Ok(LoxObject::Literal(Literal::Boolean(l <= r)))
}
