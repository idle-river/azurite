use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(f64),
    Boolean(bool),
    Null,
    Object(HashMap<String, Box<RuntimeValue>>),
}

impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Boolean(value) => write!(f, "{value}"),
            RuntimeValue::Number(value) => write!(f, "{value}"),
            RuntimeValue::Object(value) => write!(f, "{:#?}", value),
            RuntimeValue::Null => write!(f, "null"),
        }
    }
}
