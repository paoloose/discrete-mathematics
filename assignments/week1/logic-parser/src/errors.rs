use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    SyntaxError(String),
    #[error("Unkown Token: {0}")]
    UnkownToken(String)
}
