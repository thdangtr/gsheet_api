use super::charts::EmbeddedChart;
use super::common::{Color, ColorStyle, DeveloperMetadata};
use super::conditions::{BooleanCondition, ConditionalFormatRule};
use super::data_source::DataSourceColumn;
use super::filters::{BasicFilter, FilterCriteria, FilterView};
use super::formatting::{HorizontalAlign, TextFormat};
use super::grid::{GridData, GridProperties};
use super::range::ProtectedRange;
use serde::{Deserialize, Serialize};

/// Represents a single sheet within a Google Sheets spreadsheet.
/// A sheet contains data, formatting, charts, and other sheet-specific properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    /// The properties of the sheet.
    pub properties: Option<SheetProperties>,
    /// The data in the grid, if any.
    pub data: Option<Vec<GridData>>,
    /// The ranges that are merged together.
    pub merges: Option<Vec<super::grid::GridRange>>,
    /// The conditional format rules in the sheet.
    pub conditional_formats: Option<Vec<ConditionalFormatRule>>,
    /// The filter views in the sheet.
    pub filter_views: Option<Vec<FilterView>>,
    /// The protected ranges in the sheet.
    pub protected_ranges: Option<Vec<ProtectedRange>>,
    /// The basic filter applied to the sheet, if any.
    pub basic_filter: Option<BasicFilter>,
    /// The charts that are positioned on the sheet.
    pub charts: Option<Vec<EmbeddedChart>>,
    /// The banded (alternating colors) ranges on the sheet.
    pub banded_ranges: Option<Vec<BandedRange>>,
    /// The developer metadata associated with the sheet.
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// All row groups on the sheet.
    pub row_groups: Option<Vec<DimensionGroup>>,
    /// All column groups on the sheet.
    pub column_groups: Option<Vec<DimensionGroup>>,
    /// The slicers on the sheet.
    pub slicers: Option<Vec<Slicer>>,
    /// The tables on the sheet.
    pub tables: Option<Vec<Table>>,
}

/// Properties of a sheet, including its title, type, and visual properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetProperties {
    /// The ID of the sheet.
    pub sheet_id: Option<i32>,
    /// The name of the sheet.
    pub title: Option<String>,
    /// The index of the sheet within the spreadsheet (0-based).
    pub index: Option<i32>,
    /// The type of the sheet.
    pub sheet_type: Option<SheetType>,
    /// Additional properties of the sheet if this sheet is a grid.
    pub grid_properties: Option<GridProperties>,
    /// True if the sheet is hidden in the UI, false if it's visible.
    pub hidden: Option<bool>,
    /// The color of the sheet tab.
    pub tab_color: Option<Color>,
    /// The color of the sheet tab using theme colors.
    pub tab_color_style: Option<ColorStyle>,
    /// True if the sheet is an RTL (right-to-left) sheet instead of an LTR (left-to-right) sheet.
    pub right_to_left: Option<bool>,
    /// Properties of a data source sheet.
    pub data_source_sheet_properties: Option<DataSourceSheetProperties>,
}

/// The type of a sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SheetType {
    /// Default value, do not use.
    Unspecified,
    /// The sheet is a grid.
    Grid,
    /// The sheet has no grid and instead has an object like a chart.
    Object,
    /// The sheet connects to an external data source.
    DataSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceSheetProperties {
    /// The ID of the data source the sheet is connected to.
    pub data_source_id: Option<String>,
    /// The columns displayed on the sheet, corresponding to the data source.
    pub columns: Option<Vec<DataSourceColumn>>,
    /// The data execution status.
    pub data_execution_status: Option<super::common::DataExecutionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandedRange {
    /// The ID of the banded range.
    pub banded_range_id: Option<i32>,
    /// The name of the banded range.
    pub banded_range_reference: Option<String>,
    /// The range over which the banding is applied.
    pub range: Option<super::grid::GridRange>,
    /// Properties for row bands.
    pub row_properties: Option<BandingProperties>,
    /// Properties for column bands.
    pub column_properties: Option<BandingProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandingProperties {
    /// The color of the first row or column.
    pub header_color: Option<Color>,
    /// The color of the first row or column using theme colors.
    pub header_color_style: Option<ColorStyle>,
    /// The first band color.
    pub first_band_color: Option<Color>,
    /// The first band color using theme colors.
    pub first_band_color_style: Option<ColorStyle>,
    /// The second band color.
    pub second_band_color: Option<Color>,
    /// The second band color using theme colors.
    pub second_band_color_style: Option<ColorStyle>,
    /// The color of the last row or column.
    pub footer_color: Option<Color>,
    /// The color of the last row or column using theme colors.
    pub footer_color_style: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionGroup {
    /// The range over which this group exists.
    pub range: Option<super::common::DimensionRange>,
    /// The depth of the group, representing how many groups have a range that wholly contains this group's range.
    pub depth: Option<i32>,
    /// True if this group is collapsed.
    pub collapsed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slicer {
    /// The ID of the slicer.
    pub slicer_id: Option<i32>,
    /// The specification of the slicer.
    pub spec: Option<SlicerSpec>,
    /// The position of the slicer.
    pub position: Option<super::common::EmbeddedObjectPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlicerSpec {
    /// The data range that the slicer applies to.
    pub data_range: Option<super::grid::GridRange>,
    /// The filtering criteria for the slicer.
    pub filter_criteria: Option<FilterCriteria>,
    /// The column index in the data table on which the filter is applied.
    pub column_index: Option<i32>,
    /// True if the slicer should apply to pivot tables.
    pub apply_to_pivot_tables: Option<bool>,
    /// The title of the slicer.
    pub title: Option<String>,
    /// The text format of the slicer.
    pub text_format: Option<TextFormat>,
    /// The background color of the slicer.
    pub background_color: Option<Color>,
    /// The background color of the slicer using theme colors.
    pub background_color_style: Option<ColorStyle>,
    /// The horizontal alignment of the slicer.
    pub horizontal_alignment: Option<HorizontalAlign>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    /// The ID of the table.
    pub table_id: Option<String>,
    /// The name of the table.
    pub name: Option<String>,
    /// The range of the table.
    pub range: Option<super::grid::GridRange>,
    /// Properties of the table rows.
    pub rows_properties: Option<TableRowsProperties>,
    /// Properties of the table columns.
    pub column_properties: Option<Vec<TableColumnProperties>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowsProperties {
    /// The color of the header row.
    pub header_color_style: Option<ColorStyle>,
    /// The color of the first band of rows.
    pub first_band_color_style: Option<ColorStyle>,
    /// The color of the second band of rows.
    pub second_band_color_style: Option<ColorStyle>,
    /// The color of the footer row.
    pub footer_color_style: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableColumnProperties {
    /// The index of the column in the table.
    pub column_index: Option<i32>,
    /// The name of the column.
    pub column_name: Option<String>,
    /// The type of the column.
    pub column_type: Option<ColumnType>,
    /// The data validation rule for the column.
    pub data_validation_rule: Option<TableColumnDataValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnType {
    /// Default value, do not use.
    Unspecified,
    /// A column of numbers.
    Double,
    /// A column of currency values.
    Currency,
    /// A column of percentages.
    Percent,
    /// A column of dates.
    Date,
    /// A column of times.
    Time,
    /// A column of date-times.
    DateTime,
    /// A column of text.
    Text,
    /// A column of boolean values.
    Boolean,
    /// A column with a dropdown.
    Dropdown,
    /// A column with file chips.
    FilesChip,
    /// A column with people chips.
    PeopleChip,
    /// A column with finance chips.
    FinanceChip,
    /// A column with place chips.
    PlaceChip,
    /// A column with ratings chips.
    RatingsChip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableColumnDataValidationRule {
    /// The condition that data in the table column must meet.
    pub condition: Option<BooleanCondition>,
}
