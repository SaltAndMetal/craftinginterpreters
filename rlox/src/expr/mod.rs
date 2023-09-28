pub mod display;
pub mod fromToken;
pub mod stmt;

#[derive(Clone)]
pub struct Expr
{
    pub exprType: ExprType,
    pub line: u32,
}
impl Expr
{
    pub fn new(exprType: ExprType, line: u32) -> Self
    {
        Expr{exprType, line}
    }
}

#[derive(Clone)]
pub enum ExprType
{
    Literal(Literal),
    Variable(String),
    Assignment(Assignment),
    Call(Call),
    Unary(Unary),
    Logical(Logical),
    Binary(Binary),
    Grouping(Grouping),
}
#[derive(Clone)]
pub struct Call
{
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
}
#[derive(Clone)]
pub enum Literal
{
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}
#[derive(Clone)]
pub struct Assignment
{
    pub id: String,
    pub expr: Box<Expr>,
}
#[derive(Clone)]
pub struct Grouping
{
    pub expr: Box<Expr>
}
#[derive(Clone)]
pub struct Unary
{
    pub operator: Uoperator,
    pub expr: Box<Expr>,
}
#[derive(Clone)]
pub enum Uoperator
{
    Minus,
    Bang,
}
#[derive(Clone)]
pub struct Binary
{
    pub lexpr: Box<Expr>,
    pub operator: Boperator,
    pub rexpr: Box<Expr>,
}
#[derive(Clone)]
pub enum Boperator
{
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}
#[derive(Clone)]
pub struct Logical
{
    pub lexpr: Box<Expr>,
    pub operator: Loperator,
    pub rexpr: Box<Expr>,
}
#[derive(Clone)]
pub enum Loperator
{
    Or,
    And,
}
