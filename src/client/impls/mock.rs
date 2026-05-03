use std::collections::HashMap;
use crate::entity;
use super::super::MailClient;
use async_trait::async_trait;
use tokio::sync::Mutex;

/// Mock mail client implementation.
pub struct MockMailClientImpl {
    /// List of sent mails
    pub debug: Mutex<HashMap<entity::AccountId, Vec<(String, String)>>>
}

#[async_trait]
impl MailClient for MockMailClientImpl {
    async fn send(&self, id: entity::AccountId, subject: String, body: String) {
        let mut debug = self.debug.lock().await;

        // TODO: use trancing instead
        println!("--- EMAIL TO: {id} ---\nSub:{subject}\n\n{body}\n----------------------");

        debug.entry(id).or_default().push((subject, body));

    }
}
