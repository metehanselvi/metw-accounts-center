use thiserror::Error;

/// Repository error reporting
#[derive(Error, Debug)]
pub enum RepoError {
    /// Internal error
    #[error("internal error: {0}")]
    Internal(String),
    /// Error details are readacted
    #[error("error details are redacted")]
    Redacted,
}
