use std::{iter::Peekable, slice::Iter};

use crate::scanner::{Token, TokenType};

use super::{decl::{FunctionDecl, Type}, expresions::Expresion, parser::{Parsable, ParseResult, TokenExt, Wrapper}};

#[derive(Debug,Clone)]
pub enum Statment{
    VarCreation(VarCreation),
    FuncCreation(FunctionDecl),
    Expresion(Expresion),
    If(If),
    While(While),
    Return(Return),
    Block(Block),
    Continue,
    Break, 
}

pub type Return = Option<Expresion>;
pub type Block=Vec<Statment>;

#[derive(Debug,Clone)]
pub struct While{
    condition:Expresion,
    statment:Box<Statment>,
}

#[derive(Debug,Clone)]
pub struct If{
    conditionals_and_statments:Vec<(IfType,Statment)>,
    else_statment:Option<Box<Statment>>,
}

#[derive(Debug,Clone)]
pub enum IfType{
    Boolean(Expresion),
    IfLet{
        pattern:Pattern,
        expresion:Expresion
    }
}

#[derive(Debug,Clone)]
pub struct Pattern{
    type_of:Type,
    varient:Option<Token>,//Ident
    constructor:Vec<(Token,Option<Pattern>)>,//Ident
}

#[derive(Debug,Clone)]
pub struct VarCreation{
    mutable:bool,
    name:Token, // Ident
    type_of:Option<Type>,
    value:Box<Statment>,
}

impl Parsable for Statment{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->ParseResult<Self> {
        Ok(match tokens.peek().cannot_end().token_type{
            TokenType::Let=>Self::VarCreation(VarCreation::parse(tokens)?),
            TokenType::Fn=>Self::FuncCreation(FunctionDecl::parse(tokens)?),
            TokenType::Continue=>{tokens.next();Self::Continue},
            TokenType::Break=>{tokens.next();Self::Break},
            TokenType::If=>Self::If(If::parse(tokens)?),
            TokenType::While=>Self::While(While::parse(tokens)?),
            TokenType::Return=>Self::Return(Return::parse(tokens)?),
            TokenType::LBrace => {tokens.next();Self::Block({let mut temp = vec![];while tokens.peek_consume(TokenType::RBrace).is_err(){temp.push(Statment::parse(tokens)?)}temp})},
            _=>{
                let temp = Self::Expresion(Expresion::parse(tokens)?);
                tokens.consume(TokenType::SemiColon)?;
                temp
            }
        })
    }
}

impl Parsable for Return{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        tokens.consume(TokenType::Return)?;
        Ok(if tokens.peek_consume(TokenType::SemiColon).is_err(){
            Some(Expresion::parse(tokens)?)
        }else{
            None
        })
    }
}

impl Parsable for While{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        tokens.consume(TokenType::While)?;
        let condition = Expresion::parse(tokens)?;
        let statment = Box::new(Statment::parse(tokens)?);
        Ok(Self{
            condition,
            statment,
        })
    }
}

impl Parsable for If{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        let mut ifs = vec![(IfType::parse(tokens)?,Statment::parse(tokens)?)];
        while (tokens.next().cannot_end().token_type == TokenType::Else)&&(tokens.next().cannot_end().token_type == TokenType::If) {
            let condition = IfType::parse(tokens)?;
            let statment = Statment::parse(tokens)?;
            ifs.push((condition,statment));
        }
        let else_statment = if tokens.peek_consume(TokenType::Else).is_ok(){
            Some(Box::new(Statment::parse(tokens)?))
        }else{
            None
        };
        Ok(Self{
            conditionals_and_statments: ifs,
            else_statment,
        })
    }
}

impl Parsable for IfType{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        tokens.consume(TokenType::If)?;
        Ok(if tokens.peek_consume(TokenType::Let).is_ok(){
            let pattern = Pattern::parse(tokens)?;
            let expr = Expresion::parse(tokens)?;
            Self::IfLet { pattern, expresion: expr }
        }else{
            Self::Boolean(Expresion::parse(tokens)?)
        })
    }
}

impl Parsable for Pattern{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        let type_of = Type::parse(tokens)?;
        let varient = if tokens.peek_consume(TokenType::Dot).is_ok(){
            Some(tokens.consume(TokenType::Ident)?)
        }else{
            None
        };
        let constructor = tokens.optional_list_parse(TokenType::LBrace, TokenType::Comma, TokenType::RBrace)?;
        Ok(Self{
            type_of,
            varient,
            constructor,
        })
    }
}

impl Parsable for (Token, Option<Pattern>){
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        let name = tokens.consume(TokenType::Ident)?;
        let pattern = if tokens.peek_consume(TokenType::Colon).is_ok(){
            Some(Pattern::parse(tokens)?)
        }else{
            None
        };
        Ok((name,pattern))
    }
}

impl Parsable for VarCreation{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,super::parser::ParseError> {
        tokens.consume(TokenType::Let)?;
        let mutable = tokens.peek_consume(TokenType::Mut).is_ok();
        let name = tokens.consume(TokenType::Ident)?;
        let type_of = if tokens.peek_consume(TokenType::Colon).is_ok(){
            Some(Type::parse(tokens)?)
        }else{
            None
        };
        tokens.consume(TokenType::Equal)?;
        let value = Box::new(Statment::parse(tokens)?);
        Ok(Self{
            mutable,
            name,
            type_of,
            value,
        })
    }
}