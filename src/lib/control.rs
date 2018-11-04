use MainCommand;

use modscript::{PackageRoot, ExprRes, Value, VType, mserr, Type, RunCode};

use std::{thread, time};
use std::sync::mpsc::Sender;


pub const NAME: &'static str = "control";

pub fn call_ref(sender: Sender<MainCommand>) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "end_game"          => end_game(a, &sender),
            "terminate_game"    => terminate_game(a, &sender),
            "wait"              => wait(a),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn end_game(args: &[Value], sender: &Sender<MainCommand>) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    sender.send(MainCommand::EndGame).unwrap();
    Ok(Value::Null)
}

fn terminate_game(args: &[Value], sender: &Sender<MainCommand>) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    sender.send(MainCommand::Terminate).unwrap();
    Ok(Value::Null)
}

fn wait(args: &[Value]) -> ExprRes {
    use self::Value::*;
    use self::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let wait_time = match args[0] {
        Val(I(i))   => i as u64,
        Ref(ref r)  => match *r.borrow() {
            I(i)    => i as u64,
            _       => return mserr(Type::RunTime(RunCode::TypeError)),
        },
        _           => return mserr(Type::RunTime(RunCode::TypeError)),
    };

    thread::sleep(time::Duration::from_millis(wait_time));

    Ok(Value::Null)
}
