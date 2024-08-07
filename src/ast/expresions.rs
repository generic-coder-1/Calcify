use std::{iter::Peekable, slice::Iter};

use pub_fields::pub_fields;

use crate::scanner::{Token, TokenType};

use super::{decl::{SolidType, Type}, parser::{Parsable, ParseError, ParseResult, TokenExt, Wrapper}, statments::{Block, Statment}};

#[derive(Debug,Clone)]
pub enum Expresion{
    IntLitteral(Token),//Int
    FloatLitteral(Token),//Float
    StringLitteral(Token),//String
    TypeAssocatedFunction(TypeAssocatedFunction),
    VarAccess(VarAccess),
    True,
    False,
    FunctionCall(FuncCall),
    FieldAcess(FieldAccess),
    Index(Index),
    Constructor(Constructor),
    Cast(Cast),
    Parens(Parens),
    Unary(Unary),
    Binary(Binary),
    Block(Block),
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct TypeAssocatedFunction{
    type_:Token,
    func_name:Token,
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Constructor{
    areana:Option<Box<Expresion>>,
    object_to_construct:ConstructorType
}

#[derive(Debug,Clone)]
pub enum ConstructorType{
    StructConstructor(StructConstructor),
    EnumConstructor(EnumConstructor),
    ArrayConstructor(ArrayConstructor),
}

pub type ArrayConstructor = Vec<Expresion>;

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Cast{
    expr:Box<Expresion>,
    traits_to_cast_to:Vec<SolidType>
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct EnumConstructor{
    enum_name:Token, //Ident
    varient_name:Token, //Ident
    fields:Vec<FieldConstructor>
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct StructConstructor{
    struct_name:Token, // Ident
    fields:Vec<FieldConstructor>,
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct FieldConstructor{
    name:Token, //Ident
    value:Box<Expresion>,
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Index{
    expr:Box<Expresion>,
    index:Box<Expresion>
}


#[derive(Debug,Clone)]
#[pub_fields]
pub struct FieldAccess{
    expr:Box<Expresion>,
    field:Token, //Ident
}

pub type Parens = Box<Expresion>;

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Unary{
    unary_op:UnaryOp,
    expr:Box<Expresion>,
}

#[derive(Debug,Clone)]
pub enum UnaryOp{
    Negate,
    Not,
}

#[derive(Debug,Clone)]
#[pub_fields]
pub struct FuncCall{
    function:Box<Expresion>,
    generics:Vec<Type>,
    arguments:Vec<Expresion>,
}

pub type VarAccess = Token;//Ident

#[derive(Debug,Clone)]
#[pub_fields]
pub struct Binary{
    lhs:Box<Expresion>,
    rhs:Box<Expresion>,
    binary_op:BinaryOp,
}

#[derive(Debug,Clone, PartialEq, Eq)]
pub enum BinaryOp{
    Add,
    Subtract,
    Mult,
    Div,
    Mod,
    Lessthan,
    Greaterthan,
    GE,
    LE,
    Equal,
    NE,
    LogicalAnd,
    LogicalOr, 
    Assign,
    BitwiseOr,
    BitwiseAnd,
    SHL,
    SHR,
}

impl Parsable for Expresion{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->ParseResult<Self> {
        enum OpOrExpr{
            Op(BinaryOp),
            Expr(Expresion),
        }
        let mut value_stack = vec![];
        let mut op_stack: Vec<BinaryOp> = vec![];
        value_stack.push(OpOrExpr::Expr(Self::primary(tokens)?));
        while BinaryOp::is_next_bin_op(tokens){
            let op = BinaryOp::parse(tokens)?;
            while let Some(last_op) = op_stack.last(){
                if (last_op.precedence() > op.precedence()) || ((last_op.precedence() == op.precedence()) && last_op.left_assocative()){
                    value_stack.push(OpOrExpr::Op(op_stack.pop().expect("just did comparisins with this value")));
                }else{
                    break;
                }
            }
            op_stack.push(op);
            value_stack.push(OpOrExpr::Expr(Expresion::primary(tokens)?));
        }
        while let Some(op) = op_stack.pop(){
            value_stack.push(OpOrExpr::Op(op));
        }
        let mut work_stack = vec![];
        value_stack.into_iter().for_each(|value|{
            match value{
                OpOrExpr::Op(op) => {
                    let right = work_stack.pop().unwrap();
                    let left = work_stack.pop().unwrap();
                    work_stack.push(Expresion::Binary(Binary{
                        lhs: Box::new(left),
                        rhs: Box::new(right),
                        binary_op: op,
                    }));
                },
                OpOrExpr::Expr(expr) => work_stack.push(expr),
            }
        });
        Ok(work_stack.remove(0))
    }
}

impl Parsable for BinaryOp{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,ParseError> {
        Ok(match tokens.consume_multiple(vec![
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Percent,
            TokenType::LArrow,
            TokenType::RArrow,
            TokenType::LessOrEqual,
            TokenType::MoreOrEqual,
            TokenType::EqualEqual,
            TokenType::BangEqual,
            TokenType::And,
            TokenType::Or,
            TokenType::Equal,
            TokenType::Ampersand,
            TokenType::Pipe,
            TokenType::SHL,
        ])?.token_type{
            TokenType::Plus=>Self::Add,
            TokenType::Minus=>Self::Subtract,
            TokenType::Star=>Self::Mult,
            TokenType::Slash=>Self::Div,
            TokenType::Percent=>Self::Mod,
            TokenType::LArrow=>Self::Lessthan,
            TokenType::RArrow=>{if tokens.peek_consume(TokenType::RArrow).is_ok(){Self::Greaterthan}else{Self::SHR}},
            TokenType::LessOrEqual=>Self::LE,
            TokenType::MoreOrEqual=>Self::GE,
            TokenType::EqualEqual=>Self::Equal,
            TokenType::BangEqual=>Self::NE,
            TokenType::And=>Self::LogicalAnd,
            TokenType::Or=>Self::LogicalOr,
            TokenType::Equal=>Self::Assign,
            TokenType::Ampersand=>Self::BitwiseAnd,
            TokenType::Pipe=>Self::BitwiseOr,
            TokenType::SHL=>Self::SHL,
            _=>unreachable!()
        })
    }
}

impl BinaryOp{
    fn is_next_bin_op(tokens:&mut Peekable<Iter<Token>>)->bool{
        match tokens.peek().cannot_end().token_type{
            TokenType::Plus|
            TokenType::Minus|
            TokenType::Star|
            TokenType::Slash|
            TokenType::Percent|
            TokenType::LArrow|
            TokenType::RArrow|
            TokenType::LessOrEqual|
            TokenType::MoreOrEqual|
            TokenType::EqualEqual|
            TokenType::BangEqual|
            TokenType::And|
            TokenType::Or|
            TokenType::SHL|
            TokenType::Equal => true,
            _=>false,
        }
    }
    fn precedence(&self)->u8{
        match self{
            BinaryOp::Mod => 9,
            BinaryOp::Mult => 9,
            BinaryOp::Div => 9,
            BinaryOp::Subtract => 8,
            BinaryOp::Add => 8,
            BinaryOp::SHL => 7,
            BinaryOp::SHR => 7,
            BinaryOp::BitwiseAnd=>6,
            BinaryOp::BitwiseOr=>5,
            BinaryOp::Lessthan => 4,
            BinaryOp::Greaterthan => 4,
            BinaryOp::GE => 4,
            BinaryOp::LE => 4,
            BinaryOp::Equal => 3,
            BinaryOp::NE => 3,
            BinaryOp::LogicalAnd => 2,
            BinaryOp::LogicalOr => 1,
            BinaryOp::Assign => 0,
        }
    }
    fn left_assocative(&self)->bool{
        match self{
            BinaryOp::Add => true,
            BinaryOp::Subtract => true,
            BinaryOp::Mult => true,
            BinaryOp::Div => true,
            BinaryOp::Mod => true,
            BinaryOp::Lessthan => true,
            BinaryOp::Greaterthan => true,
            BinaryOp::GE => true,
            BinaryOp::LE => true,
            BinaryOp::Equal => true,
            BinaryOp::NE => true,
            BinaryOp::LogicalAnd => true,
            BinaryOp::LogicalOr => true,
            BinaryOp::Assign => false,
            BinaryOp::BitwiseOr => true,
            BinaryOp::BitwiseAnd => true,
            BinaryOp::SHL => true,
            BinaryOp::SHR => true,
        }
    }
}

impl Expresion{
    fn primary(tokens:&mut Peekable<Iter<Token>>)->ParseResult<Self>{
        let mut expr  = match tokens.peek().cannot_end().token_type{
            TokenType::Minus | TokenType::Bang => Self::Unary(Unary::parse(tokens)?),
            TokenType::Int => Self::IntLitteral(tokens.next().expect("we just checked that there is another token").clone()),
            TokenType::Float => Self::FloatLitteral(tokens.next().expect("we just checked that there is another token").clone()),
            TokenType::String => Self::StringLitteral(tokens.next().expect("we just checked that there is another token").clone()),
            TokenType::True => {tokens.next();Self::True},
            TokenType::False => {tokens.next();Self::False},
            TokenType::LBrack => {
                let cnstrcr = ConstructorType::ArrayConstructor(ArrayConstructor::parse(tokens)?);
                Self::Constructor(Constructor { areana: 
                        if tokens.peek_consume(TokenType::At).is_ok(){
                            Some(Box::new(Expresion::primary(tokens)?))
                        }else{
                            None
                        },
                        object_to_construct:cnstrcr,
                    }
                )
            },
            TokenType::LBrace => {tokens.next();Self::Block({let mut temp = vec![];while tokens.peek_consume(TokenType::RBrace).is_err(){temp.push(Statment::parse(tokens)?)}temp})},
            TokenType::Ident => {
                let name = tokens.next().expect("we just checked that there is another token").clone();
                if tokens.peek_consume(TokenType::DoubleColon).is_ok(){
                    let varient_name = tokens.consume(TokenType::Ident)?;
                    let next_token = (*tokens.peek().cannot_end()).clone();
                    [next_token].iter().peekable().consume_multiple(vec![TokenType::LBrace, TokenType::LParen, TokenType::LArrow])?;
                    match tokens.peek().cannot_end().token_type{
                        TokenType::LBrace=>{
                            let fields = tokens.optional_list_parse(TokenType::LBrace, TokenType::Comma, TokenType::RBrace)?;
                            let cnstrcr = ConstructorType::EnumConstructor(EnumConstructor{
                                enum_name: name,
                                varient_name,
                                fields,
                            });
                            Self::Constructor(Constructor { areana: 
                                if tokens.peek_consume(TokenType::At).is_ok(){
                                    Some(Box::new(Expresion::primary(tokens)?))
                                }else{
                                    None
                                },
                                object_to_construct:cnstrcr,
                            })
                        },
                        TokenType::LArrow =>{
                            let generics = tokens.list_parse(TokenType::LArrow, TokenType::Comma, TokenType::RArrow)?;
                            let arguments = tokens.list_parse(TokenType::LParen, TokenType::Comma, TokenType::RParen)?;
                            Self::FunctionCall(FuncCall { function: Box::new(Self::TypeAssocatedFunction(TypeAssocatedFunction { type_: name, func_name: varient_name })), generics, arguments })
                        },
                        TokenType::LParen => {
                            let generics = vec![];
                            let arguments = tokens.list_parse(TokenType::LParen, TokenType::Comma, TokenType::RParen)?;
                            Self::FunctionCall(FuncCall { function: Box::new(Self::TypeAssocatedFunction(TypeAssocatedFunction { type_: name, func_name: varient_name })), generics, arguments })
                        }
                        _=>unreachable!()
                    }
                }else if tokens.peek().cannot_end().token_type == (TokenType::LBrace){
                    let fields = tokens.list_parse(TokenType::LBrace, TokenType::Comma, TokenType::RBrace)?;
                    let cnstrcr = ConstructorType::StructConstructor(StructConstructor{
                        struct_name: name,
                        fields,
                    });
                    Self::Constructor(Constructor { areana: 
                        if tokens.peek_consume(TokenType::At).is_ok(){
                            Some(Box::new(Expresion::primary(tokens)?))
                        }else{
                            None
                        },
                        object_to_construct:cnstrcr,
                    }
                )
                }else{
                    Self::VarAccess(name)
                }
            },
            TokenType::LParen=> {
                tokens.consume(TokenType::LParen)?;
                let expr = Expresion::parse(tokens)?;
                tokens.consume(TokenType::RParen)?;
                expr
            }
            _=>Err(ParseError{ expected: vec![TokenType::Minus,TokenType::Int,TokenType::Float,TokenType::String,TokenType::Ident,TokenType::True,TokenType::False, TokenType::LBrace], got: tokens.next().unwrap().clone() })?
        };
        if tokens.peek_consume(TokenType::Colon).is_ok(){
            expr = Self::Cast(Cast{ expr:Box::new(expr), traits_to_cast_to: tokens.list_parse::<SolidType>(TokenType::LParen, TokenType::Plus, TokenType::RParen)?});
        }
        loop {
            match tokens.peek(){
                Some(token) => {
                    match token.token_type{
                        TokenType::Dot=> {
                            tokens.next();
                            expr = Self::FieldAcess(FieldAccess { expr:Box::new(expr), field: tokens.consume(TokenType::Ident)? });
                        },
                        TokenType::LArrow =>{
                            let generics = tokens.list_parse(TokenType::LArrow, TokenType::Comma, TokenType::RArrow)?;
                            let arguments = tokens.list_parse(TokenType::LParen, TokenType::Comma, TokenType::RParen)?;
                            expr = Self::FunctionCall(FuncCall { function: Box::new(expr), generics, arguments })
                        }
                        TokenType::LParen => {
                            let generics = vec![];
                            let arguments = tokens.list_parse(TokenType::LParen, TokenType::Comma, TokenType::RParen)?;
                            expr = Self::FunctionCall(FuncCall { function: Box::new(expr), generics, arguments })
                        }
                        TokenType::LBrack => {
                            tokens.next();
                            expr = Self::Index(Index { expr: Box::new(expr), index: Box::new(Expresion::parse(tokens)?)});
                            tokens.consume(TokenType::RBrack)?;
                        }
                        _=>break,
                    }
                },
                None => break,
            }
        }
        Ok(expr)
    }
}

impl Parsable for ArrayConstructor{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,ParseError> {
        tokens.list_parse(TokenType::LBrack, TokenType::Comma, TokenType::RBrack)
    }
}

impl Parsable for FieldConstructor{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,ParseError> {
        let name = tokens.consume(TokenType::Ident)?;
        tokens.consume(TokenType::Colon)?;
        let value = Box::new(Expresion::parse(tokens)?);
        Ok(Self{
            name,
            value,
        }) 
    }
}

impl Parsable for Unary{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        let op = UnaryOp::parse(tokens)?;
        let expr = Box::new(Expresion::primary(tokens)?);
        Ok(Self{
            unary_op: op,
            expr,
        })
    }
}

impl Parsable for UnaryOp{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        Ok(match tokens.peek_consume_multiple(vec![TokenType::Minus,TokenType::Bang])?.token_type {
            TokenType::Minus=>Self::Negate,
            TokenType::Bang=>Self::Not,
            _=>unreachable!("we just checked if it is of one of the two type")
        })
    }
}