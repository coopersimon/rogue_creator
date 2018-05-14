use super::{Statement, Expr};

pub struct Scope {
    code: Vec<Box<Statement>>,
}

pub struct VarDecl {
    name: String,
    assign: Option<Box<Expr>>,
}

pub struct IfStat {
    cond: Box<Expr>,
    then_stat: Box<Statement>,
    else_stat: Option<Box<Statement>>,
}

pub struct LoopStat {
    init: Box<Expr>,
    cond: Box<Expr>,
    end: Box<Expr>,
    loop_body: Box<Statement>,
}

pub struct WhileStat {
    cond: Box<Expr>,
    loop_body: Box<Statement>,
}

pub struct ReturnStat {
    expr: Option<Box<Expr>>,
}
