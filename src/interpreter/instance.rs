use crate::interpreter::{class::KyroClass, value::Value};
use std::collections::HashMap;
use std::rc::Rc;

pub struct KyroInstance {
    pub class: Rc<KyroClass>,
    pub fields: HashMap<String, Value>,
}
