pub type Value = f64;
pub enum ValueType{
    F64,
    I64,
    U64,
    F32,
    I32,
    U32,
    I16,
    U16,
    I8,
    U8,
    FuncPointer,//4 bytes
    Pointer,//8 bytes
    Bool,//1 byte
    Char,//1 byte
}