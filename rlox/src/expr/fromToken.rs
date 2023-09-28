use super::*;
use crate::token::TokenType;
use std::convert::TryFrom;
impl TryFrom<TokenType> for Boperator
{
    type Error = ();
    fn try_from(token: TokenType) -> Result<Self, Self::Error> {
        match token {
            TokenType::EQUAL_EQUAL => Ok(Boperator::EqualEqual),
            TokenType::BANG_EQUAL => Ok(Boperator::BangEqual),
            TokenType::LESS => Ok(Boperator::Less),
            TokenType::LESS_EQUAL => Ok(Boperator::LessEqual),
            TokenType::GREATER => Ok(Boperator::Greater),
            TokenType::GREATER_EQUAL => Ok(Boperator::GreaterEqual),
            TokenType::PLUS => Ok(Boperator::Plus),
            TokenType::MINUS => Ok(Boperator::Minus),
            TokenType::STAR => Ok(Boperator::Star),
            TokenType::SLASH => Ok(Boperator::Slash),
            _ => Err(()),
        }
    }
}
impl TryFrom<TokenType> for Loperator
{
    type Error = ();
    fn try_from(token: TokenType) -> Result<Self, Self::Error> {
        match token {
            TokenType::OR => Ok(Loperator::Or),
            TokenType::AND => Ok(Loperator::And),
            _ => Err(()),
        }
    }
}
impl TryFrom<TokenType> for Uoperator
{
    type Error = ();
    fn try_from(token: TokenType) -> Result<Self, Self::Error> {
        match token {
            TokenType::BANG=> Ok(Uoperator::Bang),
            TokenType::MINUS => Ok(Uoperator::Minus),
            _ => Err(()),
        }
    }
}
impl TryFrom<TokenType> for Literal
{
    type Error = ();
    fn try_from(token: TokenType) -> Result<Self, Self::Error> {
        match token {
            TokenType::NUMBER(n) => Ok(Literal::Num(n)),
            TokenType::STRING(s) => Ok(Literal::Str(s)),
            TokenType::BOOL(b) => Ok(Literal::Bool(b)),
            TokenType::NIL => Ok(Literal::Nil),
            _ => Err(()),
        }
    }
}
