use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::ast::{ Statement, Expr, Ident, Literal };


type Result<T> = ::std::result::Result<T, ParseError>;

pub struct Parser {
    lexer: Lexer,
    cur: Token,
    peek: Token,
    pub program: Vec<Statement>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut l = lexer;
        let cur = l.next_token();
        let peek = l.next_token();
        Parser {
            lexer: l,
            cur: cur,
            peek: peek,
            program: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.cur = self.peek.clone();
        self.peek = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<()> {
        while self.cur != Token::EOF {
            self.parse_stmt()?;
        }
        Ok(())
    }

    fn parse_stmt(&mut self) -> Result<()> {
        let stmt = match self.cur {
            Token::Let => self.parse_let_statement()?,
            Token::Return => self.parse_return_statement()?,
            _ => {
                println!("uncaught token: {:?}", self.cur);
                println!("current: {:?}", self.program);
                return Err(ParseError::LetStmt("cannot parse".into()));
            },
        };
        self.program.push(stmt);
        Ok(())
    }

    /// let <identifier> Token::Assign <expr> Token::Semicolon
    fn parse_let_statement(&mut self) -> Result<Statement> {
        self.next_token();
        let ident = self.parse_ident()?;

        self.next_token();
        if self.cur != Token::Assign {
            return Err(ParseError::LetStmt("".to_string()));
        }
        self.next_token();
        let expr = self.parse_expr()?;
        self.next_token();
        if self.cur != Token::Semicolon {
            return Err(ParseError::MissingSemicolon);
        }
        self.next_token();
        Ok(Statement::LetStatement(ident, expr))
    }

    /// return <expr> Token::Semicolon
    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.next_token();
        let expr = self.parse_expr()?;
        self.next_token();
        if self.cur != Token::Semicolon {
            return Err(ParseError::MissingSemicolon);
        }
        self.next_token();
        Ok(Statement::ReturnStatement(expr))
    }

    fn parse_ident(&mut self) -> Result<Ident> {
        match &self.cur {
            Token::Ident(s) => Ok(Ident(s.clone())),
            _ => Err(ParseError::IllegalIdent),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        match &self.cur {
            Token::Ident(s) => Ok(Expr::IdentExpr(Ident(s.clone()))),
            Token::IntLiteral(n) => Ok(Expr::LiteralExpr(Literal::IntLiteral(*n))),
            Token::StringLiteral(s) => Ok(Expr::LiteralExpr(Literal::StringLiteral(s.clone()))),
            _ => Err(ParseError::IllegalExpr),
        }
    }
}


#[derive(Debug)]
pub enum ParseError {
    LetStmt(String),
    MissingSemicolon,
    IllegalIdent,
    IllegalExpr,
}

impl ::std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ParseError::LetStmt(ref s) => write!(f, "cannot parse: {}", s),
            _ => write!(f, "uncaught error"),
        }
    }
}

impl ::std::error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::LetStmt(ref s) => s,
            _ => "uncaught error",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            ParseError::LetStmt(_) => None,
            _ => None,
        }
    }
}
