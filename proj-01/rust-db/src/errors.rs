use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ParseError: {0}")]
    ParseError(String),
    #[error("NoMatchError: {0}")]
    NoTokenError(String),
    #[error("AlreadyExistsError: {0}")]
    AlreadyExistsError(String),
    #[error("UnknownTokenError: {0}")]
    UnknownTokenError(String),
    #[error("MissingTokenError: {0}")]
    MissingTokenError(String),
}