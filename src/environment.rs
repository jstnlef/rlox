use std::collections::HashMap;
use scanner::{Token, Literal};
use interpreter::{RuntimeError, RuntimeResult};

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, name: &str, value: &Literal) {
        self.values.insert(name.to_owned(), value.clone());
    }

    pub fn get(&self, name: &Token) -> RuntimeResult<Literal> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => Err(RuntimeError::new(name, &format!("Undefined variable '{}'.", name.lexeme))),
        }
    }
}
