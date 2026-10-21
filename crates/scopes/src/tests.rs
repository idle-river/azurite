mod tests {
    use crate::*;

    #[test]
    fn declares_variable() {
        let mut scope = Scope::global();

        assert!(
            scope
                .declare_var("x".to_string(), RuntimeVal::FloatingNumber(42.0))
                .is_ok()
        );

        assert_eq!(
            scope.lookup("x").unwrap(),
            &RuntimeVal::FloatingNumber(42.0)
        );
    }

    #[test]
    fn rejects_duplicate_declaration() {
        let mut scope = Scope::global();

        scope
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        let result = scope.declare_var("x".to_string(), RuntimeVal::FloatingNumber(2.0));

        assert!(matches!(
            result,
            Err(ScopeError::VariableAlreadyDeclared {
                variable_name,
                declared_value: RuntimeVal::FloatingNumber(1.0),
            }) if variable_name == "x"
        ));

        // The original value should remain unchanged.
        assert_eq!(scope.lookup("x").unwrap(), &RuntimeVal::FloatingNumber(1.0));
    }

    #[test]
    fn assigns_existing_variable() {
        let mut scope = Scope::global();

        scope
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        scope
            .assign_var("x".to_string(), RuntimeVal::FloatingNumber(2.0))
            .unwrap();

        assert_eq!(scope.lookup("x").unwrap(), &RuntimeVal::FloatingNumber(2.0));
    }

    #[test]
    fn rejects_assignment_to_missing_variable() {
        let mut scope = Scope::global();

        let result = scope.assign_var("x".to_string(), RuntimeVal::FloatingNumber(1.0));

        assert!(matches!(
            result,
            Err(ScopeError::VariableDoesNotExist(name)) if name == "x"
        ));
    }

    #[test]
    fn looks_up_variable_in_current_scope() {
        let mut scope = Scope::global();

        scope
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(10.0))
            .unwrap();

        assert_eq!(
            scope.lookup("x").unwrap(),
            &RuntimeVal::FloatingNumber(10.0)
        );
    }

    #[test]
    fn looks_up_variable_in_parent_scope() {
        let mut parent = Scope::global();

        parent
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(10.0))
            .unwrap();

        let child = Scope::new(Box::new(parent));

        assert_eq!(
            child.lookup("x").unwrap(),
            &RuntimeVal::FloatingNumber(10.0)
        );
    }

    #[test]
    fn looks_up_variable_in_grandparent_scope() {
        let mut global = Scope::global();

        global
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(10.0))
            .unwrap();

        let parent = Scope::new(Box::new(global));
        let child = Scope::new(Box::new(parent));

        assert_eq!(
            child.lookup("x").unwrap(),
            &RuntimeVal::FloatingNumber(10.0)
        );
    }

    #[test]
    fn child_scope_shadows_parent_variable() {
        let mut parent = Scope::global();

        parent
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        let mut child = Scope::new(Box::new(parent));

        child
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(2.0))
            .unwrap();

        assert_eq!(child.lookup("x").unwrap(), &RuntimeVal::FloatingNumber(2.0));
    }

    #[test]
    fn assignment_does_not_modify_parent_variable() {
        let mut parent = Scope::global();

        parent
            .declare_var("x".to_string(), RuntimeVal::FloatingNumber(1.0))
            .unwrap();

        let mut child = Scope::new(Box::new(parent));

        assert!(matches!(
            child.assign_var("x".to_string(), RuntimeVal::FloatingNumber(2.0)),
            Err(ScopeError::VariableDoesNotExist(name)) if name == "x"
        ));

        assert_eq!(child.lookup("x").unwrap(), &RuntimeVal::FloatingNumber(1.0));
    }

    #[test]
    fn lookup_returns_error_for_missing_variable() {
        let scope = Scope::global();

        assert!(matches!(
            scope.lookup("missing"),
            Err(ScopeError::VariableDoesNotExist(name))
                if name == "missing"
        ));
    }

    #[test]
    fn lookup_returns_error_when_variable_is_missing_from_all_scopes() {
        let global = Scope::global();
        let parent = Scope::new(Box::new(global));
        let child = Scope::new(Box::new(parent));

        assert!(matches!(
            child.lookup("missing"),
            Err(ScopeError::VariableDoesNotExist(name))
                if name == "missing"
        ));
    }
}
