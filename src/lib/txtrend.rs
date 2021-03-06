use super::{to_coord, to_text_item};
use textrender::{RenderCommand, TextBox};

use modscript::{PackageRoot, ExprRes, Value, mserr, Type, RunCode};

use std::sync::mpsc::Sender;


pub const NAME: &'static str = "txtrend";

pub fn call_ref(sender: Sender<RenderCommand>) -> PackageRoot {
    Box::new(move |n, a, _| {
        match n {
            "place_print"   => place_print(a, &sender),
            "place_map"     => place_map(a, &sender),
            "place_text"    => place_text(a, &sender),
            _ => mserr(Type::RunTime(RunCode::FunctionNotFound)),
        }
    })
}

fn place_print(args: &[Value], sender: &Sender<RenderCommand>) -> ExprRes {
    if args.len() != 2 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let tl = to_coord(&args[0])?;
    let br = to_coord(&args[1])?;

    sender.send(RenderCommand::PrintBox(tl, br)).unwrap();
    Ok(Value::Null)
}

fn place_map(args: &[Value], sender: &Sender<RenderCommand>) -> ExprRes {
    if args.len() != 2 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let tl = to_coord(&args[0])?;
    let br = to_coord(&args[1])?;

    sender.send(RenderCommand::Map(tl, br)).unwrap();
    Ok(Value::Null)
}

fn place_text(args: &[Value], sender: &Sender<RenderCommand>) -> ExprRes {
    if args.len() != 3 {
        return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
    }

    let text = to_text_item(&args[0])?;
    let tl = to_coord(&args[1])?;
    let br = to_coord(&args[2])?;

    sender.send(RenderCommand::Renderable(Box::new(TextBox::new(text)), tl, br)).unwrap();
    Ok(Value::Null)
}
