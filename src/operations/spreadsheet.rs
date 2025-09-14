//! Spreadsheet operations for Google Sheets API.
//!
//! This module provides operations that work with entire Google Sheets spreadsheets,
//! such as retrieving spreadsheet metadata, properties, and accessing individual sheets.

use super::sheet::SheetOperations;
use crate::auth::AuthError;
use crate::client::GoogleSheetClient;
use crate::error::GSheetError;
use crate::models::Spreadsheet;

/// Builder for creating [`SpreadsheetOperations`] instances.
///
/// This builder provides a fluent interface for configuring spreadsheet operations
/// with the necessary Google Sheets client and spreadsheet identifier.
#[derive(Clone)]
pub struct SpreadsheetOperationsBuilder {
    /// The Google Sheets client for making API requests.
    gsheet_client: Option<GoogleSheetClient>,
    /// The unique identifier of the spreadsheet to operate on.
    spreadsheet_id: Option<String>,
}

impl Default for SpreadsheetOperationsBuilder {
    fn default() -> Self {
        Self {
            gsheet_client: None,
            spreadsheet_id: None,
        }
    }
}

impl SpreadsheetOperationsBuilder {
    /// Creates a new builder with the specified client and spreadsheet ID.
    ///
    /// # Arguments
    /// * `gsheet_client` - The Google Sheets client
    /// * `spreadsheet_id` - The unique identifier of the spreadsheet
    ///
    /// # Returns
    /// A new [`SpreadsheetOperationsBuilder`] instance.
    pub fn new(gsheet_client: GoogleSheetClient, spreadsheet_id: &str) -> Self {
        Self {
            gsheet_client: Some(gsheet_client),
            spreadsheet_id: Some(spreadsheet_id.to_string()),
        }
    }

    /// Sets the Google Sheets client for this builder.
    ///
    /// # Arguments
    /// * `gsheet_client` - The Google Sheets client
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn google_sheet_client(mut self, gsheet_client: GoogleSheetClient) -> Self {
        self.gsheet_client = Some(gsheet_client);
        self
    }

    /// Sets the spreadsheet ID for this builder.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the spreadsheet
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn spreadsheet_id(mut self, id: &str) -> Self {
        self.spreadsheet_id = Some(id.to_string());
        self
    }

    /// Builds the [`SpreadsheetOperations`] instance.
    ///
    /// # Returns
    /// A `Result` containing the configured [`SpreadsheetOperations`] or a [`GSheetError`].
    ///
    /// # Errors
    /// This method will return an error if either the client or spreadsheet ID is not set.
    pub fn build(self) -> Result<SpreadsheetOperations, GSheetError> {
        let gsheet_client = self.gsheet_client.ok_or_else(|| {
            GSheetError::Other(format!(
                "GoogleSheetClient is required to build SpreadsheetOperations"
            ))
        })?;
        let spreadsheet_id = self.spreadsheet_id.ok_or_else(|| {
            GSheetError::Other(format!(
                "spreadsheet_id is required to build SpreadsheetOperations"
            ))
        })?;

        Ok(SpreadsheetOperations::new(gsheet_client, spreadsheet_id))
    }
}

/// Operations for working with Google Sheets spreadsheets.
///
/// This struct provides methods for performing operations on an entire Google Sheets
/// spreadsheet, such as retrieving metadata or accessing individual sheets within it.
#[derive(Clone)]
pub struct SpreadsheetOperations {
    /// The Google Sheets client for making API requests.
    pub gsheet_client: GoogleSheetClient,
    /// The unique identifier of the spreadsheet.
    pub spreadsheet_id: String,
}

impl SpreadsheetOperations {
    /// Creates a new [`SpreadsheetOperations`] instance.
    ///
    /// # Arguments
    /// * `gsheet_client` - The Google Sheets client
    /// * `spreadsheet_id` - The unique identifier of the spreadsheet
    ///
    /// # Returns
    /// A new [`SpreadsheetOperations`] instance.
    pub fn new(gsheet_client: GoogleSheetClient, spreadsheet_id: String) -> Self {
        Self {
            gsheet_client,
            spreadsheet_id,
        }
    }

    /// Creates a builder for constructing [`SpreadsheetOperations`].
    ///
    /// # Arguments
    /// * `gsheet_client` - The Google Sheets client
    /// * `spreadsheet_id` - The unique identifier of the spreadsheet
    ///
    /// # Returns
    /// A new [`SpreadsheetOperationsBuilder`] instance.
    pub fn builder(
        gsheet_client: GoogleSheetClient,
        spreadsheet_id: &str,
    ) -> SpreadsheetOperationsBuilder {
        SpreadsheetOperationsBuilder::new(gsheet_client, spreadsheet_id)
    }

    /// Creates operations for working with a specific sheet in this spreadsheet.
    ///
    /// # Arguments
    /// * `title` - The title/name of the sheet
    ///
    /// # Returns
    /// A [`SheetOperations`] instance for the specified sheet.
    pub fn sheet(&self, title: &str) -> SheetOperations {
        SheetOperations::new(self.clone(), title.to_string())
    }

    /// Creates a builder for getting spreadsheet metadata.
    ///
    /// # Returns
    /// A [`GetSpreadsheetBuilder`] for configuring the get operation.
    pub fn get(&self) -> GetSpreadsheetBuilder {
        GetSpreadsheetBuilder::new(self)
    }
}

/// Builder for configuring spreadsheet retrieval operations.
///
/// This builder allows you to configure various options for retrieving spreadsheet
/// metadata and data, such as specific ranges and data inclusion options.
pub struct GetSpreadsheetBuilder {
    /// The spreadsheet operations instance.
    spreadsheet: Option<SpreadsheetOperations>,
    /// Specific ranges to retrieve (optional).
    ranges: Vec<String>,
    /// Whether to include grid data in the response.
    include_grid_data: bool,
    /// Whether to exclude tables in banded ranges.
    exclude_tables_in_banded_ranges: bool,
}
impl Default for GetSpreadsheetBuilder {
    fn default() -> Self {
        Self {
            spreadsheet: None,
            ranges: vec![],
            include_grid_data: false,
            exclude_tables_in_banded_ranges: false,
        }
    }
}

impl GetSpreadsheetBuilder {
    /// Creates a new builder for the specified spreadsheet.
    ///
    /// # Arguments
    /// * `spreadsheet` - The spreadsheet operations instance
    ///
    /// # Returns
    /// A new [`GetSpreadsheetBuilder`] instance.
    pub fn new(spreadsheet: &SpreadsheetOperations) -> Self {
        Self {
            spreadsheet: Some(spreadsheet.clone()),
            ranges: vec![],
            include_grid_data: false,
            exclude_tables_in_banded_ranges: false,
        }
    }

    /// Adds a range to retrieve from the spreadsheet.
    ///
    /// # Arguments
    /// * `range` - The A1 notation range (e.g., "A1:B10" or "Sheet1!A1:B10")
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn add_range(mut self, range: &str) -> Self {
        self.ranges.push(range.to_string());
        self
    }

    /// Sets whether to include grid data in the response.
    ///
    /// When true, the response will include the actual cell data for all sheets.
    /// When false, only metadata about the spreadsheet structure is returned.
    ///
    /// # Arguments
    /// * `include` - Whether to include grid data
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn include_grid_data(mut self, include: bool) -> Self {
        self.include_grid_data = include;
        self
    }

    /// Sets whether to exclude tables in banded ranges.
    ///
    /// # Arguments
    /// * `exclude` - Whether to exclude tables in banded ranges
    ///
    /// # Returns
    /// The builder instance for method chaining.
    pub fn exclude_tables_in_banded_ranges(mut self, exclude: bool) -> Self {
        self.exclude_tables_in_banded_ranges = exclude;
        self
    }

    /// Builds the [`GetSpreadsheetOperations`] instance.
    ///
    /// # Returns
    /// A `Result` containing the configured [`GetSpreadsheetOperations`] or a [`GSheetError`].
    ///
    /// # Errors
    /// This method will return an error if the spreadsheet is not set.
    pub fn build(self) -> Result<GetSpreadsheetOperations, GSheetError> {
        let spreadsheet = self.spreadsheet.ok_or_else(|| {
            GSheetError::Other(format!(
                "SpreadsheetOperations is required to build GetSpreadsheetOperations"
            ))
        })?;

        Ok(GetSpreadsheetOperations {
            spreadsheet,
            ranges: self.ranges,
            include_grid_data: self.include_grid_data,
            exclude_tables_in_banded_ranges: self.exclude_tables_in_banded_ranges,
        })
    }
}

/// Operation for retrieving spreadsheet metadata and data.
///
/// This struct represents a configured operation to retrieve information about
/// a Google Sheets spreadsheet, optionally including cell data and specific ranges.
pub struct GetSpreadsheetOperations {
    /// The spreadsheet operations instance.
    spreadsheet: SpreadsheetOperations,
    /// The ranges to retrieve.
    ranges: Vec<String>,
    /// Whether to include grid data.
    include_grid_data: bool,
    /// Whether to exclude tables in banded ranges.
    exclude_tables_in_banded_ranges: bool,
}

impl GetSpreadsheetOperations {
    /// Creates operations for working with a specific sheet in the retrieved spreadsheet.
    ///
    /// # Arguments
    /// * `title` - The title/name of the sheet
    ///
    /// # Returns
    /// A [`SheetOperations`] instance for the specified sheet.
    pub async fn sheet(&self, title: &str) -> SheetOperations {
        SheetOperations::new(self.spreadsheet.clone(), title.to_string())
    }

    /// Executes the spreadsheet retrieval operation.
    ///
    /// This method makes an API call to Google Sheets to retrieve the spreadsheet
    /// metadata and optionally cell data based on the configured options.
    ///
    /// # Returns
    /// A `Result` containing the [`Spreadsheet`] data or a [`GSheetError`].
    ///
    /// # Errors
    /// This method will return an error if:
    /// - Authentication fails
    /// - The HTTP request fails
    /// - The response cannot be parsed
    /// - The spreadsheet does not exist or is not accessible
    pub async fn execute(&self) -> Result<Spreadsheet, GSheetError> {
        let url = format!(
            "{}/{}",
            self.spreadsheet.gsheet_client.base_url, self.spreadsheet.spreadsheet_id
        );

        let mut auth_client = self
            .spreadsheet
            .gsheet_client
            .auth_client
            .lock()
            .map_err(|e| GSheetError::AuthError(AuthError::Other(e.to_string())))?;

        auth_client.ensure_valid_token().await?;

        let mut request = self
            .spreadsheet
            .gsheet_client
            .client
            .get(&url)
            .bearer_auth(auth_client.get_token());

        println!("Ranges: {}", self.ranges.join(", "));

        if !self.ranges.is_empty() {
            for range in &self.ranges {
                request = request.query(&[("ranges", range)]);
            }
        }

        if self.include_grid_data {
            request = request.query(&[("includeGridData", "true")]);
        }

        if self.exclude_tables_in_banded_ranges {
            request = request.query(&[("excludeTablesInBandedRanges", "true")]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let spreadsheet: Spreadsheet = response.json().await?;
            Ok(spreadsheet)
        } else {
            Err(GSheetError::HttpRequestError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
}
