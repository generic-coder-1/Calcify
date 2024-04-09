use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    rc::Rc,
};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Value),
    Body(Vec<Expr>),
    Asignment(String, Box<Expr>),
    Return(Box<Expr>),
    Variable(String),
    Call(String, Vec<Expr>),
    Function(Vec<(String, ValueType)>, Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Value {
    None,
    Int(i32),
    String(String),
    Bool(bool),
    Float(f32),
    Function(
        Vec<(String, ValueType)>,
        Box<Expr>,
        Environment<Value>,
    ),
}

impl Value {
    fn get_type(&self) -> Result<ValueType, String> {
        Ok(match self {
            Value::None => ValueType::None,
            Value::Int(_) => ValueType::Int,
            Value::String(_) => ValueType::String,
            Value::Bool(_) => ValueType::Bool,
            Value::Float(_) => ValueType::Float,
            Value::Function(inps, body, captured_env) => ValueType::Function(
                inps.iter().map(|inp| inp.1.clone()).collect(),
                Box::new(
                    {
                        let mut env = Environment::<ValueType> {
                            layers:
                                captured_env
                                    .layers
                                    .iter()
                                    .map(|layer| {
                                        layer
                                                .iter()
                                                .map(|(name, value)| {
                                                    value.borrow().get_type().and_then(
                                                        |value_type| {
                                                            Ok((
                                                                name.clone(),
                                                                Rc::new(RefCell::new(value_type)),
                                                            ))
                                                        },
                                                    )
                                                })
                                                .collect::<Result<
                                                    HashMap<String, Rc<RefCell<ValueType>>>,
                                                    String,
                                                >>()
                                    })
                                    .collect::<Result<
                                        VecDeque<HashMap<String, Rc<RefCell<ValueType>>>>,
                                        String,
                                    >>()?,
                        };
                        inps.iter().for_each(|(var_name, var_type)| {
                            env.set(var_name.clone(), Rc::new(RefCell::new(var_type.clone())))
                        });
                        body.get_result_type(&mut env)
                    }?
                    .borrow()
                    .clone(),
                ),
            ),
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ValueType {
    None,
    Int,
    String,
    Bool,
    Float,
    Function(Vec<ValueType>, Box<ValueType>),
}

#[derive(Debug, Clone)]
pub struct Environment<T: Debug> {
    layers: VecDeque<HashMap<String, Rc<RefCell<T>>>>,
}

impl<T: Debug> Environment<T> {
    pub fn new() -> Self {
        Self {
            layers: VecDeque::from([HashMap::new()]),
        }
    }
    fn get(&self, var_name: &String) -> Option<Rc<RefCell<T>>> {
        self.layers
            .iter()
            .find_map(|values| values.get(var_name))
            .cloned()
    }
    fn set(&mut self, var_name: String, var_value: Rc<RefCell<T>>) {
        self.layers
            .back_mut()
            .expect("Scope went negative. Not good")
            .insert(var_name, var_value);
    }
    fn push(&mut self) {
        self.layers.push_back(HashMap::new());
    }
    fn pop(&mut self) {
        dbg!(&self);
        self.layers.pop_back();
    }
}

impl Expr {
    pub fn eval(&self, env: &mut Environment<Value>) -> Result<Rc<RefCell<Value>>, String> {
        match self {
            Expr::Literal(value) => Ok(Rc::new(RefCell::new(value.clone()))),
            Expr::Body(exprs) => {
                env.push();
                let result = exprs
                    .iter()
                    .map(|expr| {
                        if let Expr::Return(expr) = &*expr {
                            Err(expr.eval(env))
                        } else {
                            match expr.eval(env) {
                                Ok(_) => Ok(()),
                                Err(e) => Err(Err(e)),
                            }
                        }
                    })
                    .collect::<Result<Vec<()>, Result<Rc<RefCell<Value>>, String>>>()
                    .err()
                    .unwrap_or(Ok(Rc::new(RefCell::new(Value::None))));
                env.pop();
                result
            }
            Expr::Asignment(var_name, evaluate) => {
                let evaluation = evaluate.eval(env)?;
                env.set(var_name.clone(), evaluation);
                Ok(Rc::new(RefCell::new(Value::None)))
            }
            Expr::Return(expr) => expr.eval(env),
            Expr::Call(func_name, exprs) => {
                let p_var = env.get(func_name);
                if let Some(p_func) = p_var {
                    let mut p_func = RefCell::borrow_mut(&p_func);
                    if let Value::Function(ref inputs, ref body, ref mut closure_env) = *p_func {
                        if inputs.len() != exprs.len() {
                            return Err(format!(
                                "Function \"{func_name}\" takes {} input(s) but was given {}",
                                inputs.len(),
                                exprs.len()
                            ));
                        }
                        closure_env.push();
                        inputs.iter().zip(exprs.iter()).map(|(inp,expr)|{
                            let value = expr.eval(closure_env)?;
                            let value_type = value.borrow().get_type()?;
                            if value_type == inp.1{
                                env.set(inp.0.clone(), value);
                            }else{
                                return Err(format!("Input \"{}\" of function \"{}\" is supposed to be of type {:?} not {:?}",inp.0,func_name,value_type,inp.1))
                            }
                            Ok(())
                        }).collect::<Result<Vec<()>,String>>()?;
                        let return_val = body.eval(closure_env);
                        closure_env.pop();
                        return_val
                    } else {
                        Err(format!("Variable \"{func_name}\" isn't a function"))
                    }
                } else {
                    Err(format!("Variable \"{func_name}\" doesn't exist"))
                }
            }
            Expr::Variable(var_name) => env
                .get(var_name)
                .ok_or(format!("Variable \"{var_name}\" doesn't exist")),
            Expr::Function(inps, body) => {
                fn get_outside_vars(
                    expr: &Expr,
                    mut outside_vars: Vec<(String, Rc<RefCell<Value>>)>,
                    inside_vars: &mut HashSet<String>,
                    env: &Environment<Value>,
                ) -> Result<Vec<(String, Rc<RefCell<Value>>)>, String> {
                    match expr {
                        Expr::Literal(_) => Ok(outside_vars),
                        Expr::Body(exprs) => {
                            let mut new_outside_vars = outside_vars;
                            for expr in exprs {
                                new_outside_vars =
                                    get_outside_vars(&expr, new_outside_vars, inside_vars, env)?;
                            }
                            Ok(new_outside_vars)
                        }
                        Expr::Asignment(internal_var, expr) => {
                            inside_vars.insert(internal_var.clone());
                            get_outside_vars(&expr, outside_vars, inside_vars, env)
                        }
                        Expr::Return(expr) => {
                            get_outside_vars(&expr, outside_vars, inside_vars, env)
                        }
                        Expr::Variable(var_name) => {
                            if !inside_vars.contains(var_name) {
                                outside_vars.push((
                                    var_name.clone(),
                                    env.get(&var_name)
                                        .ok_or(format!("Variable \"{var_name}\" doesn't exist"))?,
                                ))
                            }
                            Ok(outside_vars)
                        }
                        Expr::Call(_, inps) => {
                            let mut new_outside_vars = outside_vars;
                            for expr in inps {
                                new_outside_vars =
                                    get_outside_vars(&expr, new_outside_vars, inside_vars, env)?;
                            }
                            Ok(new_outside_vars)
                        }
                        Expr::Function(inps, body) => {
                            let mut nested_inside_vars = HashSet::new();
                            inps.iter().for_each(|inp| {
                                nested_inside_vars.insert(inp.0.clone());
                            });
                            get_outside_vars(&*body, outside_vars, &mut nested_inside_vars, env)
                        }
                    }
                }
                let mut nested_inside_vars = HashSet::new();
                inps.iter().for_each(|inp| {
                    nested_inside_vars.insert(inp.0.clone());
                });
                let mut func_env = Environment::new();
                get_outside_vars(body, vec![], &mut nested_inside_vars, env)?
                    .into_iter()
                    .for_each(|var| func_env.set(var.0, var.1));
                Ok(Rc::new(RefCell::new(Value::Function(
                    inps.clone(),
                    body.clone(),
                    func_env,
                ))))
            }
        }
    }
    fn get_result_type(
        &self,
        env: &mut Environment<ValueType>,
    ) -> Result<Rc<RefCell<ValueType>>, String> {
        Ok(match self {
            Expr::Literal(value) => Rc::new(RefCell::new(value.get_type()?)),
            Expr::Body(exprs) => exprs
                .iter()
                .filter_map(|expr| {
                    if let Expr::Return(expr) = expr {
                        Some(expr.get_result_type(env))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<Rc<RefCell<ValueType>>>, String>>()?
                .iter()
                .try_fold(None, |acc: Option<&Rc<RefCell<ValueType>>>, return_type| {
                    if let Some(other_return_type) = acc {
                        if *other_return_type.borrow() == *return_type.borrow() {
                            Ok(Some(return_type))
                        } else {
                            Err("Body has multiple return types")
                        }
                    } else {
                        Ok(Some(return_type))
                    }
                })?
                .unwrap_or(&Rc::new(RefCell::new(ValueType::None)))
                .clone(),
            Expr::Asignment(name, expr) => {
                let result = expr.get_result_type(env)?;
                env.set(name.clone(), result);
                Rc::new(RefCell::new(ValueType::None))
            }
            Expr::Return(expr) => expr.get_result_type(env)?,
            Expr::Call(func_name, inps) => {
                if let ValueType::Function(params, out) = &*env
                    .get(func_name)
                    .ok_or(format!("Variable \"{func_name}\""))?
                    .borrow()
                {
                    for (param, arg) in params.iter().zip(inps.iter()) {
                        let type_result = arg.get_result_type(env)?;
                        if *param != *type_result.borrow() {
                            Err(format!(
                                "Expected input of type \"{:?}\", got input of type \"{:?}\"",
                                param,
                                arg.get_result_type(env)?
                            ))?
                        }
                    }
                    Rc::new(RefCell::new(*out.clone()))
                } else {
                    Err(format!("Variable \"{func_name}\" isn't a function"))?
                }
            }
            Expr::Variable(var_name) => env
                .get(var_name)
                .ok_or(format!("Variable \"{var_name}\" doesn't exist"))?
                .clone(),
            Expr::Function(inps, body) => Rc::new(RefCell::new(ValueType::Function(
                inps.iter().map(|inp| inp.1.clone()).collect(),
                Box::new((*(body.get_result_type(env)?).borrow()).clone()),
            ))),
        })
    }
}
