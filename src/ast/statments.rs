use crate::scanner::Token;

use super::{decl::{FunctionDecl, Type}, expresions::Expresion};

pub enum Statment{
    //FuncCall(FuncCall), move to expresoin
    //VarAccess(VarAccess), move to expression
    VarCreation(VarCreation),
    FuncCreation(FunctionDecl),
    VarAssignment(VarAssignment),
    Block(Block),
    Expresion(Expresion),
    If(If),
    While(While),
    Return(Return),
    Continue,
    Break, 
}

pub type Return = Expresion;

pub struct While{
    condition:Expresion,
    statment:Box<Statment>,
}

pub struct If{
    conditionals_and_statments:Vec<(IfType,Statment)>,
    else_statment:Option<Box<Statment>>,
}

pub enum IfType{
    Boolean(Expresion),
    IfLet{
        pattern:Pattern,
        expresion:Expresion
    }
}

pub struct Pattern{
    type_of:Type,
    varient:Option<Token>,//Ident
    comstructor:Vec<(Token,Option<Pattern>)>,//Ident
}

pub struct VarCreation{
    mutable:bool,
    name:Token, // Ident
    type_of:Option<Type>,
    value:Box<Statment>,
}


//move to expresion
// pub struct FuncCall{
//     call_from:Option<Box<Statment>>,
//     func_name_to_call:Token, //Ident
//     args:Vec<Statment>,
// }

//pub type VarAccess = Token; // Ident move to expresion

pub struct VarAssignment{
    name:Token, //Ident
    value:Box<Statment>
}

pub type Block=Vec<Statment>;