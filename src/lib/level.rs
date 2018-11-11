use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};
use global::Global;
use state::State;
use super::to_coord;

use std::rc::Rc;
use std::cell::RefCell;

type Glob = Rc<RefCell<Global>>;
type S = Rc<RefCell<State>>;


pub const NAME: &'static str = "level";

pub fn call_ref(src: Glob, state: S) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "create"        => create(a, &src, &state),
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

fn create(args: &[Value], src: &Glob, state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Str(ref s) => {
            let active_id = state.borrow().get_active_level();
            let id = state.borrow_mut().create_level(&src.borrow(), &*s.borrow())?;
            state.borrow_mut().set_active_level(id);
            let level_obj = src.borrow().level_init(&*s.borrow())?;
            state.borrow_mut().set_level_obj(level_obj);
            state.borrow_mut().set_active_level(active_id);
            Ok(Val(I(id as i64)))
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn delete(args: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => {
            //src.borrow().level_delete() -> how to know which delete function to call?
            state.borrow_mut().delete_level(i)
        },
        Ref(ref r)  => match *r.borrow() {
            I(i)    => {
                // Call delete
                state.borrow_mut().delete_level(i)
            },
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn set_active(args: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => {state.borrow_mut().set_active_level(i as u64); Ok(Value::Null)},
        Ref(ref r)  => match *r.borrow() {
            I(i)    => {state.borrow_mut().set_active_level(i as u64); Ok(Value::Null)},
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn clone(args: &[Value], state: &S) -> ExprRes {
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

fn obj(args: &[Value], state: &S) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    state.borrow().get_level_obj()
}

fn instance_at(args: &[Value], state: &S) -> ExprRes {
    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let at = to_coord(&args[0])?;

    state.borrow().instance_at(at)
}

fn location_of(args: &[Value], state: &S) -> ExprRes {
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
