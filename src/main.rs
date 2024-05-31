use std::fmt::Debug;

use calcify::{ast::{decl::Program, parser::Parsable}, scanner::Scanner};



fn main() {
    // dbg!('_'.is_alphanumeric());
    // let mut chunk = Chunk::new();
    // chunk.write_chunk(ByteCode::Constant);
    // let constant = chunk.write_constant(1.2);
    // chunk.write_chunk(constant);
    // chunk.write_chunk(ByteCode::Negate);
    // chunk.write_chunk(0);
    // chunk.write_chunk(0);
    // chunk.write_chunk(0);
    // chunk.write_chunk(0);
    // chunk.disassemble("test code");
    // println!("== end of test code ==\n\n");
    // dbg!(VirtualMachine::run(&chunk));
    let tokens = Scanner::scan(
    r#"
    fn main()->Int{
        let a = 1;
        -a;
    }
    "#.to_string());
    dbg!(&tokens);
    dbg!(Program::parse(&mut tokens.iter().peekable()));
}