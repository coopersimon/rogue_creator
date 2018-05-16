use super::{Expr, AstNode};
use script::runtime::{Value, Scope, ExprSig, GlobState, FuncMap};

// DECLS
pub enum ValExpr {
    Var(String),
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
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
    package: String,
    name: String,
    args: Vec<Box<Expr>>,
}

/*pub struct IdChain {
    chain: Vec<String>,
    end_func: Option<FuncCall>,
}*/


// IMPLS

impl AstNode for ValExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for ValExpr {
    fn eval(&self, state: &mut Scope, _: &mut GlobState, _: &FuncMap) -> ExprSig {
        //match *self {
        match self {
            &ValExpr::Var(ref n) => state.get_var(&n),
            &ValExpr::Int(ref v) => ExprSig::Value(Value::Int(v.clone())),
            &ValExpr::Float(ref v) => ExprSig::Value(Value::Float(v.clone())),
            &ValExpr::Text(ref v) => ExprSig::Value(Value::Str(v.clone())),
            &ValExpr::Bool(ref v) => ExprSig::Value(Value::Bool(v.clone())),
        }
    }
}


impl AddExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        AddExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for AddExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for AddExpr {
    fn eval(&self, state: &mut Scope, g: &mut GlobState, f: &FuncMap) -> ExprSig {
        let a = match self.left.eval(state, g, f) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state, g, f) {
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
            (_,_) => ExprSig::Error("Addition type error.".to_string()),
        }
    }
}


impl SubExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        SubExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for SubExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for SubExpr {
    fn eval(&self, state: &mut Scope, g: &mut GlobState, f: &FuncMap) -> ExprSig {
        let a = match self.left.eval(state, g, f) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state, g, f) {
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


impl MulExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        MulExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for MulExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for MulExpr {
    fn eval(&self, state: &mut Scope, g: &mut GlobState, f: &FuncMap) -> ExprSig {
        let a = match self.left.eval(state, g, f) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state, g, f) {
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


impl DivExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        DivExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for DivExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for DivExpr {
    fn eval(&self, state: &mut Scope, g: &mut GlobState, f: &FuncMap) -> ExprSig {
        let a = match self.left.eval(state, g, f) {
            ExprSig::Value(v) => v,
            e => return e,
        };

        let b = match self.right.eval(state, g, f) {
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


impl FuncCall {
    pub fn new(p: &str, n: &str, a: Option<Vec<Box<Expr>>>) -> Self {
        FuncCall {
            package: p.to_string(),
            name: n.to_string(),
            args: match a {
                Some(v) => v,
                None => Vec::new(),
            },
        }
    }
}

impl AstNode for FuncCall {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for FuncCall {
    fn eval(&self, state: &mut Scope, g: &mut GlobState, f: &FuncMap) -> ExprSig {
        let mut func_args = Vec::new();

        for a in &self.args {
            match a.eval(state, g, f) {
                ExprSig::Value(v) => func_args.push(v),
                e => return e,
            }
        }

        f.call_fn(&self.package, &self.name, &func_args, g)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use script::runtime::{Scope, Value};

    // ADD

    #[test]
    fn add_int_consts() {
        let mut state = Scope::new();
        let mut g = GlobState::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Int(12)));

        assert_eq!(add.eval(&mut state, &mut g, &f), ExprSig::Value(Value::Int(37)));
    }

    #[test]
    fn add_int_to_float() {
        let mut state = Scope::new();
        let mut g = GlobState::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Float(3.3)));

        assert_eq!(add.eval(&mut state, &mut g, &f), ExprSig::Value(Value::Float(28.3)));
    }

    #[test]
    fn add_int_to_text() {
        let mut state = Scope::new();
        let mut g = GlobState::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Text(" twenty five".to_string())));

        assert_eq!(add.eval(&mut state, &mut g, &f), ExprSig::Value(Value::Str("25 twenty five".to_string())));
    }

    #[test]
    fn add_text_to_float() {
        let mut state = Scope::new();
        let mut g = GlobState::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Text("x = ".to_string())), Box::new(ValExpr::Float(3.3)));

        assert_eq!(add.eval(&mut state, &mut g, &f), ExprSig::Value(Value::Str("x = 3.3".to_string())));
    }
}
