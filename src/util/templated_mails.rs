/// Mail templates.
#[allow(missing_docs)]
pub enum Template {
    /// Signup mail format.
    Signup {
        username: String,
        signup_jwt: String,
        callback_url: &'static str,
    },
}

impl Template {
    /// Get subject of the template.
    pub fn subject(&self) -> String {
        match self {
            Self::Signup { .. } => "Verify your metw.cc account".to_string(),
        }
    }

    /// Get email body of the template.
    pub fn body(&self) -> String {
        match self {
            Self::Signup {
                username,
                signup_jwt,
                callback_url,
            } => format!(
                "Hello {username}! Please verify your account by clicking: {callback_url}?token={signup_jwt}"
            ),
        }
    }
}
