use crate::repo::RepoError;
use thiserror::Error;

/// Service error reporting
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Repository error.
    #[error("repo: {0}")]
    Repo(#[from] RepoError),
}
