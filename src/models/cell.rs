//! Cell data models and structures.
//!
//! This module contains models for representing individual cell data,
//! formatting, and related structures in Google Sheets.

use super::common::{ChipRun, Color, ColorStyle, ExtendedValue, PivotTable};
use super::conditions::DataValidationRule;
use super::data_source::{DataSourceFormula, DataSourceTable};
use super::formatting::{
    Borders, HorizontalAlign, HyperlinkDisplayType, NumberFormat, Padding, TextDirection,
    TextFormat, TextFormatRun, TextRotation, VerticalAlign, WrapStrategy,
};
use serde::{Deserialize, Serialize};

/// Data in a cell.
/// Data is typed and can be either a string, number, boolean, or formula.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellData {
    /// The value the user entered in the cell.
    pub user_entered_value: Option<ExtendedValue>,
    /// The effective value of the cell.
    pub effective_value: Option<ExtendedValue>,
    /// The formatted value of the cell.
    pub formatted_value: Option<String>,
    /// The format the user entered for the cell.
    pub user_entered_format: Option<CellFormat>,
    /// The effective format being used by the cell.
    pub effective_format: Option<CellFormat>,
    /// The hyperlink destination of the cell, if any.
    pub hyperlink: Option<String>,
    /// Any note on the cell.
    pub note: Option<String>,
    /// Runs of rich text formatting.
    pub text_format_runs: Option<Vec<TextFormatRun>>,
    /// A data validation rule on the cell, if any.
    pub data_validation: Option<DataValidationRule>,
    /// A pivot table anchored at this cell.
    pub pivot_table: Option<PivotTable>,
    /// A data source table anchored at this cell.
    pub data_source_table: Option<DataSourceTable>,
    /// A data source formula.
    pub data_source_formula: Option<DataSourceFormula>,
    /// Runs of rich text and semantic chips.
    pub chip_runs: Option<Vec<ChipRun>>,
}

/// The format of a cell.
/// Cell formatting includes number formatting, background color, borders, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellFormat {
    /// The number format of the cell.
    pub number_format: Option<NumberFormat>,
    /// The background color of the cell.
    pub background_color: Option<Color>,
    /// The background color of the cell using theme colors.
    pub background_color_style: Option<ColorStyle>,
    /// The borders of the cell.
    pub borders: Option<Borders>,
    /// The padding of the cell.
    pub padding: Option<Padding>,
    /// The horizontal alignment of the cell.
    pub horizontal_alignment: Option<HorizontalAlign>,
    /// The vertical alignment of the cell.
    pub vertical_alignment: Option<VerticalAlign>,
    /// How to wrap text in the cell.
    pub wrap_strategy: Option<WrapStrategy>,
    /// The direction of the text in the cell.
    pub text_direction: Option<TextDirection>,
    /// The format of the text in the cell.
    pub text_format: Option<TextFormat>,
    /// How a hyperlink, if it exists, should be displayed in the cell.
    pub hyperlink_display_type: Option<HyperlinkDisplayType>,
    /// The rotation applied to text in the cell.
    pub text_rotation: Option<TextRotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub address: String,
    pub sheet_id: String,
    pub sheet_title: String,
    pub value: Option<String>,
    pub col_index: usize,
    pub col: String,
    pub row_index: usize,
}
