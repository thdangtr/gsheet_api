//! # Operations Module
//!
//! This module contains high-level operations for interacting with Google Sheets.
//! It provides structured APIs for reading, writing, and manipulating spreadsheet data.
//!
//! ## Overview
//!
//! The operations are organized into two main categories:
//! - **Spreadsheet Operations**: Work with entire spreadsheets (getting metadata, etc.)
//! - **Sheet Operations**: Work with individual sheets within a spreadsheet
//!
//! ## Architecture
//!
//! Operations follow a builder pattern for configuration and provide async `execute()`
//! methods to perform the actual API calls. This allows for flexible configuration
//! and clear separation between setup and execution.
//!
//! ## Common Patterns
//!
//! Most operations support method chaining for configuration:
//!
//! ```rust,no_run
//! # use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
//! # use std::sync::{Arc, Mutex};
//! # let auth_client = Arc::new(Mutex::new(ServiceAccountAuthClient::builder().service_account_path("").build().await.unwrap()));
//! # let gsheet_client = GoogleSheetClient::builder().auth_client(auth_client).build().unwrap();
//! let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
//!
//! // Configure and execute an operation
//! let result = spreadsheet.sheet("Sheet1")
//!     .get_all_value()
//!     .execute()
//!     .await?;
//! ```
//!
//! ## Error Handling
//!
//! All operations return `Result<T, GSheetError>` where `GSheetError` provides
//! detailed information about what went wrong.

pub mod sheet;
pub mod spreadsheet;
