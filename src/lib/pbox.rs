use super::to_text_item;
use textrender::PrintCommand;

use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};

use std::sync::mpsc::Sender;


pub const NAME: &'static str = "pbox";

pub fn call_ref(sender: Sender<PrintCommand>) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "print"     => print(a, &sender),
            "next"      => next(a, &sender),
            "clear"     => clear(a, &sender),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn print(args: &[Value], sender: &Sender<PrintCommand>) -> ExprRes {
    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let text = to_text_item(&args[0])?;

    sender.send(PrintCommand::NewText(text)).unwrap();
    Ok(Value::Null)
}

fn next(args: &[Value], sender: &Sender<PrintCommand>) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    sender.send(PrintCommand::Next).unwrap();
    Ok(Value::Null)
}

fn clear(args: &[Value], sender: &Sender<PrintCommand>) -> ExprRes {
    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    sender.send(PrintCommand::Clear).unwrap();
    Ok(Value::Null)
}
