use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};
use state::State;

use std::rc::Rc;
use std::cell::RefCell;

type S = Rc<RefCell<State>>;


pub const NAME: &'static str = "glob";

pub fn call_ref(state: S) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "obj"           => obj(a, &state),
            "set_layout"    => set_layout(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn obj(a: &[Value], state: &S) -> ExprRes {
    if a.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    Ok(state.borrow().get_glob_obj())
}

fn set_layout(a: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Str(ref s) => {
            state.borrow_mut().set_current_layout(&*s.borrow());
            Ok(Value::Null)
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}
