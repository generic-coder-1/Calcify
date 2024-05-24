use crate::scanner::Token;

use super::{decl::{FunctionDecl, Type}, expresions::Expresion};

#[derive(Debug,Clone)]
pub enum Statment{
    VarCreation(VarCreation),
    FuncCreation(FunctionDecl),
    VarAssignment(VarAssignment),
    Expresion(Expresion),
    If(If),
    While(While),
    Return(Return),
    Continue,
    Break, 
}

pub type Return = Expresion;

#[derive(Debug,Clone)]
pub struct While{
    condition:Expresion,
    statment:Box<Statment>,
}

#[derive(Debug,Clone)]
pub struct If{
    conditionals_and_statments:Vec<(IfType,Statment)>,
    else_statment:Option<Box<Statment>>,
}

#[derive(Debug,Clone)]
pub enum IfType{
    Boolean(Expresion),
    IfLet{
        pattern:Pattern,
        expresion:Expresion
    }
}

#[derive(Debug,Clone)]
pub struct Pattern{
    type_of:Type,
    varient:Option<Token>,//Ident
    comstructor:Vec<(Token,Option<Pattern>)>,//Ident
}

#[derive(Debug,Clone)]
pub struct VarCreation{
    mutable:bool,
    name:Token, // Ident
    type_of:Option<Type>,
    value:Box<Statment>,
}

#[derive(Debug,Clone)]
pub struct VarAssignment{
    name:Token, //Ident
    value:Box<Statment>
}

