use std::{error::Error, fmt::Display};

use crate::RuntimeVal;
use ast::Stmt;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    NoNodeInterpretation(Stmt),
    DivideByZero(RuntimeVal), // assuming rhs is zero
}

impl Error for InterpreterError {}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InterpreterError::*;
        match self {
            NoNodeInterpretation(s) => write!(
                f,
                "This AST node: {:#?} does not have interpretation set up yet.",
                s
            ),
            DivideByZero(lhs) => write!(f, "Cannot divide {} / 0", lhs),
        }
    }
}
