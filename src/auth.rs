//! # Authentication Module
//!
//! This module provides authentication functionality for the Google Sheets API.
//! It supports service account authentication using JWT tokens.
//!
//! ## Overview
//!
//! The authentication system is built around the [`AuthProvider`] trait, which
//! defines the interface for authentication providers. Currently, the library
//! supports service account authentication via [`ServiceAccountAuthClient`].
//!
//! ## Service Account Authentication
//!
//! Service account authentication uses a JSON key file downloaded from Google Cloud Console.
//! This method is suitable for server-to-server authentication without user interaction.
//!
//! ### Example
//!
//! ```rust,no_run
//! use gsheet_api::auth::ServiceAccountAuthClient;
//!
//! let auth_client = ServiceAccountAuthClient::builder()
//!     .service_account_path("path/to/service-account.json")
//!     .build()
//!     .await?;
//! ```
//!
//! ## Security Considerations
//!
//! - Keep service account key files secure and never commit them to version control
//! - Use environment variables or secure key management systems for key file paths
//! - Regularly rotate service account keys
//! - Limit service account permissions to only what's necessary

pub mod error;
pub mod service_account;
pub mod token;

pub use service_account::ServiceAccountAuthClient;
pub use token::AccessToken;

/// Trait for authentication providers.
///
/// This trait defines the interface that all authentication providers must implement.
/// It provides methods for getting access tokens and ensuring they remain valid.
#[async_trait::async_trait]
pub trait AuthProvider {
    /// Returns the current access token as a string slice.
    ///
    /// # Returns
    /// A string slice containing the access token.
    fn get_token(&self) -> &str;

    /// Ensures that the access token is valid and refreshes it if necessary.
    ///
    /// This method should check if the current token is expired and refresh it
    /// if needed. Implementations should handle token refresh logic internally.
    ///
    /// # Returns
    /// A `Result` indicating success or an [`AuthError`](error::AuthError).
    async fn ensure_valid_token(&mut self) -> Result<(), AuthError>;
}

pub use error::AuthError;
