mod lexer;
mod errors;
mod parser;

fn main() {
    let expr = "(p & q) => r";
    // "p & q" -> [ & [p] [q]]
    // "p & (q => s)" -> [ & [p] [ => [q] [s]]]
    // "(p | q) & (q => s)" -> [ | [p] [q] ]
    let tokens = lexer::Lexer::new(expr).parse().unwrap();
    let ast = parser::Parser::new(&tokens).parse().unwrap();

    println!("{:#?}", tokens);
    println!("{:#?}", ast);
}
