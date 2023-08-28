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
    let tokens = match Lexer::new(expr).parse() {
        Ok(t) => t,
        Err(_) => return r###"{
            "status": "error"
        }"###.into(),
    };

    let ast = match Parser::new(&tokens).parse() {
        Ok(ast) => ast,
        Err(_) => return r###"{
            "status": "error"
        }"###.into(),
    };

    format!(
        r###"{{
            "status": "success", "ast": {ast}
        }}"###,
        ast=ast.as_json()
    )
}
