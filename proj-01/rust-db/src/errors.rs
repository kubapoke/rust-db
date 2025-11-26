use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ParseError: {0}")]
    ParseError(String),
    #[error("NoMatchError: {0}")]
    NoTokenError(String),
    #[error("AlreadyExistsError: {0}")]
    AlreadyExistsError(String),
    #[error("NotExistError: {0}")]
    NotExistError(String),
    #[error("UnknownTokenError: {0}")]
    UnknownTokenError(String),
    #[error("MissingTokenError: {0}")]
    MissingTokenError(String),
    #[error("NotSpecifiedError: {0}")]
    NotSpecifiedError(String),
    #[error("TypeError: {0}")]
    TypeError(String),
}