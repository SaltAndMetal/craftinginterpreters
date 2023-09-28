use super::loxValue::LoxValue;
use super::LoxRuntimeError;
use super::env::Env;

pub enum LoxType
{
    Function(u32),
    Num,
    Str,
    Bool,
    Nil
}

impl From<LoxValue> for LoxType
{
    fn from(value: LoxValue) -> Self
    {
        match value {
            LoxValue::Function(f) => LoxType::Function(f.arity()),
            LoxValue::Num(_) => LoxType::Num,
            LoxValue::Str(_) => LoxType::Str,
            LoxValue::Bool(_) => LoxType::Bool,
            LoxValue::Nil => LoxType::Nil,
        }
    }
}

pub trait LoxCallee
{
    fn arity(&self) -> u32;
    fn call(&mut self, arguments: Vec<LoxValue>) -> Result<LoxValue, LoxRuntimeError>;
}
