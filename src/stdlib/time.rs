use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::interpreter::{
    callable::KyroCallable, class::KyroClass, instance::KyroInstance, interpreter::Interpreter,
    runtime_error::RuntimeError, value::Value,
};

pub fn get_module() -> Value {
    let class = Rc::new(KyroClass {
        name: "time".to_string(),
        superclass: None,
        methods: HashMap::new(),
        doc: None,
    });
    let mut fields = HashMap::new();
    fields.insert(
        "__name__".to_string(),
        Value::String("std:time".to_string()),
    );
    fields.insert("clock".to_string(), Value::Callable(Rc::new(Clock)));
    fields.insert("now".to_string(), Value::Callable(Rc::new(Now)));
    fields.insert("format".to_string(), Value::Callable(Rc::new(Format)));
    fields.insert("sleep".to_string(), Value::Callable(Rc::new(Sleep)));

    let instance = KyroInstance { class, fields };
    Value::Instance(Rc::new(RefCell::new(instance)))
}

pub struct Clock;

impl KyroCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Ok(Value::Number(now.as_secs_f64()))
    }

    fn name(&self) -> &str {
        "clock"
    }
}

pub struct Now;

impl KyroCallable for Now {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let dt = get_datetime_components(timestamp);

        let mut map = HashMap::new();
        map.insert("year".to_string(), Value::Number(dt.year as f64));
        map.insert("month".to_string(), Value::Number(dt.month as f64));
        map.insert("day".to_string(), Value::Number(dt.day as f64));
        map.insert("hour".to_string(), Value::Number(dt.hour as f64));
        map.insert("minute".to_string(), Value::Number(dt.minute as f64));
        map.insert("second".to_string(), Value::Number(dt.second as f64));

        Ok(Value::Dict(Rc::new(RefCell::new(map))))
    }

    fn name(&self) -> &str {
        "now"
    }
}

pub struct Format;

impl KyroCallable for Format {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let timestamp = match arguments[0] {
            Value::Number(n) => n as u64,
            _ => {
                return Err(RuntimeError::new(
                    crate::parser::tokens::Token::new(
                        crate::parser::tokens::TokenType::Identifier,
                        "format".to_string(),
                        None,
                        0,
                    ),
                    "First argument to format() must be a numeric timestamp.",
                ));
            }
        };

        let format_str = match &arguments[1] {
            Value::String(s) => s,
            _ => {
                return Err(RuntimeError::new(
                    crate::parser::tokens::Token::new(
                        crate::parser::tokens::TokenType::Identifier,
                        "format".to_string(),
                        None,
                        0,
                    ),
                    "Second argument to format() must be a format string.",
                ));
            }
        };

        let dt = get_datetime_components(timestamp);

        let mut formatted = format_str.clone();
        formatted = formatted.replace("%Y", &format!("{:04}", dt.year));
        formatted = formatted.replace("%m", &format!("{:02}", dt.month));
        formatted = formatted.replace("%d", &format!("{:02}", dt.day));
        formatted = formatted.replace("%H", &format!("{:02}", dt.hour));
        formatted = formatted.replace("%M", &format!("{:02}", dt.minute));
        formatted = formatted.replace("%S", &format!("{:02}", dt.second));

        Ok(Value::String(formatted))
    }

    fn name(&self) -> &str {
        "format"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["timestamp".to_string(), "format".to_string()]
    }

    fn default_value(
        &self,
        _interpreter: &mut Interpreter,
        param_name: &str,
    ) -> Option<Result<Value, RuntimeError>> {
        if param_name == "timestamp" {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            Some(Ok(Value::Number(now)))
        } else if param_name == "format" {
            Some(Ok(Value::String("%Y-%m-%d %H:%M:%S".to_string())))
        } else {
            None
        }
    }
}

pub struct Sleep;

impl KyroCallable for Sleep {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let ms = match arguments[0] {
            Value::Number(n) => n,
            _ => {
                return Err(RuntimeError::new(
                    crate::parser::tokens::Token::new(
                        crate::parser::tokens::TokenType::Identifier,
                        "sleep".to_string(),
                        None,
                        0,
                    ),
                    "Argument to sleep() must be a number of milliseconds.",
                ));
            }
        };

        if ms < 0.0 {
            return Err(RuntimeError::new(
                crate::parser::tokens::Token::new(
                    crate::parser::tokens::TokenType::Identifier,
                    "sleep".to_string(),
                    None,
                    0,
                ),
                "Sleep duration cannot be negative.",
            ));
        }

        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
        Ok(Value::Nil)
    }

    fn name(&self) -> &str {
        "sleep"
    }

    fn parameter_names(&self) -> Vec<String> {
        vec!["ms".to_string()]
    }
}

struct DateTimeComponents {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

fn get_datetime_components(timestamp: u64) -> DateTimeComponents {
    let remaining_seconds = (timestamp % 86400) as u32;
    let hour = remaining_seconds / 3600;
    let minute = (remaining_seconds % 3600) / 60;
    let second = remaining_seconds % 60;

    let days = (timestamp / 86400) as i64;
    let (year, month, day) = civil_from_days(days);

    DateTimeComponents {
        year,
        month,
        day,
        hour,
        minute,
        second,
    }
}

fn civil_from_days(mut days: i64) -> (i32, u32, u32) {
    days += 719468;
    let era = (if days >= 0 { days } else { days - 146096 }) / 146097;
    let doe = (days - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146097) / 365;
    let y = (yoe as i32) + (era as i32) * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = y + (if m <= 2 { 1 } else { 0 });
    (year, m, d)
}
