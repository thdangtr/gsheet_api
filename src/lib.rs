//! # Google Sheets API Rust Client
//!
//! A comprehensive Rust library for interacting with Google Sheets API v4.
//! This library provides both high-level and low-level APIs for reading, writing,
//! and manipulating Google Sheets programmatically.
//!
//! ## Features
//!
//! - **Service Account Authentication**: Secure authentication using Google service accounts
//! - **Spreadsheet Operations**: Create, read, and update spreadsheets
//! - **Sheet Operations**: Work with individual sheets within spreadsheets
//! - **Batch Operations**: Efficiently perform multiple operations in a single API call
//! - **Type Safety**: Strongly typed models for all Google Sheets data structures
//! - **Async/Await**: Modern async support using tokio
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! use std::sync::{Arc, Mutex};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Authenticate with service account
//!     let auth_client = ServiceAccountAuthClient::builder()
//!         .service_account_path("path/to/service-account.json")
//!         .build()
//!         .await?;
//!
//!     let auth_client = Arc::new(Mutex::new(auth_client));
//!
//!     // Create Google Sheets client
//!     let gsheet_client = GoogleSheetClient::builder()
//!         .auth_client(auth_client)
//!         .build()?;
//!
//!     // Get spreadsheet operations
//!     let spreadsheet = gsheet_client.spreadsheet("your-spreadsheet-id");
//!
//!     // Read all values from a sheet
//!     let sheet = spreadsheet.sheet("Sheet1");
//!     let values = sheet.get_all_value().execute().await?;
//!
//!     println!("Values: {:?}", values);
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! This library supports authentication via Google service accounts. You'll need:
//!
//! 1. A Google Cloud Project with the Google Sheets API enabled
//! 2. A service account with appropriate permissions
//! 3. A service account key JSON file
//!
//! ### Setting up Service Account Authentication
//!
//! ```rust,no_run
//! use gsheet_api::auth::ServiceAccountAuthClient;
//!
//! let auth_client = ServiceAccountAuthClient::builder()
//!     .service_account_path("keys.json")
//!     .build()
//!     .await?;
//! ```
//!
//! ## Reading Data
//!
//! The library provides several ways to read data from sheets:
//!
//! ### Get All Values
//! ```rust,no_run
//! # use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! # use std::sync::{Arc, Mutex};
//! # let auth_client = Arc::new(Mutex::new(ServiceAccountAuthClient::builder().service_account_path("").build().await.unwrap()));
//! # let gsheet_client = GoogleSheetClient::builder().auth_client(auth_client).build().unwrap();
//! let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
//! let values = spreadsheet.sheet("Sheet1")
//!     .get_all_value()
//!     .execute()
//!     .await?;
//! ```
//!
//! ### Get Values as Cells
//! ```rust,no_run
//! # use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! # use std::sync::{Arc, Mutex};
//! # let auth_client = Arc::new(Mutex::new(ServiceAccountAuthClient::builder().service_account_path("").build().await.unwrap()));
//! # let gsheet_client = GoogleSheetClient::builder().auth_client(auth_client).build().unwrap();
//! let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
//! let cells = spreadsheet.sheet("Sheet1")
//!     .get_all_cell()
//!     .execute()
//!     .await?;
//! ```
//!
//! ### Get Values as HashMap
//! ```rust,no_run
//! # use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! # use std::sync::{Arc, Mutex};
//! # let auth_client = Arc::new(Mutex::new(ServiceAccountAuthClient::builder().service_account_path("").build().await.unwrap()));
//! # let gsheet_client = GoogleSheetClient::builder().auth_client(auth_client).build().unwrap();
//! let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
//! let cell_map = spreadsheet.sheet("Sheet1")
//!     .get_hash_map_cell()
//!     .execute()
//!     .await?;
//! ```
//!
//! ## Writing Data
//!
//! ### Batch Update Values
//! ```rust,no_run
//! # use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! # use std::sync::{Arc, Mutex};
//! # let auth_client = Arc::new(Mutex::new(ServiceAccountAuthClient::builder().service_account_path("").build().await.unwrap()));
//! # let gsheet_client = GoogleSheetClient::builder().auth_client(auth_client).build().unwrap();
//! let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
//! let response = spreadsheet.sheet("Sheet1")
//!     .batch_update_value_range()
//!     .add_value_range("A1:B2", vec![
//!         vec!["Name".to_string(), "Age".to_string()],
//!         vec!["Alice".to_string(), "30".to_string()],
//!     ])
//!     .execute()
//!     .await?;
//! ```
//!
//! ## Error Handling
//!
//! The library uses `thiserror` for comprehensive error handling:
//!
//! ```rust
//! use gsheet_api::error::GSheetError;
//!
//! match operation.execute().await {
//!     Ok(result) => println!("Success: {:?}", result),
//!     Err(GSheetError::AuthError(e)) => println!("Authentication error: {}", e),
//!     Err(GSheetError::HttpRequestError(e)) => println!("HTTP error: {}", e),
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```
//!
//! ## Modules
//!
//! - [`auth`]: Authentication providers and service account handling
//! - [`client`]: Main client for interacting with Google Sheets API
//! - [`models`]: Data models representing Google Sheets structures
//! - [`operations`]: High-level operations for spreadsheets and sheets
//! - [`utils`]: Utility functions for A1 notation and data conversion
//! - [`error`]: Error types and handling

pub mod auth;
pub mod client;
pub mod error;
pub mod models;
pub mod operations;
pub mod types;
pub mod utils;
