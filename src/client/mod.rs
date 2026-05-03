use async_trait::async_trait;
use crate::entity;

/// Client implementations.
pub mod impls;

/// Send emails.
#[async_trait]
pub trait MailClient {
    /// Send mail.
    async fn send(&self, id: entity::AccountId, subject: String, body: String);
}
