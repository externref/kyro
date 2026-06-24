use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
    runtime_error::RuntimeError, value::Value,
};

pub fn get_module() -> Value {
    let class = Rc::new(KyroClass {
        name: "os".to_string(),
        superclass: None,
        methods: HashMap::new(),
    });
    let mut fields = HashMap::new();
    fields.insert("args".to_string(), Value::Callable(Rc::new(ArgsFn)));

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}

pub struct ArgsFn;

impl KyroCallable for ArgsFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let args: Vec<Value> = std::env::args().map(|s| Value::String(s)).collect();
        Ok(Value::List(Rc::new(RefCell::new(args))))
    }

    fn name(&self) -> &str {
        "args"
    }
}
