use crate::scanner::Token;

use super::statments::Statment;

#[derive(Debug,Clone)]
pub enum Expresion{
    IntLitteral(Token),//Int
    FloatLitteral(Token),//Float
    StringLitteral(Token),//String
    True,
    False,
    FunctionCall(FuncCall),
    VarAccess(VarAccess),
    Binary(Binary),
    Unary(Unary),
    Parens(Parens),
    Block(Block),
    FeildAcess(FieldAccess)
}

#[derive(Debug,Clone)]
pub struct FieldAccess{
    expr:Box<Expresion>,
    feild:Token, //Ident
}

pub type Block=Vec<Statment>;

pub type Parens = Box<Expresion>;

#[derive(Debug,Clone)]
pub struct Unary{
    unary_op:UnaryOp,
    epxr:Box<Expresion>,
}

#[derive(Debug,Clone)]
pub enum UnaryOp{
    Negate,
    Not,
}

#[derive(Debug,Clone)]
pub struct FuncCall{
    function:Box<Expresion>,
    arguments:Vec<Expresion>,
}

pub type VarAccess = Token;//Ident

#[derive(Debug,Clone)]
pub struct Binary{
    lhs:Box<Expresion>,
    rhs:Box<Expresion>,
    binary_op:BinaryOp,
}

#[derive(Debug,Clone)]
pub enum BinaryOp{
    Add,
    Subtract,
    Mult,
    Div,
    Mod,
    Lessthan,
    Greaterthan,
    GE,
    LE,
    Equal,
    LogicalAnd,
    LogicalOr,
}