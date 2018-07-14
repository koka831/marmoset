

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    IntLiteral(isize),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // Keywords
    Function,
    Let,
}


impl Token {
    pub fn new(c: &char) -> Self {
        match &c {
            '=' => Token::Assign,
            '+' => Token::Plus,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            _   => Token::Illegal,
        }
    }
}
