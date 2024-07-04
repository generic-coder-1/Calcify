use std::fmt::Debug;

use calcify::{ast::{decl::Program, parser::Parsable}, ir::ir_type_data::IR, scanner::Scanner};



fn main() {

    let tokens = Scanner::scan(
    r#"
    impl Foo for Int{}
    "#.to_string());
    dbg!(&tokens);
    let program = Program::parse(&mut tokens.iter().peekable());
    dbg!(&program);
    if let Ok(program) = &program{
        dbg!(IR::try_from(program));
    }
}