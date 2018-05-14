use super::{FuncMap, Value, Signal, Runnable};
use std::collections::BTreeMap;

/*pub struct State {
    vars: Vec<Scope>,
    // globals
}*/

pub struct Scope {
    vars: Vec<BTreeMap<String, Value>>,
}

/*impl State {
    pub fn new() -> Self {
        State {
            vars: Vec::new(),
        }
    }

    pub fn add_new_namespace(&mut self, name: String, funcs: FuncMap) {
        self.funcs.insert(name, funcs);
    }

    pub fn call_function(&mut self, loc_name: &str, fn_name: &str, args: &[Value]) -> Signal {
        /*self.push_scope();
        
        let get_func = |s: &State| match s.funcs.get(loc_name) {
            Some(l) => match l.get(fn_name) {
                Some(f) => Ok(f.clone()),
                None => Err("Function not found in namespace.".to_string()),
            },
            None => Err("Namespace not found.".to_string()),
        };

        let func = match get_func(&self) {
            Ok(f) => f,
            Err(e) => return Signal::Error(e),
        };

        let arg_names = func.get_arg_names();

        if arg_names.len() != args.len() {
            return Signal::Error("Incorrect number of arguments.".to_string());
        }

        for (ref n,v) in arg_names.iter().zip(args.iter()) {
            // TODO: it shouldn't be necessary to return an error here.
            if self.register_var(&n, Some(v.clone())) {
                return Signal::Error("Multiple arguments with same name.".to_string());
            }
        }

        let ret = func.run(self);

        self.pop_scope();

        ret*/

        Signal::Done
    }

    /*pub fn get_function(&self, loc_name: &str, fn_name: &str) -> Result<Box<Runnable>,String> {
        match self.funcs.get(loc_name) {
            Some(l) => match l.get(fn_name) {
                Some(&ref f) => Ok(f),
                None => Err("Function not found in namespace.".to_string()),
            },
            None => Err("Namespace not found.".to_string()),
        }
    }*/

    pub fn push_scope(&mut self) {
        self.vars.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.vars.pop();
    }

    pub fn extend_scope(&mut self) {
        match self.vars.last_mut() {
            Some(s) => s.extend(),
            None => {},
        }
    }

    pub fn reduce_scope(&mut self) {
        match self.vars.last_mut() {
            Some(s) => s.reduce(),
            None => {},
        }
    }

    pub fn register_var(&mut self, name: &str, val: Option<Value>) -> bool {
        match self.vars.last_mut() {
            Some(s) => s.new_var(name, val),
            None => false,
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&Value> {
        match self.vars.last() {
            Some(s) => s.get_var(name),
            None => None,
        }
    }

    pub fn set_var(&mut self, name: &str, val: Value) -> bool {
        match self.vars.last_mut() {
            Some(s) => s.set_var(name, val),
            None => false,
        }
    }
}*/

impl Scope {
    pub fn new() -> Self {
        Scope {
            vars: vec![BTreeMap::new()],
        }
    }

    pub fn extend(&mut self) {
        self.vars.push(BTreeMap::new());
    }

    pub fn reduce(&mut self) {
        self.vars.pop();
    }

    pub fn new_var(&mut self, name: &str, val: Option<Value>) -> bool {
        match self.vars.last_mut() {
            Some(t) => match t.contains_key(name) {
                true => false,
                false => {t.insert(name.to_string(), val.unwrap_or(Value::Int(0))); true},
            },
            None => false,
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&Value> {
        for t in self.vars.iter().rev() {
            match t.get(name) {
                Some(v) => return Some(v),
                None => {},
            }
        }

        None
    }

    pub fn set_var(&mut self, name: &str, val: Value) -> bool {
        for t in self.vars.iter_mut().rev() {
            match t.contains_key(name) {
                true => {t.insert(name.to_string(), val); return true;}
                false => {}
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BASIC TESTS
    #[test]
    fn create_state_and_scope() {
        let mut state = State::new();

        state.push_scope();

        state.pop_scope();
    }

    #[test]
    fn declare_variable() {
        let mut state = State::new();

        state.push_scope();

        assert!(state.register_var("x", Some(Value::Int(30))));
    }

    #[test]
    fn read_variable() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));
    }

    #[test]
    fn read_undeclared_variable() {
        let mut state = State::new();

        state.push_scope();
        
        assert_eq!(state.get_var("x"), None);
    }

    #[test]
    fn set_undeclared_variable() {
        let mut state = State::new();

        state.push_scope();
        
        assert!(!state.set_var("x", Value::Int(30)));
    }

    #[test]
    fn set_variable() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.set_var("x", Value::Float(2.5));

        assert_eq!(state.get_var("x").unwrap(), &Value::Float(2.5));
    }

    #[test]
    fn set_multi_variables() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        state.register_var("y", Some(Value::Float(3.3)));
        
        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        assert_eq!(state.get_var("y").unwrap(), &Value::Float(3.3));
    }

    // STATE SCOPE TESTS
    #[test]
    fn push_and_pop_scopes() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.pop_scope();

        assert_eq!(state.get_var("x"), None);
    }

    #[test]
    fn pap_and_repush_scopes() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.pop_scope();

        state.push_scope();

        assert_eq!(state.get_var("x"), None);
    }

    #[test]
    fn pap_scopes_with_base_var() {
        let mut state = State::new();

        state.push_scope();

        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.push_scope();

        assert_eq!(state.get_var("x"), None);

        state.pop_scope();

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));
    }

    #[test]
    fn pap_scopes_with_same_name_vars() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.push_scope();

        state.register_var("x", Some(Value::Float(4.5)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Float(4.5));

        state.pop_scope();

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));
    }

    // INNER SCOPE TESTS
    #[test]
    fn extend_scope() {
        let mut state = State::new();

        state.push_scope();

        state.extend_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.reduce_scope();

        assert_eq!(state.get_var("x"), None);
    }

    #[test]
    fn shadow_variables() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.extend_scope();
        
        state.register_var("x", Some(Value::Float(2.5)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Float(2.5));
    }

    #[test]
    fn shadow_variables_and_retract() {
        let mut state = State::new();

        state.push_scope();
        
        state.register_var("x", Some(Value::Int(30)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));

        state.extend_scope();
        
        state.register_var("x", Some(Value::Float(2.5)));

        assert_eq!(state.get_var("x").unwrap(), &Value::Float(2.5));

        state.reduce_scope();

        assert_eq!(state.get_var("x").unwrap(), &Value::Int(30));
    }

    // COMBINATION TESTS (PUSH & EXTEND)
}
