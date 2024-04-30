use crate::scanner::Token;

use super::statments::{Block, Statment};
pub struct Program{
    code:Vec<Declaration>
}

pub enum Declaration{
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    FunctionDecl(FunctionDecl),
    TraitDecl(TraitDecl),
    ImplDecl(ImplDecl),
}

pub struct ImplDecl{
    generics:Vec<GenericDecl>,
    trait_to_impl:Option<Type>,
    type_to_impl_on:Type,
    funcs:Vec<FunctionDecl>,
}

pub struct TraitDecl{
    name:Token, //Ident
    generics:Vec<GenericDecl>,
    funcs:Vec<FuncSig>,
}


pub struct FuncSig{
    name:Token, //Ident
    generics:Vec<GenericDecl>,
    parameters:Vec<(Token,Type)>,//Ident
    out:Type,
}

pub struct FunctionDecl{
    sig:FuncSig,
    body:Block,
}

pub struct EnumDecl{
    name:Type,
    varients:Vec<VarientDecl>
}

pub struct VarientDecl{
    name:Token,//Ident
    fields:Vec<FieldDecl>,
}

pub struct StructDecl{
    name:Token,//Ident
    generics:Vec<GenericDecl>,
    fields:Vec<FieldDecl>,
}

pub struct GenericDecl{
    name:Token,//Ident
    constraints:Vec<Type>
}

pub struct FieldDecl{
    name:Token, //Ident
    type_of:Type,
}

pub enum Type{
    ConcreteType(ConcreteType),
    Generic(Token),//Ident
    SelfTk(Token),//Self
}

pub struct ConcreteType{
    name:Token, //Ident
    generics:Vec<Box<Type>>
}