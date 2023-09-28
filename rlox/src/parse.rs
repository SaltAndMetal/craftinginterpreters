use crate::token::*;
use crate::expr::*;
use crate::expr::stmt::{Stmt, Decl};
use crate::LoxStaticError;

pub struct Parser
{
    pub tokens: Vec<Token>,
    current: usize,
    errors: Vec<LoxStaticError>,
}
impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        Parser{tokens, current: 0, errors: Vec::new()}
    }
    pub fn parse(mut self) -> (Vec<Decl>, Vec<LoxStaticError>)
    {
        let mut decls: Vec<Decl> = Vec::new();
        while !self.end() {
            decls.push(self.declaration());
        }
        (decls, self.errors)
    }
    fn declaration(&mut self) -> Decl
    {
        if self.check(vec![TokenType::VAR]) { return self.varDeclaration(); }
        if self.check(vec![TokenType::FUN]) { return self.funDeclaration("function"); }
        Decl::Stmt(self.statement())
    }
    fn idConsume(&mut self, err: &str) -> String
    {
        if !self.end() {
            match self.tokens[self.current].tokenType.clone() {
                TokenType::IDENTIFIER(n) => {
                    self.current += 1;
                    n
                },
                _ => { self.error(err); String::new() },
            }
        }
        else {
            self.error(err);
            String::new()
        }
    }
    fn funDeclaration(&mut self, kind: &str) -> Decl
    {
        let name = self.idConsume(format!("Expected {} name", kind).as_str());
        self.consume(TokenType::LEFT_PAREN, format!(r#"Expected "(" after {} name"#, kind).as_str());
        let mut parameters = Vec::new();
        if self.tokens[self.current].tokenType != TokenType::RIGHT_PAREN {
            loop {
                if parameters.len() >= 255 {
                    self.error("Can't have more than 255 parameters");
                }
                parameters.push(self.idConsume("Expected parameter name"));
                if !self.check(vec![TokenType::COMMA]) { break }
            }
        }
        self.consume(TokenType::RIGHT_PAREN, r#"Expected ")" after parameters"#);
        self.consume(TokenType::LEFT_BRACE, format!(r#"Expected "{{" before {} body"#, kind).as_str());
        let body = self.block();
        Decl::FunDecl(name, parameters, body)
    }
    fn varDeclaration(&mut self) -> Decl
    {
        let name = self.idConsume("Expected variable name");
        let mut value = Expr::new(ExprType::Literal(Literal::Nil), self.tokens[self.current].line);
        if self.check(vec![TokenType::EQUAL]) {
            value = self.expression();
        }
        self.consume(TokenType::SEMICOLON, r#"Expected ";" after variable declaration"#);
        Decl::VarDecl(name, value)
    }
    fn statement(&mut self) -> Stmt
    {
        if self.check(vec![TokenType::IF]) { self.ifStatement() }
        else if self.check(vec![TokenType::PRINT]) { self.printStatement() }
        else if self.check(vec![TokenType::LEFT_BRACE]) { Stmt::Block(self.block()) }
        else if self.check(vec![TokenType::WHILE]) { self.whileStatement() }
        else if self.check(vec![TokenType::FOR]) { self.forStatement() }
        else if self.check(vec![TokenType::RETURN]) { self.returnStatement() }
        else { self.expressionStatement() }
    }
    fn returnStatement(&mut self) -> Stmt
    {
        if self.check(vec![TokenType::SEMICOLON]) {
            return Stmt::ReturnStmt(Expr::new(ExprType::Literal(Literal::Nil), self.tokens[self.current-1].line));
        }
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, r#"Expected ";" after return value"#);
        Stmt::ReturnStmt(expr)
    }
    fn forStatement(&mut self) -> Stmt
    {
        self.consume(TokenType::LEFT_PAREN, r#"Expected "(" after "for""#);
        let initialiser;
        if self.check(vec![TokenType::SEMICOLON]) {
            initialiser = None;
        }
        else if self.check(vec![TokenType::VAR]) {
            initialiser = Some(self.varDeclaration());
        }
        else {
            initialiser = Some(Decl::Stmt(self.expressionStatement()));
        }
        let mut condition = None;
        let condLine = self.tokens[self.current].line;
        if self.tokens[self.current].tokenType != TokenType::SEMICOLON {
            condition = Some(self.expression());
        }
        self.consume(TokenType::SEMICOLON, r#"Expected ";" after loop condition"#);
        let mut increment = None;
        if self.tokens[self.current].tokenType != TokenType::RIGHT_PAREN {
            increment = Some(self.expression());
        }
        self.consume(TokenType::RIGHT_PAREN, r#"Expected ")" after for clauses"#);
        let mut body = self.statement();
        if let Some(i) = increment {
            body = Stmt::Block(vec![
                Decl::Stmt(body),
                Decl::Stmt(Stmt::ExprStmt(i)),
            ])
        };
        if condition.is_none() {
            condition = Some(Expr::new(ExprType::Literal(Literal::Bool(true)), condLine));
        };
        body = Stmt::WhileStmt(condition.unwrap(), Box::new(body));
        if let Some(i) = initialiser {
            body = Stmt::Block(vec![
                i,
                Decl::Stmt(body),
            ])
        }
        body
    }
    fn whileStatement(&mut self) -> Stmt
    {
        self.consume(TokenType::LEFT_PAREN, r#"Expected "(" after "while""#);
        let condition = self.expression();
        self.consume(TokenType::RIGHT_PAREN, r#"Expected ")" after while condition"#);
        let body = self.statement();
        Stmt::WhileStmt(condition, Box::new(body))
    }
    fn ifStatement(&mut self) -> Stmt
    {
        self.consume(TokenType::LEFT_PAREN, r#"Expected "(" after "if""#);
        let condition = self.expression();
        self.consume(TokenType::RIGHT_PAREN, r#"Expected ")" after if condition"#);
        let thenBranch = self.statement();
        let mut elseBranch = None;
        if self.check(vec![TokenType::ELSE]) {
            elseBranch = Some(Box::new(self.statement()));
        }
        Stmt::IfStmt(condition, Box::new(thenBranch), elseBranch)
    }
    fn block(&mut self) -> Vec<Decl>
    {
        let mut decls: Vec<Decl> = Vec::new();
        while !self.check(vec![TokenType::RIGHT_BRACE]) && !self.end() {
            decls.push(self.declaration());
        }
        self.current -= 1;
        self.consume(TokenType::RIGHT_BRACE, r#"Expected "}" after block"#);
        decls
    }
    fn printStatement(&mut self) -> Stmt
    {
        let value = self.expression();
        self.consume(TokenType::SEMICOLON, r#"Expected ";" after value"#);
        Stmt::PrintStmt(value)
    }
    fn expressionStatement(&mut self) -> Stmt
    {
        let value = self.expression();
        self.consume(TokenType::SEMICOLON, r#"Expected ";" after value"#);
        Stmt::ExprStmt(value)
    }
    fn end(&self) -> bool
    {
        self.tokens[self.current].tokenType == TokenType::EOF
    }
    fn check(&mut self, matches: Vec<TokenType>) -> bool
    {
        for token in matches {
            if (self.tokens[self.current].tokenType == token) && (!self.end()) {
                self.current += 1;
                return true
            }
        }
        false
    }
    fn expression(&mut self) -> Expr
    {
        self.assignment()
    }
    fn assignment(&mut self) -> Expr
    {
        let expr = self.logic_or();
        while self.check(vec![TokenType::EQUAL]) {
            let line = self.tokens[self.current-1].line;
            let value = self.assignment();
            if let ExprType::Variable(n) = expr.exprType {
                return Expr::new(ExprType::Assignment(Assignment{id: n, expr: Box::new(value)}), line)
            }
            self.errors.push(LoxStaticError::new(line, "Invalid assignment target"));
        }
        expr
    }
    fn logic_or(&mut self) -> Expr
    {
        let mut expr = self.logic_and();
        while self.check(vec![TokenType::OR]) {
            let line = self.tokens[self.current-1].line;
            let operator = Loperator::try_from(self.tokens[self.current-1].tokenType.clone()).unwrap();
            let right = self.comparison();
            expr = Expr::new(ExprType::Logical(Logical{operator, lexpr: Box::new(expr), rexpr: Box::new(right)}), line);
        }
        expr
    }
    fn logic_and(&mut self) -> Expr
    {
        let mut expr = self.equality();
        while self.check(vec![TokenType::AND]) {
            let line = self.tokens[self.current-1].line;
            let operator = Loperator::try_from(self.tokens[self.current-1].tokenType.clone()).unwrap();
            let right = self.comparison();
            expr = Expr::new(ExprType::Logical(Logical{operator, lexpr: Box::new(expr), rexpr: Box::new(right)}), line);
        }
        expr
    }
    fn equality(&mut self) -> Expr
    {
        let mut expr = self.comparison();
        while self.check(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let line = self.tokens[self.current-1].line;
            let operator = Boperator::try_from(self.tokens[self.current-1].tokenType.clone()).unwrap();
            let right = self.comparison();
            expr = Expr::new(ExprType::Binary(Binary{operator, lexpr: Box::new(expr), rexpr: Box::new(right)}), line);
        }
        expr
    }
    fn comparison(&mut self) -> Expr
    {
        let mut expr = self.term();
        while self.check(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let line = self.tokens[self.current-1].line;
            let operator = Boperator::try_from(self.tokens[self.current-1].tokenType.clone()).unwrap();
            let right = self.term();
            expr = Expr::new(ExprType::Binary(Binary{operator, lexpr: Box::new(expr), rexpr: Box::new(right)}), line);
        }
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.check(vec![TokenType::MINUS, TokenType::PLUS]) {
            let line = self.tokens[self.current-1].line;
            let operator = Boperator::try_from(self.tokens[self.current-1].tokenType.clone()).unwrap();
            let right = self.factor();
            expr = Expr::new(ExprType::Binary(Binary{operator, lexpr: Box::new(expr), rexpr: Box::new(right)}), line);
        }
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.check(vec![TokenType::SLASH, TokenType::STAR]) {
            let line = self.tokens[self.current-1].line;
            let operator = Boperator::try_from(self.tokens[self.current-1].tokenType.clone()).unwrap();
            let right = self.unary();
            expr = Expr::new(ExprType::Binary(Binary{operator, lexpr: Box::new(expr), rexpr: Box::new(right)}), line);
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        if self.check(vec![TokenType::BANG, TokenType::MINUS]) {
            let line = self.tokens[self.current-1].line;
            let operator = Uoperator::try_from(self.tokens[self.current-1].tokenType.clone()).unwrap();
            let right = self.unary();
            return Expr::new(ExprType::Unary(Unary{operator, expr: Box::new(right)}), line);
        }
        self.call()
    }
    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            if self.check(vec![TokenType::LEFT_PAREN]) {
                expr = self.finishCall(expr);
            }
            else { break }
        }
        expr
    }
    fn finishCall(&mut self, callee: Expr) -> Expr {
        let mut arguments = Vec::new();
        if self.tokens[self.current].tokenType != TokenType::RIGHT_PAREN {
            arguments.push(self.expression());
            while self.tokens[self.current].tokenType == TokenType::COMMA {
                if arguments.len() >= 255 {
                    self.error("Can't have more than 255 arguments");
                }
                arguments.push(self.expression());
            }
        }
        let line = self.tokens[self.current].line; 
        self.consume(TokenType::RIGHT_PAREN, r#"Expected ")" after argument"#);
        Expr::new(ExprType::Call(Call{ callee: Box::new(callee), arguments }), line)
    }
    fn primary(&mut self) -> Expr {
        let literal = Literal::try_from(self.tokens[self.current].tokenType.clone());
        let line = self.tokens[self.current].line;
        match literal {
            Ok(l) => {
                if !self.end(){
                    self.current += 1;
                }
                return Expr::new(ExprType::Literal(l), line);
            },
            Err(_) => {
                if self.check(vec![TokenType::LEFT_PAREN]) {
                    let expr = self.expression();
                    self.consume(TokenType::RIGHT_PAREN, r#"Expected ")""#);
                    return Expr::new(ExprType::Grouping(Grouping{expr: Box::new(expr)}), line);
                }
                if !self.end() {
                    if let TokenType::IDENTIFIER(n) = self.tokens[self.current].tokenType.clone() {
                        self.current += 1;
                        return Expr::new(ExprType::Variable(n), line);
                    }
                }
            },
        }
    
        self.error("Expected expression");
        Expr::new(ExprType::Literal(Literal::Nil), line)
    }
    fn error(&mut self, message: &str) {
        self.errors.push(LoxStaticError::new(self.tokens[self.current].line, message));
        self.synch();
    }
    fn consume(&mut self, token: TokenType, message: &str) {
        if self.tokens[self.current].tokenType != token {
            self.error(message);
        }
        if !self.end() {
            self.current += 1;
        }
    }
    fn synch(&mut self) {
        if !self.end() { self.current += 1; }
        while !self.end() {
            if self.tokens[self.current-1].tokenType == TokenType::SEMICOLON { return; }
    
            match self.tokens[self.current].tokenType {
                TokenType::CLASS => return,
                TokenType::FUN => return,
                TokenType::VAR => return,
                TokenType::FOR => return,
                TokenType::IF => return,
                TokenType::WHILE => return,
                TokenType::PRINT => return,
                TokenType::RETURN => return,
                _ => (),
            }
            self.current += 1
        }
    }
}
