// Layout objects
use modscript::{Callable, FuncMap, ExprRes, Value};

use std::collections::HashMap;
use std::rc::Rc;

pub struct Layout {
    // inputs: HashMap<_input, Script[/Expr]
    inputs: HashMap<char, Callable>,
    render: Callable,
    source: Rc<FuncMap>,
}

impl Layout {
    pub fn new(inputs: HashMap<char, Callable>, render: Callable, source: Rc<FuncMap>) -> Self {
        Layout {
            inputs: inputs,
            render: render,
            source: source,
        }
    }

    pub fn render(&self) -> ExprRes {
        self.render.call(&self.source, &[])
    }

    pub fn run_input(&self, input: char) -> ExprRes {
        match self.inputs.get(&input) {
            Some(c) => c.call(&self.source, &[]),
            None => Ok(Value::Null),
        }
    }
}
