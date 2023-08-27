mod lexer;
mod errors;
mod parser;

fn main() {
    let expr = "p & q";
    // "p & q" -> [ & [p] [q]]
    // "p & (q => s)" -> [ & [p] [ => [q] [s]]]
    // "(p | q) & (q => s)" -> [ | [p] [q] ]
    let tokens = lexer::Lexer::new(expr).parse().unwrap();
    let ast = parser::Parser::new(&tokens).parse();

    println!("{:#?}", tokens);
    println!("{:#?}", ast);
}
