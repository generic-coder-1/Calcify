use std::{cell::RefCell, rc::Rc};

use calcify::ast::{Environment, Expr, Function, Value, ValueType};

fn main() {
    let add = |inps:Vec<Rc<RefCell<Value>>>|->Result<Rc<RefCell<Value>>,String>{
        match (&*inps[0].borrow(),&*inps[1].borrow()){
            (Value::Int(lhs),Value::Int(rhs))=>{
                Ok(
                    Rc::new(RefCell::new(Value::Int(lhs+rhs)))
                )
            },
            _=>{Err("shouldn't get here".into())}
        }
    };
    let a = Expr::Body(vec![
        Expr::Asignment("add".into(), Box::new(Expr::Literal(Value::Function(
            Function::InbuiltFunction(
                vec![("LHS".into(),ValueType::Int),("RHS".into(),ValueType::Int)],
                Box::new(ValueType::Int),
                Rc::new(RefCell::new(add)))
        )))),
        Expr::Return(Box::new(Expr::Call("add".into(), vec![
            Expr::Literal(Value::Int(10)),
            Expr::Literal(Value::Int(20)),
        ]))),
    ]);
    let mut env = Environment::new();
    dbg!(a.eval(&mut env));
}
