use lexer::token::Token;



#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize, // current position in input (points to current char)
    read_pos: usize, // current reading position (after current char)
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Lexer {
            input: s.chars().collect::<Vec<char>>(),
            pos: 0,
            read_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok = Token::new(&self.input[self.pos]);
        self.pos += 1;
        tok
    }
}
