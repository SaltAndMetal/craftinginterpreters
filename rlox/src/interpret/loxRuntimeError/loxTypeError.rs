use super::super::loxType::LoxType;

pub enum LoxTypeError
{
    Mismatch(LoxTypeMismatch),
}

pub struct LoxTypeMismatch
{
    pub found: LoxType,
    pub expected: Vec<LoxType>,
}
impl LoxTypeMismatch
{
    pub fn new(found: LoxType, expected: LoxType) -> LoxTypeMismatch
    {
        LoxTypeMismatch{found, expected: vec![expected]}
    }
    pub fn newMany(found: LoxType, expected: Vec<LoxType>) -> LoxTypeMismatch
    {
        LoxTypeMismatch{found, expected}
    }
}
