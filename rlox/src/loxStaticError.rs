pub struct LoxStaticError
{
    line: u32,
    message: String,
}
impl LoxStaticError
{
    pub fn new(line: u32, message: &str) -> LoxStaticError
    {
        LoxStaticError{ line, message: message.to_string() }
    }
}
impl std::fmt::Display for LoxStaticError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error on line {}: {}", self.line, self.message)
    }
}
