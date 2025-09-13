//! # Client Module
//!
//! This module provides the main client for interacting with the Google Sheets API.
//! It handles HTTP requests, authentication, and provides access to spreadsheet operations.
//!
//! ## Overview
//!
//! The [`GoogleSheetClient`] is the primary entry point for making API calls to Google Sheets.
//! It uses a builder pattern for configuration and supports custom HTTP clients and base URLs.
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! use std::sync::{Arc, Mutex};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let auth_client = ServiceAccountAuthClient::builder()
//!     .service_account_path("keys.json")
//!     .build()
//!     .await?;
//!
//! let auth_client = Arc::new(Mutex::new(auth_client));
//!
//! let gsheet_client = GoogleSheetClient::builder()
//!     .auth_client(auth_client)
//!     .build()?;
//!
//! let spreadsheet = gsheet_client.spreadsheet("your-spreadsheet-id");
//! # Ok(())
//! # }
//! ```
//!
//! ## Custom Configuration
//!
//! The client supports custom HTTP clients and API base URLs:
//!
//! ```rust,no_run
//! # use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! # use std::sync::{Arc, Mutex};
//! # let auth_client = Arc::new(Mutex::new(ServiceAccountAuthClient::builder().service_account_path("").build().await.unwrap()));
//! let custom_client = reqwest::Client::builder()
//!     .timeout(std::time::Duration::from_secs(30))
//!     .build()?;
//!
//! let gsheet_client = GoogleSheetClient::builder()
//!     .auth_client(auth_client)
//!     .client(&custom_client)
//!     .api_base_url("https://sheets.googleapis.com/v4/spreadsheets")
//!     .build()?;
//! ```

pub mod gsheet_client;

pub use gsheet_client::{GoogleSheetClient, GoogleSheetClientBuilder};
