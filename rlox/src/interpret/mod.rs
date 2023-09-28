use crate::expr::*;
use crate::expr::stmt::Stmt;
use crate::expr::stmt::Decl;

pub mod loxValue;
use loxValue::*;

pub mod loxType;
use loxType::*;

pub mod display;

pub mod env;
use env::Env;

pub mod loxRuntimeError;
use loxRuntimeError::{LoxRuntimeError, LoxTypeError, LoxVariableError, LoxFunctionError};
use loxRuntimeError::loxTypeError::LoxTypeMismatch;

mod helpers;
use helpers::*;

pub fn execute(program: Vec<Decl>, mut env: Env, ret: &mut Option<LoxValue>) -> Result<Env, LoxRuntimeError>
{
    for decl in program {
        match decl {
            Decl::Stmt(stmt) => {
                match stmt {
                    Stmt::WhileStmt(c, b) => {
                        let mut condition = evaluate(c.clone(), &mut env,)?;
                        while truthy(&condition) {
                            env = execute(vec![Decl::Stmt(*b.clone())], env, ret)?;
                            if ret.is_some() { return Ok(env); }
                            condition = evaluate(c.clone(), &mut env)?;
                        }
                    }
                    Stmt::IfStmt(c, i, e) => {
                        let condition = evaluate(c.clone(), &mut env)?;
                        match condition {
                            LoxValue::Bool(true) => env = execute(vec![Decl::Stmt(*i)], env, ret)?,
                            LoxValue::Bool(false) => {
                                if let Some(x) = e {
                                    env = execute(vec![Decl::Stmt(*x)], env, ret)?;
                                }
                            },
                            _ => return Err(LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::new(condition.into(), LoxType::Bool)).into(), c.line)),
                        }
                        if ret.is_some() { return Ok(env); }
                    },
                    Stmt::PrintStmt(e) => {
                        let value = evaluate(e, &mut env)?;
                        println!("{}", value);
                    },
                    Stmt::ExprStmt(e) => {
                        let _ = evaluate(e, &mut env)?;
                    },
                    Stmt::ReturnStmt(e) => {
                        let value = evaluate(e, &mut env)?;
                        *ret = Some(value);
                    },
                    Stmt::Block(b) => {
                        let mut newEnv = Env::new(env.clone());
                        newEnv = execute(b, newEnv, ret)?;
                        if ret.is_some() { return Ok(env); }
                        env = *newEnv.end().unwrap();
                    }
                }
            },
            Decl::VarDecl(name, content) => {
                let v = evaluate(content, &mut env)?;
                env.decl(name.to_string(), v);
            },
            Decl::FunDecl(name, arguments, body) => {
                let f = LoxValue::Function(LoxFn::new(name.to_string(), body, arguments, env.clone()));
                env.decl(name.to_string(), f);
            },
        }
    };
    Ok(env)
}

fn evaluate(value: Expr, env: &mut Env) -> Result<LoxValue, LoxRuntimeError>
{
    match value.exprType {
        ExprType::Literal(l) => {
            match l {
                Literal::Num(n) => Ok(LoxValue::Num(n)),
                Literal::Str(s) => Ok(LoxValue::Str(s)),
                Literal::Bool(b) => Ok(LoxValue::Bool(b)),
                Literal::Nil => Ok(LoxValue::Nil),
            }
        },
        ExprType::Variable(n) => {
            match env.get(&n) {
                Some(v) => Ok(v.clone()),
                None => Err(LoxRuntimeError::new(LoxVariableError::Missing(n).into(), value.line))
            }
        },
        ExprType::Assignment(a) => {
            if env.contains_key(&a.id) {
                let v = evaluate(*a.expr, env)?;
                env.assign(a.id, v.clone());
                Ok(v)
            }
            else {
                Err(LoxRuntimeError::new(LoxVariableError::Missing(a.id).into(), value.line))
            }
        },
        ExprType::Call(c) => {
            let callee = evaluate(*c.callee, env)?;
            let mut evArgs: Vec<LoxValue> = Vec::with_capacity(c.arguments.len());
            for arg in c.arguments {
                evArgs.push(evaluate(arg, env)?);
            }
            for arg in &evArgs {
                println!("arg {}", arg);
            }
            let retVal;
            match callee {
                LoxValue::Function(mut f) if f.arity() == evArgs.len() as u32 => retVal = f.call(evArgs)?,
                LoxValue::Function(f)  => return Err(LoxRuntimeError::new(LoxFunctionError::ArgPrmCountMismatch(evArgs.len() as u32, f.arity()).into(), value.line)),
                _ => return Err(LoxRuntimeError::new(LoxFunctionError::NotCallable(callee).into(), value.line)),
            }
            Ok(retVal)
        },
        ExprType::Grouping(g) => evaluate(*g.expr, env),
        ExprType::Unary(u) => {
            let result = evaluate(*u.expr, env)?;
            match u.operator {
                Uoperator::Minus => {
                    if let LoxValue::Bool(b) = result.clone() {
                        Ok(LoxValue::Bool(!b))
                    }
                    else {
                        Err(LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::new(result.into(), LoxType::Num)).into(), value.line))
                    }
                },
                Uoperator::Bang => Ok(LoxValue::Bool(!truthy(&result))),
            }
        },
        ExprType::Logical(l) => {
            let lresult = evaluate(*l.lexpr, env)?;                
            match l.operator {
                Loperator::Or => if truthy(&lresult) { return Ok(lresult) },
                Loperator::And => if !truthy(&lresult) { return Ok(lresult) },
            };
            evaluate(*l.rexpr, env)
        },
        ExprType::Binary(b) => {
            let lresult = evaluate(*b.lexpr, env)?;                
            let rresult = evaluate(*b.rexpr, env)?;                
            match b.operator {
                Boperator::Minus => binary_num_op(lresult, rresult, |x1, x2| {x1-x2}, value.line), Boperator::Plus => {
                    match (lresult, rresult) {
                        (LoxValue::Num(n1), LoxValue::Num(n2)) => Ok(LoxValue::Num(n1+n2)),
                        (LoxValue::Str(s1), LoxValue::Str(s2)) => Ok(LoxValue::Str(s1+s2.as_str())),
                        (LoxValue::Num(_), x) => Err(LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::new(x.into(), LoxType::Num)).into(), value.line)),
                        (LoxValue::Str(_), x) => Err(LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::new(x.into(), LoxType::Str)).into(), value.line)),
                        (x, LoxValue::Num(_)) => Err(LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::new(x.into(), LoxType::Num)).into(), value.line)),
                        (x, LoxValue::Str(_)) => Err(LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::new(x.into(), LoxType::Str)).into(), value.line)),

                        (x, _) => Err(LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::newMany(x.into(), vec![LoxType::Num, LoxType::Str])).into(), value.line)),
                    }
                },
            Boperator::Slash => binary_num_op(lresult, rresult, |x1, x2| {x1/x2}, value.line),
            Boperator::Star => binary_num_op(lresult, rresult, |x1, x2| {x1*x2}, value.line),
            Boperator::Less => binary_cmp_op(lresult, rresult, |x1, x2| {x1<x2}, value.line),
            Boperator::LessEqual => binary_cmp_op(lresult, rresult, |x1, x2| {x1<=x2}, value.line),
            Boperator::Greater => binary_cmp_op(lresult, rresult, |x1, x2| {x1<=x2}, value.line),
            Boperator::GreaterEqual => binary_cmp_op(lresult, rresult, |x1, x2| {x1<=x2}, value.line),
            Boperator::EqualEqual=> Ok(LoxValue::Bool(equal(&lresult, &rresult))),
            Boperator::BangEqual => Ok(LoxValue::Bool(!equal(&lresult, &rresult))),
            }
        },
    }
}
