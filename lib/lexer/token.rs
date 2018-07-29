

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Illegal(char),
    EOF,

    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    IntLiteral(usize),

    // Operators
    Assign,     // =
    Plus,       // +
    Minus,      // -
    Bang,       // !
    Asterisk,   // *
    Slash,      // /
    LT,         // <
    GT,         // >

    // Delimiters
    Comma,      // ,
    Semicolon,  // ;

    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }

    // Keywords
    Function,   // fn
    Let,        // let
    True,       // true
    False,      // false
    If,         // if
    Else,       // else
    Return,     // return
    Equal,      // ==
    NEq,        // !=
}


impl Token {
    pub fn new(c: &char) -> Self {
        match &c {
            '=' => Token::Assign,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => Token::Bang,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            _   => Token::Illegal(*c),
        }
    }

    pub fn from_str(s: String) -> Self {
        match &*s {
            "let" => Token::Let,
            "fn"  => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "=" => Token::Assign,
            "==" => Token::Equal,
            "=!" => Token::NEq,
            _ => Token::Ident(s),
        }
    }
}
