use thiserror::Error;
use crate::lexing::token::Span;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String, Span),
    #[error("Unexpected EOF: {0}")]
    UnexpectedEOF(String, Span),
}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Syntax error: {0}")]
    SyntaxError(String, Span),
    #[error("Unknown Token: {0}")]
    UnknownToken(String, Span)
}

