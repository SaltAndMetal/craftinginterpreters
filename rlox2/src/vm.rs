use crate::chunk::display::disassembleInstruction;
use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::DEBUG_TRACE_EXECUTION;
use crate::{LoxOk, LoxRuntimeErr};

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(8),
        }
    }
    pub fn execute(&mut self) -> Result<LoxOk, LoxRuntimeErr> {
        fn binary_num_op(
            stack: &mut Vec<Value>,
            o: impl Fn(f64, f64) -> f64,
            line: u32,
        ) -> Result<LoxOk, LoxRuntimeErr> {
            let b = stack.pop().expect("Stack empty");
            let a = stack.pop().expect("Stack empty");
            let (a, b) = match (a, b) {
                (Value::Double(d1), Value::Double(d2)) => (d1, d2),
                (Value::Double(d1), _) => {
                    return Err(LoxRuntimeErr::new(
                        format!("Expected number, found {}", b),
                        line,
                    ))
                }
                _ => {
                    return Err(LoxRuntimeErr::new(
                        format!("Expected number, found {}", a),
                        line,
                    ))
                }
            };
            stack.push(Value::Double(o(a, b)));
            Ok(())
        }
        self.chunk.printCode();
        loop {
            let instruction = match self.chunk.readInstr(self.ip) {
                Ok(x) => x,
                Err(e) => panic!("{}", e),
            };
            if DEBUG_TRACE_EXECUTION {
                let mut stackStr = "           ".to_string();
                for value in &self.stack {
                    stackStr.push_str(format!("[{}]", value).as_str());
                }
                println!("{}", stackStr);

                println!(
                    "{}",
                    disassembleInstruction(&self.chunk, &mut (self.ip.clone()))
                );
            }
            let line = self.chunk.readLine(self.ip);
            self.ip += 1;
            match instruction {
                OpCode::Return => {
                    match self.stack.pop() {
                        Some(v) => println!("{}", v),
                        None => println!(""),
                    }
                    return Ok(());
                }
                OpCode::Constant => {
                    let index = self.chunk.readByte(self.ip);
                    let constant: Value = self.chunk.readConstant(index);
                    self.stack.push(constant);
                    self.ip += 1;
                }
                OpCode::Negate => {
                    let end = self.stack.len() - 1;
                    match self.stack[end] {
                        Value::Double(d) => self.stack[end] = Value::Double(-d),
                    }
                }
                OpCode::Add => binary_num_op(&mut self.stack, |a, b| a + b, line)?,
                OpCode::Subtract => binary_num_op(&mut self.stack, |a, b| a - b, line)?,
                OpCode::Multiply => binary_num_op(&mut self.stack, |a, b| a * b, line)?,
                OpCode::Divide => binary_num_op(&mut self.stack, |a, b| a / b, line)?,
            }
        }
    }
}
