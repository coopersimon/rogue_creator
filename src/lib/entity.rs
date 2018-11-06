use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};
use global::Global;

use std::rc::Rc;
use std::cell::RefCell;

type Glob = Rc<RefCell<Global>>;


pub const NAME: &'static str = "entity";

pub fn call_ref(state: Glob) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "create_global" => create_global(a, &state),
            "create"        => create(a, &state),
            "delete"        => delete(a, &state),
            "data"          => obj(a, &state),
            "this"          => this_obj(a, &state),
            // name, thisname
            "run_actions"   => run_actions(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn create_global(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Str(ref s) => state.borrow_mut().create_glob_entity(&*s.borrow()),
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn create(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Str(ref s) => state.borrow_mut().create_local_entity(&*s.borrow()),
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
        Val(I(i))   => state.borrow_mut().delete_entity(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow_mut().delete_entity(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn obj(args: &[Value], state: &Glob) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => state.borrow().entity_obj(i),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow().entity_obj(i),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn this_obj(args: &[Value], state: &Glob) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    state.borrow().active_entity_obj()
}

fn run_actions(args: &[Value], state: &Glob) -> ExprRes {
    if args.len() == 0 {
        state.borrow_mut().run_actions()
    } /*else if args.len() == 1 {
        state.borrow_mut().run_actions_ordered(&args[0])
    }*/ else {
        mserr(Type::RunTime(RunCode::WrongNumberOfArguments))
    }
}
