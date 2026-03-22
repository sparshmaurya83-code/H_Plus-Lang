use crate::ast::*;
use crate::runtime::environment::Environment;
use crate::runtime::builtins::call_builtin;

pub struct Interpreter {
    env: Environment,
    loaded_modules: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            loaded_modules: vec![],
        }
    }

    pub fn run(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Say(expr) => {
                let val = self.eval(expr);
                println!("💬 {}", val);
            }

            Stmt::Let(name, expr) => {
                let val = self.eval(expr);
                self.env.set(name, val);
            }

            Stmt::Use(module) => {
                self.loaded_modules.push(module.clone());
                println!("📦 Loaded package '{}'", module);
            }

            Stmt::When(cond, then_block, else_block) => {
                if self.is_truthy(&self.eval(cond)) {
                    for stmt in then_block {
                        self.execute(stmt);
                    }
                } else if let Some(block) = else_block {
                    for stmt in block {
                        self.execute(stmt);
                    }
                }
            }
        }
    }

    fn eval(&mut self, expr: Expr) -> String {
        match expr {
            Expr::String(s) => s,
            Expr::Number(n) => n.to_string(),
            Expr::Bool(b) => b.to_string(),

            Expr::Variable(name) => {
                self.env.get(&name).unwrap_or("null".into())
            }

            Expr::Call { module, function, args } => {
                if !self.loaded_modules.contains(&module) {
                    return format!("Error: module '{}' not loaded", module);
                }

                let evaluated_args: Vec<String> =
                    args.into_iter().map(|a| self.eval(a)).collect();

                call_builtin(&module, &function, evaluated_args)
                    .unwrap_or("null".into())
            }

            Expr::Binary(left, op, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);

                match op {
                    Operator::Plus => format!("{}{}", l, r),
                    Operator::EqualEqual => (l == r).to_string(),
                    Operator::Greater => {
                        let ln = l.parse::<f64>().unwrap_or(0.0);
                        let rn = r.parse::<f64>().unwrap_or(0.0);
                        (ln > rn).to_string()
                    }
                    Operator::Less => {
                        let ln = l.parse::<f64>().unwrap_or(0.0);
                        let rn = r.parse::<f64>().unwrap_or(0.0);
                        (ln < rn).to_string()
                    }
                }
            }
        }
    }

    fn is_truthy(&self, value: &str) -> bool {
        match value {
            "true" => true,
            "false" => false,
            "" => false,
            "null" => false,
            _ => true,
        }
    }
}