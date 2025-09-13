use serde::{Deserialize, Serialize};

use super::common::DataExecutionStatus;
use super::filters::{FilterSpec, SortSpec};
use super::grid::GridRange;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    pub data_source_id: Option<String>,
    pub spec: Option<DataSourceSpec>,
    pub calculated_columns: Option<Vec<DataSourceColumn>>,
    pub sheet_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookerDataSourceSpec {
    pub instance_uri: Option<String>,
    pub model: Option<String>,
    pub explore: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceSpec {
    pub parameters: Option<Vec<DataSourceParameter>>,
    pub big_query: Option<BigQueryDataSourceSpec>,
    pub looker: Option<LookerDataSourceSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceParameter {
    pub name: Option<String>,
    pub named_range_id: Option<String>,
    pub range: Option<GridRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceColumn {
    pub reference: Option<DataSourceColumnReference>,
    pub formula: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshSchedule {
    pub enabled: Option<bool>,
    pub refresh_scope: Option<DataSourceRefreshScope>,
    pub next_run: Option<Interval>,
    pub daily_schedule: Option<DataSourceRefreshDailySchedule>,
    pub weekly_schedule: Option<DataSourceRefreshWeeklySchedule>,
    pub monthly_schedule: Option<DataSourceRefreshMonthlySchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSourceRefreshScope {
    Unspecified,
    AllDataSources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshDailySchedule {
    pub start_time: Option<TimeOfDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshWeeklySchedule {
    pub start_time: Option<TimeOfDay>,
    pub days_of_week: Option<Vec<DayOfWeek>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    Unspecified,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshMonthlySchedule {
    pub start_time: Option<TimeOfDay>,
    pub days_of_month: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeOfDay {
    pub hours: Option<i32>,
    pub minutes: Option<i32>,
    pub seconds: Option<i32>,
    pub nanos: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceColumnReference {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryDataSourceSpec {
    pub project_id: Option<String>,
    pub query_spec: Option<BigQueryQuerySpec>,
    pub table_spec: Option<BigQueryTableSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryQuerySpec {
    pub raw_query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryTableSpec {
    pub table_project_id: Option<String>,
    pub table_id: Option<String>,
    pub dataset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSourceTableColumnSelectionType {
    Unspecified,
    Selected,
    SyncAll,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceTable {
    pub data_source_id: Option<String>,
    pub column_selection_type: Option<DataSourceTableColumnSelectionType>,
    pub columns: Option<Vec<DataSourceColumnReference>>,
    pub filter_specs: Option<Vec<FilterSpec>>,
    pub sort_specs: Option<Vec<SortSpec>>,
    pub row_limit: Option<i32>,
    pub data_execution_status: Option<DataExecutionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceFormula {
    pub data_source_id: Option<String>,
    pub data_execution_status: Option<DataExecutionStatus>,
}
