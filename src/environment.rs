use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use scanner::{Literal, Token};
use interpreter::{RuntimeError, RuntimeResult};

pub struct Environment {
    enclosing: Option<Rc<Environment>>,
    values: RefCell<HashMap<String, Literal>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_enclosed(enclosing: Rc<Self>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn define(&self, name: &str, value: &Literal) {
        self.values.borrow_mut().insert(name.to_owned(), value.clone());
    }

    pub fn assign(&self, name: &Token, value: &Literal) -> RuntimeResult<()> {
        let mut values = self.values.borrow_mut();

        if values.contains_key(&name.lexeme) {
            values.insert(name.lexeme.to_owned(), value.clone());
            Ok(())
        } else {
            if let Some(ref enclosing_env) = self.enclosing {
                enclosing_env.assign(name, value)
            } else {
                Err(RuntimeError::new(
                    name,
                    &format!("Undefined variable '{}'.", name.lexeme),
                ))
            }
        }
    }

    pub fn get(&self, name: &Token) -> RuntimeResult<Literal> {
        match self.values.borrow().get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => {
                if let Some(ref env) = self.enclosing {
                    env.get(name)
                } else {
                    Err(RuntimeError::new(
                        name,
                        &format!("Undefined variable '{}'.", name.lexeme),
                    ))
                }
            }
        }
    }
}
