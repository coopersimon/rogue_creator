// scripting engine ast
mod stat;
mod expr;

pub use self::stat::*;
pub use self::expr::*;

//use super::runtime::{Runnable};
use super::runtime::{State, Value};

#[derive(Clone, PartialEq, Debug)]
pub enum ExprSig {
    Value(Value),
    Error(String),
}

pub trait AstNode {
    // print
    fn print(&self) -> String;
    // compile
}

pub trait Expr: AstNode {
    fn eval(&self, state: &mut State) -> ExprSig;
}

pub trait Statement: AstNode {
    fn run(&self, state: &mut State);
}
