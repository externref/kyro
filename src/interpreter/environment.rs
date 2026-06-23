use super::value::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

    pub fn get_values(&self) -> HashMap<String, Value> {
        self.values.clone()
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

    pub fn get_at(env: EnvRef, distance: usize, name: &str) -> Option<Value> {
        let ancestor_env = Self::ancestor(env, distance);
        ancestor_env.borrow().values.get(name).cloned()
    }

    pub fn assign_at(env: EnvRef, distance: usize, name: &str, value: Value) {
        let ancestor_env = Self::ancestor(env, distance);
        ancestor_env
            .borrow_mut()
            .values
            .insert(name.to_string(), value);
    }

    fn ancestor(env: EnvRef, distance: usize) -> EnvRef {
        let mut curr = env;
        for _ in 0..distance {
            let next = {
                let borrowed = curr.borrow();
                borrowed
                    .enclosing
                    .clone()
                    .expect("Environment ancestor out of bounds.")
            };
            curr = next;
        }
        curr
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
