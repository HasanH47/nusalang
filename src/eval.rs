// src/eval.rs

use crate::ast::{BinaryOp, Expr, Program, Stmt};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    UndefinedVar(String),
    UndefinedFunc(String),
    ArgumentMismatch,
    Runtime(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UndefinedVar(name) => write!(f, "Undefined variable: '{}'", name),
            EvalError::UndefinedFunc(name) => write!(f, "Undefined function: '{}'", name),
            EvalError::ArgumentMismatch => write!(f, "Function argument count mismatch"),
            EvalError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

#[derive(Clone)]
enum Value {
    Number(f64),
    String(String),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

type Env = HashMap<String, Value>;

pub fn eval(prog: &Program) -> Result<(), EvalError> {
    let mut env = Env::new();
    exec_block(&prog.statements, &mut env)
}

fn exec_block(stmts: &[Stmt], env: &mut Env) -> Result<(), EvalError> {
    for stmt in stmts {
        exec_stmt(stmt, env)?;
    }
    Ok(())
}

fn exec_stmt(stmt: &Stmt, env: &mut Env) -> Result<(), EvalError> {
    match stmt {
        Stmt::Let(name, expr) => {
            let val = eval_expr(expr, env)?;
            env.insert(name.clone(), val);
        }
        Stmt::Print(expr) => {
            let val = eval_expr(expr, env)?;
            match val {
                Value::Number(n) => println!("{}", n),
                Value::String(s) => println!("{}", s),
                _ => println!("<function>"),
            }
        }
        Stmt::Expr(expr) => {
            eval_expr(expr, env)?;
        }
        Stmt::FuncDef { name, params, body } => {
            let val = Value::Function {
                params: params.clone(),
                body: body.clone(),
            };
            env.insert(name.clone(), val);
        }
    }
    Ok(())
}

fn eval_expr(expr: &Expr, env: &mut Env) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Ident(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| EvalError::UndefinedVar(name.clone())),
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env)?;
            let r = eval_expr(right, env)?;
            match (l, r) {
                (Value::Number(a), Value::Number(b)) => match op {
                    BinaryOp::Add => Ok(Value::Number(a + b)),
                    BinaryOp::Sub => Ok(Value::Number(a - b)),
                    BinaryOp::Mul => Ok(Value::Number(a * b)),
                    BinaryOp::Div => Ok(Value::Number(a / b)),
                },
                (Value::String(a), Value::String(b)) => {
                    match op {
                        BinaryOp::Add => Ok(Value::String(format!("{}{}", a, b))), // Konkatenasi string
                        _ => Err(EvalError::Runtime(
                            "Operasi tidak valid untuk string".into(),
                        )),
                    }
                }
                _ => Err(EvalError::Runtime("Expected numbers".into())),
            }
        }
        Expr::Call { name, args } => {
            let func = env
                .get(name)
                .ok_or_else(|| EvalError::UndefinedFunc(name.clone()))?
                .clone();

            match func {
                Value::Function { params, body } => {
                    if params.len() != args.len() {
                        return Err(EvalError::ArgumentMismatch);
                    }

                    let mut new_env = env.clone();
                    for (p, a) in params.iter().zip(args) {
                        new_env.insert(p.clone(), eval_expr(a, env)?);
                    }
                    exec_block(&body, &mut new_env)?;
                    Ok(Value::Number(0.0)) // No return yet
                }
                _ => Err(EvalError::Runtime("Cannot call non-function".into())),
            }
        }
    }
}
