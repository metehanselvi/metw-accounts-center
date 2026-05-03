mod account;
mod authentication;
mod authorization;

mod error;

#[cfg(test)]
mod tests;

pub use account::AccountHandler;
pub use authentication::AuthenticationHandler;
pub use authorization::AuthorizationHandler;

pub use error::HandlerError;

/// Handler result type.
pub type HandlerResult<T> = Result<T, HandlerError>;
