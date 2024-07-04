use std::collections::HashMap;

use super::IRChunk::IRChunk;

pub struct IR{
    structs:HashMap<String,Struct>,
    enums:HashMap<String,Enum>,
    traits:HashMap<String,Trait>,
    impls:HashMap<String,Impl>,
    function:HashMap<String,Function>
}

pub struct Function{
    tag:FuncTag,
    body:IRChunk
}

pub struct Impl{
    generics:Vec<Generics>,
    trait_to_impl:ActualType,
    type_to_impl_on:Type,
    funcs:Vec<Function>
}

pub struct Trait{
    generics:Vec<Generics>,
    func_tag:Vec<FuncTag>,
}

pub struct FuncTag{
    inputs:Vec<(String,Type)>,
    output:Type
}
pub struct Enum{
    generics:Vec<Generics>,
    varients:HashMap<String,Fields>,
}

pub struct Struct{
    generics:Vec<Generics>,
    fields:Fields,
}

pub type Fields = HashMap<String,Type>;

pub struct Generics{
    name:String,
    constraits:Vec<Type>,
}

pub enum Type{
    Unit,
    Actual(ActualType),
    Array(ActualType)
}
pub struct ActualType{
    name:String,
    types_in_generics:Vec<ActualType>
}