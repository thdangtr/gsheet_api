use super::common::{DimensionProperties, RowData};
use serde::{Deserialize, Serialize};

/// The data in the grid of a sheet.
/// This contains the actual cell data and metadata for rows and columns.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridData {
    /// The row this GridData starts on (0-based).
    pub start_row: Option<i32>,
    /// The column this GridData starts on (0-based).
    pub start_column: Option<i32>,
    /// The data in the rows of this grid.
    pub row_data: Option<Vec<RowData>>,
    /// Metadata about the requested rows in the grid.
    pub row_metadata: Option<Vec<DimensionProperties>>,
    /// Metadata about the requested columns in the grid.
    pub column_metadata: Option<Vec<DimensionProperties>>,
}

/// A range on a sheet.
/// All indexes are zero-based.
/// Indexes are half open: the start index is inclusive and the end index is exclusive.
/// Missing indexes indicate the range is unbounded on that side.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRange {
    /// The sheet this range is on.
    pub sheet_id: Option<i32>,
    /// The start row (inclusive) of the range, or not set if unbounded.
    pub start_row_index: usize,
    /// The end row (exclusive) of the range, or not set if unbounded.
    pub end_row_index: usize,
    /// The start column (inclusive) of the range, or not set if unbounded.
    pub start_column_index: usize,
    /// The end column (exclusive) of the range, or not set if unbounded.
    pub end_column_index: usize,
}

/// Properties of a grid.
/// These properties define the structure and appearance of the grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridProperties {
    /// The number of rows in the grid.
    pub row_count: Option<i32>,
    /// The number of columns in the grid.
    pub column_count: Option<i32>,
    /// The number of rows that are frozen in the grid.
    pub frozen_row_count: Option<i32>,
    /// The number of columns that are frozen in the grid.
    pub frozen_column_count: Option<i32>,
    /// True if the grid isn't showing gridlines in the UI.
    pub hide_gridlines: Option<bool>,
    /// True if the row grouping control toggle is shown after the group.
    pub row_group_control_after: Option<bool>,
    /// True if the column grouping control toggle is shown after the group.
    pub column_group_control_after: Option<bool>,
}
