use super::super::LoxValue;

pub enum LoxFunctionError
{
    ArgPrmCountMismatch(u32, u32),
    NotCallable(LoxValue),
    NonFnReturn,
}
