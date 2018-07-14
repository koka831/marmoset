pub mod token;
pub mod lexer;


#[cfg(test)]
mod tests {
    use super::token::*;
    use super::lexer::*;

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut l = Lexer::new(input);
        for j in 0..expected.len() {
            let tok = l.next_token();
            assert_eq!(expected[j], tok);
        }
    }
}
