use lexer::token::Token;



#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize, // current position in input (points to current char)
    read_pos: usize, // current reading position (after current char)
    chr: Option<char>,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Lexer {
            input: s.chars().collect::<Vec<char>>(),
            pos: 0,
            read_pos: 0,
            chr: None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let chr = self.read_char();
        let tok = Token::new(&chr);
        tok
    }

    fn read_char(&mut self) -> Option<char> {
        // check if it reached the end of input.
        if self.read_pos >= self.input.len() {
            self.chr = None;
        } else {
            self.chr = Some(self.input[self.read_pos]);
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
        self.chr
    }
}
