pub mod loxTypeError;
pub use loxTypeError::LoxTypeError;
pub mod loxVariableError;
pub use loxVariableError::LoxVariableError;
pub mod loxFunctionError;
pub use loxFunctionError::LoxFunctionError;

pub struct LoxRuntimeError
{
    pub errorType: LoxRuntimeErrorType,
    pub line: u32,
}

impl LoxRuntimeError
{
    pub fn new(errorType: LoxRuntimeErrorType, line: u32) -> Self
    {
        LoxRuntimeError{errorType, line}
    }
}

pub enum LoxRuntimeErrorType
{
    TypeError(LoxTypeError),
    VariableError(LoxVariableError),
    FunctionError(LoxFunctionError),
}

impl From<LoxTypeError> for LoxRuntimeErrorType
{
    fn from(typeError: LoxTypeError) -> Self
    {
        LoxRuntimeErrorType::TypeError(typeError)
    }
}
impl From<LoxVariableError> for LoxRuntimeErrorType
{
    fn from(variableError: LoxVariableError) -> Self
    {
        LoxRuntimeErrorType::VariableError(variableError)
    }
}
impl From<LoxFunctionError> for LoxRuntimeErrorType
{
    fn from(functionError: LoxFunctionError) -> Self
    {
        LoxRuntimeErrorType::FunctionError(functionError)
    }
}
