

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    IntLiteral(usize),

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
    Space,

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
            ' ' => Token::Space,
            _   => Token::Illegal,
        }
    }

    pub fn from_str(s: String) -> Self {
        match &*s {
            "let" => Token::Let,
            "fn"  => Token::Function,
            _ => Token::Ident(s),
        }
    }
}
