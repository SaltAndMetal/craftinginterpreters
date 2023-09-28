use super::*;
use super::stmt::{Stmt, Decl};
use std::fmt;
use std::fmt::Display;
impl Display for Boperator
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::EqualEqual => "==",
            Self::BangEqual => "!=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
        };
        write!(f, "{}", repr)
    }
}
impl Display for Loperator
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Or => "or",
            Self::And => "and",
        };
        write!(f, "{}", repr)
    }
}
impl Display for Uoperator
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Minus => "-",
            Self::Bang => "!",
        };
        write!(f, "{}", repr)
    }
}
impl Display for Literal
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Num(n) => format!("{}", n),
            Self::Str(s) => s.to_string(),
            Self::Bool(b) => format!("{}", b),
            Self::Nil => "nil".to_owned(),
        };
        write!(f, "{}", repr)
    }
}
impl Display for Call
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut arguments = String::new();
        for a in &self.arguments {
            arguments.push_str(format!("{}", a).as_str());
        }
        write!(f, "{}({})", self.callee, arguments)
    }
}
impl Display for Unary
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.operator, self.expr)
    }
}
impl Display for Binary
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.operator, self.lexpr, self.rexpr)
    }
}
impl Display for Logical
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.operator, self.lexpr, self.rexpr)
    }
}
impl Display for Grouping
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "group {}", self.expr)
    }
}
impl Display for Assignment
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.id, self.expr)
    }
}
impl Display for Expr
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self.exprType {
            ExprType::Literal(ref l) => format!("{}", l),
            ExprType::Variable(ref n) => n.clone(),
            ExprType::Assignment(ref a) => format!("{}", a),
            ExprType::Call(ref c) => format!("{}", c),
            ExprType::Unary(ref u) => format!("({})", u),
            ExprType::Logical(ref l) => format!("({})", l),
            ExprType::Binary(ref b) => format!("({})", b),
            ExprType::Grouping(ref g) => format!("({})", g),
        };
        write!(f, "{}", repr)
    }
}
impl Display for Stmt
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::PrintStmt(e) => format!("print {};", e),
            Self::IfStmt(c, i, e) => {
                let elseStr = match e {
                    Some(x) => format!("\nelse {}", x),
                    None => String::new(),
                };
                format!("if {}\n{}{}", c, i, elseStr)
            },
            Self::WhileStmt(c, b) => format!("while {}\n{};", c, b),
            Self::ExprStmt(e) => format!("{};", e),
            Self::ReturnStmt(r) => format!("return {};", r),
            Self::Block(b) => {
                let mut repr = "{\n".to_string();
                for d in b.iter() {
                    repr.push_str(format!("{}\n", d).as_str());
                }
                repr.push('}');
                repr.to_string()
            },
        };
        write!(f, "{}", repr)
    }
}
impl Display for Decl
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::FunDecl(n, p, b) => {
                let first = format!("fun {}", n);
                let mut params = "(".to_string();
                for a in p {
                    params.push_str(format!("{}", a).as_str());
                }
                params.push_str(") {\n");
                let mut body = String::new();
                for d in b {
                    body.push_str(format!("{}\n", d).as_str());
                }
                body.push_str("}");
                format!("{}{}{}", first, params, body)
            },
            Self::VarDecl(n, e) => format!("var {} = {};", n, e),
            Self::Stmt(s) => format!("{}", s),
        };
        write!(f, "{}", repr)
    }
}
