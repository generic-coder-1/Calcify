use std::{iter::Peekable, slice::Iter};

use pub_fields::pub_fields;

use crate::scanner::{Token, TokenType};

use super::{
    parser::{Parsable, ParseError, ParseResult, TokenExt, Wrapper}, statments::{Statment,Block},
};
#[derive(Debug, Clone)]
#[pub_fields]
pub struct Program {
    code: Vec<Declaration>,
}
#[derive(Debug, Clone)]

pub enum Declaration {
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    FunctionDecl(FunctionDecl),
    TraitDecl(TraitDecl),
    ImplDecl(ImplDecl),
}
#[derive(Debug, Clone)]

#[pub_fields]
pub struct ImplDecl {
    generics: Vec<GenericDecl>,
    trait_to_impl: Option<Type>,
    type_to_impl_on: Type,
    funcs: Vec<FunctionDecl>,
}
#[derive(Debug, Clone)]

#[pub_fields]
pub struct TraitDecl {
    name: Token, //Ident
    generics: Vec<GenericDecl>,
    funcs: Vec<FuncSig>,
}

#[derive(Debug, Clone)]

#[pub_fields]
pub struct FuncSig {
    name: Token, //Ident
    generics: Vec<GenericDecl>,
    parameters: Vec<(Token, Type)>, //Ident
    out: Type,
}
#[derive(Debug, Clone)]
#[pub_fields]
pub struct FunctionDecl {
    sig: FuncSig,
    body: Block,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct EnumDecl {
    name: Token, //Ident
    generics: Vec<GenericDecl>,
    varients: Vec<VarientDecl>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct VarientDecl {
    name: Token, //Ident
    fields: Vec<FieldDecl>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct StructDecl {
    name: Token, //Ident
    generics: Vec<GenericDecl>,
    fields: Vec<FieldDecl>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct GenericDecl {
    name: Token, //Ident
    constraints: Vec<Type>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct FieldDecl {
    name: Token, //Ident
    type_of: Type,
}

#[derive(Debug, Clone)]
pub enum Type{
    ActualType(SolidType),
    Unit,
    Array(SolidType),
    FP(FunctionPointer),
    DynamicType(Vec<SolidType>)
} 


#[derive(Debug, Clone)]
#[pub_fields]
pub struct FunctionPointer{
    args:Vec<Type>,
    out:Box<Type>,
}

#[derive(Debug, Clone)]
#[pub_fields]
pub struct SolidType{
    name: Token, //Ident or Self
    generics: Vec<Type>,
}

impl Parsable for Program {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        Ok(Self {
            code: {
                let mut code = vec![];
                while let Some(Token { token_type, .. }) = tokens.peek() {
                    if TokenType::EOF == *token_type {
                        break;
                    };
                    code.push(Declaration::parse(tokens)?);
                }
                code
            },
        })
    }
}

impl Parsable for Declaration {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        Ok({
            let token = tokens.peek().expect(
                "The only place this can be called we explicitly check that there is another token",
            );
            match token.token_type {
                TokenType::Struct => Self::StructDecl(StructDecl::parse(tokens)?),
                TokenType::Enum => Self::EnumDecl(EnumDecl::parse(tokens)?),
                TokenType::Fn => Self::FunctionDecl(FunctionDecl::parse(tokens)?),
                TokenType::Trait => Self::TraitDecl(TraitDecl::parse(tokens)?),
                TokenType::Impl => Self::ImplDecl(ImplDecl::parse(tokens)?),
                _ => Err(ParseError {
                    expected: vec![
                        TokenType::Struct,
                        TokenType::Enum,
                        TokenType::Fn,
                        TokenType::Trait,
                        TokenType::Impl,
                    ],
                    got: (*token).clone(),
                })?,
            }
        })
    }
}

impl Parsable for ImplDecl {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> Result<Self, ParseError> {
        tokens.consume(TokenType::Impl)?;
        let generics = tokens.optional_list_parse::<GenericDecl>(
            TokenType::LArrow,
            TokenType::Comma,
            TokenType::RArrow,
        )?;
        let mut type_to_impl_on = Type::parse(tokens)?;
        let mut trait_to_impl = None;
        if tokens.peek_consume(TokenType::For).is_ok() {
            trait_to_impl = Some(type_to_impl_on);
            type_to_impl_on = Type::parse(tokens)?;
        }
        let mut funcs = vec![];
        tokens.consume(TokenType::LBrace)?;
        while tokens.peek_consume(TokenType::RBrace).is_err() {
            funcs.push(FunctionDecl::parse(tokens)?);
        }
        Ok(Self {
            generics,
            trait_to_impl,
            type_to_impl_on,
            funcs,
        })
    }
}

impl Parsable for FunctionDecl{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,ParseError> {
        let sig = FuncSig::parse(tokens)?;
        let mut body = vec![];
        tokens.consume(TokenType::LBrace)?;
        while tokens.peek_consume(TokenType::RBrace).is_err(){
            body.push(Statment::parse(tokens)?);
        }
        Ok(Self{
            sig,
            body,
        })
    }
}

impl Parsable for TraitDecl {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        tokens.consume(TokenType::Trait)?;
        let name = tokens.consume(TokenType::Ident)?;
        let generics = tokens.optional_list_parse::<GenericDecl>(
            TokenType::LArrow,
            TokenType::Comma,
            TokenType::RArrow,
        )?;
        let funcs =
            tokens.list_parse(TokenType::LBrace, TokenType::SemiColon, TokenType::RBrace)?;
        Ok(Self {
            name,
            generics,
            funcs,
        })
    }
}

impl Parsable for FuncSig {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        tokens.consume(TokenType::Fn)?;
        let name = tokens.consume(TokenType::Ident)?;
        let generics = tokens.optional_list_parse::<GenericDecl>(
            TokenType::LArrow,
            TokenType::Comma,
            TokenType::RArrow,
        )?;
        let parameters = tokens.list_parse::<(Token, Type)>(
            TokenType::LParen,
            TokenType::Comma,
            TokenType::RParen,
        )?;
        let out = if tokens.peek_consume(TokenType::SmallArrow).is_ok(){
            Type::parse(tokens)?
        }else{
            Type::Unit
        };
        Ok(Self {
            name,
            generics,
            parameters,
            out,
        })
    }
}

impl Parsable for (Token, Type) {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> Result<Self, ParseError> {
        let name = tokens.consume(TokenType::Ident)?;
        tokens.consume(TokenType::Colon)?;
        let type_ = Type::parse(tokens)?;
        Ok((name, type_))
    }
}

impl Parsable for EnumDecl {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        tokens.consume(TokenType::Enum)?;
        let name = tokens.consume(TokenType::Ident)?;
        let generics = tokens.optional_list_parse::<GenericDecl>(
            TokenType::LArrow,
            TokenType::Comma,
            TokenType::RArrow,
        )?;
        let mut varients = vec![];
        tokens.consume(TokenType::LBrace)?;
        if tokens.peek_consume(TokenType::RBrace).is_err() {
            loop {
                varients.push(VarientDecl::parse(tokens)?);
                if tokens.peek_consume(TokenType::Comma).is_ok() {
                    continue;
                }
                tokens.consume(TokenType::RBrace)?;
                break;
            }
        }
        Ok(Self {
            name,
            generics,
            varients,
        })
    }
}

impl Parsable for VarientDecl {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let name = tokens.consume(TokenType::Ident)?;
        let fields = tokens.optional_list_parse::<FieldDecl>(
            TokenType::LBrace,
            TokenType::Comma,
            TokenType::RBrace,
        )?;
        Ok(Self { name, fields })
    }
}

impl Parsable for StructDecl {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let _ = tokens.consume(TokenType::Struct)?;
        let name = tokens.consume(TokenType::Ident)?;
        let generics = tokens.optional_list_parse::<GenericDecl>(
            TokenType::LArrow,
            TokenType::Comma,
            TokenType::RArrow,
        )?;
        let fields = tokens.list_parse(TokenType::LBrace, TokenType::Comma, TokenType::RBrace)?;
        Ok(Self {
            name: name,
            generics,
            fields,
        })
    }
}

impl Parsable for FieldDecl {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let name = tokens.consume(TokenType::Ident)?;
        tokens.consume(TokenType::Colon)?;
        let type_ = Type::parse(tokens)?;
        Ok(Self {
            name,
            type_of: type_,
        })
    }
}

impl Parsable for GenericDecl {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        let name = tokens.peek_consume(TokenType::Ident)?;
        let constraints = if tokens.peek_consume(TokenType::Colon).is_ok() {
            let mut temp = vec![];
            loop{
                temp.push(Type::parse(tokens)?);
                if tokens.peek_consume(TokenType::Plus).is_err(){
                    break;
                }
            }
            temp
        } else {
            vec![]
        };
        Ok(Self {
            name: name.clone(),
            constraints,
        })
    }
}

impl Parsable for Type {
    fn parse(tokens: &mut Peekable<Iter<Token>>) -> ParseResult<Self> {
        Ok(if tokens.peek().cannot_end().token_type == TokenType::LParen{
            let traits = tokens.list_parse::<SolidType>(TokenType::LParen, TokenType::Plus, TokenType::RParen)?;
            if traits.is_empty(){
                Self::Unit
            }else{
                Self::DynamicType(traits)
            }
        }else if tokens.peek().cannot_end().token_type == TokenType::Pipe{
            Self::FP(FunctionPointer::parse(tokens)?)
        }else if tokens.peek().cannot_end().token_type == TokenType::LBrack{
            tokens.consume(TokenType::LBrack)?;
            let res = Self::Array(SolidType::parse(tokens)?);
            tokens.consume(TokenType::RBrack)?;
            res
        }else{
            Self::ActualType(SolidType::parse(tokens)?)
        })
    }
}

impl Parsable for FunctionPointer{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,ParseError> {
        let args = tokens.list_parse(TokenType::Pipe, TokenType::Comma, TokenType::Pipe)?;
        let out = Box::new(if tokens.peek_consume(TokenType::Colon).is_ok(){
            Type::parse(tokens)?
        }else{
            Type::Unit
        });
        Ok(Self{
            args,
            out,
        })
    }
}

impl Parsable for SolidType{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,ParseError> {
        let type_ = tokens.consume_multiple(vec![TokenType::Ident, TokenType::Self_])?;
        let generics = tokens.optional_list_parse::<Type>(
            TokenType::LArrow,
            TokenType::Comma,
            TokenType::RArrow,
        )?;
        Ok(Self {
            name: type_,
            generics,
        })
    }
}
