use crate::scanner::Token;

pub type Type = Token; //Ident
pub struct Program{
    code:Vec<Declaration>
}

pub enum Declaration{
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    FunctionDecl(FunctionDecl),
    VarDecl(VarDecl),
    TraitDecl(TraitDecl),
    ImplDecl(ImplDecl),
    Statement(Statement),
}

pub struct EnumDecl{
    name:Type, //Ident
    fields: Vec<(Token,Option<StructField>)> //Ident
}

pub type StructField = (Token, Type); // Ident 

pub struct TraitDecl{
    name:Type, // Ident
    functions:Vec<FunctionType>,
}

pub struct ImplDecl{
    cal_trait:Option<Type>, //Ident
    cal_type:Type, //Ident
    functions:Vec<Function>,
}

pub enum Statement{
    Expr(Expresion),
    If(If),
    IfLet(IfLet),
    Return(Return),
    While(While),
    Block(Block)
}

pub struct IfLet{
    if_lets:OneOrMore<(Pattern,Expresion,Box<Statement>)>,
    else_statment:Box<Statement>,
}

pub struct Pattern{
    type_of:Type,//Ident
    varient:Option<Token>, //Ident; only for enums
    fields:Vec<(Token,PatternType)>
}

pub enum PatternType{
    SubPattern(Box<Pattern>),
    Var(Token),//Ident
    Constant(Token) //Num, String, True, False
}

pub struct While{
    expr:Expresion,
    statment:Box<Statement>
}

pub struct Return{
    return_val:Option<Expresion>,
}

pub struct If{
    if_and_else_if:OneOrMore<(Expresion,Box<Statement>)>,
    else_statment:Option<Box<Statement>>,
}

pub struct StructDecl{
    name:Type, //Ident
    fields:Vec<StructField> //(Ident, Ident)
}

pub type FunctionDecl = Function;

pub struct FunctionType{
    name:Token, //Ident
    params:Vec<(Token,Type)>, //Vec<(Ident,Ident)>}
    out:Type, //Ident
}

pub struct Function{
    f_type:FunctionType,
    block:Block
}

pub type Block = Vec<Declaration>;

pub struct VarDecl{
    name:Token, // Ident
    type_of:Type,
    expresion:Expresion,
}

pub enum Expresion{
    Assignment(Assignment),
    Or(Or)
}

pub struct Assignment{
    call:Option<Call>,
    ident:Token, // Ident
    assignment_type:Box<Expresion>,
}

pub type Or = OneOrMore<And>;
pub type And = OneOrMore<Equality>;
pub type Equality = OneOrMoreWith<Comparison,EqualityType>;
pub enum EqualityType{Eq, Neq}
pub type Comparison = OneOrMoreWith<Term,ComparisonType>;
pub enum ComparisonType{Great,GreaterOrEqual,Less,LessOrEqual}
pub type Term = OneOrMoreWith<Factor,TermType>;
pub enum TermType{Add, Subtract}
pub type Factor = OneOrMoreWith<Unary,FactorType>;
pub enum FactorType{Mult,Div}
pub enum Unary{
    Unary(UnaryType,Box<Unary>),
    Call(Call),
}
pub enum UnaryType{Not, Negative}


pub struct Call{
    primary:Primary,
    call_type:Vec<CallType>
}

pub enum Primary{
    Token(Token), //True | False | Self | Num | String | Ident
    Expr(Box<Expresion>),
    StructCreation(StructCreation),
    EnumCreation(EnumCreation),
}

pub struct StructCreation{
    name:Type, //Ident
    fields:Fields
}
pub struct EnumCreation{
    name:Type, //Ident
    varient:Token, //Ident
    fields:Fields
}


pub type Fields = Vec<(Token,Box<Expresion>)>; // Ident;
pub enum CallType{
    Args(Vec<Expresion>),
    StructField(Token), // Ident
}


pub struct OneOrMoreWith<T,U>{
    first:T,
    rest:Vec<(T,U)>
}
impl<T,U> Into<Vec<(T,Option<U>)>> for OneOrMoreWith<T,U>{
    fn into(self) -> Vec<(T,Option<U>)> {
        let mut res = self.rest.into_iter().map(|(t,u)|(t,Some(u))).collect::<Vec<(T,Option<U>)>>();
        res.insert(0, (self.first,None));
        res        
    }
}
pub struct OneOrMore<T>{
    first:T,
    rest:Vec<T>
}

impl<T> Into<Vec<T>> for OneOrMore<T>{
    fn into(mut self) -> Vec<T> {
        self.rest.insert(0, self.first);
        self.rest
    }
}