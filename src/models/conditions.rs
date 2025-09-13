use serde::{Deserialize, Serialize};

use super::cell::CellFormat;
use super::common::{Color, ColorStyle};
use super::grid::GridRange;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalFormatRule {
    pub ranges: Option<Vec<GridRange>>,
    pub boolean_rule: Option<BooleanRule>,
    pub gradient_rule: Option<GradientRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanRule {
    pub condition: Option<BooleanCondition>,
    pub format: Option<CellFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradientRule {
    pub minpoint: Option<InterpolationPoint>,
    pub midpoint: Option<InterpolationPoint>,
    pub maxpoint: Option<InterpolationPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanCondition {
    #[serde(rename = "type")]
    pub type_: Option<ConditionType>,
    pub values: Option<Vec<ConditionValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionType {
    Unspecified,
    NumberGreater,
    NumberGreaterThanEq,
    NumberLess,
    NumberLessThanEq,
    NumberEq,
    NumberNotEq,
    NumberBetween,
    NumberNotBetween,
    TextContains,
    TextNotContains,
    TextStartsWith,
    TextEndsWith,
    TextEq,
    TextIsEmail,
    TextIsUrl,
    DateEq,
    DateBefore,
    DateAfter,
    DateOnOrBefore,
    DateOnOrAfter,
    DateBetween,
    DateNotBetween,
    DateIsValid,
    OneOfRange,
    OneOfList,
    Blank,
    NotBlank,
    CustomFormula,
    Boolean,
    TextNotEq,
    DateNotEq,
    FilterExpression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionValue {
    pub relative_date: Option<RelativeDate>,
    pub user_entered_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelativeDate {
    Unspecified,
    PastYear,
    PastMonth,
    PastWeek,
    Yesterday,
    Today,
    Tomorrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterpolationPoint {
    pub color: Option<Color>,
    pub color_style: Option<ColorStyle>,
    #[serde(rename = "type")]
    pub type_: Option<InterpolationPointType>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterpolationPointType {
    Unspecified,
    Min,
    Max,
    Number,
    Percent,
    Percentile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataValidationRule {
    pub condition: Option<BooleanCondition>,
    pub input_message: Option<String>,
    pub strict: Option<bool>,
    pub show_custom_ui: Option<bool>,
}
