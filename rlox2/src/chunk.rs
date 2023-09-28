use crate::value::Value;

#[repr(u8)]
pub enum OpCode {
    Return = 0u8,
    Constant = 1u8,
    Negate = 2u8,
    Add = 3u8,
    Subtract = 4u8,
    Multiply = 5u8,
    Divide = 6u8,
}
impl TryFrom<u8> for OpCode {
    type Error = String;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0u8 => Ok(OpCode::Return),
            1u8 => Ok(OpCode::Constant),
            2u8 => Ok(OpCode::Negate),
            3u8 => Ok(OpCode::Add),
            4u8 => Ok(OpCode::Subtract),
            5u8 => Ok(OpCode::Multiply),
            6u8 => Ok(OpCode::Divide),
            _ => Err("Byte is not a valid instruction".to_string()),
        }
    }
}

pub struct Chunk {
    name: String,
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}
impl Chunk {
    pub fn new(name: String) -> Chunk {
        Chunk {
            name,
            code: Vec::with_capacity(8),
            constants: Vec::new(),
            lines: Vec::with_capacity(8),
        }
    }
    pub fn writeByte(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }
    pub fn readByte(&self, offset: usize) -> u8 {
        self.code[offset]
    }
    pub fn readLine(&self, offset: usize) -> u32 {
        self.lines[offset]
    }
    pub fn printCode(&self) {
        println!("{:?}", self.code);
    }
    pub fn readInstr(&self, offset: usize) -> Result<OpCode, String> {
        self.code[offset].try_into()
    }
    pub fn addConstant(&mut self, constant: Value) -> u8 {
        self.constants.push(constant);
        (self.constants.len() - 1) as u8
    }
    pub fn readConstant(&self, index: u8) -> Value {
        self.constants[index as usize].clone()
    }
}

pub mod display {
    use super::Chunk;
    use super::OpCode;
    use std::fmt;
    use std::fmt::Display;

    impl Display for Chunk {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut string = format!("== {} ==", self.name);

            let mut offset = 0;
            while offset < self.code.len() {
                let instrStr = disassembleInstruction(self, &mut offset);
                string.push_str("\n");
                string.push_str(instrStr.to_string().as_str());
            }
            write!(f, "{}", string)
        }
    }
    impl Display for OpCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let string = match self {
                OpCode::Return => "RETURN",
                OpCode::Constant => "CONSTANT",
                OpCode::Negate => "NEGATE",
                OpCode::Add => "ADD",
                OpCode::Subtract => "SUBTRACT",
                OpCode::Multiply => "MULTIPLY",
                OpCode::Divide => "DIVIDE",
            };
            write!(f, "{}", string)
        }
    }
    pub fn disassembleInstruction(chunk: &Chunk, offset: &mut usize) -> String {
        fn simpleInstruction(name: &str, offset: &mut usize) -> String {
            *offset += 1;
            format!("{}", name)
        }
        fn constantInstruction(name: &str, chunk: &Chunk, offset: &mut usize) -> String {
            let index: u8 = chunk.code[*offset + 1];
            let value = &chunk.constants[index as usize];
            *offset += 2;
            format!("{}         {} {}", name, index, *value)
        }
        let prefix = format!("{:04}  ", offset);
        let line = if (*offset > 0) && (chunk.lines[*offset] == chunk.lines[*offset - 1]) {
            "   | ".to_string()
        } else {
            format!("{:04} ", chunk.lines[*offset])
        };

        let instruction = chunk.readInstr(*offset).unwrap();
        let instrStr = match instruction {
            OpCode::Return => simpleInstruction(instruction.to_string().as_str(), offset),
            OpCode::Constant => {
                constantInstruction(instruction.to_string().as_str(), chunk, offset)
            }
            OpCode::Negate => simpleInstruction(instruction.to_string().as_str(), offset),
            OpCode::Add => simpleInstruction(instruction.to_string().as_str(), offset),
            OpCode::Subtract => simpleInstruction(instruction.to_string().as_str(), offset),
            OpCode::Multiply => simpleInstruction(instruction.to_string().as_str(), offset),
            OpCode::Divide => simpleInstruction(instruction.to_string().as_str(), offset),
        };
        format!("{}{}{}", prefix, line, instrStr)
    }
}
