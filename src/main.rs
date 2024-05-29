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
        let a = 2+3.not_a_func("hi")[0];
        a=5;
        while true {
            print<String>("kys");
            return 0
        }
        while false a+b;
    }
    "#.to_string());
    dbg!(&tokens);
    dbg!(Program::parse(&mut tokens.iter().peekable()));
}