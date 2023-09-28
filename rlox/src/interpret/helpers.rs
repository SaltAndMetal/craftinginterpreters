use super::loxValue::LoxValue;
use super::loxRuntimeError::{LoxRuntimeError, loxTypeError::{LoxTypeError, LoxTypeMismatch}};
use super::loxType::LoxType;

pub fn truthy(value: &LoxValue) -> bool
{
    match value {
        LoxValue::Num(_) => true,
        LoxValue::Str(_) => true,
        LoxValue::Bool(b) => *b,
        LoxValue::Function(_) => true,
        LoxValue::Nil => false,
    }
}

pub fn equal(v1: &LoxValue, v2: &LoxValue) -> bool
{
    match (v1, v2) {
        (LoxValue::Num(n1), LoxValue::Num(n2)) => n1==n2,
        (LoxValue::Str(s1), LoxValue::Str(s2)) => s1==s2,
        (LoxValue::Bool(b1), LoxValue::Bool(b2)) => b1==b2,
        (LoxValue::Nil, LoxValue::Nil,) => true,
        _ => false
    }
}
fn num_mismatch_err(x: LoxValue, line: u32) -> LoxRuntimeError
{
    LoxRuntimeError::new(LoxTypeError::Mismatch(LoxTypeMismatch::new(x.into(), LoxType::Num)).into(), line)
}
pub fn binary_num_op(l: LoxValue, r: LoxValue, o: impl Fn(f64, f64) -> f64, line: u32) -> Result<LoxValue, LoxRuntimeError>
{
    match (l, r) {
        (LoxValue::Num(n1), LoxValue::Num(n2)) => Ok(LoxValue::Num(o(n1, n2))),
        (LoxValue::Num(_), x) => Err(num_mismatch_err(x, line)),
        (x, _) => Err(num_mismatch_err(x, line)),
    }
}
pub fn binary_cmp_op(l: LoxValue, r: LoxValue, o: impl Fn(f64, f64) -> bool, line: u32) -> Result<LoxValue, LoxRuntimeError>
{
    match (l, r) {
        (LoxValue::Num(n1), LoxValue::Num(n2)) => Ok(LoxValue::Bool(o(n1, n2))),
        (LoxValue::Num(_), x) => Err(num_mismatch_err(x, line)),
        (x, _) => Err(num_mismatch_err(x, line)),
    }
}
