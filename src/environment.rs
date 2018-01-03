use std::collections::HashMap;
use scanner::{Literal, Token};
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

    pub fn assign(&mut self, name: &Token, value: &Literal) -> RuntimeResult<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_owned(), value.clone());
            Ok(())
        } else {
            Err(RuntimeError::new(
                name,
                &format!("Undefined variable '{}'.", name.lexeme)
            ))
        }
    }

    pub fn get(&self, name: &Token) -> RuntimeResult<Literal> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => Err(RuntimeError::new(
                name,
                &format!("Undefined variable '{}'.", name.lexeme),
            )),
        }
    }
}
