use std::panic;
use serde_json::json;
use wasm_bindgen::prelude::*;

use logic_parser::lexing::Lexer;
use logic_parser::parsing::Parser;
use logic_parser::errors::{LexerError, ParserError};

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

macro_rules! generate_json_error {
    ($span: expr, $error: expr) => {
        json!({
            "status": "error",
            "span": [$span.start, $span.end],
            "error": $error.to_string()
        }).to_string()
    }
}

#[wasm_bindgen]
pub fn parse_expression(expr: &str) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // "p & q" -> [ & [p] [q]]
    // "p & (q => s)" -> [ & [p] [ => [q] [s]]]
    // "(p | q) & (q => s)" -> [ | [p] [q] ]
    let tokens = match Lexer::new(expr).parse() {
        Ok(t) => t,
        Err(ref e) => {
            match e {
                LexerError::SyntaxError(_, span) => {
                    return generate_json_error!(span, e);
                },
                LexerError::UnknownToken(_, span) => {
                    return generate_json_error!(span, e);
                }
            }
        }
    };

    let ast = match Parser::new(&tokens).parse() {
        Ok(ast) => ast,
        Err(ref e) => {
            match e {
                ParserError::UnexpectedEOF(_, span) => {
                    return generate_json_error!(span, e);
                },
                ParserError::UnexpectedToken(_, span) => {
                    return generate_json_error!(span, e);
                },
            }
        }
    };

    format!(
        r###"{{
            "status": "success", "ast": {ast}
        }}"###,
        ast=ast.as_json()
    )
}
