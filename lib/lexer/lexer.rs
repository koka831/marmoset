use lexer::token::Token;



#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize, // current position in input (points to current char)
    read_pos: usize, // current reading position (after current char)
    chr: char,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Lexer {
            input: s.chars().collect::<Vec<char>>(),
            pos: 0,
            read_pos: 0,
            chr: '0'
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok = Token::new(&self.input[self.pos]);
        self.pos += 1;
        tok
    }

    fn read_char(&mut self) {
        // check if it reached the end of input.
        if self.read_pos >= self.input.len() {
            self.chr = '0';
        } else {
            self.chr = self.input[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }
}
