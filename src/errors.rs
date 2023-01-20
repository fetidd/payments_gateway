use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("ValidationError: {0}")]
    ValidationError(String),

    #[error("FieldError: {0}")]
    FieldError(String),
}

pub type GwResult<T> = Result<T, Error>;
