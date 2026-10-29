use std::{error::Error, fmt::Display};

use crate::RuntimeVal;

#[derive(Debug, Clone)]
pub enum ScopeError {
    VariableAlreadyDeclared(String, RuntimeVal),
    VariableDoesNotExist(String),
    ReassigningConstant(String),
}

impl Error for ScopeError {}

impl Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ScopeError::*;
        match self {
            VariableAlreadyDeclared(ident, old_value) => {
                write!(f, "{} has already been declared: {}", ident, old_value)
            }
            ReassigningConstant(val) => write!(f, "Cannot reassign constant: {val}"),
            VariableDoesNotExist(ident) => {
                write!(f, "variable {} does not exist", ident)
            }
        }
    }
}
