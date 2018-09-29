use modscript::{PackageRoot, ExprRes, Value, expr_err};

pub fn call_ref() -> PackageRoot {
    Box::new(|n, a, _| {
        match n {
            "sin" => sin(a),
            "cos" => cos(a),
            "pow" => pow(a),
            "sqrt" => sqrt(a),
            _ => ExprSig::Error(format!("Function {} not found in Math package.", n)),
        }
    })
}

fn sin(a: &[Value]) -> ExprRes {
    if a.len() != 1 {
        return expr_err("sin: Incorrect number of args.");
    }

    match a[0] {
        Value::Float(f) => ExprSig::Value(Value::Float(f.sin())),
        Value::Int(i) => ExprSig::Value(Value::Float((i as f64).sin())),
        _ => expr_err("sin: Type error."),
    }
}

fn cos(a: &[Value]) -> ExprRes {
    if a.len() != 1 {
        return expr_err("cos: Incorrect number of args.");
    }

    match a[0] {
        Value::Float(f) => ExprSig::Value(Value::Float(f.cos())),
        Value::Int(i) => ExprSig::Value(Value::Float((i as f64).cos())),
        _ => expr_err("cos: Type error."),
    }
}

fn pow(a: &[Value]) -> ExprRes {
    if a.len() != 2 {
        return expr_err("pow: Incorrect number of args.");
    }

    match (&a[0], &a[1]) {
        (&Value::Float(x), &Value::Float(y)) => ExprSig::Value(Value::Float(x.powf(y))),
        (&Value::Int(x), &Value::Float(y)) => ExprSig::Value(Value::Float((x as f64).powf(y))),
        (&Value::Float(x), &Value::Int(y)) => ExprSig::Value(Value::Float(x.powi(y as i32))),
        (&Value::Int(x), &Value::Int(y)) => ExprSig::Value(Value::Int(x.pow(y as u32))),
        _ => expr_err("pow: Type error."),
    }
}

fn sqrt(a: &[Value]) -> ExprRes {
    if a.len() != 1 {
        return expr_err("sqrt: Incorrect number of args.");
    }

    match a[0] {
        Value::Float(f) => ExprSig::Value(Value::Float(f.sqrt())),
        Value::Int(i) => ExprSig::Value(Value::Float((i as f64).sqrt())),
        _ => expr_err("sqrt: Type error."),
    }
}
