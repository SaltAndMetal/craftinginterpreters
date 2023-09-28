use crate::token::{Token, TokenType, RESERVED};
use crate::LoxStaticError;

pub fn scan(source: String) -> (Vec<Token>, Vec<LoxStaticError>)
{
    let mut errors: Vec<LoxStaticError> = Vec::new();
    let mut source = source.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: u32 = 1;
    let mut addToken = |tokenType, line| {tokens.push(Token::new(tokenType, line))};

    while let Some(c) = source.next() {
        match c {
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => line += 1,
            '(' => addToken(TokenType::LEFT_PAREN, line),
            ')' => addToken(TokenType::RIGHT_PAREN, line),
            '{' => addToken(TokenType::LEFT_BRACE, line),
            '}' => addToken(TokenType::RIGHT_BRACE, line),
            ',' => addToken(TokenType::COMMA, line),
            '.' => addToken(TokenType::DOT, line),
            '-' => addToken(TokenType::MINUS, line),
            '+' => addToken(TokenType::PLUS, line),
            ';' => addToken(TokenType::SEMICOLON, line),
            '*' => addToken(TokenType::STAR, line),
            '!' => {
                if let Some('=') = source.peek() {
                    source.next();
                    addToken(TokenType::BANG_EQUAL, line);
                }
                else {
                    addToken(TokenType::BANG, line);
                }
            }
            '=' => {
                if let Some('=') = source.peek() {
                    source.next();
                    addToken(TokenType::EQUAL_EQUAL, line);
                }
                else {
                    addToken(TokenType::EQUAL, line);
                }
            }
            '<' => {
                if let Some('=') = source.peek() {
                    source.next();
                    addToken(TokenType::LESS_EQUAL, line);
                }
                else {
                    addToken(TokenType::LESS, line);
                }
            }
            '>' => {
                if let Some('=') = source.peek() {
                    source.next();
                    addToken(TokenType::GREATER_EQUAL, line);
                }
                else {
                    addToken(TokenType::GREATER, line);
                }
            }
            '/' => {
                if let Some('/') = source.peek() {
                    while let Some(nc) = source.peek() {
                        if *nc != '\n' {
                            source.next();
                        }
                        else {
                            break;
                        }
                    }
                    
                }
                else {
                    addToken(TokenType::SLASH, line);
                }
            }
            '"' => {
                let mut string = String::new();
                loop {
                    match source.next() {
                        Some(nc) => {
                            match nc {
                                '\n' => line += 1,
                                '"' => break,
                                _ => string.push(nc),
                            }
                        }
                        None => {
                            errors.push(LoxStaticError::new(line, "Unterminated string"));
                            break;
                        }
                    }
                }
                addToken(TokenType::STRING(string), line);
            },
            x if x.is_digit(10) => {
                let mut str_rep = String::new();
                str_rep.push(x);
                let mut dot = false;
                let mut last: Option<char>;
                let mut end = false;
                loop {
                    last = source.next();
                    match last {
                        Some(nc) => {
                            match nc {
                                x if x.is_digit(10) => str_rep.push(nc),
                                x if x == '.' => {
                                    let next = source.peek().unwrap_or(&'_');
                                    if !dot && next.is_digit(10){
                                        str_rep.push(nc);
                                        dot = true;
                                    }
                                    else { 
                                        break; 
                                    }
                                },
                                _ => break,
                            }
                        },
                        None => { end = true; break },
                    }
                }
                addToken(TokenType::NUMBER(str_rep.parse::<f64>().unwrap()), line);
                if !end {
                    let mut extra = scan(String::from(last.unwrap()));
                    errors.append(&mut extra.1);
                    addToken(extra.0[0].tokenType.clone(), line);
                }
            },
            x if x.is_alphabetic() || x == '_' => {

                let mut ident = String::new();
                ident.push(x);
                while let Some(nc) = source.peek() {
                    match nc {
                        x if x.is_alphanumeric() || *x == '_' => ident.push(*x),
                        _ => break,
                    }
                    source.next();
                }
                match RESERVED.get(ident.as_str()) {
                    Some(t) => addToken(t.clone(), line),
                    None => addToken(TokenType::IDENTIFIER(ident), line),
                }
            },
            
            _ => errors.push(LoxStaticError::new(line, "Unexpected character")),
        }
    }

    tokens.push(Token::new(TokenType::EOF, line));
    (tokens, errors)
}
