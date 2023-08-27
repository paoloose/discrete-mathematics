mod lexer;
mod errors;

fn main() {
    let expr = "(69p & q) => r";
    let tokens = lexer::Lexer::new(expr).parse();

    println!("{:#?}", tokens);
}
