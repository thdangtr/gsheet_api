use super::common::{Color, ColorStyle};
use super::conditions::BooleanCondition;
use super::data_source::DataSourceColumnReference;
use super::grid::GridRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterView {
    pub filter_view_id: Option<i32>,
    pub title: Option<String>,
    pub range: Option<GridRange>,
    pub named_range_id: Option<String>,
    pub table_id: Option<String>,
    pub sort_specs: Option<Vec<SortSpec>>,
    pub criteria: Option<std::collections::HashMap<String, FilterCriteria>>,
    pub filter_specs: Option<Vec<FilterSpec>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortSpec {
    pub sort_order: Option<SortOrder>,
    pub foreground_color: Option<Color>,
    pub foreground_color_style: Option<ColorStyle>,
    pub background_color: Option<Color>,
    pub background_color_style: Option<ColorStyle>,
    pub dimension_index: Option<i32>,
    pub data_source_column_reference: Option<DataSourceColumnReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCriteria {
    pub hidden_values: Option<Vec<String>>,
    pub condition: Option<BooleanCondition>,
    pub visible_background_color: Option<Color>,
    pub visible_background_color_style: Option<ColorStyle>,
    pub visible_foreground_color: Option<Color>,
    pub visible_foreground_color_style: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortOrder {
    Unspecified,
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterSpec {
    pub filter_criteria: Option<FilterCriteria>,
    pub column_index: Option<i32>,
    pub data_source_column_reference: Option<DataSourceColumnReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicFilter {
    pub range: Option<GridRange>,
    pub table_id: Option<String>,
    pub sort_specs: Option<Vec<SortSpec>>,
    pub criteria: Option<std::collections::HashMap<String, FilterCriteria>>,
    pub filter_specs: Option<Vec<FilterSpec>>,
}
