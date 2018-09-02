use lexer::token::Token;


#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement(Ident, Expr),
    ReturnStatement(Expr),
    ExprStatement(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    IdentExpr(Ident),
    LiteralExpr(Literal),
    InfixExpr(Infix, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub String);


#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(usize),
    BooleanLiteral(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Mul,
    Equal,
    NEq,
}

impl Infix {
    pub fn from_token(tok: Token) -> Infix {
        match tok {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Asterisk => Infix::Mul,
            Token::Slash => Infix::Divide,
            _ => unreachable!(),
        }
    }
}

impl Expr {
    pub fn from_token(tok: Token) -> Expr {
        match tok {
            Token::Ident(i) => Expr::IdentExpr(Ident(i.clone())),
            Token::IntLiteral(i) => Expr::LiteralExpr(Literal::IntLiteral(i)),
            Token::StringLiteral(ref s) => Expr::LiteralExpr(Literal::StringLiteral(s.clone())),
            _ => unreachable!(),
        }
    }
}
