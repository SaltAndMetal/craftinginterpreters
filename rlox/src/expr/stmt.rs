use super::Expr;

#[derive(Clone)]
pub enum Decl
{
    FunDecl(String, Vec<String>, Vec<Decl>),
    VarDecl(String, Expr),
    Stmt(Stmt),
}

#[derive(Clone)]
pub enum Stmt
{
    ExprStmt(Expr),
    IfStmt(Expr, Box<Stmt>, Option<Box<Stmt>>),
    WhileStmt(Expr, Box<Stmt>),
    PrintStmt(Expr),
    ReturnStmt(Expr),
    Block(Vec<Decl>),
}
