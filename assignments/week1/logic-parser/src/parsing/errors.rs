use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Unknown Token: {0}")]
    UnknownToken(String)
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("Unexpected EOF")]
    UnexpectedEOF(String),
}
