use crate::interpreter::function::KyroFunction;
use std::collections::HashMap;
pub struct KyroClass {
    pub name: String,
    pub methods: HashMap<String, KyroFunction>,
}
