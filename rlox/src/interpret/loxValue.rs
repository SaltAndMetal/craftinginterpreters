use super::Decl;
use super::LoxRuntimeError;
use super::execute;
use super::env::Env;
use super::loxType::LoxCallee;

#[derive(Clone)]
pub enum LoxValue
{
    Function(LoxFn),
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

#[derive(Clone)]
pub struct LoxFn
{
    body: Vec<Decl>,
    arguments: Vec<String>,
    env: Env,
}
impl LoxFn
{
    pub fn new(name: String, body: Vec<Decl>, arguments: Vec<String>, env: Env) -> LoxFn
    {
        let loxFn = LoxFn{ body, arguments, env };
        loxFn
    }
}

impl LoxCallee for LoxFn
{
    fn arity(&self) -> u32
    {
        self.arguments.len() as u32
    }
    fn call(&mut self, parameters: Vec<LoxValue>) -> Result<LoxValue, LoxRuntimeError>
    {
        let mut locEnv = Env::new(self.env.clone());
        for (i, parameter) in parameters.into_iter().enumerate() {
            locEnv.decl(self.arguments[i].clone(), parameter);
        }
        let mut val = None;
        match execute(self.body.clone(), locEnv, &mut val) {
            Ok(e) => { self.env = *e.end().unwrap(); Ok(val.unwrap_or(LoxValue::Nil)) },
            Err(e) => Err(e),
        }
    }
}
