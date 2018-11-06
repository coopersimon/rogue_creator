// Layout objects
use modscript::{ScriptExpr, FuncMap, ExprRes, Value};

use std::collections::HashMap;
use std::rc::Rc;

pub struct Layout {
    // inputs: HashMap<_input, Script[/Expr]
    inputs: HashMap<char, ScriptExpr>,
    render: ScriptExpr,
    source: Rc<FuncMap>,
}

impl Layout {
    pub fn new(inputs: HashMap<char, ScriptExpr>, render: ScriptExpr, source: Rc<FuncMap>) -> Self {
        Layout {
            inputs: inputs,
            render: render,
            source: source,
        }
    }

    pub fn render(&self) -> ExprRes {
        self.render.run(&self.source)
    }

    pub fn run_input(&self, input: char) -> ExprRes {
        match self.inputs.get(&input) {
            Some(c) => c.run(&self.source),
            None => Ok(Value::Null),
        }
    }
}
