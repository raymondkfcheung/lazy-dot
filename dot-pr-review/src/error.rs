use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Missing token")]
    MissingToken,
    #[error("Not found: {0}")]
    NotFound(String),
}
