use std::{fmt::Debug, marker::PhantomData};

use calcify::{ast::{decl::Program, parser::Parsable}, ir::ir_type_data::IR, scanner::Scanner};

// trait Bar<T>{}

// trait Biz{}

// impl<U> Bar<U> for Baz{}

// impl<T:Biz,U> Bar<U> for T {}

// struct Baz{}

// impl Biz for Baz{}


//struct Foo<T:Bar<U>,U>{val:PhantomData<T>}



fn main() {
    let tokens = Scanner::scan(
    r#"
    struct Foo<T<U>,U:(Foo<T> + Bar)>{}
    "#.to_string());
    dbg!(&tokens);
    let program = Program::parse(&mut tokens.iter().peekable());
    dbg!(&program);
    if let Ok(program) = &program{
        dbg!(IR::try_from(program));
    }
}