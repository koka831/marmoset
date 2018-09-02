extern crate marmoset;

use marmoset::lexer::lexer::Lexer;
use marmoset::parser::parser::Parser;

fn main() {
    let input = r#"
    let foo = 5;
    let bar = "Hello World";
    let x = a + b;
    "#.to_string();

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    p.parse().unwrap();
    println!("{:#?}", p.program);
}
