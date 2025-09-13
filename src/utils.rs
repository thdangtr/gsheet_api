//! # Utilities Module
//!
//! This module provides utility functions for working with Google Sheets data,
//! particularly for parsing and converting A1 notation ranges and cell references.
//!
//! ## A1 Notation
//!
//! Google Sheets uses A1 notation for referencing cells and ranges. This module
//! provides functions to parse and convert between A1 notation and internal
//! representations.
//!
//! ### Examples
//!
//! ```rust
//! use gsheet_api::utils::{parse_a1_cell, a1_to_grid_range};
//!
//! // Parse a single cell
//! let (col, row) = parse_a1_cell("B3").unwrap();
//! assert_eq!(col, 2);
//! assert_eq!(row, 3);
//!
//! // Convert A1 range to GridRange
//! let grid_range = a1_to_grid_range("A1:B10").unwrap();
//! ```
//!
//! ## Data Conversion
//!
//! The module also provides functions for converting between different data
//! representations used by the Google Sheets API, such as converting ValueRange
//! responses to Cell structures or HashMap representations.

use std::collections::HashMap;

use crate::error::GSheetError;
use crate::models::{Cell, GridRange, ValueRange};

/// Parses an A1 notation cell reference into column and row indices.
///
/// This function converts a cell reference like "A1", "B2", "AA10" into
/// zero-based column and row indices. Column letters are converted to numbers
/// where A=1, B=2, ..., Z=26, AA=27, etc.
///
/// # Arguments
/// * `a1` - The A1 notation cell reference (e.g., "A1", "B2", "AA10")
///
/// # Returns
/// A `Result` containing a tuple `(column_index, row_index)` or a [`GSheetError`].
///
/// # Examples
/// ```rust
/// use gsheet_api::utils::parse_a1_cell;
///
/// let (col, row) = parse_a1_cell("A1").unwrap();
/// assert_eq!(col, 1);
/// assert_eq!(row, 1);
///
/// let (col, row) = parse_a1_cell("B3").unwrap();
/// assert_eq!(col, 2);
/// assert_eq!(row, 3);
/// ```
///
/// # Errors
/// This function will return an error if:
/// - The input contains invalid characters
/// - The cell reference is malformed
/// - Column or row indices are zero or negative
pub fn parse_a1_cell(a1: &str) -> Result<(usize, usize), GSheetError> {
    let mut col = 0;
    let mut row = 0;
    let mut col_part = true;

    for c in a1.chars() {
        if c.is_ascii_alphabetic() && col_part {
            col = col * 26 + (c.to_ascii_uppercase() as usize - 'A' as usize + 1);
        } else if c.is_ascii_digit() {
            col_part = false;
            row = row * 10 + (c as usize - '0' as usize);
        } else {
            return Err(GSheetError::UtilsError("Invalid character".into()));
        }
    }

    if col > 0 && row > 0 {
        Ok((col, row))
    } else {
        Err(GSheetError::UtilsError("Invalid A1 notation".into()))
    }
}

/// Converts an A1 notation range to a GridRange structure.
///
/// This function parses A1 notation ranges like "A1:B10" or "Sheet1!A1:B10"
/// and converts them to the internal GridRange representation used by the API.
///
/// # Arguments
/// * `a1` - The A1 notation range (e.g., "A1:B10", "Sheet1!A1:Z100")
///
/// # Returns
/// A `Result` containing a [`GridRange`] or a [`GSheetError`].
///
/// # Examples
/// ```rust
/// use gsheet_api::utils::a1_to_grid_range;
///
/// let range = a1_to_grid_range("A1:B10").unwrap();
/// assert_eq!(range.start_row_index, Some(1));
/// assert_eq!(range.end_row_index, Some(10));
/// assert_eq!(range.start_column_index, Some(1));
/// assert_eq!(range.end_column_index, Some(2));
/// ```
///
/// # Errors
/// This function will return an error if:
/// - The range format is invalid
/// - The cell references are malformed
/// - Sheet name parsing fails (if present)
pub fn a1_to_grid_range(a1: &str) -> Result<GridRange, GSheetError> {
    let mut range_part: &str = a1.trim();

    if range_part.contains("!") == true {
        let (_, range_part_str) = split_sheet_range(range_part)?;
        range_part = range_part_str;
    }

    let range_parts: Vec<&str> = range_part.split(':').collect();

    let (start, end) = match range_parts.len() {
        1 => (range_parts[0], range_parts[0]),
        2 => (range_parts[0], range_parts[1]),
        _ => return Err(GSheetError::UtilsError("Invalid range".into())),
    };

    let (start_col, start_row) = parse_a1_cell(start)?;
    let (end_col, end_row) = parse_a1_cell(end)?;

    Ok(GridRange {
        sheet_id: None,
        start_row_index: Some(start_row),
        end_row_index: Some(end_row),
        start_column_index: Some(start_col),
        end_column_index: Some(end_col),
    })
}

/// Splits a sheet-qualified range into sheet name and range components.
///
/// This function takes a range like "Sheet1!A1:B10" and splits it into
/// the sheet name ("Sheet1") and the range part ("A1:B10").
///
/// # Arguments
/// * `a1` - The sheet-qualified A1 notation range
///
/// # Returns
/// A `Result` containing a tuple `(sheet_name, range)` or a [`GSheetError`].
///
/// # Errors
/// This function will return an error if the range doesn't contain a "!" separator.
pub fn split_sheet_range(a1: &str) -> Result<(&str, &str), GSheetError> {
    let range_part: &str = a1.trim();

    if range_part.contains("!") == true {
        let parts: Vec<&str> = range_part.split('!').collect();
        if parts.len() != 2 {
            return Err(GSheetError::UtilsError("Invalid range".into()));
        }

        return Ok((parts[0], parts[1]));
    }

    Err(GSheetError::UtilsError("Invalid range".into()))
}

/// Converts a column index to A1 notation column letters.
///
/// This function converts a 1-based column index to the corresponding
/// letter representation used in A1 notation (A, B, ..., Z, AA, AB, etc.).
///
/// # Arguments
/// * `col_index` - The 1-based column index
///
/// # Returns
/// A `Result` containing the A1 column string or a [`GSheetError`].
///
/// # Examples
/// ```rust
/// use gsheet_api::utils::col_index_to_a1;
///
/// let col = col_index_to_a1(1).unwrap();
/// assert_eq!(col, "A");
///
/// let col = col_index_to_a1(26).unwrap();
/// assert_eq!(col, "Z");
///
/// let col = col_index_to_a1(27).unwrap();
/// assert_eq!(col, "AA");
/// ```
///
/// # Errors
/// This function will return an error if the column index is 0.
pub fn col_index_to_a1(col_index: usize) -> Result<String, GSheetError> {
    if col_index == 0 {
        return Err(GSheetError::UtilsError("Invalid column index".into()));
    }

    let mut col_index = col_index;
    let mut col_str = String::new();

    while col_index > 0 {
        let rem = (col_index - 1) % 26;
        col_str.insert(0, (b'A' + rem as u8) as char);
        col_index = (col_index - 1) / 26;
    }

    Ok(col_str)
}

/// Converts a ValueRange response to a vector of Cell structures.
///
/// This function takes the raw API response from Google Sheets and converts it
/// into a more convenient vector of Cell structures with proper addressing.
///
/// # Arguments
/// * `sheet_id` - The ID of the spreadsheet
/// * `sheet_title` - The title of the sheet
/// * `value_range` - The ValueRange response from the API
///
/// # Returns
/// A `Result` containing a vector of [`Cell`] structures or a [`GSheetError`].
pub fn value_range_to_cells(
    sheet_id: &str,
    sheet_title: &str,
    value_range: &ValueRange,
) -> Result<Vec<Cell>, GSheetError> {
    let range = value_range
        .range
        .as_ref()
        .ok_or_else(|| GSheetError::UtilsError("ValueRange.range is None".into()))?;
    let grid_range = a1_to_grid_range(range)?;

    let all_values = vec![vec![]];
    let all_values = value_range.values.as_ref().unwrap_or(&all_values);

    let mut cells = Vec::new();
    for row_index in grid_range.start_row_index.unwrap()..=grid_range.end_row_index.unwrap() {
        for col_index in
            grid_range.start_column_index.unwrap()..=grid_range.end_column_index.unwrap()
        {
            let i = row_index - grid_range.start_row_index.unwrap();
            let j = col_index - grid_range.start_column_index.unwrap();

            let _cell_value = all_values.get(i).and_then(|r| r.get(j)).cloned();

            let col = col_index_to_a1(col_index)?;

            let address = format!("{}{}", col, row_index);
            let cell = Cell {
                sheet_id: sheet_id.to_string(),
                sheet_title: sheet_title.to_string(),
                address,
                value: _cell_value,
                col,
                col_index,
                row_index,
            };
            cells.push(cell);
        }
    }

    Ok(cells)
}

/// Converts a ValueRange response to a HashMap of column-to-row Cell mappings.
///
/// This function creates a nested HashMap structure where the outer key is the
/// column letter (A, B, C, etc.) and the inner key is the row number, with Cell
/// structures as values. This provides efficient lookup of cells by address.
///
/// # Arguments
/// * `sheet_id` - The ID of the spreadsheet
/// * `sheet_title` - The title of the sheet
/// * `value_range` - The ValueRange response from the API
///
/// # Returns
/// A `Result` containing a HashMap of column-to-row Cell mappings or a [`GSheetError`].
pub fn value_range_to_hash_cell_map(
    sheet_id: &str,
    sheet_title: &str,
    value_range: &ValueRange,
) -> Result<std::collections::HashMap<String, HashMap<usize, Cell>>, GSheetError> {
    let range = value_range
        .range
        .as_ref()
        .ok_or_else(|| GSheetError::UtilsError("ValueRange.range is None".into()))?;

    let grid_range = a1_to_grid_range(range)?;

    let all_values = vec![vec![]];
    let all_values = value_range.values.as_ref().unwrap_or(&all_values);

    let mut hash_map: HashMap<String, HashMap<usize, Cell>> = HashMap::new();

    for _row_index in grid_range.start_row_index.unwrap()..=grid_range.end_row_index.unwrap() {
        for _col_index in
            grid_range.start_column_index.unwrap()..=grid_range.end_column_index.unwrap()
        {
            let i = _row_index - grid_range.start_row_index.unwrap();
            let j = _col_index - grid_range.start_column_index.unwrap();
            let col = col_index_to_a1(_col_index)?;

            let _cell_value = all_values.get(i).and_then(|r| r.get(j)).cloned();

            let cell = Cell {
                sheet_id: sheet_id.to_string(),
                sheet_title: sheet_title.to_string(),
                address: format!("{}{}", col, _row_index),
                value: _cell_value,
                col: col.clone(),
                col_index: _col_index,
                row_index: _row_index,
            };

            if hash_map.contains_key(&cell.col) {
                hash_map
                    .get_mut(&cell.col)
                    .unwrap()
                    .insert(cell.row_index, cell);
            } else {
                let mut row_map = HashMap::new();
                row_map.insert(cell.row_index, cell);
                hash_map.insert(col, row_map);
            }
        }
    }

    Ok(hash_map)
}
