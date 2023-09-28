#[derive(Debug, Copy, Clone)]
pub enum Value {
    Double(f64),
}

pub mod display {
    use super::Value;
    use std::fmt;
    use std::fmt::Display;

    impl Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let string = match self {
                Value::Double(f) => f.to_string(),
            };
            write!(f, "'{}'", string)
        }
    }
}
