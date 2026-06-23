use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::value::Value;

pub type EnvRef = Rc<RefCell<Environment>>;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<EnvRef>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from_enclosing(enclosing: EnvRef) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(val) = self.values.get(name) {
            return Some(val.clone());
        }

        if let Some(parent) = &self.enclosing {
            return parent.borrow().get(name);
        }

        None
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return true;
        }

        if let Some(parent) = &self.enclosing {
            return parent.borrow_mut().assign(name, value);
        }

        false
    }
}
