use super::data_source::{DataSource, DataSourceRefreshSchedule};
use super::range::NamedRange;
use super::sheet::Sheet;
use super::{cell::CellFormat, common::*};
use serde::{Deserialize, Serialize};

/// Represents a Google Sheets spreadsheet.
/// This is the top-level structure containing all sheets, properties, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spreadsheet {
    /// The unique identifier of the spreadsheet.
    pub spreadsheet_id: Option<String>,
    /// The properties of the spreadsheet.
    pub properties: Option<SpreadsheetProperties>,
    /// The sheets that comprise the spreadsheet.
    pub sheets: Option<Vec<Sheet>>,
    /// The named ranges defined in the spreadsheet.
    pub named_ranges: Option<Vec<NamedRange>>,
    /// The URL of the spreadsheet.
    pub spreadsheet_url: Option<String>,
    /// The developer metadata associated with the spreadsheet.
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// The data sources connected to the spreadsheet.
    pub data_sources: Option<Vec<DataSource>>,
    /// The data source refresh schedules.
    pub data_source_schedules: Option<Vec<DataSourceRefreshSchedule>>,
}

/// Properties of a spreadsheet, such as title, locale, and calculation settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetProperties {
    /// The title of the spreadsheet.
    pub title: Option<String>,
    /// The locale of the spreadsheet in ISO 639-1 format.
    pub locale: Option<String>,
    /// The amount of time to wait before volatile functions are recalculated.
    pub auto_recalc: Option<RecalculationInterval>,
    /// The time zone of the spreadsheet, in CLDR format such as America/New_York.
    pub time_zone: Option<String>,
    /// The default format for new cells.
    pub default_format: Option<CellFormat>,
    /// Settings for iterative calculation.
    pub iterative_calculation_settings: Option<IterativeCalculationSettings>,
    /// The theme applied to the spreadsheet.
    pub spreadsheet_theme: Option<SpreadsheetTheme>,
    /// Whether to allow external URLs in import functions.
    pub import_functions_external_url_access_allowed: Option<bool>,
}

/// The theme applied to a spreadsheet.
/// Themes define the visual appearance, including fonts and colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetTheme {
    /// The primary font family used in the spreadsheet.
    pub primary_font_family: Option<String>,
    /// The color pairs that define the theme.
    pub theme_colors: Option<Vec<ThemeColorPair>>,
}
