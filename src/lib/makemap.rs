use super::to_coord;
use state::State;

use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};

use std::rc::Rc;
use std::cell::RefCell;

type S = Rc<RefCell<State>>;


pub const NAME: &'static str = "makemap";

pub fn call_ref(state: S) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "fill_tile" => fill_tile(a, &state),
            "draw_line" => draw_line(a, &state),
            "spawn"     => spawn(a, &state),
            "despawn"   => despawn(a, &state),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn fill_tile(args: &[Value], state: &S) -> ExprRes {
    if args.len() != 2 && args.len() != 3 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let tile_name = match args[0] {
        Value::Str(ref s) => s.borrow(),
        _ => return mserr(Type::RunTime(RunCode::TypeError)),
    };

    let tl = to_coord(&args[1])?;
    let br = if args.len() == 3 {
        to_coord(&args[2])?
    } else {
        tl
    };

    state.borrow_mut().fill_tiles(&*tile_name, tl, br)
}

fn draw_line(args: &[Value], state: &S) -> ExprRes {
    if args.len() != 3 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let tile_name = match args[0] {
        Value::Str(ref s) => s.borrow(),
        _ => return mserr(Type::RunTime(RunCode::TypeError)),
    };

    let s = to_coord(&args[1])?;
    let e = to_coord(&args[2])?;

    state.borrow_mut().draw_line(&*tile_name, s, e)
}

fn spawn(args: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 2 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let id = match args[0] {
        Val(I(i))   => i,
        Ref(ref r)  => match *r.borrow() {
            I(i)    => i,
            _ => return mserr(Type::RunTime(RunCode::TypeError)),
        }
        _ => return mserr(Type::RunTime(RunCode::TypeError)),
    };

    let loc = to_coord(&args[1])?;

    state.borrow_mut().spawn_entity(id, loc)
}

fn despawn(args: &[Value], state: &S) -> ExprRes {
    use modscript::Value::*;
    use modscript::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let id = match args[0] {
        Val(I(i))   => i,
        Ref(ref r)  => match *r.borrow() {
            I(i)    => i,
            _ => return mserr(Type::RunTime(RunCode::TypeError)),
        }
        _ => return mserr(Type::RunTime(RunCode::TypeError)),
    };

    state.borrow_mut().despawn_entity(id)
}
