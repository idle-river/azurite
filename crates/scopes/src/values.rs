use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    Object(HashMap<String, Box<RuntimeVal>>),
    FloatingNumber(f64),
    Boolean(bool),
    Integer(i32),
    Null,
}
