use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::ast::{ Statement, Expr, Ident, Literal, Infix };


type ParseResult<T> = ::std::result::Result<T, ParseError>;

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

    fn cur_token_is(&self, tok: Token) -> bool {
        self.cur == tok
    }

    pub fn parse(&mut self) -> ParseResult<()> {
        while !self.cur_token_is(Token::EOF) {
            self.parse_stmt()?;
        }
        Ok(())
    }

    fn parse_stmt(&mut self) -> ParseResult<()> {
        let stmt = match self.cur {
            Token::Let => self.parse_let_statement()?,
            Token::Return => self.parse_return_statement()?,
            _ => unreachable!(),
        };
        self.program.push(stmt);
        Ok(())
    }

    /// let <identifier> Token::Assign *<expr> Token::Semicolon
    fn parse_let_statement(&mut self) -> ParseResult<Statement> {
        self.next_token();
        let ident = self.parse_ident()?;
        self.next_token();
        self.assert_token(&self.cur, &Token::Assign, ParseError::LetStmt("".into()))?;
        self.next_token();
        let expr = self.parse_expression()?;
        self.next_token();
        self.assert_token(&self.cur, &Token::Semicolon, ParseError::MissingSemicolon)?;
        self.next_token();
        Ok(Statement::LetStatement(ident, expr))
    }

    /// return <expr> Token::Semicolon
    fn parse_return_statement(&mut self) -> ParseResult<Statement> {
        self.next_token();
        let expr = self.parse_expression()?;
        self.next_token();
        self.assert_token(&self.cur, &Token::Semicolon, ParseError::MissingSemicolon)?;
        self.next_token();
        Ok(Statement::ReturnStatement(expr))
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        let cur = self.cur.clone();
        match cur {
            Token::Ident(ref s) => {
                match self.peek {
                    Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => self.parse_operator_expression(),
                    _ => Ok(Expr::IdentExpr(Ident(s.clone()))),
                }
            },
            Token::IntLiteral(n) => {
                match self.peek {
                    Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => self.parse_operator_expression(),
                    _ => Ok(Expr::LiteralExpr(Literal::IntLiteral(n))),
                }
            },
            Token::StringLiteral(ref s) => Ok(Expr::LiteralExpr(Literal::StringLiteral(s.clone()))),
            Token::True => Ok(Expr::LiteralExpr(Literal::BooleanLiteral(true))),
            Token::False => Ok(Expr::LiteralExpr(Literal::BooleanLiteral(false))),
            _ => Err(ParseError::IllegalExpr),
        }
    }

    fn parse_operator_expression(&mut self) -> ParseResult<Expr> {
        let lhv = self.cur.clone();
        self.next_token();
        let op = self.cur.clone();
        self.next_token();
        let rhv = self.parse_expression()?;
        Ok(Expr::InfixExpr(
            Infix::from_token(op),
            Box::new(Expr::from_token(lhv)),
            Box::new(rhv),
        ))
    }

    fn parse_ident(&mut self) -> ParseResult<Ident> {
        match &self.cur {
            Token::Ident(s) => Ok(Ident(s.clone())),
            _ => Err(ParseError::IllegalIdent),
        }
    }

    fn assert_token(&self, a: &Token, b: &Token, e: ParseError) -> ParseResult<()> {
        if a != b { return Err(e); }
        Ok(())
    }
}


#[derive(Debug)]
pub enum ParseError {
    LetStmt(String),
    MissingSemicolon,
    IllegalIdent,
    IllegalExpr,
    IllegalFn,
}

impl ::std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ParseError::LetStmt(ref s) => write!(f, "cannot parse: {}", s),
            ref e => write!(f, "cannot parse: {}", e),
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
