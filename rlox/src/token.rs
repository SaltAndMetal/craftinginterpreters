use phf::{phf_map, Map};

#[derive(Debug)]
pub struct Token
{
    pub tokenType: TokenType,
    pub line: u32,
}
impl Token
{
    pub fn new(tokenType: TokenType, line: u32) -> Token { Token{tokenType, line} }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType
{
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),
    BOOL(bool),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    VAR,
    WHILE,
    EOF,
}
pub const RESERVED: Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::AND,
    "class" => TokenType::CLASS,
    "else" => TokenType::ELSE,
    "false" => TokenType::BOOL(false),
    "for" => TokenType::FOR,
    "fun" => TokenType::FUN,
    "if" => TokenType::IF,
    "nil" => TokenType::NIL,
    "or" => TokenType::OR,
    "print" => TokenType::PRINT,
    "return" => TokenType::RETURN,
    "super" => TokenType::SUPER,
    "this" => TokenType::THIS,
    "true" => TokenType::BOOL(true),
    "var" => TokenType::VAR,
    "while" => TokenType::WHILE,
};
