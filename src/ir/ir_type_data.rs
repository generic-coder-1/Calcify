use std::collections::HashMap;

use pub_fields::pub_fields;

use crate::ast::decl::{EnumDecl, FieldDecl, FuncSig, FunctionPointer, GenericDecl, Program, SolidType, StructDecl, TraitDecl, Type as ASTType, VarientDecl};

use super::IRChunk::IRChunk;

#[derive(Debug,Clone)]
#[pub_fields]
pub struct IR{
    structs:HashMap<String,Struct>,
    enums:HashMap<String,Enum>,
    traits:HashMap<String,Trait>,
    impls:HashMap<String,Impl>,
    function:HashMap<String,Function>
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Function{
    tag:FuncTag,
    body:IRChunk
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Impl{
    generics:Vec<Generics>,
    trait_to_impl:ActualType,
    type_to_impl_on:Type,
    funcs:Vec<Function>
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Trait{
    generics:Vec<Generics>,
    func_tag:HashMap<String, FuncTag>,
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct FuncTag{
    generics:Vec<Generics>,
    inputs:Vec<(String, Type)>,
    output:Type
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Enum{
    generics:Vec<Generics>,
    varients:HashMap<String,Fields>,
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Struct{
    generics:Vec<Generics>,
    fields:Fields,
}

pub type Fields = HashMap<String,Type>;

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Generics{
    name:String,
    constraits:Vec<Type>,
}

#[derive(Debug,Clone)]
pub enum Type{
    Unit,
    Actual(ActualType),
    Array(ActualType),
    TraitType(Vec<ActualType>),
    FP(FP),
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct FP{
    arg:Vec<Type>,
    out:Box<Type>,
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct ActualType{
    name:String,
    types_in_generics:Vec<Type>
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct CompileError{}

impl TryFrom<&Program> for IR{
    type Error = CompileError;
    
    fn try_from(program: &Program) -> Result<Self, Self::Error> {
        let (structs, enums, traits, impls, functions) = 
            program.code.iter().fold((vec![],vec![],vec![],vec![],vec![]), |mut acc,decl|{
                match decl{
                    crate::ast::decl::Declaration::StructDecl(struct_) => acc.0.push(struct_),
                    crate::ast::decl::Declaration::EnumDecl(enum_) => acc.1.push(enum_),
                    crate::ast::decl::Declaration::TraitDecl(trait_) => acc.2.push(trait_),
                    crate::ast::decl::Declaration::FunctionDecl(function_) => acc.3.push(function_),
                    crate::ast::decl::Declaration::ImplDecl(impl_) => acc.4.push(impl_),
                }
                acc
            });
        let mut ir = IR{
            structs: structs.into_iter().map(<(String, Struct)>::from).collect::<HashMap<String,Struct>>(),
            enums: enums.into_iter().map(<(String, Enum)>::from).collect::<HashMap<String,Enum>>(),
            traits: traits.into_iter().map(<(String, Trait)>::from).collect::<HashMap<String,Trait>>(),
            impls: HashMap::new(),
            function: HashMap::new(),
        };
        //not done yet
        Ok(ir)
    }
}

impl From<&TraitDecl> for (String,Trait){
    fn from(value: &TraitDecl) -> Self {
        (
            value.name.lexeme.clone(),
            Trait{
                generics:value.generics.iter().map(Generics::from).collect::<Vec<Generics>>(),
                func_tag:value.funcs.iter().map(<(String,FuncTag)>::from).collect()
            }
        )
    }
}

impl From<&FuncSig> for (String, FuncTag){
    fn from(value: &FuncSig) -> Self {
        (
            value.name.lexeme.clone(),
            FuncTag{
                generics: value.generics.iter().map(Generics::from).collect::<Vec<Generics>>(),
                inputs: value.parameters.iter().map(|(tok,type_)|{(tok.lexeme.clone() ,Type::from(type_))}).collect(),
                output: Type::from(&value.out),
            }
        )
    }
}

impl From<&EnumDecl> for (String,Enum){
    fn from(value: &EnumDecl) -> Self {
        (
            value.name.lexeme.clone(),
            Enum{
                generics: value.generics.iter().map(Generics::from).collect::<Vec<Generics>>(),
                varients: value.varients.iter().map(<(String,HashMap<String,Type>)>::from).collect(),
            }
        )
    }
}

impl From<&VarientDecl> for (String,HashMap<String,Type>){
    fn from(value: &VarientDecl) -> Self {
        (
            value.name.lexeme.clone(),
            value.fields.iter().map(<(String,Type)>::from).collect::<HashMap<String,Type>>()
        )
    }
}

impl From<&StructDecl> for (String, Struct){

    fn from(value: &StructDecl) -> Self{
        (
            value.name.lexeme.clone(),
            Struct{
                generics: value.generics.iter().map(Generics::from).collect::<Vec<Generics>>(),
                fields: value.fields.iter().map(<(String,Type)>::from).collect::<HashMap<String,Type>>(),
            }
        )
    }
}

impl From<&FieldDecl> for (String, Type){
    fn from(value: &FieldDecl) -> Self {
        (
            value.name.lexeme.clone(),
            Type::from(&value.type_of)
        )
    }
}

impl From<&GenericDecl> for Generics{
    fn from(value: &GenericDecl) -> Self {
        Self { name: value.name.lexeme.clone(), constraits: value.constraints.iter().map(Type::from).collect() }
    }
}

impl From<&ASTType> for Type{
    fn from(value: &ASTType) -> Self {
        match value{
            ASTType::ActualType(actual_type) => Self::Actual(ActualType::from(actual_type)),
            ASTType::Unit => Self::Unit,
            ASTType::Array(actual_type) => Self::Array(ActualType::from(actual_type)),
            ASTType::FP(fp) => Self::FP(FP::from(fp)),
            ASTType::DynamicType(traits) => Self::TraitType(traits.iter().map(ActualType::from).collect()),
        }
    }
}

impl From<&FunctionPointer> for FP{
    fn from(value: &FunctionPointer) -> Self {
        Self { arg: value.args.iter().map(Type::from).collect(), out: Box::new(Type::from(&*value.out)) }
    }
}

impl From<&SolidType> for ActualType{
    fn from(value: &SolidType) -> Self {
        Self { name: value.name.lexeme.clone(), types_in_generics: value.generics.iter().map(Type::from).collect() }
    }
} 