use ast::Stmt;
use std::collections::HashMap;
use std::fmt;

use crate::Scope;

pub type FunctionCall = fn(args: Vec<RuntimeVal>, env: &mut Scope) -> RuntimeVal;

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    Object(HashMap<String, Box<RuntimeVal>>),
    NativeFn(FunctionCall),
    FloatingNumber(f64),
    Boolean(bool),
    Integer(i32),
    Function {
        name: String,
        parameters: Vec<String>,
        dec_env: &mut Scope,
        body: Vec<Stmt>,
    },
    Null,
}

impl fmt::Display for RuntimeVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeVal::Integer(i) => write!(f, "{i}"),
            // `{:?}` keeps the trailing ".0" like Python (42.0, not 42)
            RuntimeVal::FloatingNumber(n) if n.is_nan() => write!(f, "nan"),
            RuntimeVal::FloatingNumber(n) => write!(f, "{n:?}"),
            RuntimeVal::Boolean(true) => write!(f, "true"),
            RuntimeVal::Boolean(false) => write!(f, "false"),
            RuntimeVal::Null => write!(f, "null"),
            RuntimeVal::Function { name, .. } => write!(f, "<user-defined function: {}>", name),
            RuntimeVal::NativeFn(_) => write!(f, "<built-in function>"),
            RuntimeVal::Object(map) => {
                write!(f, "{{")?;
                for (i, (key, value)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "'{key}': {value}")?;
                }
                write!(f, "}}")
            }
        }
    }
}
