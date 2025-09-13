//! Google Sheets API client implementation.
//!
//! This module contains the main client structures for interacting with the Google Sheets API.
//! The [`GoogleSheetClient`] handles authentication and provides access to spreadsheet operations.

use crate::error::GSheetError;
use crate::{auth::AuthProvider, operations::spreadsheet::SpreadsheetOperations};
use std::sync::{Arc, Mutex};

/// Builder for creating [`GoogleSheetClient`] instances.
///
/// This builder provides a fluent interface for configuring the Google Sheets client
/// with authentication, HTTP client, and API endpoint settings.
pub struct GoogleSheetClientBuilder {
    /// The authentication provider for API requests.
    auth_client: Option<Arc<Mutex<dyn AuthProvider>>>,
    /// Optional custom HTTP client.
    client: Option<reqwest::Client>,
    /// Optional custom API base URL.
    api_base_url: Option<String>,
}

impl GoogleSheetClientBuilder {
    /// Sets the authentication client for API requests.
    ///
    /// # Arguments
    /// * `auth_client` - The authentication provider wrapped in an Arc<Mutex<>>
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn auth_client(mut self, auth_client: Arc<Mutex<dyn AuthProvider>>) -> Self {
        self.auth_client = Some(auth_client);
        self
    }

    /// Sets a custom HTTP client for making requests.
    ///
    /// # Arguments
    /// * `client` - The reqwest client to use for HTTP requests
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn client(mut self, client: &reqwest::Client) -> Self {
        self.client = Some(client.clone());
        self
    }

    /// Sets a custom base URL for the Google Sheets API.
    ///
    /// # Arguments
    /// * `url` - The base URL for API requests
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn api_base_url(mut self, url: &str) -> Self {
        self.api_base_url = Some(url.to_string());
        self
    }

    /// Builds the [`GoogleSheetClient`] instance.
    ///
    /// # Returns
    /// A `Result` containing the configured [`GoogleSheetClient`] or a [`GSheetError`].
    ///
    /// # Errors
    /// This method will return an error if the authentication client is not set.
    pub fn build(self) -> Result<GoogleSheetClient, GSheetError> {
        let auth_client = self
            .auth_client
            .ok_or_else(|| GSheetError::Other("Auth client is required".into()))?;
        let client = self.client.unwrap_or_else(|| reqwest::Client::new());
        let base_url = self
            .api_base_url
            .unwrap_or_else(|| "https://sheets.googleapis.com/v4/spreadsheets".to_string());

        Ok(GoogleSheetClient {
            auth_client,
            client,
            base_url,
        })
    }
}

impl Default for GoogleSheetClientBuilder {
    fn default() -> Self {
        Self {
            auth_client: None,
            client: None,
            api_base_url: None,
        }
    }
}

/// Main client for interacting with the Google Sheets API.
///
/// This struct provides the primary interface for making authenticated requests
/// to the Google Sheets API. It handles authentication token management and
/// provides access to spreadsheet operations.
#[derive(Clone)]
pub struct GoogleSheetClient {
    /// The authentication provider for managing access tokens.
    pub auth_client: Arc<Mutex<dyn AuthProvider>>,
    /// The HTTP client for making API requests.
    pub client: reqwest::Client,
    /// The base URL for Google Sheets API endpoints.
    pub base_url: String,
}

impl GoogleSheetClient {
    /// Creates a new Google Sheets client with explicit configuration.
    ///
    /// # Arguments
    /// * `auth_client` - The authentication provider
    /// * `client` - The HTTP client for requests
    /// * `base_url` - The base URL for API endpoints
    ///
    /// # Returns
    /// A new [`GoogleSheetClient`] instance.
    pub fn new(
        auth_client: Arc<Mutex<dyn AuthProvider>>,
        client: reqwest::Client,
        base_url: String,
    ) -> Self {
        GoogleSheetClient {
            auth_client,
            client,
            base_url,
        }
    }

    /// Creates a new builder for constructing a [`GoogleSheetClient`].
    ///
    /// # Returns
    /// A new [`GoogleSheetClientBuilder`] instance.
    pub fn builder() -> GoogleSheetClientBuilder {
        GoogleSheetClientBuilder::default()
    }

    /// Creates a [`SpreadsheetOperations`] instance for the specified spreadsheet.
    ///
    /// This method provides access to operations that can be performed on a specific
    /// Google Sheets spreadsheet identified by its ID.
    ///
    /// # Arguments
    /// * `spreadsheet_id` - The unique identifier of the Google Sheets spreadsheet
    ///
    /// # Returns
    /// A [`SpreadsheetOperations`] instance for the specified spreadsheet.
    pub fn spreadsheet(&self, spreadsheet_id: &str) -> SpreadsheetOperations {
        SpreadsheetOperations::new(self.clone(), spreadsheet_id.to_string())
    }
}
