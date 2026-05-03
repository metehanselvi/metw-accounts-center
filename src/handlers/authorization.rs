use crate::{
    client::MailClient,
    service::{AccountService, TokenService},
};
use std::sync::Arc;

/// Account handlers that **does require** escalated privileges.
pub struct AuthorizationHandler {
    _account_service: Arc<AccountService>,
    _token_service: Arc<TokenService>,
    _email_client: Arc<dyn MailClient>,
}
