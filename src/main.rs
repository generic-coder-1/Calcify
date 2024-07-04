use calcify::{ast::{decl::Program, parser::Parsable}, scanner::Scanner};



fn main() {
    let tokens = Scanner::scan(
    r#"
    fn main(a:(Printable + Yes<()>))->Int{
        [9,0]@Arena::new<[A]>();
    }
    "#.to_string());
    dbg!(&tokens);
    dbg!(Program::parse(&mut tokens.iter().peekable()));
}