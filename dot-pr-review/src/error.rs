pub enum FetchError {
    IoError(String),
    NotFound(String),
}
