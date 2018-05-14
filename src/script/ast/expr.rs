use super::{Expr, AstNode, ExprSig};
use script::runtime::{State, Value};

// DECLS
pub enum ValExpr {
    Var(String),
    Int(i64),
    Float(f64),
    Text(String),
}

pub struct AddExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct SubExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct MulExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct DivExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct FuncCall {
    name: String,
    args: Vec<Box<Expr>>,
    location: String,
}

pub struct IdChain {
    chain: Vec<String>,
    end_func: Option<FuncCall>,
}


// IMPLS

impl AstNode for ValExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for ValExpr {
    fn eval(&self, state: &mut State) -> ExprSig {
        match self {
            &ValExpr::Var(ref n) => match state.get_var(&n) {
                Some(ref v) => ExprSig::Value((*v).clone()),
                None => ExprSig::Error("Variable not declared!".to_string()),
            },
            &ValExpr::Int(ref v) => ExprSig::Value(Value::Int(v.clone())),
            &ValExpr::Float(ref v) => ExprSig::Value(Value::Float(v.clone())),
            &ValExpr::Text(ref v) => ExprSig::Value(Value::Str(v.clone())),
        }
    }
}


impl AstNode for AddExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for AddExpr {
    fn eval(&self, state: &mut State) -> ExprSig {
        let a = match self.left.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => ExprSig::Value(Value::Int(x + y)),
            (Value::Int(x),Value::Float(y)) => ExprSig::Value(Value::Float(x as f64 + y)),
            (Value::Int(x),Value::Str(y)) => ExprSig::Value(Value::Str(x.to_string() + &y)),
            (Value::Float(x),Value::Int(y)) => ExprSig::Value(Value::Float(x + y as f64)),
            (Value::Float(x),Value::Float(y)) => ExprSig::Value(Value::Float(x + y)),
            (Value::Float(x),Value::Str(y)) => ExprSig::Value(Value::Str(x.to_string() + &y)),
            (Value::Str(x),Value::Int(y)) => ExprSig::Value(Value::Str(x + &y.to_string())),
            (Value::Str(x),Value::Float(y)) => ExprSig::Value(Value::Str(x + &y.to_string())),
            (Value::Str(x),Value::Str(y)) => ExprSig::Value(Value::Str(x + &y)),
        }
    }
}


impl AstNode for SubExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for SubExpr {
    fn eval(&self, state: &mut State) -> ExprSig {
        let a = match self.left.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => ExprSig::Value(Value::Int(x - y)),
            (Value::Int(x),Value::Float(y)) => ExprSig::Value(Value::Float(x as f64 - y)),
            (Value::Float(x),Value::Int(y)) => ExprSig::Value(Value::Float(x - y as f64)),
            (Value::Float(x),Value::Float(y)) => ExprSig::Value(Value::Float(x - y)),
            (_,_) => ExprSig::Error("Subtraction type error.".to_string()),
        }
    }
}


impl AstNode for MulExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for MulExpr {
    fn eval(&self, state: &mut State) -> ExprSig {
        let a = match self.left.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => ExprSig::Value(Value::Int(x * y)),
            (Value::Int(x),Value::Float(y)) => ExprSig::Value(Value::Float(x as f64 * y)),
            (Value::Float(x),Value::Int(y)) => ExprSig::Value(Value::Float(x * y as f64)),
            (Value::Float(x),Value::Float(y)) => ExprSig::Value(Value::Float(x * y)),
            (Value::Str(x),Value::Int(y)) => ExprSig::Value(Value::Str(x.repeat(y as usize))),
            (_,_) => ExprSig::Error("Multiplication type error.".to_string()),
        }
    }
}


impl AstNode for DivExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for DivExpr {
    fn eval(&self, state: &mut State) -> ExprSig {
        let a = match self.left.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        match (a,b) {
            (_,Value::Int(0)) => ExprSig::Error("Divide by zero error.".to_string()),
            (Value::Int(x),Value::Int(y)) => ExprSig::Value(Value::Int(x / y)),
            (Value::Int(x),Value::Float(y)) => ExprSig::Value(Value::Float(x as f64 / y)),
            (Value::Float(x),Value::Int(y)) => ExprSig::Value(Value::Float(x / y as f64)),
            (Value::Float(x),Value::Float(y)) => ExprSig::Value(Value::Float(x / y)),
            (_,_) => ExprSig::Error("Division type error.".to_string()),
        }
    }
}


/*impl AstNode for FuncCall {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for FuncCall {
    fn eval(&self, state: &mut State) -> ExprSig {
        //name args location

        // eval args

        // new scope with args

        // eval function and return val

        ExprSig::Value(Value::Int(0))
    }
}*/



#[cfg(test)]
mod tests {
    use super::*;
    use script::runtime::{State, Value};

    // ADD

    #[test]
    fn add_int_consts() {
        let mut state = State::new();

        let add = AddExpr {
            left: Box::new(ValExpr::Int(25)),
            right: Box::new(ValExpr::Int(12))
        };

        assert_eq!(add.eval(&mut state), ExprSig::Value(Value::Int(37)));
    }

    #[test]
    fn add_int_to_float() {
        let mut state = State::new();

        let add = AddExpr {
            left: Box::new(ValExpr::Int(25)),
            right: Box::new(ValExpr::Float(3.3))
        };

        assert_eq!(add.eval(&mut state), ExprSig::Value(Value::Float(28.3)));
    }

    #[test]
    fn add_int_to_text() {
        let mut state = State::new();

        let add = AddExpr {
            left: Box::new(ValExpr::Int(25)),
            right: Box::new(ValExpr::Text(" twenty five".to_string()))
        };

        assert_eq!(add.eval(&mut state), ExprSig::Value(Value::Str("25 twenty five".to_string())));
    }

    #[test]
    fn add_text_to_float() {
        let mut state = State::new();

        let add = AddExpr {
            left: Box::new(ValExpr::Text("x = ".to_string())),
            right: Box::new(ValExpr::Float(3.3))
        };

        assert_eq!(add.eval(&mut state), ExprSig::Value(Value::Str("x = 3.3".to_string())));
    }
}
