use chrono::{DateTime, Utc};

use crate::auth::error::AuthError;

// Trail clone
pub trait TokenProviderClone {
    fn clone_box(&self) -> Box<dyn TokenProvider>;
}

// Token provider trait
pub trait TokenProvider: Send + Sync + TokenProviderClone {
    fn get_access_token(&self) -> &str;
    fn is_expired(&self) -> bool;
    fn set_token(&mut self, token: String, expires_in: i64);
}

// Type T implement TokenProviderClone must implement TokenProvider, Clone, 'static lifetime trail
impl<T> TokenProviderClone for T
where
    T: TokenProvider + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn TokenProvider> {
        return Box::new(self.clone());
    }
}

// Implement Clone box for TokenProvider
impl Clone for Box<dyn TokenProvider> {
    fn clone(&self) -> Self {
        return self.clone_box();
    }
}

pub struct AccessTokenBuilder {
    token: Option<String>,
    expires_in: Option<i64>,
}

impl Default for AccessTokenBuilder {
    fn default() -> Self {
        Self {
            token: None,
            expires_in: None,
        }
    }
}

impl AccessTokenBuilder {
    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn expires_in(mut self, expires_in: i64) -> Self {
        self.expires_in = Some(expires_in);
        self
    }

    pub fn build(self) -> Result<AccessToken, AuthError> {
        if self.token.is_none() || self.expires_in.is_none() {
            return Err(AuthError::Other("token and expires_in must be set".into()));
        }

        Ok(AccessToken::new(
            self.token.unwrap(),
            self.expires_in.unwrap(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct AccessToken {
    token: String,
    expires_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn new(token: String, expires_in: i64) -> Self {
        Self {
            token,
            expires_at: Utc::now() + chrono::Duration::seconds(expires_in - 10),
        }
    }

    pub fn builder() -> AccessTokenBuilder {
        AccessTokenBuilder::default()
    }
}

impl TokenProvider for AccessToken {
    fn get_access_token(&self) -> &str {
        &self.token
    }

    fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    fn set_token(&mut self, token: String, expires_in: i64) {
        self.token = token;
        self.expires_at = Utc::now() + chrono::Duration::seconds(expires_in - 10);
    }
}
