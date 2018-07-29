pub mod token;
pub mod lexer;


#[cfg(test)]
mod tests {
    use super::token::Token;
    use super::token::Token::*;
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
            Assign,
            EOF,
        ];
        run(input, &expected);
    }

    #[test]
    fn test_next_token() {
        let input = r#"
        let x = 5;
        let y = 10;
        let add = fn(x, y) {
            x + y
        };
        let result = add(x, y);

        if (x < y) {
            return true;
        } else if (x == y){
            return false;
        } else if (x > y) {
            return x;
        }
        "#.to_string();
        let expected = vec![
            Let, Ident("x".into()), Assign, IntLiteral(5), Semicolon,
            Let, Ident("y".into()), Assign, IntLiteral(10), Semicolon,
            Let, Ident("add".into()), Assign, Function,
                LParen, Ident("x".into()), Comma, Ident("y".into()), RParen,
            LBrace,
                Ident("x".into()), Plus, Ident("y".into()),
            RBrace,
            Semicolon,
            Let, Ident("result".into()), Assign, Ident("add".into()),
                LParen, Ident("x".into()), Comma, Ident("y".into()), RParen,
                Semicolon,
            If, LParen, Ident("x".into()), LT, Ident("y".into()), RParen,
            LBrace,
                Return, True, Semicolon,
            RBrace, Else, If, LParen, Ident("x".into()), Equal, Ident("y".into()), RParen, LBrace,
                Return, False, Semicolon,
            RBrace, Else, If, LParen, Ident("x".into()), GT, Ident("y".into()), RParen, LBrace,
                Return, Ident("x".into()), Semicolon,
            RBrace,
            EOF,
        ];
        run(input, &expected);
    }

    #[test]
    fn test_equal() {
        let input = r#"
        x == y;
        x != y;
        x = y;
        "#.to_string();
        let expected = vec![
            Ident("x".into()), Equal, Ident("y".into()), Semicolon,
            Ident("x".into()), NEq, Ident("y".into()), Semicolon,
            Ident("x".into()), Assign, Ident("y".into()), Semicolon,
        ];
        run(input, &expected);
    }

    #[test]
    fn test_illegal_token() {
        let input = "@".to_string();
        let expected = vec![Illegal('@')];
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
