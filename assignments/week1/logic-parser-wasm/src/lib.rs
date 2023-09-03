use std::{panic, vec};
use image::ImageEncoder;
use logic_parser::image_generation::render::render_to_image;
use serde_json::json;
use wasm_bindgen::prelude::*;

use logic_parser::lexing::Lexer;
use logic_parser::parsing::{Parser, ASTNode};
use logic_parser::errors::{LexerError, ParserError};

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

#[wasm_bindgen]
pub fn generate_image(ast: JsValue) -> Vec<u8> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let ast: ASTNode = serde_wasm_bindgen::from_value(ast).unwrap();
    let img = render_to_image(ast).unwrap();
    let mut vec = Vec::new();

    let encode = image::codecs::png::PngEncoder::new(&mut vec);
    encode.write_image(&img, img.width(), img.height(), image::ColorType::Rgb8).unwrap();
    vec
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
