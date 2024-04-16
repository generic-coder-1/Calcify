use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::{stdin, stdout, Write},
    rc::Rc,
};

use calcify::ast::{Environment, Expr, ExtendLayout, Function, FunctionType, Value, ValueType};

fn main() -> Result<(), String> {
    let read_line = |_: Vec<Rc<RefCell<Value>>>| -> Result<Rc<RefCell<Value>>, String> {
        let mut new_string: String = "".into();
        stdin()
            .read_line(&mut new_string)
            .map_err(|err| err.to_string())?;
        Ok(Rc::new(RefCell::new(Value::String(new_string))))
    };
    let print_line = |inp: Vec<Rc<RefCell<Value>>>| -> Result<Rc<RefCell<Value>>, String> {
        if let Value::String(ref string) = &*inp[0].borrow() {
            stdout()
                .write(string.as_bytes())
                .map_err(|err| err.to_string())?;
            return Ok(Rc::new(RefCell::new(Value::None)));
        }
        Err(Default::default())
    };
    let add = |inps:Vec<Rc<RefCell<Value>>>|{
        match (&*inps[0].borrow(),&*inps[1].borrow()){
            (Value::Int(lhs),Value::Int(rhs))=>{Ok(Rc::new(RefCell::new(Value::Int(lhs+rhs))))},
            _=>{Err("shouldn't be here".into())}
        }
    };
    let a = Expr::Body(vec![
        Expr::Asignment(
            "add_int".into(),
            Box::new(
                Expr::Literal(
                    Value::Function(
                        Function::InbuiltFunction(
                            vec![("lhs".into(),ValueType::Pack(["Int".into()].into())),("rhs".into(),ValueType::Pack(["Int".into()].into()))],
                            Box::new(ValueType::Int),
                            Rc::new(RefCell::new(add)),
                        )
                    )
                ),
            ),
        ),
        Expr::Asignment(
            "Int".into(), 
            Box::new(Expr::FunctionDecl(
                vec![("int".into(),ValueType::Int)],
                Box::new(
                    Expr::Body(vec![
                        Expr::Asignment("int2".into(), Box::new(Expr::Variable("int".into()))),
                        Expr::Return(
                            Box::new(
                                Expr::PackDecl(
                                    [
                                        (
                                            "Add".into(),
                                            [
                                                (
                                                    "add".into(),
                                                    Expr::FunctionDecl(
                                                        vec![("other".into(),ValueType::Int)],
                                                        Box::new(Expr::Return(Box::new(Expr::Call(Box::new(Expr::Variable("add_int".into())), vec![Expr::Variable("int2".into()),Expr::Variable("other".into())]))))
                                                    ),
                                                )
                                            ].into()
                                        )
                                    ].into()
                                )
                            )
                        )
                    ])
                )
            ))
        ),
        Expr::Asignment(
            "read".into(),
            Box::new(Expr::Literal(Value::Function(Function::InbuiltFunction(
                vec![],
                Box::new(ValueType::String),
                Rc::new(RefCell::new(read_line)),
            )))),
        ),
        Expr::Asignment(
            "print".into(),
            Box::new(Expr::Literal(Value::Function(Function::InbuiltFunction(
                vec![("string".into(), ValueType::String)],
                Box::new(ValueType::None),
                Rc::new(RefCell::new(print_line)),
            )))),
        ),
        Expr::Call(
            Box::new(Expr::Variable("print".into())),
            vec![Expr::Call(Box::new(Expr::Variable("read".into())), vec![])],
        ),
        Expr::Asignment(
            "NewInt".into(),
            Box::new(
                Expr::Call(
                    Box::new(
                        Expr::Variable("Int".into())
                    ),
                    vec![Expr::Literal(Value::Int(3))],
                )
            )
        ),
        Expr::Return(Box::new(Expr::Call(Box::new(Expr::PackFnGet(Box::new(Expr::Variable("NewInt".into())), "Add".into(), "add".into())), vec![Expr::Literal(Value::Int(4))])))
    ]);
    let extensions = [(
        "Add".into(),
        ExtendLayout(
            [(
                "add".into(),
                FunctionType(
                    vec![ValueType::Pack(["Any".into()].into())],
                    Box::new(ValueType::Pack(["Any".into()].into())),
                ),
            )]
            .into(),
        ),
    )]
    .into();
    let mut env = Environment::new(extensions);
    dbg!(a.eval(&mut env)).and_then(|_| Ok(()))
}
