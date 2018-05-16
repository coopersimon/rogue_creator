// Runtime tools for script engine
mod state;
mod function;

pub use self::state::*;
pub use self::function::*;

use std::collections::BTreeMap;
use std::any::Any;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
    // List
    // Object
}

// Runtime Signals
#[derive(Clone, PartialEq, Debug)]
pub enum Signal {
    Error(String),
    Return(Value),
    Done,
    //Continue
    //Break
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExprSig {
    Value(Value),
    Error(String),
}

// Global state for extension packages
pub struct GlobState {
    ext_data: BTreeMap<String, Box<ExtData>>,
}

impl GlobState {
    pub fn new() -> Self {
        GlobState {
            ext_data: BTreeMap::new(),
        }
    }

    pub fn attach_data(&mut self, package_name: &str, data: Box<ExtData>) {
        self.ext_data.insert(package_name.to_string(), data);
    }
}

pub trait ExtData {
    fn as_any(&mut self) -> &mut Any;
    //fn run(&str, &[Value], &mut GlobState) -> ExprSig where Self: Sized;
    fn call_ref() -> PackageRoot where Self: Sized;
}
