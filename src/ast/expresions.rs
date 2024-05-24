use crate::scanner::Token;

use super::{parser::Parsable, statments::Statment};

#[derive(Debug,Clone)]
pub enum Expresion{
    IntLitteral(Token),//Int
    FloatLitteral(Token),//Float
    StringLitteral(Token),//String
    True,
    False,
    FunctionCall(FuncCall),
    VarAccess(VarAccess),
    Product(Binary),
    Unary(Unary),
    Parens(Parens),
    Block(Block),
    FieldAcess(FieldAccess)
}

#[derive(Debug,Clone)]
pub struct FieldAccess{
    expr:Box<Expresion>,
    field:Token, //Ident
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

impl Parsable for Expresion{
    fn parse(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>)->Result<Self,super::parser::ParseError> {
        todo!()
    }
}