use lexer::token::Token;



#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize, // current position in input (points to current char)
    read_pos: usize, // current reading position (after current char)
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
        Lexer {
            input: s.chars().collect::<Vec<char>>(),
            pos: 0,
            read_pos: 0,
            chr: None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let chr = self.read_char();
        if let Some(c) = chr {
            if is_letter(c) { // ident or literal
                return Token::from_str(self.read_ident());
            } else if is_digit(c) {
                return Token::IntLiteral(self.read_number());
            } else { // reserved word
                Token::new(&c)
            }
        } else { // None
            Token::EOF
        }
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

    fn read_ident(&mut self) -> String {
        let mut buf = String::new();
        while let Some(c) = self.chr {
            if !is_letter(c) { break; }
            buf += c.to_string().as_str();
            let _ = self.read_char();
        }
        buf
    }

    fn read_number(&mut self) -> usize {
        let mut buf = String::new();
        while let Some(c) = self.chr {
            if !is_digit(c) { break; }
            buf += c.to_string().as_str();
            let _ = self.read_char();
        }
        buf.parse().unwrap()
    }
}
