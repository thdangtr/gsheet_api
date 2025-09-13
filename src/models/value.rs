use serde::{Deserialize, Serialize};

/// Indicates which dimension an operation should apply to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default, strum_macros::Display)]
pub enum Dimension {
    /// The default value, do not use.
    #[strum(to_string = "DIMENSION_UNSPECIFIED")]
    DimensionUnspecified,
    /// Operates on the rows of a sheet.
    #[default]
    #[strum(to_string = "ROWS")]
    Rows,
    /// Operates on the columns of a sheet.
    #[strum(to_string = "COLUMNS")]
    Columns,
}

/// Data within a range of the spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRange {
    /// The range the values cover, in A1 notation.
    /// For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns.
    /// When appending values, this field represents the range to search for a table, after which values will be appended.
    pub range: Option<String>,
    /// The major dimension of the values.
    /// For output, if the spreadsheet data is: A1=1,B1=2,A2=3,B2=4, then requesting range=A1:B2,majorDimension=ROWS will return [[1,2],[3,4]],
    /// whereas requesting range=A1:B2,majorDimension=COLUMNS will return [[1,3],[2,4]].
    /// For input, with range=A1:B2,majorDimension=ROWS then [[1,2],[3,4]] will set A1=1,B1=2,A2=3,B2=4.
    /// With range=A1:B2,majorDimension=COLUMNS then [[1,2],[3,4]] will set A1=1,B1=3,A2=2,B2=4.
    /// When writing, if this field is not set, it defaults to ROWS.
    pub major_dimension: Option<Dimension>,
    /// The data that was read or to be written.
    /// This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension.
    /// Each item in the inner array corresponds with one cell.
    /// For output, empty trailing rows and columns will not be included.
    /// For input, supported value types are: bool, string, and double. Null values will be skipped.
    /// To set a cell to an empty value, set the string value to an empty string.
    pub values: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchValueRanges {
    spreadsheet_id: String,
    value_ranges: Vec<ValueRange>,
}

/// Determines how values should be rendered in the output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default, strum_macros::Display)]
pub enum ValueRenderOption {
    /// Values will be calculated & formatted in the response according to the cell's formatting.
    /// Formatting is based on the spreadsheet's locale, not the requesting user's locale.
    /// For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "$1.23".
    #[default]
    #[strum(to_string = "FORMATTED_VALUE")]
    FormattedValue,
    /// Values will be calculated, but not formatted in the reply.
    /// For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return the number 1.23.
    #[strum(to_string = "UNFORMATTED_VALUE")]
    UnformattedValue,
    /// Values will not be calculated. The reply will include the formulas.
    /// For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "=A1".
    #[strum(to_string = "FORMULA")]
    Formula,
}

/// Determines how dates should be rendered in the output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default, strum_macros::Display)]
pub enum DateTimeRenderOption {
    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format.
    /// The whole number portion counts the days since December 30th 1899.
    /// The fractional portion counts the time as a fraction of the day.
    #[default]
    #[strum(to_string = "SERIAL_NUMBER")]
    SerialNumber,
    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format.
    #[strum(to_string = "FORMATTED_STRING")]
    FormattedString,
}

/// Determines how input data should be interpreted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default, strum_macros::Display)]
pub enum ValueInputOption {
    /// Default input value. This value must not be used.
    #[strum(to_string = "INPUT_VALUE_OPTION_UNSPECIFIED")]
    InputValueOptionUnspecified,
    /// The values the user has entered will not be parsed and will be stored as-is.
    #[strum(to_string = "RAW")]
    Raw,
    /// The values will be parsed as if the user typed them into the UI.
    /// Numbers will stay as numbers, but strings may be converted to numbers, dates, etc.
    /// following the same rules that are applied when entering text into a cell via the Google Sheets UI.
    #[default]
    #[strum(to_string = "USER_ENTERED")]
    UserEntered,
}

/// The response when updating a range of values in a spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateValuesResponse {
    /// The spreadsheet the updates were applied to.
    pub spreadsheet_id: String,
    /// The range (in A1 notation) that updates were applied to.
    pub updated_range: String,
    /// The number of rows where at least one cell in the row was updated.
    pub updated_rows: i32,
    /// The number of columns where at least one cell in the column was updated.
    pub updated_columns: i32,
    /// The number of cells updated.
    pub updated_cells: i32,
    /// The values of the cells after updates were applied.
    /// This is only included if the request's includeValuesInResponse field was true.
    pub updated_data: Option<ValueRange>,
}

/// The response when updating a range of values in a spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateValuesResponse {
    /// The spreadsheet the updates were applied to.
    pub spreadsheet_id: String,
    /// The total number of rows where at least one cell in the row was updated.
    pub total_updated_rows: i32,
    /// The total number of columns where at least one cell in the column was updated.
    pub total_updated_columns: i32,
    /// The total number of cells updated.
    pub total_updated_cells: i32,
    /// The total number of sheets where at least one cell in the sheet was updated.
    pub total_updated_sheets: i32,
    /// One UpdateValuesResponse per requested range, in the same order as the requests appeared.
    pub responses: Vec<UpdateValuesResponse>,
}
