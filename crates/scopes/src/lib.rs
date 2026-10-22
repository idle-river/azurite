mod values;

use std::collections::{HashMap, HashSet};
pub use values::{FunctionCall, RuntimeVal};

#[derive(Debug, Clone, PartialEq)]
// Runtime Variables
pub struct Scope {
    parent: Option<Box<Scope>>,
    variables: HashMap<String, RuntimeVal>,
    constants: HashSet<String>,
}

#[derive(Debug)]
pub enum ScopeError {
    VariableAlreadyDeclared {
        variable_name: String,
        declared_value: RuntimeVal,
    },
    VariableDoesNotExist(String),
    VariableInsertError(String),
    ReassignConstantError(String),
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
            ScopeError::ReassignConstantError(c) => write!(f, "constant {c} cannot be reassigned"),
        }
    }
}

impl std::error::Error for ScopeError {}

impl Scope {
    pub fn new(parent: Box<Scope>) -> Self {
        Self {
            parent: Some(parent),
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }

    pub fn global() -> Self {
        let mut env = Self {
            parent: None,
            variables: HashMap::new(),
            constants: HashSet::new(),
        };

        // Runtime Variables
        declare_variable!(env, "true", RuntimeVal::Boolean(true), true);
        declare_variable!(env, "false", RuntimeVal::Boolean(false), true);
        declare_variable!(env, "null", RuntimeVal::Null, true);

        // Native Functions
        declare_native_fn!(env, "print", |args, _scope| {
            for arg in args {
                println!("{}", arg);
            }

            RuntimeVal::Null
        });

        env
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

    pub fn declare_const_var(&mut self, name: String, value: RuntimeVal) -> Result<(), ScopeError> {
        use std::collections::hash_map::Entry;

        match self.variables.entry(name.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(value);
                self.constants.insert(name);
                Ok(())
            }
            Entry::Occupied(entry) => Err(ScopeError::VariableAlreadyDeclared {
                variable_name: entry.key().clone(),
                declared_value: entry.get().clone(),
            }),
        }
    }

    // Making a new native (builtin) function
    pub fn declare_native_fn(&mut self, name: &str, func: FunctionCall) -> Result<(), ScopeError> {
        let value = RuntimeVal::NativeFn(func);

        self.declare_const_var(name.to_string(), value)?;
        Ok(())
    }

    // Assigning to an already existing variable
    pub fn assign_var(&mut self, name: String, value: RuntimeVal) -> Result<(), ScopeError> {
        let old_env = self.resolve(&name)?;
        if old_env.constants.contains(&name) {
            Err(ScopeError::ReassignConstantError(name))
        } else {
            *old_env.variables.get_mut(&name).unwrap() = value;

            Ok(())
        }
    }

    pub fn lookup_var(&mut self, name: String) -> Result<&RuntimeVal, ScopeError> {
        let old_env = self.resolve(&name)?;
        Ok(old_env.variables.get(&name).unwrap())
    }

    pub fn resolve(&mut self, name: &str) -> Result<&mut Scope, ScopeError> {
        let mut current = Some(self);

        while let Some(scope) = current {
            if scope.variables.contains_key(name) {
                return Ok(scope);
            }

            current = scope.parent.as_deref_mut();
        }

        Err(ScopeError::VariableDoesNotExist(name.to_string()))
    }
}

#[macro_export]
macro_rules! declare_variable {
    ($env:expr, $ident: expr, $t:expr) => {
        $env.declare_var($ident.to_string(), $t).unwrap()
    };
    ($env:expr, $ident: expr, $t:expr, $constant: expr) => {{
        if $constant {
            $env.declare_const_var($ident.to_string(), $t).unwrap()
        } else {
            $env.declare_var($ident.to_string(), $t).unwrap()
        }
    }};
}

#[macro_export]
macro_rules! declare_native_fn {
    ($env:expr, $name:expr, |$args:ident, $scope:ident| $body:block) => {{
        $env.declare_native_fn(
            $name,
            |$args: Vec<$crate::RuntimeVal>, $scope: &mut $crate::Scope| -> $crate::RuntimeVal {
                $body
            },
        )
        .unwrap()
    }};
}

#[cfg(test)]
mod tests;
