use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum RuntimeVal {
    Number(f64),
    Null,
}

impl Display for RuntimeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVal::Number(num) => write!(f, "{}", num),
            RuntimeVal::Null => write!(f, "null"),
        }
    }
}
