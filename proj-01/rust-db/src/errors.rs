use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing {0}")]
    ParseError(String),
}