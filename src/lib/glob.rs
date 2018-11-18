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
            "last_key"      => last_key(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn obj(args: &[Value], state: &S) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    Ok(state.borrow().get_glob_obj())
}

fn set_layout(args: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Str(ref s) => {
            state.borrow_mut().set_current_layout(&*s.borrow());
            Ok(Value::Null)
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn last_key(args: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    Ok(Value::Str(Rc::new(RefCell::new(state.borrow().get_last_key()))))
}
