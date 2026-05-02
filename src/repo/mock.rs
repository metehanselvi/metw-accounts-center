use super::{AccountRepo, AccountRepoTransaction, RepoResult};
use crate::{dto, entity};
use async_trait::async_trait;
use sqlx::types::chrono;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, OwnedMutexGuard};

/// Mock account repository implementatoin
pub struct AccountRepoImpl {
    state: Arc<Mutex<State>>,
}

/// Create a new mock repository.
pub fn new_mock() -> Box<dyn AccountRepo> {
    Box::new(AccountRepoImpl {
        state: Arc::new(Mutex::new(State::default())),
    })
}

#[async_trait]
impl AccountRepo for AccountRepoImpl {
    async fn begin(&self) -> Box<dyn AccountRepoTransaction> {
        Box::new(AccountRepoTransactionImpl {
            state: Arc::clone(&self.state).lock_owned().await,
        })
    }
}

#[derive(Default)]
struct State {
    accounts: HashMap<entity::AccountId, entity::Account>,
    emails: HashMap<String, entity::Email>,
    usernames: HashMap<String, entity::Username>,
}

struct AccountRepoTransactionImpl {
    state: OwnedMutexGuard<State>,
}

#[async_trait]
impl AccountRepoTransaction for AccountRepoTransactionImpl {
    async fn commit(self: Box<Self>) -> RepoResult<()> {
        Ok(())
    }

    async fn upsert_account(
        &mut self,
        id: entity::AccountId,
        password_hash: &str,
        keys: &dto::repo::Keys,
    ) -> RepoResult<()> {
        let account_entity = self.state.accounts.entry(id).or_default();

        account_entity.id = id;

        account_entity.password_hash = password_hash.to_string();
        account_entity.identity_key = keys.identity_key.clone();
        account_entity.encrpyted_private_key = keys.encrypted_private_key.clone();
        account_entity.encrpyted_master_key = keys.encrypted_master_key.clone();

        Ok(())
    }

    async fn get_login_by_email(&mut self, email: &str) -> RepoResult<Option<dto::repo::Login>> {
        if let Some(email_entity) = self.state.emails.get(email) {
            Ok(Some(dto::repo::Login {
                id: email_entity.account_id,
                password_hash: self.state.accounts[&email_entity.account_id]
                    .password_hash
                    .clone(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_login_by_username(
        &mut self,
        username: &str,
    ) -> RepoResult<Option<dto::repo::Login>> {
        if let Some(username_entity) = self.state.usernames.get(username) {
            Ok(Some(dto::repo::Login {
                id: username_entity.account_id,
                password_hash: self.state.accounts[&username_entity.account_id]
                    .password_hash
                    .clone(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_primary_username(&mut self, _id: entity::AccountId) -> RepoResult<Option<String>> {
        todo!()
    }

    async fn get_nonexpiring_username_aliases(
        &mut self,
        _id: entity::AccountId,
    ) -> RepoResult<Vec<String>> {
        todo!()
    }

    async fn get_primary_email(&mut self, _id: entity::AccountId) -> RepoResult<Option<String>> {
        todo!()
    }

    async fn get_secondary_emails(&mut self, _id: entity::AccountId) -> RepoResult<Vec<String>> {
        todo!()
    }

    async fn get_keys(&mut self, _id: entity::AccountId) -> RepoResult<Option<dto::repo::Keys>> {
        todo!()
    }

    async fn add_email(&mut self, id: entity::AccountId, email: &str) -> RepoResult<bool> {
        if self.state.emails.contains_key(email) {
            Ok(false)
        } else {
            self.state.emails.insert(
                email.to_string(),
                entity::Email {
                    email: email.to_string(),
                    account_id: id,
                    is_primary: false,
                    created_at: chrono::Utc::now(),
                },
            );

            Ok(true)
        }
    }

    async fn add_username(&mut self, id: entity::AccountId, username: &str) -> RepoResult<bool> {
        if self.state.usernames.contains_key(username) {
            Ok(false)
        } else {
            self.state.usernames.insert(
                username.to_string(),
                entity::Username {
                    username: username.to_string(),
                    account_id: id,
                    is_primary: false,
                    created_at: chrono::Utc::now(),
                    expires_at: None,
                },
            );

            Ok(true)
        }
    }

    async fn set_primary_email(&mut self, email: &str, is_primary: bool) -> RepoResult<bool> {
        if let Some(email) = self.state.emails.get_mut(email) {
            email.is_primary = is_primary;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn set_primary_username(&mut self, username: &str, is_primary: bool) -> RepoResult<bool> {
        if let Some(username) = self.state.usernames.get_mut(username) {
            username.is_primary = is_primary;

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
