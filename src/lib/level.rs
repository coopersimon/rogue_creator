use modscript::{PackageRoot, ExprRes, Value, VType, mserr, Type, RunCode};
use global::Global;
use super::to_coord;

use std::rc::Rc;
use std::cell::RefCell;

type Glob = Rc<RefCell<Global>>;


pub const NAME: &'static str = "level";

pub fn call_ref(state: Glob) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "create"        => create(a, &state),
            "delete"        => delete(a, &state),
            "load"          => set_active(a, &state),
            "clone"         => clone(a, &state),
            "data"          => obj(a, &state),
            "instance_at"   => instance_at(a, &state),
            "location_of"   => location_of(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn create(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Str(ref s) => state.borrow_mut().create_level(&*s.borrow()),
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn delete(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => state.borrow_mut().delete_level(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow_mut().delete_level(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn set_active(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => state.borrow_mut().set_active_level(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow_mut().set_active_level(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn clone(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => state.borrow_mut().clone_level(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow_mut().clone_level(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn obj(args: &[Value], state: &Glob) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    state.borrow().level_obj()
}

fn instance_at(args: &[Value], state: &Glob) -> ExprRes {
    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let at = to_coord(&args[0])?;

    state.borrow().instance_at(at)
}

fn location_of(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => state.borrow().location_of(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow().location_of(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}
