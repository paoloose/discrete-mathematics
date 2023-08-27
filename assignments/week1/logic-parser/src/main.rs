mod lexer;
mod errors;

fn main() {
    let expr = "(p & q) => r";
    let tokens = lexer::LexerBuilder::new(expr).parse();

    println!("{:#?}", tokens);
}
