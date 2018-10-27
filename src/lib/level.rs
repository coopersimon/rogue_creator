use modscript::{PackageRoot, ExprRes, Value, VType, mserr, Type, RunCode};
use global::Global;

use std::rc::Rc;
use std::cell::RefCell;

type Glob = Rc<RefCell<Global>>;


pub const NAME: &'static str = "level";

pub fn call_ref(state: Glob) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "create"    => create(a, &state),
            "delete"    => delete(a, &state),
            "load"      => set_active(a, &state),
            "clone"     => clone(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn create(a: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Str(ref s) => state.borrow_mut().create_level(&*s.borrow()),
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn delete(a: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Val(I(i))   => state.borrow_mut().delete_level(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow_mut().delete_level(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn set_active(a: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Val(I(i))   => state.borrow_mut().set_active_level(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow_mut().set_active_level(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn clone(a: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if a.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match a[0] {
        Val(I(i))   => state.borrow_mut().clone_level(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow_mut().clone_level(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}
