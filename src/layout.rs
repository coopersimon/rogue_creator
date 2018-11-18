// Layout objects
use modscript::{ScriptExpr, FuncMap, ExprRes, Value};
use pancurses::Input;

use std::collections::HashMap;
use std::rc::Rc;

pub struct Layout {
    // inputs: HashMap<_input, Script[/Expr]
    inputs: HashMap<Input, ScriptExpr>,
    default: Option<ScriptExpr>,
    render: ScriptExpr,
}

impl Layout {
    pub fn new(inputs: HashMap<Input, ScriptExpr>, default: Option<ScriptExpr>, render: ScriptExpr) -> Self {
        Layout {
            inputs: inputs,
            default: default,
            render: render,
        }
    }

    pub fn render(&self, source: &FuncMap) -> ExprRes {
        self.render.run(source)
    }

    pub fn run_input(&self, input: &Input, source: &FuncMap) -> ExprRes {
        match self.inputs.get(input) {
            Some(c) => c.run(source),
            None => match self.default {
                Some(ref d) => d.run(source),
                None => Ok(Value::Null),
            }
        }
    }
}
