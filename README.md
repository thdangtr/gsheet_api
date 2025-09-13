# Google Sheets API Rust Client

[![Crates.io](https://img.shields.io/crates/v/gsheet_api.svg)](https://crates.io/crates/gsheet_api)
[![Documentation](https://docs.rs/gsheet_api/badge.svg)](https://docs.rs/gsheet_api)
[![License](https://img.shields.io/crates/l/gsheet_api.svg)](https://github.com/yourusername/gsheet_api)

A comprehensive, async Rust library for interacting with Google Sheets API v4. This library provides both high-level and low-level APIs for reading, writing, and manipulating Google Sheets programmatically.

## Features

- **Service Account Authentication**: Secure authentication using Google service accounts
- **Spreadsheet Operations**: Create, read, and update spreadsheets
- **Sheet Operations**: Work with individual sheets within spreadsheets
- **Batch Operations**: Efficiently perform multiple operations in a single API call
- **Type Safety**: Strongly typed models for all Google Sheets data structures
- **Async/Await**: Modern async support using tokio
- **A1 Notation Support**: Parse and convert A1 notation ranges and cell references

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gsheet_api = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### 1. Set up Google Cloud Project

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the Google Sheets API
4. Create a service account and download the JSON key file

### 2. Basic Usage

```rust,no_run
use gsheet_api::{auth::ServiceAccountAuthClient, client::GoogleSheetClient};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate with service account
    let auth_client = ServiceAccountAuthClient::builder()
        .service_account_path("path/to/service-account.json")
        .build()
        .await?;

    let auth_client = Arc::new(Mutex::new(auth_client));

    // Create Google Sheets client
    let gsheet_client = GoogleSheetClient::builder()
        .auth_client(auth_client)
        .build()?;

    // Get spreadsheet operations
    let spreadsheet = gsheet_client.spreadsheet("your-spreadsheet-id");

    // Read all values from a sheet
    let sheet = spreadsheet.sheet("Sheet1");
    let values = sheet.get_all_value().execute().await?;

    println!("Values: {:?}", values);
    Ok(())
}
```

## Authentication

This library supports authentication via Google service accounts. You'll need:

1. A Google Cloud Project with the Google Sheets API enabled
2. A service account with appropriate permissions
3. A service account key JSON file

### Setting up Service Account Authentication

```rust,no_run
use gsheet_api::auth::ServiceAccountAuthClient;

let auth_client = ServiceAccountAuthClient::builder()
    .service_account_path("keys.json")
    .build()
    .await?;
```

### Security Considerations

- Keep service account key files secure and never commit them to version control
- Use environment variables or secure key management systems for key file paths
- Regularly rotate service account keys
- Limit service account permissions to only what's necessary

## Reading Data

The library provides several ways to read data from sheets:

### Get All Values

```rust,no_run
let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
let values = spreadsheet.sheet("Sheet1")
    .get_all_value()
    .execute()
    .await?;
```

### Get Values as Cells

```rust,no_run
let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
let cells = spreadsheet.sheet("Sheet1")
    .get_all_cell()
    .execute()
    .await?;
```

### Get Values as HashMap

```rust,no_run
let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
let cell_map = spreadsheet.sheet("Sheet1")
    .get_hash_map_cell()
    .execute()
    .await?;
```

### Batch Get Values

```rust,no_run
let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
let batch_values = spreadsheet.sheet("Sheet1")
    .batch_get_value_range()
    .range("A1:B10")
    .range("C1:D10")
    .execute()
    .await?;
```

## Writing Data

### Batch Update Values

```rust,no_run
let spreadsheet = gsheet_client.spreadsheet("spreadsheet-id");
let response = spreadsheet.sheet("Sheet1")
    .batch_update_value_range()
    .add_value_range("A1:B2", vec![
        vec!["Name".to_string(), "Age".to_string()],
        vec!["Alice".to_string(), "30".to_string()],
    ])
    .execute()
    .await?;
```

## Advanced Usage

### Custom HTTP Client

```rust,no_run
let custom_client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build()?;

let gsheet_client = GoogleSheetClient::builder()
    .auth_client(auth_client)
    .client(&custom_client)
    .build()?;
```

### Working with Ranges

```rust,no_run
use gsheet_api::utils::{parse_a1_cell, a1_to_grid_range};

// Parse cell references
let (col, row) = parse_a1_cell("B3")?;
assert_eq!(col, 2);
assert_eq!(row, 3);

// Convert ranges
let grid_range = a1_to_grid_range("A1:B10")?;
```

### Error Handling

```rust
use gsheet_api::error::GSheetError;

match operation.execute().await {
    Ok(result) => println!("Success: {:?}", result),
    Err(GSheetError::AuthError(e)) => println!("Authentication error: {}", e),
    Err(GSheetError::HttpRequestError(e)) => println!("HTTP error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## API Reference

### Core Types

- [`GoogleSheetClient`](https://docs.rs/gsheet_api/latest/gsheet_api/client/struct.GoogleSheetClient.html) - Main client for API interactions
- [`SpreadsheetOperations`](https://docs.rs/gsheet_api/latest/gsheet_api/operations/spreadsheet/struct.SpreadsheetOperations.html) - Operations on spreadsheets
- [`SheetOperations`](https://docs.rs/gsheet_api/latest/gsheet_api/operations/sheet/struct.SheetOperations.html) - Operations on individual sheets
- [`ServiceAccountAuthClient`](https://docs.rs/gsheet_api/latest/gsheet_api/auth/service_account/struct.ServiceAccountAuthClient.html) - Service account authentication

### Models

- [`Spreadsheet`](https://docs.rs/gsheet_api/latest/gsheet_api/models/spreadsheet/struct.Spreadsheet.html) - Spreadsheet representation
- [`Sheet`](https://docs.rs/gsheet_api/latest/gsheet_api/models/sheet/struct.Sheet.html) - Sheet representation
- [`Cell`](https://docs.rs/gsheet_api/latest/gsheet_api/models/cell/struct.Cell.html) - Cell data structure
- [`ValueRange`](https://docs.rs/gsheet_api/latest/gsheet_api/models/value/struct.ValueRange.html) - Range of cell values

## Examples

See the [examples](examples/) directory for complete working examples:

- `basic_read.rs` - Basic reading operations
- `batch_operations.rs` - Batch read/write operations
- `authentication.rs` - Different authentication methods

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Clone the repository
2. Run tests: `cargo test`
3. Run examples: `cargo run --example basic_read`
4. Check documentation: `cargo doc --open`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built on top of the [Google Sheets API v4](https://developers.google.com/sheets/api)
- Uses [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- Uses [serde](https://github.com/serde-rs/serde) for JSON serialization
- Uses [jsonwebtoken](https://github.com/Keats/jsonwebtoken) for JWT handling
