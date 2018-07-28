pub mod token;
pub mod lexer;


#[cfg(test)]
mod tests {
    use super::token::*;
    use super::lexer::*;

    fn run(input: String, expected: &Vec<Token>) {
        let mut l = Lexer::new(input);
        for j in 0..expected.len() {
            let tok = l.next_token();
            assert_eq!(expected[j], tok);
        }
    }

    #[test]
    fn test_eof() {
        let input = String::from("=");
        let expected = vec![
            Token::Assign,
            Token::EOF,
        ];
        run(input, &expected);
    }

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
            Token::EOF,
        ];
        run(input, &expected);
    }

    #[test]
    fn test_integer_literal() {
        let input = String::from("let five=5;");
        let expected = vec![
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::IntLiteral(5),
            Token::Semicolon,
            Token::EOF,
        ];
        run(input, &expected);
    }

    #[test]
    fn test_missing_semicolon() {
        let input = String::from("let five=5");
        let expected = vec![
            Token::Let,
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::IntLiteral(5),
            Token::EOF,
        ];
        run(input, &expected);
    }

    #[test]
    fn test_is_letter() {
        assert!(is_letter('b'));
        assert!(is_letter('B'));
        assert!(!is_letter('?'));
        assert!(is_letter('_'));
    }

    #[test]
    fn test_is_digit() {
        assert!(is_digit('0'));
        assert!(is_digit('9'));
        assert!(is_digit('2'));
        assert!(is_digit('5'));
        assert!(is_digit('8'));
        assert!(!is_digit('a'));
    }
}
