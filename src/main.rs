use calcify::{ast::{decl::Program, parser::Parsable}, scanner::Scanner};



fn main() {
    let tokens = Scanner::scan(
    r#"
    fn main(){
        panic;
        -09
    }
    "#.to_string());
    dbg!(&tokens);
    dbg!(Program::parse(&mut tokens.iter().peekable()));
}