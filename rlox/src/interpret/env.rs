use std::collections::HashMap;
use super::{LoxValue, LoxFn};
use crate::expr::stmt::{Decl, Stmt};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Env
{
    env: HashMap<String, LoxValue>,
    enclosing: Option<Box<Env>>,
}
impl Env
{
    pub fn global() -> Env
    {
        Env{ env: HashMap::new(), enclosing: None }
    }
    pub fn new(enclosing: Env) -> Env
    {
        Env{ env: HashMap::new(), enclosing: Some(Box::new(enclosing)) }
    }
    pub fn end(self) -> Option<Box<Env>>
    {
        self.enclosing
    }
    pub fn get(&self, k: &String) -> Option<&LoxValue>
    {
        match self.env.get(k) {
            Some(v) => Some(v),
            None => {
                match &self.enclosing {
                    Some(p) => p.get(k),
                    None => None,
                }
            },
        }
    }
    pub fn assign(&mut self, k: String, v: LoxValue) -> Option<LoxValue>
    {
        match self.env.contains_key(&k) {
            true => self.env.insert(k, v),
            false => {
                match &mut self.enclosing {
                    Some(p) => p.assign(k, v),
                    None => self.env.insert(k, v),
                }
            },
        }
    }
    pub fn decl(&mut self, k: String, v: LoxValue) -> Option<LoxValue>
    {
        self.env.insert(k, v)
    }
    pub fn contains_key(&self, k: &String) -> bool
    {
        match self.env.contains_key(k) {
            true => true,
            false => {
                match &self.enclosing {
                    Some(p) => p.contains_key(k),
                    None => false,
                }
            },
        }
    }
}
