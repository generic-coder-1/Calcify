use calcify::{chunk::{ByteCode, Chunk}, scanner::Scanner, virtual_machine::VirtualMachine};

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
    dbg!(Scanner::scan("while let or and ident ident2 _test Struct{}[]()<>->@* &;:;,#\"asd\" @".to_string()));
}