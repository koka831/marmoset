

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement(Ident, Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    IdentExpr(Ident),
    LiteralExpr(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub String);


#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(usize),
}
