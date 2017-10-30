use parser::expr::{Expr, Visitor, AST};
use scanner::{Literal, Token, TokenType};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&mut self, ast: &AST) {
        let val = self.evaluate(&ast.root);
        match val {
            Ok(literal) => println!("{}", literal),
            Err(err) => {}
        }
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> RuntimeResult<Literal> {
        self.visit_expr(expr)
    }
}

impl Visitor<RuntimeResult<Literal>> for Interpreter {
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
                    _ => Err(RuntimeError::new(token, "Unrecognized token for Binary operation.")),
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
                    _ => Err(RuntimeError::new(token, "Unrecognized token for Unary operation.")),
                }

            }

            Expr::Grouping(ref e) => self.evaluate(e),
        }
    }
}

type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    fn new(token: &Token, message: &str) -> Self {
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

fn get_number_operands(left: &Literal,
                       right: &Literal,
                       token: &Token)
                       -> RuntimeResult<(f64, f64)> {
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
        _ => Err(RuntimeError::new(token, "Operands must be two numbers or two strings.")),
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
