// Layout objects
use modscript::{ScriptExpr, FuncMap, ExprRes, Value};

use std::collections::HashMap;
use std::rc::Rc;

pub struct Layout {
    // inputs: HashMap<_input, Script[/Expr]
    inputs: HashMap<char, ScriptExpr>,
    render: ScriptExpr,
}

impl Layout {
    pub fn new(inputs: HashMap<char, ScriptExpr>, render: ScriptExpr) -> Self {
        Layout {
            inputs: inputs,
            render: render,
        }
    }

    pub fn render(&self, source: &FuncMap) -> ExprRes {
        self.render.run(source)
    }

    pub fn run_input(&self, input: char, source: &FuncMap) -> ExprRes {
        match self.inputs.get(&input) {
            Some(c) => c.run(source),
            None => Ok(Value::Null),
        }
    }
}
