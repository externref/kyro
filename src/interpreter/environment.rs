// MIT License

// Copyright (c) 2026 sarthak

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::value::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type EnvRef = Rc<RefCell<Environment>>;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<EnvRef>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
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
            let next = curr
                .borrow()
                .enclosing
                .clone()
                .expect("Environment ancestor out of bounds.");
            curr = next;
        }
        curr
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values
            .get(name)
            .cloned()
            .or_else(|| self.enclosing.as_ref()?.borrow().get(name))
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if let Some(val) = self.values.get_mut(name) {
            *val = value;
            true
        } else if let Some(parent) = &self.enclosing {
            parent.borrow_mut().assign(name, value)
        } else {
            false
        }
    }
}
