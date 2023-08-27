use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    SyntaxError(String),
    #[error("Unkown Token: {0}")]
    UnkownToken(String)
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected character: {0}")]
    UnexpectedToken(String),
    #[error("Unkown Token: {0}")]
    UnkownToken(String)
}
