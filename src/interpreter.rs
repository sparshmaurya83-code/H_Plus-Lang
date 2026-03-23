use crate::ast::*;
use crate::runtime::environment::Environment;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,

    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn run(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            let _ = self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::Say(expr) => {
                let val = self.eval(expr)?;
                println!("{:?}", val);
            }

            Stmt::Let(name, expr) => {
                let val = self.eval(expr)?;
                self.env.set(name, val);
            }

            Stmt::Assign(name, expr) => {
                let val = self.eval(expr)?;
                let _ = self.env.assign(name, val);
            }

            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.execute(stmt)?;
                }
            }

            Stmt::Function { name, params, body } => {
                self.env.set(name, Value::Function { params, body });
            }

            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval(e)?
                } else {
                    Value::Null
                };

                return Err(val);
            }

            _ => {}
        }

        Ok(())
    }

    fn eval(&mut self, expr: Expr) -> Result<Value, Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::String(s) => Ok(Value::String(s)),
            Expr::Variable(name) => {
                self.env.get(&name).ok_or(Value::Null)
            }

            Expr::Call { callee, args } => {
                let func = self.eval(*callee)?;

                if let Value::Function { params, body } = func {
                    let mut new_env = Environment::new_enclosed(self.env.clone());

                    for (i, param) in params.iter().enumerate() {
                        let val = self.eval(args[i].clone())?;
                        new_env.set(param.clone(), val);
                    }

                    let prev = self.env.clone();
                    self.env = new_env;

                    for stmt in body {
                        match self.execute(stmt) {
                            Err(val) => {
                                self.env = prev;
                                return Ok(val);
                            }
                            _ => {}
                        }
                    }

                    self.env = prev;
                    Ok(Value::Null)
                } else {
                    Err(Value::Null)
                }
            }

            _ => Ok(Value::Null),
        }
    }
}