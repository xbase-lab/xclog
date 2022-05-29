use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("`{0}` is unprocessable")]
    Unprocessable(String),
    #[error("Fail to parse compilation step `{0}`")]
    Failure(String),
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("Unexpected EOF while trying to parse `{0}`, key: `{0}`")]
    EOF(String, String),
}
