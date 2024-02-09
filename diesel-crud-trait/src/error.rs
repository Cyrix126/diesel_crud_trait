use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorCrud {
    #[error("error from diesel")]
    Diesel(#[from] diesel::result::Error),
    #[error("the value is invalid:\n{0}")]
    InvalidValue(String),
}
