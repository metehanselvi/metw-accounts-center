use crate::{dto, entity, repo::AccountRepo};
use std::sync::Arc;

mod error;

#[cfg(test)]
mod tests;

pub use error::ServiceError;

/// Service result type.
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Account state.
pub struct AccountService {
    repo: Arc<dyn AccountRepo>,
}

impl AccountService {
    /// Creates a new account service.
    pub fn new(repo: Box<dyn AccountRepo>) -> Self {
        Self { repo: repo.into() }
    }

    /// Signup a new account
    pub async fn signup(&self, signup_dto: dto::request::Signup) -> ServiceResult<()> {
        let mut transaction = self.repo.begin().await;

        let keys = dto::repo::Keys {
            identity_key: signup_dto.keys.identity_key,
            encrypted_private_key: signup_dto.keys.encrypted_private_key,
            encrypted_master_key: signup_dto.keys.encrypted_master_key,
        };

        /// TODO: snowflake ID generator
        let account_id = entity::AccountId(0);

        transaction
            .upsert_account(account_id, &signup_dto.password_hash, &keys)
            .await?;

        transaction
            .add_email(account_id, &signup_dto.email)
            .await?;

        transaction
            .add_username(account_id, &signup_dto.username)
            .await?;

        transaction
            .set_primary_email(&signup_dto.email, true)
            .await?;

        transaction
            .set_primary_username(&signup_dto.username, true)
            .await?;

        transaction.commit().await?;

        Ok(())
    }
}
