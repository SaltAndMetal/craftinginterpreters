pub mod scanner;
use scanner::Scanner;

pub mod token;
use token::{Token, TokenType};

use crate::loxErr::{LoxCompileErr, LoxOk};

use crate::chunk::{Chunk, OpCode};

use crate::value::Value;

pub struct Compiler {
    compilingChunk: Chunk,
    errors: Vec<LoxCompileErr>,
    scanner: Scanner,
    source: String,
    previous: Token,
    current: Token,
    panicMode: bool,
}

#[repr(u8)]
enum Precedence {
    Nothing = 0u8,
    Assignment = 1u8,
    Or = 2u8,
    And = 3u8,
    Equality = 4u8,
    Comparison = 5u8,
    Term = 6u8,
    Factor = 7u8,
    Unary = 8u8,
    Call = 9u8,
    Primary = 10u8,
}

impl Compiler {
    pub fn new(source: String) -> Compiler {
        //previous and current are practically speaking uninitialised, I just had to set them to
        //something for the type system
        Compiler {
            compilingChunk: Chunk::new("".to_string()),
            errors: Vec::new(),
            scanner: Scanner::new(source),
            source,
            previous: Token::new(TokenType::Nil, 0),
            current: Token::new(TokenType::Nil, 0),
            panicMode: false,
        }
    }
    fn currentChunk(&self) -> &mut Chunk {
        &mut self.compilingChunk
    }
    fn reportErr(&mut self, e: LoxCompileErr) {
        self.panicMode = true;
        self.errors.push(e);
    }
    fn advance(&mut self) {
        self.previous = self.current;
        loop {
            match self.scanner.scanToken() {
                Ok(t) => {
                    self.current = t;
                    break;
                }
                Err(e) => self.reportErr(e),
            }
        }
    }
    fn consume(&mut self, tokenType: TokenType, message: &str) {
        match (self.current.tokenType, tokenType) {
            (TokenType::Str(_), TokenType::Str(_)) => self.advance(),
            (TokenType::Number(_), TokenType::Number(_)) => self.advance(),
            (TokenType::Identifier(_), TokenType::Identifier(_)) => self.advance(),
            (a, b) if a == b => self.advance(),
            _ => self.reportErr(LoxCompileErr::new(message.to_string(), self.current.line)),
        }
    }
    fn emitByte(&mut self, byte: u8) {
        self.currentChunk().writeByte(byte, self.previous.line)
    }
    fn emitReturn(&mut self) {
        self.currentChunk()
            .writeByte(OpCode::Return as u8, self.previous.line)
    }
    fn expression(&mut self) {
        self.parsePrecedence(Precedence::Assignment);
    }
    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expected ')' after expression");
    }
    fn unary(&mut self) {
        let operator = self.previous.tokenType;

        self.parsePrecedence(Precedence::Unary);

        match operator {
            TokenType::Minus => self.emitByte(OpCode::Negate as u8),
            _ => unreachable!(),
        }
    }
    fn binary(&mut self) {
        let operator = self.previous.tokenType;
    }
    fn number(&mut self) {
        let mut value;
        if let TokenType::Number(v) = self.previous.tokenType {
            value = Value::Double(v);
        } else {
            unreachable!()
        }
        self.emitConstant(value);
    }
    fn emitConstant(&mut self, value: Value) {
        self.emitByte(OpCode::Constant as u8);
        self.emitByte(self.makeConstant(value));
    }
    fn makeConstant(&mut self, value: Value) -> u8 {
        let index = self.currentChunk().addConstant(value);
        if index > std::u8::MAX {
            self.reportErr(LoxCompileErr::new("Too many constants in one chunk".to_string(), self.previous.line));
            return 0u8;
        }
        index
    }
    pub fn compile(&mut self, source: String) -> Result<Chunk, Vec<LoxCompileErr>> {
        let mut chunk = Chunk::new("This".to_string());
        self.advance();
        let expr = self.expression();
        self.consume(TokenType::EOF, "Expect end of expression");

        match self.errors.len() {
            0 => Ok(chunk),
            _ => Err(self.errors),
        }
    }
}
