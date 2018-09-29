// Layout objects

use std::collections::HashMap;

pub struct Layout {
    name: String,
    // inputs: HashMap<_input, Script[/Expr]
    inputs: HashMap<char, Value>,
    render: Value,
}

impl Layout {
    pub fn new(name: &str, inputs: HashMap<char, Value>, render: Value) -> Self {
        Layout {
            name: name.to_string(),
            inputs: inputs,
            render: render,
        }
    }

    pub fn render() {
    }

    pub fn run_input(input: char) {
        
    }
}
