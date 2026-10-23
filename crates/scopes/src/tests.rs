mod tests {
    use crate::*;

    #[test]
    fn declares_variable() {
        let scope = Scope::global();

        assert!(
            scope
                .borrow_mut()
                .declare_var("x".to_string(), RuntimeVal::FloatingNumber(42.0))
                .is_ok()
        );

        assert_eq!(
            scope.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(42.0)
        );
    }

    #[test]
    fn rejects_duplicate_declaration() {
        let scope = Scope::global();

        scope
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        let result = scope
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(2.0));

        assert!(matches!(
            result,
            Err(ScopeError::VariableAlreadyDeclared {
                variable_name,
                declared_value: RuntimeVal::FloatingNumber(1.0),
            }) if variable_name == "x".to_string()
        ));

        // The original value should remain unchanged.
        assert_eq!(
            scope.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(1.0)
        );
    }

    #[test]
    fn assigns_existing_variable() {
        let scope = Scope::global();

        scope
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        scope
            .borrow_mut()
            .assign_var("x".to_string(), RuntimeVal::FloatingNumber(2.0))
            .unwrap();

        assert_eq!(
            scope.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(2.0)
        );
    }

    #[test]
    fn rejects_assignment_to_missing_variable() {
        let scope = Scope::global();

        let result = scope
            .borrow_mut()
            .assign_var("x".to_string(), RuntimeVal::FloatingNumber(1.0));

        assert!(matches!(
            result,
            Err(ScopeError::VariableDoesNotExist(name)) if name == "x".to_string()
        ));
    }

    #[test]
    fn rejects_assignment_to_constant() {
        let scope = Scope::global();

        let result = scope
            .borrow_mut()
            .assign_var("true".to_string(), RuntimeVal::Boolean(false));

        assert!(matches!(
            result,
            Err(ScopeError::ReassignConstantError(name)) if name == "true"
        ));
    }

    #[test]
    fn looks_up_variable_in_current_scope() {
        let scope = Scope::global();

        scope
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(10.0))
            .unwrap();

        assert_eq!(
            scope.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(10.0)
        );
    }

    #[test]
    fn looks_up_variable_in_parent_scope() {
        let parent = Scope::global();

        parent
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(10.0))
            .unwrap();

        let child = Scope::child(&parent);

        assert_eq!(
            child.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(10.0)
        );
    }

    #[test]
    fn looks_up_variable_in_grandparent_scope() {
        let global = Scope::global();

        global
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(10.0))
            .unwrap();

        let parent = Scope::child(&global);
        let child = Scope::child(&parent);

        assert_eq!(
            child.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(10.0)
        );
    }

    #[test]
    fn child_scope_shadows_parent_variable() {
        let parent = Scope::global();

        parent
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        let child = Scope::child(&parent);

        child
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(2.0))
            .unwrap();

        assert_eq!(
            child.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(2.0)
        );
        assert_eq!(
            parent.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(1.0)
        );
    }

    #[test]
    fn assignment_in_child_updates_parent_variable() {
        let parent = Scope::global();
        parent
            .borrow_mut()
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        let child = Scope::child(&parent);

        child
            .borrow_mut()
            .assign_var("x".to_string(), RuntimeVal::FloatingNumber(2.0))
            .unwrap();

        // The variable lives in the parent, so assigning from the child updates it there.
        assert_eq!(
            child.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(2.0)
        );
        assert_eq!(
            parent.borrow().lookup_var("x").unwrap(),
            RuntimeVal::FloatingNumber(2.0)
        );
    }

    #[test]
    fn lookup_var_returns_error_for_missing_variable() {
        let scope = Scope::global();

        assert!(matches!(
            scope.borrow().lookup_var("missing"),
            Err(ScopeError::VariableDoesNotExist(name))
                if name == "missing"
        ));
    }

    #[test]
    fn lookup_var_returns_error_when_variable_is_missing_from_all_scopes() {
        let global = Scope::global();
        let parent = Scope::child(&global);
        let child = Scope::child(&parent);

        assert!(matches!(
            child.borrow().lookup_var("missing"),
            Err(ScopeError::VariableDoesNotExist(name))
                if name == "missing"
        ));
    }
}
