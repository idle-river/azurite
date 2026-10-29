pub mod error;
mod values;
use std::collections::{HashMap, HashSet, hash_map::Entry};

use error::ScopeError;
pub use values::RuntimeVal;

pub struct Scope {
    #[allow(dead_code)]
    parent: Option<Box<Scope>>,
    variables: HashMap<String, RuntimeVal>,
    constants: HashSet<String>,
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }

    pub fn global() -> Self {
        let mut scope = Self::new(None);

        scope
            .declare_variable("false".to_string(), RuntimeVal::Boolean(false), true)
            .unwrap();
        scope
            .declare_variable("true".to_string(), RuntimeVal::Boolean(false), true)
            .unwrap();
        scope
            .declare_variable("null".to_string(), RuntimeVal::Null, true)
            .unwrap();

        scope
    }

    pub fn declare_variable(
        &mut self,
        ident: String,
        val: RuntimeVal,
        is_constant: bool,
    ) -> Result<(), ScopeError> {
        match self.variables.entry(ident.clone()) {
            Entry::Occupied(e) => Err(ScopeError::VariableAlreadyDeclared(
                ident.clone(),
                e.get().clone(),
            )),
            Entry::Vacant(v) => {
                v.insert(val);

                if is_constant {
                    let _ = self.constants.insert(ident);
                }

                Ok(())
            }
        }
    }

    pub fn resolve(&mut self, ident: &str) -> Option<&mut Scope> {
        if self.variables.contains_key(ident) {
            return Some(self);
        }

        match self.parent.as_deref_mut() {
            Some(parent) => parent.resolve(ident),
            None => None,
        }
    }

    pub fn lookup_variable(&mut self, ident: &str) -> Option<RuntimeVal> {
        match self.resolve(ident) {
            Some(scope) => Some(scope.variables.get(ident).unwrap().clone()),
            None => None,
        }
    }

    pub fn reassign_variable(
        &mut self,
        ident: &str,
        new_val: RuntimeVal,
    ) -> Result<(), ScopeError> {
        let is_constant = self.constants.contains(ident);

        if is_constant {
            Err(ScopeError::ReassigningConstant(ident.to_string()))
        } else {
            if let Some(val) = self.variables.get_mut(ident) {
                *val = new_val;
                Ok(())
            } else {
                Err(ScopeError::VariableDoesNotExist(ident.to_string()))
            }
        }
    }
}
