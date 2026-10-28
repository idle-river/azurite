use std::{error::Error, fmt::Display};

use crate::RuntimeVal;

#[derive(Debug, Clone)]
pub enum ScopeError {
    VariableAlreadyDeclared(String, RuntimeVal),
}

impl Error for ScopeError {}

impl Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ScopeError::*;
        match self {
            VariableAlreadyDeclared(ident, old_value) => {
                write!(f, "{} has already been declared: {}", ident, old_value)
            }
        }
    }
}
