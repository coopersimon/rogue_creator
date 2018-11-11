use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};
use global::Global;
use state::State;

use std::rc::Rc;
use std::cell::RefCell;

type Glob = Rc<RefCell<Global>>;
type S = Rc<RefCell<State>>;


pub const NAME: &'static str = "entity";

pub fn call_ref(src: Glob, state: S) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "create_global" => create_global(a, &src, &state),
            "create"        => create(a, &src, &state),
            "delete"        => delete(a, &src, &state),
            "data"          => obj(a, &state),
            "this"          => this_obj(a, &state),
            // name, thisname
            "run_actions"   => run_actions(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn create_global(args: &[Value], src: &Glob, state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Str(ref s) => {
            let active_id = state.borrow().get_active_entity();
            let id = state.borrow_mut().create_glob_entity(&src.borrow(), &*s.borrow())?;
            state.borrow_mut().set_active_entity(id);
            let entity_obj = src.borrow().entity_init(&*s.borrow())?;
            state.borrow_mut().set_entity_obj(id, entity_obj);
            state.borrow_mut().set_active_entity(active_id);
            Ok(Val(I(id as i64)))
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn create(args: &[Value], src: &Glob, state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Str(ref s) => {
            let active_id = state.borrow().get_active_entity();
            let id = state.borrow_mut().create_local_entity(&src.borrow(), &*s.borrow())?;
            state.borrow_mut().set_active_entity(id);
            let entity_obj = src.borrow().entity_init(&*s.borrow())?;
            state.borrow_mut().set_entity_obj(id, entity_obj);
            state.borrow_mut().set_active_entity(active_id);
            Ok(Val(I(id as i64)))
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn delete(args: &[Value], src: &Glob, state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => {
            let name = state.borrow().get_entity_name(i);
            src.borrow().entity_delete(&name);
            state.borrow_mut().delete_entity(i)
        },
        Ref(ref r)  => match *r.borrow() {
            I(i)    => {
                let name = state.borrow().get_entity_name(i);
                src.borrow().entity_delete(&name);
                state.borrow_mut().delete_entity(i)
            },
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn obj(args: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    match args[0] {
        Val(I(i))   => state.borrow().get_entity_obj(i as u64),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => state.borrow().get_entity_obj(i as u64),
            _       => mserr(Type::RunTime(RunCode::TypeError)),
        },
        _ => mserr(Type::RunTime(RunCode::TypeError)),
    }
}

fn this_obj(args: &[Value], state: &S) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    state.borrow().active_entity_obj()
}

fn run_actions(args: &[Value], state: &S) -> ExprRes {
    if args.len() == 0 {
        state.borrow_mut().run_actions()
    } /*else if args.len() == 1 {
        state.borrow_mut().run_actions_ordered(&args[0])
    }*/ else {
        mserr(Type::RunTime(RunCode::WrongNumberOfArguments))
    }
}
