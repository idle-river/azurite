mod values;

use std::collections::HashMap;
pub use values::RuntimeVal;

pub struct Scope {
    parent: Option<Box<Scope>>,
    variables: HashMap<String, RuntimeVal>,
}

#[derive(Debug)]
pub enum ScopeError {
    VariableAlreadyDeclared {
        variable_name: String,
        declared_value: RuntimeVal,
    },
    VariableInsertError(String),
}

impl std::fmt::Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeError::VariableAlreadyDeclared {
                variable_name,
                declared_value,
            } => write!(
                f,
                "{} has already been declared; value is declared as {:?}",
                variable_name, declared_value
            ),
            ScopeError::VariableInsertError(v) => write!(f, "error inserting variable {v}"),
        }
    }
}

impl std::error::Error for ScopeError {}

impl Scope {
    pub fn new(parent: Box<Scope>) -> Self {
        Self {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn global() -> Self {
        Self {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn declare_var(
        &mut self,
        name: String,
        value: RuntimeVal,
    ) -> Result<RuntimeVal, ScopeError> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(value.clone());
                Ok(value)
            }
            Entry::Occupied(entry) => Err(ScopeError::VariableAlreadyDeclared {
                variable_name: entry.key().clone(),
                declared_value: entry.get().clone(),
            }),
        }
    }
}
