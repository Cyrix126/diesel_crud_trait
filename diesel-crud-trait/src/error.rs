use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors variant from the CRUD methods
pub enum ErrorCrud {
    /// Error from Diesel
    #[error(transparent)]
    Diesel(#[from] DieselError),
    #[error("the value is invalid:\n{0}")]
    /// Error from check
    InvalidValue(String),
}
