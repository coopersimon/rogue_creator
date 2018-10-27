use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};
use global::Global;

use std::rc::Rc;
use std::cell::RefCell;

type Glob = Rc<RefCell<Global>>;


pub const NAME: &'static str = "glob";

pub fn call_ref(state: Glob) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "obj"           => obj(a, &state),
            "set_layout"    => set_layout(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn obj(a: &[Value], state: &Glob) -> ExprRes {
    if a.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    Ok(state.borrow().glob_obj.clone())
}

fn set_layout(a: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Str(ref s) => {
            state.borrow_mut().current_layout = s.borrow().clone();
            Ok(Value::Null)
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}
