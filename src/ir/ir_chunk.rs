use std::collections::HashMap;

use pub_fields::pub_fields;

use super::ir_type_data::Function;

#[derive(Debug,Clone)]
#[pub_fields]
pub struct IRChunk{
    functions:HashMap<String,Function>,
    code:Vec<IRCode>,
}

#[derive(Debug,Clone)]
pub enum IRCode{
    VarCreate(u16,Calc),
    VarAssign(u16,Calc),
    FieldAssign(u16,u16,Calc),
    Jmp(usize),
    JNE(ValueAccess,ValueAccess),
    Call(ValueAccess,Vec<u16>),
}

#[derive(Debug,Clone)]
pub enum Calc{
    Alloc(u16),
    Clone(ValueAccess),
    ArrayAcess(ValueAccess,ValueAccess),
    FieldAccess(ValueAccess,u16),
    Add(ValueAccess,ValueAccess),
    Subtract(ValueAccess,ValueAccess),
    Multiply(ValueAccess,ValueAccess),
    Div(ValueAccess,ValueAccess),
    Mod(ValueAccess, ValueAccess),
    EQ(ValueAccess, ValueAccess),
    NE(ValueAccess, ValueAccess),
    LT(ValueAccess, ValueAccess),
    GT(ValueAccess, ValueAccess),
    LE(ValueAccess, ValueAccess),
    GE(ValueAccess, ValueAccess),
    BITAnd(ValueAccess, ValueAccess),
    BITOr(ValueAccess, ValueAccess),
    SHL(ValueAccess, ValueAccess),
    SHR(ValueAccess, ValueAccess),
    Negate(ValueAccess),
    Not(ValueAccess),
}

#[derive(Debug,Clone)]
pub enum ValueAccess{
    VarAccess(u16),
    IntConst(u64),
    FloatConst(f64),
    StringConst(Box<str>),
    True,
    False,
    Func(String),
}