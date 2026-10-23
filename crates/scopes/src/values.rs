use ast::Stmt;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::Env;

pub type FunctionCall = fn(args: Vec<RuntimeVal>, env: &Env) -> RuntimeVal;

pub struct Closure {
    pub name: String,
    pub parameters: Vec<String>,
    pub dec_env: Env,
    pub body: Vec<Box<Stmt>>,
}

impl fmt::Debug for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Deliberately skips `dec_env` (and `body`): see the type docs.
        f.debug_struct("Closure")
            .field("name", &self.name)
            .field("parameters", &self.parameters)
            .finish_non_exhaustive()
    }
}

/// Two closures are equal only if they are the same function object.
impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    Object(HashMap<String, Box<RuntimeVal>>),
    NativeFn(FunctionCall),
    FloatingNumber(f64),
    Boolean(bool),
    Integer(i32),
    Function(Rc<Closure>),
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
            RuntimeVal::Function(func) => write!(f, "<user-defined function: {}>", func.name),
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
