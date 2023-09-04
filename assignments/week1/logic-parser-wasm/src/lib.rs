use wasm_bindgen::prelude::*;
use serde_json::json;

use logic_parser::lexing::Lexer;
use logic_parser::parsing::{Parser, ASTNode};
use logic_parser::errors::{LexerError, ParserError};
use logic_parser::svg_generation::render::render_to_svg;

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
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // "p & q" -> [ & [p] [q]]
    // "p & (q => s)" -> [ & [p] [ => [q] [s]]]
    // "(p | q) & (q => s)" -> [ | [p] [q] ]
    let tokens = match Lexer::new(expr).tokenize() {
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

#[wasm_bindgen]
pub fn generate_svg(ast: JsValue) -> String {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let ast: ASTNode = serde_wasm_bindgen::from_value(ast).unwrap();
    let horizontal_separation: f32 = 20_f32;
    let vertical_separation: f32 = 60_f32;
    let radius: f32 = 15_f32;
    let svg = render_to_svg(ast, horizontal_separation, vertical_separation, radius);

    svg.as_xml()
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_json::assert_json;

    #[test]
    fn detects_unclosed_paren() {
        let result = parse_expression(")p => q");
        assert_json!(result.as_str(), {
            "error": "Unexpected token: R_PAREN",
            "span": [0, 1],
            "status": "error"
        });
    }
}
