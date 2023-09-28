use super::token::{Token, TokenType};

use crate::loxErr::LoxCompileErr;

pub struct Scanner {
    source: String,
    current: usize,
    line: u32,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            current: 0,
            line: 0,
            source,
        }
    }
    fn end(&self) -> bool {
        self.current == self.source.chars().count()
    }
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
    fn match_ahead(&mut self, expected: char) -> bool {
        if self.end() {
            return false;
        };
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        };
        self.current += 1;
        true
    }
    fn look_ahead_equal(&mut self, with: TokenType, without: TokenType) -> TokenType {
        if self.match_ahead('=') {
            with
        } else {
            without
        }
    }
    fn skip_white_space(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' => {
                    self.advance();
                }
                '\r' => {
                    self.advance();
                }
                '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                }
                '/' => {
                    if let Some('/') = self.peek_next() {
                        while (self.peek() != '\n') && (!self.end()) {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            };
        }
    }
    fn string(&mut self) -> Result<Token, LoxCompileErr> {
        //Advances through string, consuming it, and stopping at a quote or EOF
        let mut string = String::new();
        while (self.peek() != '"') && (!self.end()) {
            //Tracks new-lines
            if self.peek() == '\n' {
                self.line += 1;
            };
            string.push(self.advance());
        }

        if self.end() {
            return Err(LoxCompileErr::new(
                "Unterminated string".to_string(),
                self.line,
            ));
        };

        //The closing quote
        self.advance();

        Ok(Token::new(TokenType::Str(string), self.line))
    }
    fn number(&mut self) -> Token {
        let mut number = String::new();

        //Whole number part
        while self.peek().is_digit(10) {
            number.push(self.advance())
        }

        //Decimal point
        if (self.peek() == '.') && (self.peek_next().unwrap_or(' ').is_digit(10)) {
            number.push(self.advance());
        }

        //Fractional part
        while self.peek().is_digit(10) {
            number.push(self.advance())
        }

        Token::new(TokenType::Number(number.parse().unwrap()), self.line)
    }
    fn identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while (self.peek().is_alphabetic()) || (self.peek() == '_') || (self.peek().is_digit(10)) {
            identifier.push(self.advance());
        }
        Token::new(Self::identifierType(identifier), self.line)
    }
    fn identifierType(identifier: String) -> TokenType {
        fn check(identifier: String, position: usize, rest: &str, retVal: TokenType) -> TokenType {
            let mut identifierAdvanced = identifier.chars();
            for _ in 0..position {
                identifierAdvanced.next();
            }
            let identifierAdvanced: String = identifierAdvanced.collect();
            if (position + rest.chars().count() == identifier.chars().count())
                && (rest == identifierAdvanced)
            {
                retVal
            } else {
                TokenType::Identifier(identifier)
            }
        }
        let mut chars = identifier.chars();

        let c = chars.next();
        if let None = c {
            return TokenType::Identifier(identifier);
        }
        let c = c.unwrap();

        match c {
            'a' => check(identifier, 1, "nd", TokenType::And),
            'c' => check(identifier, 1, "lass", TokenType::Class),
            'e' => check(identifier, 1, "lse", TokenType::Else),
            'i' => check(identifier, 1, "f", TokenType::If),
            'n' => check(identifier, 1, "il", TokenType::Nil),
            'o' => check(identifier, 1, "r", TokenType::Or),
            'p' => check(identifier, 1, "rint", TokenType::Print),
            'r' => check(identifier, 1, "eturn", TokenType::Return),
            's' => check(identifier, 1, "uper", TokenType::Super),
            'v' => check(identifier, 1, "ar", TokenType::Var),
            'w' => check(identifier, 1, "hile", TokenType::While),

            'f' => {
                let c = chars.next();
                if let None = c {
                    return TokenType::Identifier(identifier);
                }
                let c = c.unwrap();
                match c {
                    'a' => check(identifier, 2, "lse", TokenType::False),
                    'o' => check(identifier, 2, "r", TokenType::For),
                    'u' => check(identifier, 2, "n", TokenType::Fun),
                }
            }

            't' => {
                let c = chars.next();
                if let None = c {
                    return TokenType::Identifier(identifier);
                }
                let c = c.unwrap();
                match c {
                    'h' => check(identifier, 3, "is", TokenType::This),
                    'r' => check(identifier, 3, "ue", TokenType::True),
                }
            }

            _ => TokenType::If,
        };
        TokenType::If
    }
    pub fn scanToken(&mut self) -> Result<Token, LoxCompileErr> {
        self.skip_white_space();
        if self.end() {
            return Ok(Token::new(TokenType::EOF, self.line));
        };
        let c = self.advance();
        match c {
            '(' => return Ok(Token::new(TokenType::LeftParen, self.line)),
            ')' => return Ok(Token::new(TokenType::RightParen, self.line)),
            '{' => return Ok(Token::new(TokenType::LeftBrace, self.line)),
            '}' => return Ok(Token::new(TokenType::RightBrace, self.line)),
            ';' => return Ok(Token::new(TokenType::Semicolon, self.line)),
            ',' => return Ok(Token::new(TokenType::Comma, self.line)),
            '.' => return Ok(Token::new(TokenType::Dot, self.line)),
            '-' => return Ok(Token::new(TokenType::Minus, self.line)),
            '+' => return Ok(Token::new(TokenType::Plus, self.line)),
            '/' => return Ok(Token::new(TokenType::Slash, self.line)),
            '*' => return Ok(Token::new(TokenType::Star, self.line)),

            '!' => {
                return Ok(Token::new(
                    self.look_ahead_equal(TokenType::BangEqual, TokenType::Bang),
                    self.line,
                ))
            }
            '=' => {
                return Ok(Token::new(
                    self.look_ahead_equal(TokenType::EqualEqual, TokenType::Equal),
                    self.line,
                ))
            }
            '<' => {
                return Ok(Token::new(
                    self.look_ahead_equal(TokenType::LessEqual, TokenType::Less),
                    self.line,
                ))
            }
            '>' => {
                return Ok(Token::new(
                    self.look_ahead_equal(TokenType::GreaterEqual, TokenType::Greater),
                    self.line,
                ))
            }

            '"' => return self.string(),

            x if x.is_digit(10) => return Ok(self.number()),

            x if x.is_alphabetic() || x == '_' => return Ok(self.identifier()),

            _ => {
                return Err(LoxCompileErr::new(
                    "Unrecognised character".to_string(),
                    self.line,
                ))
            }
        }
    }
}
