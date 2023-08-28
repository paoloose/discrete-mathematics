use std::panic;
use wasm_bindgen::prelude::*;

use logic_parser::lexer::Lexer;
use logic_parser::parser::Parser;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn parse_expression(expr: &str) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // "p & q" -> [ & [p] [q]]
    // "p & (q => s)" -> [ & [p] [ => [q] [s]]]
    // "(p | q) & (q => s)" -> [ | [p] [q] ]
    let tokens = Lexer::new(expr).parse().unwrap();
    let ast = Parser::new(&tokens).parse().unwrap();

    format!("{ast}", ast=ast.as_json())
}
