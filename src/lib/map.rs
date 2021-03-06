use super::to_coord;
use textrender::MapCommand;

use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};

use std::sync::mpsc::Sender;


pub const NAME: &'static str = "map";

pub fn call_ref(sender: Sender<MapCommand>) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "display"   => display(a, &sender),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn display(args: &[Value], sender: &Sender<MapCommand>) -> ExprRes {
    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let tl = to_coord(&args[0])?;

    sender.send(MapCommand::Display(tl)).unwrap();
    Ok(Value::Null)
}
