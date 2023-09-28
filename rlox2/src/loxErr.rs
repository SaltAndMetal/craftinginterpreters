pub enum LoxErr {
    Runtime(LoxRuntimeErr),
    Compile(LoxCompileErr),
}
pub type LoxOk = ();
pub struct LoxRuntimeErr {
    message: String,
    line: u32,
}
impl LoxRuntimeErr {
    pub fn new(message: String, line: u32) -> LoxRuntimeErr {
        LoxRuntimeErr { message, line }
    }
}
pub struct LoxCompileErr {
    message: String,
    line: u32,
}
impl LoxCompileErr {
    pub fn new(message: String, line: u32) -> LoxCompileErr {
        LoxCompileErr { message, line }
    }
}
impl From<LoxRuntimeErr> for LoxErr {
    fn from(e: LoxRuntimeErr) -> LoxErr {
        LoxErr::Runtime(e)
    }
}
impl From<LoxCompileErr> for LoxErr {
    fn from(e: LoxCompileErr) -> LoxErr {
        LoxErr::Compile(e)
    }
}

pub mod display {
    use super::{LoxCompileErr, LoxErr, LoxRuntimeErr};
    use std::fmt;
    use std::fmt::Display;

    impl Display for LoxCompileErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Compile error on line {}: {}", self.line, self.message)
        }
    }
    impl Display for LoxRuntimeErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Runtime error on line {}: {}", self.line, self.message)
        }
    }
    impl Display for LoxErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let string = match self {
                Self::Runtime(r) => r.to_string(),
                Self::Compile(c) => c.to_string(),
            };
            write!(f, "{}", string)
        }
    }
}
