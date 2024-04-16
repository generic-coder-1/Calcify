use std::{
    cell::RefCell,
    collections::{btree_map::Values, HashMap, HashSet, VecDeque},
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
    Call(Box<Expr>, Vec<Expr>),
    FunctionDecl(Vec<(String, ValueType)>, Box<Expr>),
    Clone(Box<Expr>),
    PackDecl(HashMap<String, HashMap<String, Expr>>),
    PackFnGet(Box<Expr>, String, String),
}
#[derive(Clone, Debug)]
pub struct ExtendLayout(pub HashMap<String, FunctionType>);
#[derive(Clone, Debug)]
pub struct Extend(HashMap<String, Rc<RefCell<Value>>>);

#[derive(Clone, Debug)]
pub enum Value {
    None,
    Int(i32),
    String(String),
    Bool(bool),
    Float(f32),
    Function(Function),
    Pack(HashMap<String, Extend>),
}

impl Value {
    fn get_type(
        &self,
        extend_definitions: &HashMap<String, ExtendLayout>,
    ) -> Result<ValueType, String> {
        Ok(match self {
            Value::None => ValueType::None,
            Value::Int(_) => ValueType::Int,
            Value::String(_) => ValueType::String,
            Value::Bool(_) => ValueType::Bool,
            Value::Float(_) => ValueType::Float,
            Value::Function(Function::UserFunction(inps, body, captured_env)) => {
                ValueType::Function(FunctionType(
                    inps.iter().map(|inp| inp.1.clone()).collect(),
                    Box::new(
                        {
                            let mut env = Environment::<ValueType> {
                                layers: captured_env
                                    .layers
                                    .iter()
                                    .map(|layer| {
                                        layer
                                                .iter()
                                                .map(|(name, value)| {
                                                    value
                                                        .borrow()
                                                        .get_type(&extend_definitions)
                                                        .and_then(|value_type| {
                                                            Ok((
                                                                name.clone(),
                                                                Rc::new(RefCell::new(value_type)),
                                                            ))
                                                        })
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
                                extend_definitions: extend_definitions.clone(),
                            };
                            inps.iter().for_each(|(var_name, var_type)| {
                                env.set(var_name.clone(), Rc::new(RefCell::new(var_type.clone())))
                            });
                            body.get_result_type(&mut env)
                        }?
                        .borrow()
                        .clone(),
                    ),
                ))
            }
            Value::Function(Function::InbuiltFunction(inps, out, ..)) => ValueType::Function(
                FunctionType(inps.iter().map(|inp| inp.1.clone()).collect(), out.clone()),
            ),
            Value::Pack(extends) => {
                ValueType::Pack(extends.keys().map(|extend| extend.clone()).collect())
            }
        })
    }
}

#[derive(Clone, Debug)]
pub enum ValueType {
    None,
    Int,
    String,
    Bool,
    Float,
    Function(FunctionType),
    Pack(HashSet<String>),
}

impl PartialEq for ValueType{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function(l0), Self::Function(r0)) => l0 == r0,
            (Self::Pack(l0), Self::Pack(r0)) => l0.is_superset(r0),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FunctionType(pub Vec<ValueType>, pub Box<ValueType>);

#[derive(Clone)]
pub enum Function {
    UserFunction(Vec<(String, ValueType)>, Box<Expr>, Environment<Value>),
    InbuiltFunction(
        Vec<(String, ValueType)>,
        Box<ValueType>,
        Rc<RefCell<dyn FnMut(Vec<Rc<RefCell<Value>>>) -> Result<Rc<RefCell<Value>>, String>>>,
    ),
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserFunction(arg0, arg1, arg2) => f
                .debug_tuple("UserFunction")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::InbuiltFunction(arg0, arg1, ..) => f
                .debug_tuple("InbuiltFunction")
                .field(arg0)
                .field(arg1)
                .field(&"inbuilt_function".to_string())
                .finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment<T: Debug> {
    extend_definitions: HashMap<String, ExtendLayout>,
    layers: VecDeque<HashMap<String, Rc<RefCell<T>>>>,
}

impl<T: Debug> Environment<T> {
    pub fn new(mut extend_definitions: HashMap<String, ExtendLayout>) -> Self {
        extend_definitions.insert("Any".into(), ExtendLayout(HashMap::new()));
        Self {
            extend_definitions,
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
            Expr::Call(func, exprs) => {
                let p_func_raw = func.eval(env)?;
                let mut p_func = p_func_raw.borrow_mut();
                if let Value::Function(ref mut func) = *(p_func) {
                    match func{
                        Function::UserFunction(ref inputs, ref body, ref mut closure_env) => {
                            if inputs.len() != exprs.len() {
                                return Err(format!(
                                    "Function takes {} input(s) but was given {}",
                                    inputs.len(),
                                    exprs.len()
                                ));
                            }
                            closure_env.push();
                            inputs.iter().zip(exprs.iter()).map(|(inp,expr)|{
                                let value = expr.eval(closure_env)?;
                                let value_type = value.borrow().get_type(&closure_env.extend_definitions)?;
                                if value_type == inp.1{
                                    env.set(inp.0.clone(), value);
                                }else{
                                    return Err(format!("Input \"{}\" of function is supposed to be of type {:?} not {:?}",inp.0,value_type,inp.1))
                                }
                                Ok(())
                            }).collect::<Result<Vec<()>,String>>()?;
                            let return_val = body.eval(closure_env);
                            closure_env.pop();
                            return_val
                        },
                        Function::InbuiltFunction(inps, out, func) => {
                            if inps.len() != exprs.len(){
                                return Err(format!("Function accepts {} input(s) but got {} input(s)",inps.len(),exprs.len()));
                            }
                            func.borrow_mut()(
                                exprs.iter()
                                    .enumerate()
                                    .map(|(i,expr)|{
                                        expr.eval(env)
                                        .and_then(|inp|{
                                            let inp_type = inp.borrow().get_type(&env.extend_definitions)?;
                                                if inp_type == inps[i].1{
                                                Ok(inp)
                                            }else{
                                                Err(format!("Input \"{}\" to function is supposed to be of type \"{:?}\" not \"{:?}\"",inps[i].0,inps[i].1,inp_type))
                                            }
                                        })
                                    }).collect::<Result<Vec<Rc<RefCell<Value>>>,String>>()?
                                ).and_then(|actual_output|{
                                    let out_type = actual_output.borrow().get_type(&env.extend_definitions)?;
                                    if out_type == **out{
                                        Ok(actual_output)
                                    }else{
                                        Err(format!("Output to function is supposed to be of type \"{:?}\" not \"{:?}\"",out,out_type))
                                    }
                                })
                        },
                    }
                } else {
                    Err(format!("Expresion isn't a function"))
                }
            }
            Expr::Variable(var_name) => env
                .get(var_name)
                .ok_or(format!("Variable \"{var_name}\" doesn't exist")),
            Expr::FunctionDecl(inps, body) => {
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
                        Expr::FunctionDecl(inps, body) => {
                            let mut nested_inside_vars = HashSet::new();
                            inps.iter().for_each(|inp| {
                                nested_inside_vars.insert(inp.0.clone());
                            });
                            get_outside_vars(&*body, outside_vars, &mut nested_inside_vars, env)
                        }
                        Expr::Clone(expr) => get_outside_vars(expr, outside_vars, inside_vars, env),
                        Expr::PackDecl(pack) => {
                            let mut new_outside_vars = outside_vars;
                            for expr in pack.iter().map(|extension|{extension.1.iter().map(|func|{func.1})}).flatten(){
                                new_outside_vars = get_outside_vars(expr, new_outside_vars, inside_vars, env)?;
                            }
                            Ok(new_outside_vars)
                        },
                        Expr::PackFnGet(expr, _, _) => get_outside_vars(expr, outside_vars, inside_vars, env),
                    }
                }
                let mut nested_inside_vars = HashSet::new();
                inps.iter().for_each(|inp| {
                    nested_inside_vars.insert(inp.0.clone());
                });
                let mut func_env = Environment::new(env.extend_definitions.clone());
                get_outside_vars(body, vec![], &mut nested_inside_vars, env)?
                    .into_iter()
                    .for_each(|var| dbg!(func_env.set(var.0, var.1)));
                dbg!(&func_env);
                Ok(Rc::new(RefCell::new(Value::Function(
                    Function::UserFunction(inps.clone(), body.clone(), func_env.clone()),
                ))))
            }
            Expr::Clone(expr) => expr.eval(env).and_then(|val|Ok(Rc::new(RefCell::new(val.borrow().clone())))),
            Expr::PackDecl(packs) => Ok(Rc::new(RefCell::new(Value::Pack(packs.iter().map(|pack|{
                Ok((pack.0.clone(),env.extend_definitions.clone().get(pack.0).ok_or(format!("Extend \"{}\" doesn't exist",pack.0))?.0.iter().map(|(name,func_type)|{
                    let func = pack.1.get(name).ok_or(format!("Extending with extend \"{}\" requires adding function \"{name}\"",pack.0))?.eval(env)?;
                    let actual_func_type = func.borrow().get_type(&env.extend_definitions)?;
                    if actual_func_type == ValueType::Function(func_type.clone()){
                        Ok((name.clone(),func))
                    }else{
                        Err(format!("Function \"{name}\" of extend \"{}\" is ment to be of type \"{:?}\" not \"{:?}\"",pack.0,func_type,actual_func_type))
                    }
                }).collect::<Result<HashMap<String,Rc<RefCell<Value>>>,String>>().and_then(|extend|Ok(Extend(extend)))?))
            }).collect::<Result<HashMap<String,Extend>,String>>()?)))),
            Expr::PackFnGet(pack, extend, func) => {
                let p_pack_raw = pack.eval(env)?;
                let p_pack = p_pack_raw.borrow();
                if let Value::Pack(ref extensions) = *p_pack{
                    extensions.get(extend).ok_or(format!("Pack doesn't have extension \"{extend}\"")).and_then(|extension|extension.0.get(func).ok_or(format!("Extension \"{extend}\" doesn't have function \"{func}\"")).cloned())
                }else{
                    Err(format!("Expresion isn't a Pack"))
                }
            },
        }
    }
    fn get_result_type(
        &self,
        env: &mut Environment<ValueType>,
    ) -> Result<Rc<RefCell<ValueType>>, String> {
        Ok(match self {
            Expr::Literal(value) => Rc::new(RefCell::new(value.get_type(&env.extend_definitions)?)),
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
            Expr::Call(func, inps) => {
                let p_func_raw = func.get_result_type(env)?;
                let p_func = p_func_raw.borrow();
                if let ValueType::Function(FunctionType(params, out)) = &*p_func {
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
                    Err(format!("Value given isn't a function"))?
                }
            }
            Expr::Variable(var_name) => env
                .get(var_name)
                .ok_or(format!("Variable \"{var_name}\" doesn't exist"))?
                .clone(),
            Expr::FunctionDecl(inps, body) => {
                Rc::new(RefCell::new(ValueType::Function(FunctionType(
                    inps.iter().map(|inp| inp.1.clone()).collect(),
                    Box::new((*(body.get_result_type(env)?).borrow()).clone()),
                ))))
            }
            Expr::Clone(expr) => expr.get_result_type(env)?,
            Expr::PackDecl(extensions) => Rc::new(RefCell::new(ValueType::Pack(extensions.iter().map(|extension|extension.0.clone()).collect()))),
            Expr::PackFnGet(.., extension, func) => Rc::new(RefCell::new(ValueType::Function(env.extend_definitions.get(extension).ok_or(format!("Extension \"{extension}\" isn't defined"))?.0.get(func).ok_or(format!("Extension \"{extension}\" doesn't have function \"{func}\""))?.clone()))),
        })
    }
}
