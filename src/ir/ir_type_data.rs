use std::collections::HashMap;

use itertools::Itertools;
use pub_fields::pub_fields;

use crate::ast::decl::{
    self, EnumDecl, FieldDecl, FuncSig, FunctionDecl, FunctionPointer, GenericDecl, ImplDecl, Program, SolidType, StructDecl, TraitDecl, Type as ASTType, VarientDecl
};

use super::{ir_chunk::IRChunk, ir_validator::IRValidator};

#[derive(Debug, Clone)]
#[pub_fields]
pub struct IR {
    structs: HashMap<String, Struct>,
    enums: HashMap<String, Enum>,
    traits: HashMap<String, Trait>,
    impls: Vec<Impl>,
    function: HashMap<String, Function>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct Function {
    tag: FuncTag,
    body: IRChunk,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct Impl {
    generics: Vec<Generic>,
    trait_to_impl: Option<ActualType>,
    type_to_impl_on: Type,
    funcs: Vec<(String, Function)>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct Trait {
    generics: Vec<Generic>,
    func_tag: HashMap<String, FuncTag>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct FuncTag {
    generics: Vec<Generic>,
    inputs: Vec<(String, Type)>,
    output: Type,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct Enum {
    generics: Vec<Generic>,
    varients: HashMap<String, Fields>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct Struct {
    generics: Vec<Generic>,
    fields: Fields,
}

pub type Fields = HashMap<String, Type>;

#[derive(Debug, Clone)]
#[pub_fields]
pub struct Generic {
    name: String,
    constraits: Vec<TraitType>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Unit,
    Actual(ActualType),
    Array(ActualType),
    DynType(Vec<TraitType>),
    FP(FP),
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct FP {
    arg: Vec<Type>,
    out: Box<Type>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct ActualType {
    name: String,
    types_in_generics: Vec<Type>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct TraitType {
    name: String,
    types_in_generics: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum CompileError {
    GenericsCantHaveTypeParameters(ActualType),
    NumberOfGenericsAndTypesGivenDoNotMatchInItem(String,usize, usize),
    OnlyTraitsCanBeImplimentedOn(Type),
    NoTraitWithThatNameFound(String),
    NoTypeWithThatNameFound(Type),
    DuplicateStructs(Vec<String>),
    DuplicateEnums(Vec<String>),
    DuplicateTraits(Vec<String>),
    DuplicateFunctions(Vec<String>),
}



impl TryFrom<&Program> for IR {
    type Error = CompileError;

    fn try_from(program: &Program) -> Result<Self, Self::Error> {
        let (structs, enums, traits, impls, functions) =
            program
                .code
                .iter()
                .fold((vec![], vec![], vec![], vec![], vec![]), |mut acc, decl| {
                    match decl {
                        crate::ast::decl::Declaration::StructDecl(struct_) => acc.0.push(struct_),
                        crate::ast::decl::Declaration::EnumDecl(enum_) => acc.1.push(enum_),
                        crate::ast::decl::Declaration::TraitDecl(trait_) => acc.2.push(trait_),
                        crate::ast::decl::Declaration::ImplDecl(impl_) => acc.3.push(impl_),
                        crate::ast::decl::Declaration::FunctionDecl(function_) => {
                            acc.4.push(function_)
                        }
                    }
                    acc
                });
        let duplicate_things = structs.iter().map(|thing|thing.name.lexeme.clone()).duplicates().collect_vec();
        if !duplicate_things.is_empty(){
            Err(CompileError::DuplicateStructs(duplicate_things))?;
        }
        let duplicate_things = enums.iter().map(|thing|thing.name.lexeme.clone()).duplicates().collect_vec();
        if !duplicate_things.is_empty(){
            Err(CompileError::DuplicateEnums(duplicate_things))?;
        }
        let duplicate_things = traits.iter().map(|thing|thing.name.lexeme.clone()).duplicates().collect_vec();
        if !duplicate_things.is_empty(){
            Err(CompileError::DuplicateTraits(duplicate_things))?;
        }
        let duplicate_things = functions.iter().map(|thing|thing.sig.name.lexeme.clone()).duplicates().collect_vec();
        if !duplicate_things.is_empty(){
            Err(CompileError::DuplicateFunctions(duplicate_things))?;
        }
        let mut ir = IR {
            structs: structs
                .into_iter()
                .map(<(String, Struct)>::from)
                .collect::<HashMap<String, Struct>>(),
            enums: enums
                .into_iter()
                .map(<(String, Enum)>::from)
                .collect::<HashMap<String, Enum>>(),
            traits: traits
                .into_iter()
                .map(<(String, Trait)>::from)
                .collect::<HashMap<String, Trait>>(),
            impls: impls
                .into_iter()
                .map(Impl::try_from)
                .collect::<Result<Vec<Impl>, CompileError>>()?,
            function: functions
                .into_iter()
                .map(<(String, Function)>::from)
                .collect::<HashMap<String, Function>>()
        };
        
        IRValidator::validate(&ir);

        Ok(ir)
    }
}

impl TryFrom<&ImplDecl> for Impl {
    type Error = CompileError;

    fn try_from(value: &ImplDecl) -> Result<Self, Self::Error> {
        Ok(Self {
            generics: value
                .generics
                .iter()
                .map(Generic::from)
                .collect::<Vec<Generic>>(),
            trait_to_impl: {
                if let Some(type_) = &value.trait_to_impl {
                    Some({
                        match Type::from(type_) {
                            Type::Actual(actual) => actual,
                            other => return Err(CompileError::OnlyTraitsCanBeImplimentedOn(other)),
                        }
                    })
                } else {
                    None
                }
            },
            type_to_impl_on: Type::from(&value.type_to_impl_on),
            funcs: value.funcs.iter().map(<(String, Function)>::from).collect(),
        })
    }
}

impl From<&FunctionDecl> for (String, Function) {
    fn from(value: &FunctionDecl) -> Self {
        (
            value.sig.name.lexeme.clone(),
            Function {
                tag: <(String, FuncTag)>::from(&value.sig).1,
                body: todo!(),
            },
        )
    }
}

impl From<&TraitDecl> for (String, Trait) {
    fn from(value: &TraitDecl) -> Self {
        (
            value.name.lexeme.clone(),
            Trait {
                generics: value
                    .generics
                    .iter()
                    .map(Generic::from)
                    .collect::<Vec<Generic>>(),
                func_tag: value.funcs.iter().map(<(String, FuncTag)>::from).collect(),
            },
        )
    }
}

impl From<&FuncSig> for (String, FuncTag) {
    fn from(value: &FuncSig) -> Self {
        (
            value.name.lexeme.clone(),
            FuncTag {
                generics: value
                    .generics
                    .iter()
                    .map(Generic::from)
                    .collect::<Vec<Generic>>(),
                inputs: value
                    .parameters
                    .iter()
                    .map(|(tok, type_)| (tok.lexeme.clone(), Type::from(type_)))
                    .collect(),
                output: Type::from(&value.out),
            },
        )
    }
}

impl From<&EnumDecl> for (String, Enum) {
    fn from(value: &EnumDecl) -> Self {
        (
            value.name.lexeme.clone(),
            Enum {
                generics: value
                    .generics
                    .iter()
                    .map(Generic::from)
                    .collect::<Vec<Generic>>(),
                varients: value
                    .varients
                    .iter()
                    .map(<(String, HashMap<String, Type>)>::from)
                    .collect(),
            },
        )
    }
}

impl From<&VarientDecl> for (String, HashMap<String, Type>) {
    fn from(value: &VarientDecl) -> Self {
        (
            value.name.lexeme.clone(),
            value
                .fields
                .iter()
                .map(<(String, Type)>::from)
                .collect::<HashMap<String, Type>>(),
        )
    }
}

impl From<&StructDecl> for (String, Struct) {
    fn from(value: &StructDecl) -> Self {
        (
            value.name.lexeme.clone(),
            Struct {
                generics: value
                    .generics
                    .iter()
                    .map(Generic::from)
                    .collect::<Vec<Generic>>(),
                fields: value
                    .fields
                    .iter()
                    .map(<(String, Type)>::from)
                    .collect::<HashMap<String, Type>>(),
            },
        )
    }
}

impl From<&FieldDecl> for (String, Type) {
    fn from(value: &FieldDecl) -> Self {
        (value.name.lexeme.clone(), Type::from(&value.type_of))
    }
}

impl From<&GenericDecl> for Generic {
    fn from(value: &GenericDecl) -> Self {
        Self {
            name: value.name.lexeme.clone(),
            constraits: value.constraints.iter().map(TraitType::from).collect() ,
        }
    }
}

impl From<&decl::TraitType> for TraitType{
    fn from(value: &decl::TraitType) -> Self {
        Self {
            name: value.name.lexeme.clone(),
            types_in_generics: value.generics.iter().map(Type::from).collect(),
        }
    }
}

impl From<&ASTType> for Type {
    fn from(value: &ASTType) -> Self {
        match value {
            ASTType::PhysicalType(actual_type) => Self::Actual(ActualType::from(actual_type)),
            ASTType::Unit => Self::Unit,
            ASTType::Array(actual_type) => Self::Array(ActualType::from(actual_type)),
            ASTType::FP(fp) => Self::FP(FP::from(fp)),
            ASTType::DynamicType(traits) => {
                Self::DynType(traits.iter().map(TraitType::from).collect())
            }
        }
    }
}

impl From<&FunctionPointer> for FP {
    fn from(value: &FunctionPointer) -> Self {
        Self {
            arg: value.args.iter().map(Type::from).collect(),
            out: Box::new(Type::from(&*value.out)),
        }
    }
}

impl From<&SolidType> for ActualType {
    fn from(value: &SolidType) -> Self {
        Self {
            name: value.name.lexeme.clone(),
            types_in_generics: value.generics.iter().map(Type::from).collect(),
        }
    }
}
