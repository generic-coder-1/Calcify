use calcify::{ast::{decl::Program, parser::Parsable}, chunk::{ByteCode, Chunk}, scanner::Scanner, virtual_machine::VirtualMachine};

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
    let tokens = Scanner::scan("struct Foo<T:Iter+No> { bar:Int, baz:T} enum Option<T>{Some{value:T},None} trait Iter{fn next(self:Self)} impl<T> What for Option<T>{} fn main(argv:Array<Char>){}".to_string());
    dbg!(&tokens);
    dbg!(Program::parse(&mut tokens.iter().peekable()));
}