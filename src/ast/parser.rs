use std::{iter::Peekable, slice::Iter};
use crate::scanner::{Token, TokenType};

pub type ParseResult<T> = Result<T,ParseError>;

#[derive(Debug)]
pub struct ParseError{
    pub expected:Vec<TokenType>,
    pub got:Token,
}

pub trait Parsable
where Self:Sized{
    fn parse(tokens: &mut Peekable<Iter<Token>>)->Result<Self,ParseError>;
}

pub trait TokenExt{
    fn consume(&mut self, token_type:TokenType) -> ParseResult<Token>;
    fn peek_consume(&mut self, token_type:TokenType) -> ParseResult<Token>;
    fn consume_multiple(&mut self, token_types:Vec<TokenType>) -> ParseResult<Token>;
    fn peek_consume_multiple(&mut self, token_types:Vec<TokenType>) -> ParseResult<Token>;
    fn optional_list_parse<T:Parsable>(&mut self, start:TokenType,delimter:TokenType,end:TokenType)->ParseResult<Vec<T>>;
    fn list_parse<T:Parsable>(&mut self, start:TokenType,delimter:TokenType,end:TokenType)->ParseResult<Vec<T>>;
}

impl TokenExt for Peekable<Iter<'_,Token>>{
    fn consume(&mut self, token_type:TokenType) -> ParseResult<Token> {
        self.consume_multiple(vec![token_type])
    }
    fn peek_consume(&mut self, token_type:TokenType) -> ParseResult<Token> {
        self.peek_consume_multiple(vec![token_type])
    }
    fn consume_multiple(&mut self, token_types:Vec<TokenType>) -> ParseResult<Token> {
        let res = self.peek_consume_multiple(token_types);
        if res.is_err(){
            self.next();
        }
        res
    }
    fn peek_consume_multiple(&mut self, token_types:Vec<TokenType>) -> ParseResult<Token> {
        let token = (*self.peek().cannot_end()).clone();
        if token_types.contains(&token.token_type){
            self.next();
            return Ok(token)
        }else{
            Err(ParseError{
                expected:token_types,
                got:token,
            })
        }
    }
    fn optional_list_parse<T:Parsable>(&mut self, start:TokenType,delimter:TokenType,end:TokenType)->ParseResult<Vec<T>>{
        let mut list = vec![];
        if self.peek_consume(start).is_ok() && self.peek_consume(end).is_err(){
            loop {
                let type_ = T::parse(self)?;
                list.push(type_);
                if self.peek_consume(delimter).is_ok(){
                    continue;
                }
                self.consume(end)?;
                break;
            }
        }
        Ok(list)
    }
    fn list_parse<T:Parsable>(&mut self, start:TokenType,delimter:TokenType,end:TokenType)->ParseResult<Vec<T>>{
        let mut list = vec![];
        self.consume(start)?;
        if self.peek_consume(end).is_err(){
            loop {
                let type_ = T::parse(self)?;
                list.push(type_);
                if self.peek_consume(delimter).is_ok(){
                    continue;
                }
                self.consume(end)?;
                break;
            }
        }
        Ok(list)
    }
}

pub trait Wrapper<T>{
    fn cannot_end(self)->T;
}

impl<T> Wrapper<T> for Option<T>{
    fn cannot_end(self)->T {
        self.expect("Should've found the end of file before getting here")
    }
}