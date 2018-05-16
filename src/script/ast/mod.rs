// scripting engine ast
mod stat;
mod expr;

pub use self::stat::*;
pub use self::expr::*;

use super::runtime::{Value, Scope, ExprSig, Signal, GlobState, FuncMap, PackageRoot};

use std::collections::BTreeMap;

pub trait AstNode {
    fn print(&self) -> String;
    // compile
}

pub trait Expr: AstNode {
    fn eval(&self, &mut Scope, &mut GlobState, &FuncMap) -> ExprSig;
}

pub trait Statement: AstNode {
    fn run(&self, &mut Scope, &mut GlobState, &FuncMap) -> Signal;
}


// AST entry point for function
pub struct FuncRoot {
    arg_names: Vec<String>,
    stat_list: Vec<Box<Statement>>,
}

impl FuncRoot {
    pub fn new(arg_names: Vec<String>, stat_list: Vec<Box<Statement>>) -> Self {
        FuncRoot {
            arg_names: arg_names,
            stat_list: stat_list,
        }
    }

    pub fn call(&self, args: &[Value], g: &mut GlobState, f: &FuncMap) -> ExprSig {
        let mut state = Scope::new();

        for (a,n) in args.iter().zip(self.arg_names.iter()) {
            state.new_var(&n, a.clone());
        }

        for s in &self.stat_list {
            match s.run(&mut state, g, f) {
                Signal::Done => {},
                Signal::Error(e) => return ExprSig::Error(e),
                Signal::Return(v) => return ExprSig::Value(v),
                //Continue
                //Break
            }
        }

        ExprSig::Value(Value::Null)
    }

    pub fn get_arg_names(&self) -> &[String] {
        self.arg_names.as_slice()
    }
}

impl AstNode for FuncRoot {
    fn print(&self) -> String {
        "var".to_string()
    }
}


// For packages of functions
pub struct ScriptPackage {
    pub funcs: BTreeMap<String, FuncRoot>,
}

impl ScriptPackage {
    pub fn new(f: BTreeMap<String, FuncRoot>) -> Self {
        ScriptPackage {
            funcs: f,
        }
    }

    pub fn call_ref(self) -> PackageRoot {
        Box::new(move |n, a, g, f| {
            match self.funcs.get(n) {
                Some(func) => func.call(a, g, f),
                None => ExprSig::Error(format!("Couldn't find function {}.", n)),
            }
        })
    }
}
