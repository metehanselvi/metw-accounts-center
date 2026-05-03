use super::HandlerResult;
use crate::{
    entity,
    service::{AccountService, TokenService},
    token::TokenScope,
};
use std::sync::Arc;

/// Account handlers that **does require** escalated privileges.
pub struct AuthorizationHandler {
    account_service: Arc<AccountService>,
    token_service: Arc<TokenService>,
}

impl AuthorizationHandler {
    /// Creates a new authentication hander.
    pub fn new(account_service: Arc<AccountService>, token_service: Arc<TokenService>) -> Self {
        Self {
            account_service,
            token_service,
        }
    }

    async fn handle_token(&self, id: entity::AccountId, scope: TokenScope) -> HandlerResult<()> {
        match scope {
            TokenScope::Authenticate => Ok(()),

            TokenScope::AddEmail(email) => {
                Ok(self.account_service.auth_add_email(id, email).await?)
            }

            TokenScope::SetPrimaryEmail {
                current_primary_email,
                new_primary_email,
            } => Ok(self
                .account_service
                .auth_change_primary_email(id, current_primary_email, new_primary_email)
                .await?),

            TokenScope::Signup { email } => {
                Ok(self.account_service.auth_complete_signup(id, email).await?)
            }
        }
    }

    /// Handle privileged tokens.
    pub async fn auth(&self, base64_encoded_token: String) -> HandlerResult<()> {
        let token = self.token_service.verify(&base64_encoded_token).await?;

        self.token_service.revoke(&base64_encoded_token).await?;

        for scope in token.scopes {
            self.handle_token(token.id, scope).await?;
        }

        Ok(())
    }
}
