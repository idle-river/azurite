use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    Identifier(String),
    Boolean(bool),
    Number(f64),
    Null,
}

impl Display for RuntimeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVal::Number(num) => write!(f, "{}", num),
            RuntimeVal::Null => write!(f, "null"),
            RuntimeVal::Boolean(bool) => write!(f, "{}", bool),
            RuntimeVal::Identifier(str) => write!(f, "{}", str),
        }
    }
}
