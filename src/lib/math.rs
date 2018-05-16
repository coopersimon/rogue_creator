use script::runtime::{ExtData, PackageRoot, ExprSig, Value};
use std::any::Any;

pub struct MathLib {}

impl MathLib {
    /*pub fn new() -> Self {
        MathLib{}
    }*/

    fn sin(a: &[Value]) -> ExprSig {
        if a.len() != 1 {
            return ExprSig::Error("sin: Incorrect number of args.".to_string());
        }

        match a[0] {
            Value::Float(f) => ExprSig::Value(Value::Float(f.sin())),
            Value::Int(i) => ExprSig::Value(Value::Float((i as f64).sin())),
            _ => ExprSig::Error("sin: Type error.".to_string()),
        }
    }

    fn cos(a: &[Value]) -> ExprSig {
        if a.len() != 1 {
            return ExprSig::Error("cos: Incorrect number of args.".to_string());
        }

        match a[0] {
            Value::Float(f) => ExprSig::Value(Value::Float(f.cos())),
            Value::Int(i) => ExprSig::Value(Value::Float((i as f64).cos())),
            _ => ExprSig::Error("cos: Type error.".to_string()),
        }
    }

    fn pow(a: &[Value]) -> ExprSig {
        if a.len() != 2 {
            return ExprSig::Error("pow: Incorrect number of args.".to_string());
        }

        match (&a[0], &a[1]) {
            (&Value::Float(x), &Value::Float(y)) => ExprSig::Value(Value::Float(x.powf(y))),
            (&Value::Int(x), &Value::Float(y)) => ExprSig::Value(Value::Float((x as f64).powf(y))),
            (&Value::Float(x), &Value::Int(y)) => ExprSig::Value(Value::Float(x.powi(y as i32))),
            (&Value::Int(x), &Value::Int(y)) => ExprSig::Value(Value::Int(x.pow(y as u32))),
            _ => ExprSig::Error("pow: Type error.".to_string()),
        }
    }

    fn sqrt(a: &[Value]) -> ExprSig {
        if a.len() != 1 {
            return ExprSig::Error("sqrt: Incorrect number of args.".to_string());
        }

        match a[0] {
            Value::Float(f) => ExprSig::Value(Value::Float(f.sqrt())),
            Value::Int(i) => ExprSig::Value(Value::Float((i as f64).sqrt())),
            _ => ExprSig::Error("sqrt: Type error.".to_string()),
        }
    }
}

impl ExtData for MathLib {
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn call_ref() -> PackageRoot {
        Box::new(|n, a, _, _| {
            /*let l = match g.get_mut("math").unwrap().as_any().downcast_mut::<Self>() {
                Some(c) => c,
                None => ExprSig::Error("Critical library linking error."),
            };*/

            match n {
                "sin" => MathLib::sin(a),
                "cos" => MathLib::cos(a),
                "pow" => MathLib::pow(a),
                "sqrt" => MathLib::sqrt(a),
                _ => ExprSig::Error(format!("Function {} not found in Math package.", n)),
            }
        })
    }
}
