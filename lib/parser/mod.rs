pub mod ast;
pub mod parser;


#[cfg(test)]
mod tests {
    use super::super::lexer::lexer::Lexer;
    use super::parser::{ Parser, ParseError };
    use super::ast::{ Statement, Expr, Literal, Ident };

    #[test]
    fn test_let_statements() {
        let input = r#"
        let foo = 5;
        "#.to_string();
        let expect = vec![
            Statement::LetStatement(
                Ident("foo".into()),
                Expr::LiteralExpr(Literal::IntLiteral(5)),
            ),
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, ""),
        }
        assert_eq!(p.program, expect);
    }

    #[test]
    fn test_let_statements_with_missing_semicolon() {
        let input = "let foo = 5".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Ok(_) => assert!(false),
            Err(ParseError::MissingSemicolon) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_let_statements_with_illegal_expr() {
        let input = "let foo = ;".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Ok(_) => assert!(false),
            Err(ParseError::IllegalExpr) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_let_statements_with_illegal_ident() {
        let input = "let = 5;".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Ok(_) => assert!(false),
            Err(ParseError::IllegalIdent) => assert!(true),
            Err(_) => assert!(false),
        }
    }

}
