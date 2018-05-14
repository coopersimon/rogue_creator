// Runtime tools for script engine
mod state;
mod function;

pub use self::state::*;

use std::collections::BTreeMap;
use script::ast::expr::ExprSignal;

// Runtime Signals
pub enum Signal {
    Error(String),
    Done,
}

// Global state for extension packages
pub type GlobState = BTreeMap<String, Box<ExtData>>;

impl GlobState {
    pub fn new() -> Self {
        BTreeMap::new()
    }

    pub fn attach_data(&mut self, package_name: &str, data: Box<ExtData>) {
        BTreeMap.insert(package_name.to_string(), data);
    }
}

pub trait ExtData {
    fn as_any(&mut self) -> &mut Any;
    fn run(&str, &[Value], &mut GlobState) -> ExprSignal where Self: Sized;
}


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    // Null,
}
