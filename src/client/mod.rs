use crate::{entity, util::templated_mails};
use async_trait::async_trait;

/// Client implementations.
pub mod impls;

/// Send emails.
#[async_trait]
pub trait MailClient {
    /// Send mail.
    async fn send(&self, id: entity::AccountId, template: templated_mails::Template);
}
