use crate::interpreter::function::KyroFunction;
use std::{collections::HashMap, rc::Rc};

pub struct KyroClass {
    pub name: String,
    pub superclass: Option<Rc<KyroClass>>,
    pub methods: HashMap<String, KyroFunction>,
    pub doc: Option<String>,
}

impl KyroClass {
    pub fn find_method(&self, name: &str) -> Option<KyroFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(superclass) = &self.superclass {
            return superclass.find_method(name);
        }

        None
    }
}
