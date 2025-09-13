use super::common::*;
use super::data_source::DataSourceColumnReference;
use super::filters::{FilterSpec, SortSpec};
use super::formatting::{HorizontalAlign, TextFormat};
use super::grid::GridRange;
use serde::{Deserialize, Serialize};

/// A chart embedded in a spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedChart {
    /// The ID of the chart.
    pub chart_id: Option<i32>,
    /// The specification of the chart.
    pub spec: Option<ChartSpec>,
    /// The position of the chart.
    pub position: Option<EmbeddedObjectPosition>,
    /// The border of the chart.
    pub border: Option<EmbeddedObjectBorder>,
}

/// The specifications of a chart.
/// This contains all the properties for a chart, including its type and data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartSpec {
    /// The title of the chart.
    pub title: Option<String>,
    /// The alternative text for the chart.
    pub alt_text: Option<String>,
    /// The text format of the chart title.
    pub title_text_format: Option<TextFormat>,
    /// The position of the chart title.
    pub title_text_position: Option<TextPosition>,
    /// The subtitle of the chart.
    pub subtitle: Option<String>,
    /// The text format of the chart subtitle.
    pub subtitle_text_format: Option<TextFormat>,
    /// The position of the chart subtitle.
    pub subtitle_text_position: Option<TextPosition>,
    /// The name of the font to use for the chart.
    pub font_name: Option<String>,
    /// True if the chart is maximized.
    pub maximized: Option<bool>,
    /// The background color of the chart.
    pub background_color: Option<Color>,
    /// The background color of the chart using theme colors.
    pub background_color_style: Option<ColorStyle>,
    /// Properties for a data source chart.
    pub data_source_chart_properties: Option<DataSourceChartProperties>,
    /// The filters applied to the chart.
    pub filter_specs: Option<Vec<FilterSpec>>,
    /// The sorting applied to the chart.
    pub sort_specs: Option<Vec<SortSpec>>,
    /// Determines how to display dimensions that are hidden in the data source.
    pub hidden_dimension_strategy: Option<ChartHiddenDimensionStrategy>,
    /// A basic chart specification.
    pub basic_chart: Option<BasicChartSpec>,
    /// A pie chart specification.
    pub pie_chart: Option<PieChartSpec>,
    /// A bubble chart specification.
    pub bubble_chart: Option<BubbleChartSpec>,
    /// A candlestick chart specification.
    pub candlestick_chart: Option<CandlestickChartSpec>,
    /// An organizational chart specification.
    pub org_chart: Option<OrgChartSpec>,
    /// A histogram chart specification.
    pub histogram_chart: Option<HistogramChartSpec>,
    /// A waterfall chart specification.
    pub waterfall_chart: Option<WaterfallChartSpec>,
    /// A treemap chart specification.
    pub treemap_chart: Option<TreemapChartSpec>,
    /// A scorecard chart specification.
    pub scorecard_chart: Option<ScorecardChartSpec>,
}

/// The position of text within a chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextPosition {
    /// The horizontal alignment of the text.
    pub horizontal_alignment: Option<HorizontalAlign>,
}

/// Properties for a chart that uses a data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceChartProperties {
    /// The ID of the data source that the chart is associated with.
    pub data_source_id: Option<String>,
    /// The status of the data execution for the chart.
    pub data_execution_status: Option<DataExecutionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartSpec {
    pub chart_type: Option<BasicChartType>,
    pub legend_position: Option<BasicChartLegendPosition>,
    pub axis: Option<Vec<BasicChartAxis>>,
    pub domains: Option<Vec<BasicChartDomain>>,
    pub series: Option<Vec<BasicChartSeries>>,
    pub header_count: Option<i32>,
    pub three_dimensional: Option<bool>,
    pub interpolate_nulls: Option<bool>,
    pub stacked_type: Option<BasicChartStackedType>,
    pub line_smoothing: Option<bool>,
    pub compare_mode: Option<BasicChartCompareMode>,
    pub total_data_label: Option<DataLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartType {
    Unspecified,
    Bar,
    Line,
    Area,
    Column,
    Scatter,
    Combo,
    SteppedArea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartLegendPosition {
    Unspecified,
    BottomLegend,
    LeftLegend,
    RightLegend,
    TopLegend,
    NoLegend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartAxis {
    pub position: Option<BasicChartAxisPosition>,
    pub title: Option<String>,
    pub format: Option<TextFormat>,
    pub title_text_position: Option<TextPosition>,
    pub view_window_options: Option<ChartAxisViewWindowOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartAxisPosition {
    Unspecified,
    BottomAxis,
    LeftAxis,
    RightAxis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartAxisViewWindowOptions {
    pub view_window_min: Option<f64>,
    pub view_window_max: Option<f64>,
    pub view_window_mode: Option<ViewWindowMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ViewWindowMode {
    DefaultViewWindowMode,
    ViewWindowModeUnsupported,
    Explicit,
    Pretty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartDomain {
    pub domain: Option<ChartData>,
    pub reversed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    pub group_rule: Option<ChartGroupRule>,
    pub aggregate_type: Option<ChartAggregateType>,
    pub source_range: Option<ChartSourceRange>,
    pub column_reference: Option<DataSourceColumnReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartSourceRange {
    pub sources: Option<Vec<GridRange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartGroupRule {
    pub date_time_rule: Option<ChartDateTimeRule>,
    pub histogram_rule: Option<ChartHistogramRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartDateTimeRule {
    pub type_: Option<ChartDateTimeRuleType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartDateTimeRuleType {
    Unspecified,
    Second,
    Minute,
    Hour,
    HourMinute,
    HourMinuteAmpm,
    DayOfWeek,
    DayOfYear,
    DayOfMonth,
    DayMonth,
    Month,
    Quarter,
    Year,
    YearMonth,
    YearQuarter,
    YearMonthDay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartHistogramRule {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub interval_size: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartAggregateType {
    Unspecified,
    Average,
    Count,
    Max,
    Median,
    Min,
    Sum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicChartSeries {
    pub series: Option<ChartData>,
    pub target_axis: Option<BasicChartAxisPosition>,
    pub type_: Option<BasicChartType>,
    pub line_style: Option<LineStyle>,
    pub data_label: Option<DataLabel>,
    pub color: Option<Color>,
    pub color_style: Option<ColorStyle>,
    pub point_style: Option<PointStyle>,
    pub style_overrides: Option<Vec<BasicSeriesDataPointStyleOverride>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    pub width: Option<i32>,
    pub type_: Option<LineDashType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LineDashType {
    Unspecified,
    Invisible,
    Custom,
    Solid,
    Dotted,
    MediumDashed,
    MediumDashedDotted,
    LongDashed,
    LongDashedDotted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataLabel {
    pub type_: Option<DataLabelType>,
    pub text_format: Option<TextFormat>,
    pub placement: Option<DataLabelPlacement>,
    pub custom_label_data: Option<ChartData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataLabelType {
    Unspecified,
    None,
    Data,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataLabelPlacement {
    Unspecified,
    Center,
    Left,
    Right,
    Above,
    Below,
    InsideEnd,
    InsideBase,
    OutsideEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointStyle {
    pub size: Option<f64>,
    pub shape: Option<PointShape>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PointShape {
    Unspecified,
    Circle,
    Diamond,
    Hexagon,
    Pentagon,
    Square,
    Star,
    Triangle,
    XMark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicSeriesDataPointStyleOverride {
    pub index: Option<i32>,
    pub color: Option<Color>,
    pub color_style: Option<ColorStyle>,
    pub point_style: Option<PointStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartStackedType {
    Unspecified,
    NotStacked,
    Stacked,
    PercentStacked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasicChartCompareMode {
    Unspecified,
    Datum,
    Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PieChartSpec {
    pub legend_position: Option<PieChartLegendPosition>,
    pub domain: Option<ChartData>,
    pub series: Option<ChartData>,
    pub three_dimensional: Option<bool>,
    pub pie_hole: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PieChartLegendPosition {
    Unspecified,
    BottomLegend,
    LeftLegend,
    RightLegend,
    TopLegend,
    NoLegend,
    LabeledLegend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BubbleChartSpec {
    pub legend_position: Option<BubbleChartLegendPosition>,
    pub bubble_labels: Option<ChartData>,
    pub domain: Option<ChartData>,
    pub series: Option<ChartData>,
    pub group_ids: Option<ChartData>,
    pub bubble_sizes: Option<ChartData>,
    pub bubble_opacity: Option<f64>,
    pub bubble_border_color: Option<Color>,
    pub bubble_border_color_style: Option<ColorStyle>,
    pub bubble_max_radius_size: Option<i32>,
    pub bubble_min_radius_size: Option<i32>,
    pub bubble_text_style: Option<TextFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BubbleChartLegendPosition {
    Unspecified,
    BottomLegend,
    LeftLegend,
    RightLegend,
    TopLegend,
    NoLegend,
    InsideLegend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickChartSpec {
    pub domain: Option<CandlestickDomain>,
    pub data: Option<Vec<CandlestickData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickDomain {
    pub data: Option<ChartData>,
    pub reversed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickData {
    pub low_series: Option<CandlestickSeries>,
    pub open_series: Option<CandlestickSeries>,
    pub close_series: Option<CandlestickSeries>,
    pub high_series: Option<CandlestickSeries>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickSeries {
    pub data: Option<ChartData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgChartSpec {
    pub node_size: Option<OrgChartNodeSize>,
    pub node_color: Option<Color>,
    pub node_color_style: Option<ColorStyle>,
    pub selected_node_color: Option<Color>,
    pub selected_node_color_style: Option<ColorStyle>,
    pub labels: Option<ChartData>,
    pub parent_labels: Option<ChartData>,
    pub tooltips: Option<ChartData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrgChartNodeSize {
    Unspecified,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistogramChartSpec {
    pub series: Option<Vec<HistogramSeries>>,
    pub legend_position: Option<HistogramChartLegendPosition>,
    pub show_item_dividers: Option<bool>,
    pub bucket_size: Option<f64>,
    pub outlier_percentile: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistogramSeries {
    pub bar_color: Option<Color>,
    pub bar_color_style: Option<ColorStyle>,
    pub data: Option<ChartData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HistogramChartLegendPosition {
    Unspecified,
    BottomLegend,
    LeftLegend,
    RightLegend,
    TopLegend,
    NoLegend,
    InsideLegend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaterfallChartSpec {
    pub domain: Option<WaterfallChartDomain>,
    pub series: Option<Vec<WaterfallChartSeries>>,
    pub stacked_type: Option<WaterfallChartStackedType>,
    pub first_value_is_total: Option<bool>,
    pub hide_connector_lines: Option<bool>,
    pub connector_line_style: Option<LineStyle>,
    pub total_data_label: Option<DataLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaterfallChartDomain {
    pub data: Option<ChartData>,
    pub reversed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaterfallChartSeries {
    pub data: Option<ChartData>,
    pub positive_columns_style: Option<WaterfallChartColumnStyle>,
    pub negative_columns_style: Option<WaterfallChartColumnStyle>,
    pub subtotal_columns_style: Option<WaterfallChartColumnStyle>,
    pub hide_trailing_subtotal: Option<bool>,
    pub custom_subtotals: Option<Vec<WaterfallChartCustomSubtotal>>,
    pub data_label: Option<DataLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaterfallChartColumnStyle {
    pub label: Option<String>,
    pub color: Option<Color>,
    pub color_style: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaterfallChartCustomSubtotal {
    pub subtotal_index: Option<i32>,
    pub label: Option<String>,
    pub data_is_subtotal: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaterfallChartStackedType {
    Unspecified,
    Stacked,
    Sequential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreemapChartSpec {
    pub labels: Option<ChartData>,
    pub parent_labels: Option<ChartData>,
    pub size_data: Option<ChartData>,
    pub color_data: Option<ChartData>,
    pub text_format: Option<TextFormat>,
    pub levels: Option<i32>,
    pub hinted_levels: Option<i32>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub header_color: Option<Color>,
    pub header_color_style: Option<ColorStyle>,
    pub color_scale: Option<TreemapChartColorScale>,
    pub hide_tooltips: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreemapChartColorScale {
    pub min_value_color: Option<Color>,
    pub min_value_color_style: Option<ColorStyle>,
    pub mid_value_color: Option<Color>,
    pub mid_value_color_style: Option<ColorStyle>,
    pub max_value_color: Option<Color>,
    pub max_value_color_style: Option<ColorStyle>,
    pub no_data_color: Option<Color>,
    pub no_data_color_style: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScorecardChartSpec {
    pub key_value_data: Option<ChartData>,
    pub baseline_value_data: Option<ChartData>,
    pub aggregate_type: Option<ChartAggregateType>,
    pub key_value_format: Option<KeyValueFormat>,
    pub baseline_value_format: Option<BaselineValueFormat>,
    pub scale_factor: Option<f64>,
    pub number_format_source: Option<ChartNumberFormatSource>,
    pub custom_format_options: Option<ChartCustomNumberFormatOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValueFormat {
    pub text_format: Option<TextFormat>,
    pub position: Option<TextPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineValueFormat {
    pub comparison_type: Option<ComparisonType>,
    pub text_format: Option<TextFormat>,
    pub position: Option<TextPosition>,
    pub description: Option<String>,
    pub positive_color: Option<Color>,
    pub positive_color_style: Option<ColorStyle>,
    pub negative_color: Option<Color>,
    pub negative_color_style: Option<ColorStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComparisonType {
    Undefined,
    AbsoluteDifference,
    PercentageDifference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartNumberFormatSource {
    Undefined,
    FromData,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartCustomNumberFormatOptions {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartHiddenDimensionStrategy {
    Unspecified,
    SkipHiddenRowsAndColumns,
    SkipHiddenRows,
    SkipHiddenColumns,
    ShowAll,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedObjectBorder {
    pub color: Option<Color>,
    pub color_style: Option<ColorStyle>,
}
