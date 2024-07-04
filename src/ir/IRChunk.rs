use std::collections::HashMap;

use super::ir_type_data::Function;

pub struct IRChunk{
    functions:HashMap<String,Function>,
    code:Vec<Code>,
}

pub enum Code{
    VarCreate(u16,Calc),
    VarAssign(u16,Calc),
    FieldAssign(u16,u16,Calc),
    Jmp(usize),
    JNE(ValueAccess,ValueAccess),
    Call(ValueAccess,Vec<u16>),
}

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

pub enum ValueAccess{
    VarAccess(u16),
    IntConst(u64),
    FloatConst(f64),
    StringConst(String),
    True,
    False,
    Func(Func),
}

pub struct Func{}