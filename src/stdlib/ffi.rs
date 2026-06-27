use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
    runtime_error::RuntimeError, value::Value,
};
use crate::parser::tokens::{Token, TokenType};

pub fn get_module() -> Value {
    let class = Rc::new(KyroClass {
        name: "ffi".to_string(),
        superclass: None,
        methods: HashMap::new(),
        doc: Some("Foreign Function Interface for calling C libraries.".to_string()),
    });
    let mut fields = HashMap::new();
    fields.insert("__name__".to_string(), Value::String("std:ffi".to_string()));
    fields.insert("load".to_string(), Value::Callable(Rc::new(LoadFn)));

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}

fn dummy_token() -> Token {
    Token {
        r#type: TokenType::Identifier,
        lexeme: "native".to_string(),
        literal: None,
        line: 0,
    }
}

pub struct LoadFn;

impl KyroCallable for LoadFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let path = match &arguments[0] {
            Value::String(s) => s,
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Library path must be a string.",
                ));
            }
        };

        let lib = unsafe { libloading::Library::new(path) }.map_err(|e| {
            RuntimeError::new(
                dummy_token(),
                format!("Failed to load shared library '{path}': {e}"),
            )
        })?;

        let rc_lib = Rc::new(lib);

        let class = Rc::new(KyroClass {
            name: "FfiLibrary".to_string(),
            superclass: None,
            methods: HashMap::new(),
            doc: None,
        });

        let mut fields = HashMap::new();
        fields.insert(
            "bind".to_string(),
            Value::Callable(Rc::new(BindFn {
                library: rc_lib.clone(),
            })),
        );

        let instance = KyroInstance { class, fields };
        Ok(Value::Instance(Rc::new(RefCell::new(instance))))
    }

    fn name(&self) -> &str {
        "load"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["path".to_string()]
    }
}

pub struct BindFn {
    pub library: Rc<libloading::Library>,
}

impl KyroCallable for BindFn {
    fn arity(&self) -> usize {
        3
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let name = match &arguments[0] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Function name must be a string.",
                ));
            }
        };

        let return_type = match &arguments[1] {
            Value::String(s) => s.clone(),
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Return type must be a string.",
                ));
            }
        };

        let raw_param_types = match &arguments[2] {
            Value::List(list_ref) => list_ref.borrow(),
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    "Parameter types must be supplied as a list of strings.",
                ));
            }
        };

        let mut param_types = Vec::new();
        for val in raw_param_types.iter() {
            match val {
                Value::String(s) => param_types.push(s.clone()),
                _ => {
                    return Err(RuntimeError::new(
                        dummy_token(),
                        "All parameter types must be strings.",
                    ));
                }
            }
        }

        let symbol_ptr = unsafe {
            let sym: libloading::Symbol<unsafe extern "C" fn()> =
                self.library.get(name.as_bytes()).map_err(|e| {
                    RuntimeError::new(
                        dummy_token(),
                        format!("Failed to find C symbol '{name}': {e}"),
                    )
                })?;
            *sym as *const ()
        };

        Ok(Value::Callable(Rc::new(FfiFunction {
            _library: self.library.clone(),
            symbol_ptr,
            name,
            return_type,
            param_types,
        })))
    }

    fn name(&self) -> &str {
        "bind"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec![
            "name".to_string(),
            "return_type".to_string(),
            "param_types".to_string(),
        ]
    }
}

pub struct FfiFunction {
    pub _library: Rc<libloading::Library>,
    pub symbol_ptr: *const (),
    pub name: String,
    pub return_type: String,
    pub param_types: Vec<String>,
}

impl KyroCallable for FfiFunction {
    fn arity(&self) -> usize {
        self.param_types.len()
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        if arguments.len() != self.param_types.len() {
            return Err(RuntimeError::new(
                dummy_token(),
                format!(
                    "FFI function '{}' expected {} arguments but got {}.",
                    self.name,
                    self.param_types.len(),
                    arguments.len()
                ),
            ));
        }

        let mut ffi_params = Vec::new();
        for ty in &self.param_types {
            let ffi_ty = match ty.as_str() {
                "int" => libffi::middle::Type::i32(),
                "double" => libffi::middle::Type::f64(),
                "string" => libffi::middle::Type::pointer(),
                _ => {
                    return Err(RuntimeError::new(
                        dummy_token(),
                        format!(
                            "Unsupported FFI parameter type '{ty}' in function '{}'.",
                            self.name
                        ),
                    ));
                }
            };
            ffi_params.push(ffi_ty);
        }

        let ffi_ret = match self.return_type.as_str() {
            "void" | "nil" => libffi::middle::Type::void(),
            "int" => libffi::middle::Type::i32(),
            "double" => libffi::middle::Type::f64(),
            "string" => libffi::middle::Type::pointer(),
            _ => {
                return Err(RuntimeError::new(
                    dummy_token(),
                    format!(
                        "Unsupported FFI return type '{}' in function '{}'.",
                        self.return_type, self.name
                    ),
                ));
            }
        };

        let cif = libffi::middle::Cif::new(ffi_params, ffi_ret);

        let mut c_strings = Vec::with_capacity(self.param_types.len());
        let mut int_vals = Vec::with_capacity(self.param_types.len());
        let mut double_vals = Vec::with_capacity(self.param_types.len());
        let mut ptr_vals = Vec::with_capacity(self.param_types.len());

        for (i, val) in arguments.iter().enumerate() {
            let ty = &self.param_types[i];
            match ty.as_str() {
                "int" => {
                    let n = match val {
                        Value::Number(n) => *n as i32,
                        _ => {
                            return Err(RuntimeError::new(
                                dummy_token(),
                                "Expected number for C int.",
                            ));
                        }
                    };
                    int_vals.push(n);
                }
                "double" => {
                    let n = match val {
                        Value::Number(n) => *n,
                        _ => {
                            return Err(RuntimeError::new(
                                dummy_token(),
                                "Expected number for C double.",
                            ));
                        }
                    };
                    double_vals.push(n);
                }
                "string" => {
                    let s = match val {
                        Value::String(s) => s,
                        _ => {
                            return Err(RuntimeError::new(
                                dummy_token(),
                                "Expected string for C string.",
                            ));
                        }
                    };
                    let c_str = std::ffi::CString::new(s.clone()).unwrap();
                    c_strings.push(c_str);
                }
                _ => unreachable!(),
            }
        }

        for c_str in &c_strings {
            ptr_vals.push(c_str.as_ptr());
        }

        let mut ffi_args = Vec::with_capacity(self.param_types.len());
        let mut int_idx = 0;
        let mut double_idx = 0;
        let mut str_idx = 0;

        for ty in &self.param_types {
            match ty.as_str() {
                "int" => {
                    let arg = libffi::middle::Arg::new(&int_vals[int_idx]);
                    ffi_args.push(arg);
                    int_idx += 1;
                }
                "double" => {
                    let arg = libffi::middle::Arg::new(&double_vals[double_idx]);
                    ffi_args.push(arg);
                    double_idx += 1;
                }
                "string" => {
                    let arg = libffi::middle::Arg::new(&ptr_vals[str_idx]);
                    ffi_args.push(arg);
                    str_idx += 1;
                }
                _ => unreachable!(),
            }
        }

        let code_ptr = libffi::middle::CodePtr(self.symbol_ptr as *mut _);

        let result_value = unsafe {
            match self.return_type.as_str() {
                "void" | "nil" => {
                    cif.call::<()>(code_ptr, &ffi_args);
                    Value::Nil
                }
                "int" => {
                    let res = cif.call::<i32>(code_ptr, &ffi_args);
                    Value::Number(res as f64)
                }
                "double" => {
                    let res = cif.call::<f64>(code_ptr, &ffi_args);
                    Value::Number(res)
                }
                "string" => {
                    let res = cif.call::<*const std::ffi::c_char>(code_ptr, &ffi_args);
                    if res.is_null() {
                        Value::Nil
                    } else {
                        Value::String(std::ffi::CStr::from_ptr(res).to_string_lossy().into_owned())
                    }
                }
                _ => unreachable!(),
            }
        };

        Ok(result_value)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
