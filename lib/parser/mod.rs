pub mod ast;
pub mod parser;


#[cfg(test)]
mod tests {
    use super::super::lexer::lexer::Lexer;
    use super::parser::{ Parser, ParseError };
    use super::ast::{ Statement, Expr, Literal, Ident, Infix };

    fn run(input: String, expect: Vec<Statement>) {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Ok(_) => assert!(true),
            _ => assert!(false),
        }
        assert_eq!(p.program, expect);
    }

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
        run(input, expect);
    }

    #[test]
    fn test_string_literal() {
        let input = r#"let bar = "Hello, World";"#.to_string();
        let expect = vec![
            Statement::LetStatement(
                Ident("bar".into()),
                Expr::LiteralExpr(Literal::StringLiteral("Hello, World".into())),
            ),
        ];
        run(input, expect);
    }

    #[test]
    fn test_let_statements_with_ident() {
        let input = r#"let foo = bar;"#.to_string();
        let expect = vec![
            Statement::LetStatement(
                Ident("foo".into()),
                Expr::IdentExpr(Ident("bar".into())),
            ),
        ];
        run(input, expect);
    }

    #[test]
    fn test_let_statements_with_missing_semicolon() {
        let input = "let foo = 5".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Err(ParseError::MissingSemicolon) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_let_statements_with_illegal_expr() {
        let input = "let foo = ;".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Err(ParseError::IllegalExpr) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_let_statements_with_illegal_ident() {
        let input = "let = 5;".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Err(ParseError::IllegalIdent) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_return_statement() {
        let input = "return 5;".to_string();
        let expected = vec![
            Statement::ReturnStatement(
                Expr::LiteralExpr(Literal::IntLiteral(5)),
            ),
        ];
        run(input, expected);
    }

    #[test]
    fn test_return_statement_with_missing_semicolon() {
        let input = "return x".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        match p.parse() {
            Err(ParseError::MissingSemicolon) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_infix_expression() {
        let input = "let x = a + b;".to_string();
        let expect = vec![
            Statement::LetStatement(
                Ident("x".into()),
                Expr::InfixExpr(
                    Infix::Plus,
                    Box::new(Expr::IdentExpr(Ident("a".into()))),
                    Box::new(Expr::IdentExpr(Ident("b".into())))
                ),
            )
        ];
        run(input, expect);
    }
}
