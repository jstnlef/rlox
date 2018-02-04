use std::cmp::PartialEq;
use std::fmt;
use std::rc::Rc;

use interpreter::{Interpreter, RuntimeResult};
use scanner::Literal;

#[derive(Clone)]
pub enum LoxObject {
    Function(Rc<Callable>),
    Literal(Literal)
}

impl LoxObject {
    pub fn is_truthy(&self) -> bool {
        match self {
            &LoxObject::Literal(ref literal) => match *literal {
                Literal::Nil => false,
                Literal::Boolean(b) => b,
                _ => true,
            }
            _ => false
        }
    }
}

impl PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&LoxObject::Literal(ref lhs), &LoxObject::Literal(ref rhs)) => lhs == rhs,
            _ => false
        }
    }
}

impl fmt::Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LoxObject::Literal(ref literal) => write!(f, "{}", literal),
            &LoxObject::Function(_) => write!(f, "<function>")
        }
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[LoxObject]) -> RuntimeResult<LoxObject>;
}
