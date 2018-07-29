use lexer::token::Token;


#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize, // current position in input (points to current char)
    chr: Option<char>,
}

pub fn is_letter(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}

pub fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

impl Lexer {
    pub fn new(s: String) -> Self {
        let input = s.chars().collect::<Vec<char>>();
        Lexer {
            input: input,
            pos: 0,
            chr: None,
        }
    }

    fn skip_whitespace(&mut self) {
        if let Some(t) = self.peek_next() {
            if t.is_whitespace() {
                self.read_char();
                self.skip_whitespace();
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if let Some(_) = self.peek_next() {
            match self.read_char() {
                c if is_letter(c) => Token::from_str(self.read_ident()),
                c if is_digit(c)  => Token::IntLiteral(self.read_number()),
                c @ '='| c @ '!' => {
                    if self.peek_next_is('=') {
                        self.read_char();
                        Token::from_str(c.to_string() + "=")
                    } else { Token::Assign }
                },
                c @ _ => Token::new(&c),
            }
        } else { // None
            Token::EOF
        }
    }

    /// check if next char is valid
    fn peek_next(&self) -> Option<char> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }

    fn peek_next_is(&self, c: char) -> bool {
        if let Some(a) = self.peek_next() {
            if a == c { true } else { false }
        } else { false }
    }

    fn read_char(&mut self) -> char {
        self.chr = self.peek_next();
        self.pos += 1;
        self.input[self.pos - 1]
    }

    fn read_ident(&mut self) -> String {
        let mut buf = self.chr.unwrap().to_string();
        while let Some(c) = self.peek_next() {
            if !is_letter(c) { break; }
            buf += self.read_char().to_string().as_str();
        }
        buf
    }

    fn read_number(&mut self) -> usize {
        let mut buf = self.chr.unwrap().to_string();
        while let Some(c) = self.peek_next() {
            if !is_digit(c) { break; }
            buf += self.read_char().to_string().as_str();
        }
        buf.parse().unwrap()
    }
}
