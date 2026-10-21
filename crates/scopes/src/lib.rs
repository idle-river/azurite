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
    VariableDoesNotExist(String),
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
            ScopeError::VariableDoesNotExist(v) => write!(f, "variable {v} does not exist"),
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

    // Making a new variable
    pub fn declare_var(&mut self, name: String, value: RuntimeVal) -> Result<(), ScopeError> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Occupied(entry) => Err(ScopeError::VariableAlreadyDeclared {
                variable_name: entry.key().clone(),
                declared_value: entry.get().clone(),
            }),
        }
    }

    // Assigning to an already existing variable
    pub fn assign_var(&mut self, name: String, value: RuntimeVal) -> Result<(), ScopeError> {
        let old_env = self.resolve(&name)?;
        *old_env.variables.get_mut(&name).unwrap() = value;

        Ok(())
    }

    pub fn lookup_var(&mut self, name: String) -> Result<&RuntimeVal, ScopeError> {
        let old_env = self.resolve(&name)?;
        Ok(old_env.variables.get(&name).unwrap())
    }

    pub fn resolve(&mut self, name: &str) -> Result<&mut Scope, ScopeError> {
        let mut current = Some(self);

        while let Some(scope) = current {
            if let Some(_) = scope.variables.get(name) {
                return Ok(scope);
            }

            current = scope.parent.as_deref_mut();
        }

        Err(ScopeError::VariableDoesNotExist(name.to_string()))
    }
}

#[cfg(test)]
mod tests;
