use crate::parser::token::Operator;

#[derive(Debug)]
pub enum Item {
    ItemFn(ItemFn),
    ItemConst(ItemConst),
}
#[derive(Debug)]

pub struct ItemFn {
    pub signature: FnSignature,
    pub block: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Local(Local),
    FnCall(FnCall),
    Return(Expr),
}

#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub var_type: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    ExprLit(String),
    ExprBinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    ExprVariable(String),
    ExprFnCall(FnCall),
}

#[derive(Debug)]
pub struct FnSignature {
    pub ident: String,
    pub args: Vec<FnParams>,
    pub output: Option<String>,
}

#[derive(Debug)]
pub struct FnParams {
    pub name: String,
    pub arg_type: String,
}

#[derive(Debug)]
pub struct ItemConst {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}
