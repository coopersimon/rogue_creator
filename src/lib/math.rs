use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};

pub const NAME: &'static str = "math";

pub fn call_ref() -> PackageRoot {
    Box::new(|n, a, _| {
        match n {
            "sin" => sin(a),
            "cos" => cos(a),
            "pow" => pow(a),
            "sqrt" => sqrt(a),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn sin(a: &[Value]) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Val(F(f)) => Ok(Val(F(f.sin()))),
        Val(I(i)) => Ok(Val(F((i as f64).sin()))),
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn cos(a: &[Value]) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Val(F(f)) => Ok(Val(F(f.cos()))),
        Val(I(i)) => Ok(Val(F((i as f64).cos()))),
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn pow(a: &[Value]) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if a.len() != 2 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match (&a[0], &a[1]) {
        (&Val(F(x)), &Val(F(y))) => Ok(Val(F(x.powf(y)))),
        (&Val(I(x)), &Val(F(y))) => Ok(Val(F((x as f64).powf(y)))),
        (&Val(F(x)), &Val(I(y))) => Ok(Val(F(x.powi(y as i32)))),
        (&Val(I(x)), &Val(I(y))) => Ok(Val(I(x.pow(y as u32)))),
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn sqrt(a: &[Value]) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Val(F(f)) => Ok(Val(F(f.sqrt()))),
        Val(I(i)) => Ok(Val(F((i as f64).sqrt()))),
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}
