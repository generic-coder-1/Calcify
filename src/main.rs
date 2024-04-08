use calcify::ast::{Environment, Expr, Value};

fn main() {
    let a = Expr::Body(vec![
        Expr::Asignment("no".into(), Box::new(Expr::Literal(Value::Bool(true)))),
        Expr::Asignment(
            "foo".into(),
            Box::new(Expr::Body(
                vec![
                    Expr::Asignment("bar".into(), Box::new(Expr::Literal(Value::Int(90)))),
                    Expr::Return(Box::new(Expr::Literal(Value::Function(
                        vec![],
                        Box::new(Expr::Return(Box::new(Expr::Variable("bar".into())))),
                    ))))
                ]
            )),
        ),
        Expr::Return(Box::new(Expr::Call("foo".into(), vec![]))),
    ]);
    let mut env = Environment::new();
    dbg!(a.eval(&mut env));
    dbg!(env);
}
