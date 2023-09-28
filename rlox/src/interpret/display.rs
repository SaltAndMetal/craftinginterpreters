use std::fmt::Display;
use std::fmt;

use super::loxType::LoxType;
use super::loxValue::{LoxValue, LoxFn};
use super::loxRuntimeError::{LoxRuntimeError, LoxRuntimeErrorType, LoxVariableError, LoxTypeError, loxTypeError::LoxTypeMismatch, LoxFunctionError};
use super::loxType::LoxCallee;

impl Display for LoxType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Self::Function(c) => write!(f, "Function with arity {}", c),
            Self::Num => write!(f, "Num"),
            Self::Str => write!(f, "Str"),
            Self::Bool => write!(f, "Bool"),
            Self::Nil => write!(f, "Nil"),
        }
    }
}
impl Display for LoxTypeMismatch
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut string = "Expected ".to_string();
        let mut expected = self.expected.iter();
        string.push_str(format!("{}", expected.next().unwrap()).as_str());
        for loxType in expected {
            string.push_str(format!("or {}", loxType).as_str());
        }
        write!(f, "{}, but found {}", string, self.found)
    }
}
impl Display for LoxTypeError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let string = match self {
            Self::Mismatch(m) => format!("{}", m),
        };
        write!(f, "{}", string)
    }
}
impl Display for LoxVariableError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let string = match self {
            Self::Missing(m) => format!("Variable {} does not exist", m),
        };
        write!(f, "{}", string)
    }
}
impl Display for LoxRuntimeErrorType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let string = match self {
            Self::VariableError(e) => format!("{}", e),
            Self::TypeError(e) => format!("{}", e),
            Self::FunctionError(e) => format!("{}", e),
        };
        write!(f, "{}", string)
    }
}
impl Display for LoxFunctionError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let string = match self {
            Self::ArgPrmCountMismatch(a, p) => format!("Expected {} arguments but found {}", a, p),
            Self::NotCallable(e) => format!("Expression {} is not callable", e),
            Self::NonFnReturn => "Cannot return from outside a function".to_string(),
        };
        write!(f, "{}", string)
    }
}
impl Display for LoxRuntimeError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Error on line {}: {}", self.line, self.errorType,)
    }
}
impl Display for LoxFn
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Function with arity {}", self.arity())
    }
}
impl Display for LoxValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let string = match self {
            Self::Num(n) => format!("{}", n),
            Self::Str(s) => format!(r#""{}""#, s),
            Self::Bool(b) => format!("{}", b),
            Self::Function(f) => format!("{}", f),
            Self::Nil => "nil".to_string(),
        };
        write!(f, "{}", string)
    }
}
