//! Service account authentication implementation.
//!
//! This module provides the [`ServiceAccountAuthClient`] which handles authentication
//! using Google service accounts. It automatically manages JWT token creation and
//! access token refresh.

use chrono::{Duration, Utc};
use jsonwebtoken::Header;
use jsonwebtoken::{Algorithm, EncodingKey, encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::AuthProvider;
use crate::auth::error::AuthError;
use crate::auth::token::{AccessToken, TokenProvider};

/// Service account key structure as defined by Google.
///
/// This struct represents the JSON structure of a Google service account key file.
/// The key file contains all the necessary information to authenticate with Google APIs.
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceAccountKey {
    /// The type of the key, typically "service_account".
    #[serde(rename = "type")]
    pub key_type: String,
    /// The ID of the Google Cloud project.
    pub project_id: String,
    /// The unique identifier for the private key.
    pub private_key_id: String,
    /// The private key in PEM format.
    pub private_key: String,
    /// The email address of the service account.
    pub client_email: String,
    /// The client ID of the service account.
    pub client_id: String,
    /// The URI for Google's OAuth 2.0 authorization endpoint.
    pub auth_uri: String,
    /// The URI for Google's token endpoint.
    pub token_uri: String,
    /// The URI for the public X.509 certificates for the service account.
    pub auth_provider_x509_cert_url: String,
    /// The URI for the service account's X.509 certificate.
    pub client_x509_cert_url: String,
}

/// JWT claims structure for Google service account authentication.
///
/// This struct defines the claims that are included in the JWT token sent to Google
/// for authentication. The claims include the issuer, scope, audience, and timestamps.
#[derive(Debug, Serialize)]
pub struct Claims {
    /// The issuer of the JWT - the service account email.
    pub iss: String,
    /// The scope of access requested.
    pub scope: String,
    /// The audience for the JWT - Google's token endpoint.
    pub aud: String,
    /// The expiration time of the JWT.
    pub exp: i64,
    /// The issued-at time of the JWT.
    pub iat: i64,
}

/// Response structure from Google's token endpoint.
///
/// This struct represents the JSON response received from Google's OAuth 2.0
/// token endpoint when exchanging a JWT for an access token.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    /// The access token that can be used to authenticate API requests.
    pub access_token: String,
    /// The type of token, typically "Bearer".
    pub token_type: String,
    /// The number of seconds until the access token expires.
    pub expires_in: i64,
}

/// Builder for creating [`ServiceAccountAuthClient`] instances.
///
/// This builder provides a fluent interface for configuring and creating
/// service account authentication clients.
#[derive(Debug)]
pub struct ServiceAccountAuthClientBuilder {
    service_account_path: Option<String>,
}

impl ServiceAccountAuthClientBuilder {
    /// Creates a new builder instance.
    pub fn new() -> Self {
        Self {
            service_account_path: None,
        }
    }

    /// Sets the path to the service account key JSON file.
    ///
    /// # Arguments
    /// * `path` - The file system path to the service account key file.
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn service_account_path(mut self, path: &str) -> Self {
        self.service_account_path = Some(path.to_string());
        self
    }

    /// Builds the [`ServiceAccountAuthClient`] instance.
    ///
    /// This method reads the service account key file, parses it, creates an initial
    /// access token, and returns a configured authentication client.
    ///
    /// # Returns
    /// A `Result` containing the configured [`ServiceAccountAuthClient`] or an [`AuthError`].
    ///
    /// # Errors
    /// This method will return an error if:
    /// - The service account path is not set
    /// - The key file cannot be read
    /// - The key file JSON is invalid
    /// - The initial token request fails
    pub async fn build(self) -> Result<ServiceAccountAuthClient, AuthError> {
        // Load service account key from file
        let service_account_path = self
            .service_account_path
            .ok_or_else(|| AuthError::Other("Service account path is required".into()))?;

        let service_account_content = std::fs::read_to_string(&service_account_path)?;

        // Parse service account key
        let service_account: ServiceAccountKey = serde_json::from_str(&service_account_content)?;

        // Create HTTP client
        let client = reqwest::Client::new();

        // Get initial access token
        let token = ServiceAccountAuthClient::get_access_token(&client, &service_account).await?;

        // Create AccessToken
        let access_token = AccessToken::builder()
            .token(&token.access_token)
            .expires_in(token.expires_in)
            .build()?;

        // Return the auth client
        Ok(ServiceAccountAuthClient {
            service_account,
            client,
            token: Box::new(access_token),
        })
    }
}

impl Default for ServiceAccountAuthClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Service account authentication client.
///
/// This struct handles authentication with Google APIs using a service account.
/// It automatically manages access token refresh and provides the necessary
/// authentication headers for API requests.
#[derive(Clone)]
pub struct ServiceAccountAuthClient {
    /// The service account key information.
    service_account: ServiceAccountKey,
    /// The HTTP client for making token requests.
    client: reqwest::Client,
    /// The token provider that manages the access token.
    token: Box<dyn TokenProvider>,
}

impl ServiceAccountAuthClient {
    /// Requests an access token from Google's OAuth 2.0 endpoint.
    ///
    /// This method creates a JWT assertion using the service account credentials,
    /// sends it to Google's token endpoint, and returns the access token response.
    ///
    /// # Arguments
    /// * `client` - The HTTP client to use for the request.
    /// * `service_account` - The service account key information.
    ///
    /// # Returns
    /// A `Result` containing the [`TokenResponse`] or an [`AuthError`].
    async fn get_access_token(
        client: &reqwest::Client,
        service_account: &ServiceAccountKey,
    ) -> Result<TokenResponse, AuthError> {
        // Create JWT claims
        let now = Utc::now();
        let claims = Claims {
            iss: service_account.client_email.clone(),
            scope: "https://www.googleapis.com/auth/spreadsheets".to_string(),
            aud: service_account.token_uri.clone(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(1)).timestamp(),
        };

        // Create JWT header
        let header = Header::new(Algorithm::RS256);

        // Encode private key
        let encoding_key = EncodingKey::from_rsa_pem(service_account.private_key.as_bytes())?;

        // Generate JWT
        let jwt = encode(&header, &claims, &encoding_key)?;

        let mut params = HashMap::new();

        params.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
        params.insert("assertion", &jwt);

        let response = client
            .post(&service_account.token_uri)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AuthError::RequestError(format!(
                "HTTP request failed: {}",
                error_text
            )));
        }

        let token_response: TokenResponse = response.json().await?;
        Ok(token_response)
    }

    /// Creates a new builder for constructing a [`ServiceAccountAuthClient`].
    ///
    /// # Returns
    /// A new [`ServiceAccountAuthClientBuilder`] instance.
    pub fn builder() -> ServiceAccountAuthClientBuilder {
        ServiceAccountAuthClientBuilder::new()
    }
}

#[async_trait::async_trait]
impl AuthProvider for ServiceAccountAuthClient {
    /// Returns the current access token.
    ///
    /// # Returns
    /// A string slice containing the current access token.
    fn get_token(&self) -> &str {
        &self.token.get_access_token()
    }

    /// Ensures the access token is valid, refreshing it if necessary.
    ///
    /// This method checks if the current token is expired and requests a new one
    /// from Google if needed.
    ///
    /// # Returns
    /// A `Result` indicating success or an [`AuthError`].
    async fn ensure_valid_token(&mut self) -> Result<(), AuthError> {
        if self.token.is_expired() {
            let new_token =
                ServiceAccountAuthClient::get_access_token(&self.client, &self.service_account)
                    .await?;
            self.token
                .set_token(new_token.access_token, new_token.expires_in);
        }
        Ok(())
    }
}
