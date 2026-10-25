use std::{error::Error, fmt::Display};

use ast::Stmt;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    NoNodeInterpretation(Stmt),
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
        }
    }
}
