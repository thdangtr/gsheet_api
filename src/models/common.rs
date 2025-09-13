use serde::{Deserialize, Serialize};

/// The kinds of value that a cell in a spreadsheet can have.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedValue {
    /// Represents a double value. Note: Dates, Times and DateTimes are represented as doubles in SERIAL_NUMBER format.
    pub number_value: Option<f64>,
    /// Represents a string value. Leading single quotes are not included.
    pub string_value: Option<String>,
    /// Represents a boolean value.
    pub bool_value: Option<bool>,
    /// Represents a formula.
    pub formula_value: Option<String>,
    /// Represents an error. This field is read-only.
    pub error_value: Option<ErrorValue>,
}

/// An error in a cell.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorValue {
    /// The type of error.
    #[serde(rename = "type")]
    pub type_: Option<ErrorType>,
    /// A message with more information about the error (in the spreadsheet's locale).
    pub message: Option<String>,
}

/// The type of error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorType {
    /// Default value, do not use.
    Unspecified,
    /// Corresponds to the #ERROR! error.
    Error,
    /// Corresponds to the #NULL! error.
    NullValue,
    /// Corresponds to the #DIV/0 error.
    DivideByZero,
    /// Corresponds to the #VALUE! error.
    Value,
    /// Corresponds to the #REF! error.
    Ref,
    /// Corresponds to the #NAME? error.
    Name,
    /// Corresponds to the #NUM! error.
    Num,
    /// Corresponds to the #N/A error.
    NA,
    /// Corresponds to the #LOADING! error.
    Loading,
}

/// Developer metadata associated with a location or object in a spreadsheet.
/// Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperMetadata {
    /// The spreadsheet-scoped unique ID that identifies the metadata.
    pub metadata_id: Option<i32>,
    /// The metadata key. There may be multiple metadata in a spreadsheet with the same key.
    pub metadata_key: Option<String>,
    /// Data associated with the metadata's key.
    pub metadata_value: Option<String>,
    /// The location where the metadata is associated.
    pub location: Option<DeveloperMetadataLocation>,
    /// The visibility of the metadata.
    pub visibility: Option<DeveloperMetadataVisibility>,
}

/// A location where metadata may be associated in a spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperMetadataLocation {
    /// The type of location.
    pub location_type: Option<DeveloperMetadataLocationType>,
    /// True if the location is a spreadsheet, false otherwise.
    pub spreadsheet: Option<bool>,
    /// The ID of the sheet when the location applies to a sheet.
    pub sheet_id: Option<i32>,
    /// The dimension range when the location applies to a range of cells.
    pub dimension_range: Option<DimensionRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeveloperMetadataLocationType {
    /// Default value, do not use.
    Unspecified,
    /// Developer metadata associated on an entire row dimension.
    Row,
    /// Developer metadata associated on an entire column dimension.
    Column,
    /// Developer metadata associated on an entire sheet.
    Sheet,
    /// Developer metadata associated on the entire spreadsheet.
    Spreadsheet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeveloperMetadataVisibility {
    /// Default value.
    Unspecified,
    /// Document-visible metadata is accessible from any developer project with access to the document.
    Document,
    /// Project-visible metadata is only visible to and accessible by the developer project that created the metadata.
    Project,
}

/// A range along a single dimension on a sheet.
/// All indexes are zero-based.
/// Indexes are half open: the start index is inclusive and the end index is exclusive.
/// Missing indexes indicate the range is unbounded on that side.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionRange {
    /// The sheet this dimension range is on.
    pub sheet_id: Option<i32>,
    /// The dimension of the span.
    pub dimension: Option<String>,
    /// The start (inclusive) of the span, or not set if unbounded.
    pub start_index: Option<i32>,
    /// The end (exclusive) of the span, or not set if unbounded.
    pub end_index: Option<i32>,
}

/// Properties about a dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionProperties {
    /// True if this dimension is being filtered. This field is read-only.
    pub hidden_by_filter: Option<bool>,
    /// True if this dimension is explicitly hidden.
    pub hidden_by_user: Option<bool>,
    /// The height (if a row) or width (if a column) of the dimension in pixels.
    pub pixel_size: Option<i32>,
    /// The developer metadata associated with a single dimension.
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// The data source column reference.
    pub data_source_column_reference: Option<super::data_source::DataSourceColumnReference>,
}

/// Data about each cell in a row.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowData {
    /// The values in the row, one per column.
    pub values: Option<Vec<crate::models::cell::CellData>>,
}

/// The editors of a protected range.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Editors {
    /// The email addresses of users with edit access to the protected range.
    pub users: Option<Vec<String>>,
    /// The email addresses of groups with edit access to the protected range.
    pub groups: Option<Vec<String>>,
    /// True if anyone in the document's domain has edit access to the protected range.
    pub domain_users_can_edit: Option<bool>,
}

/// The position of an embedded object such as a chart.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedObjectPosition {
    /// The sheet this is on.
    pub sheet_id: Option<i32>,
    /// The position at which the object is overlaid on top of a grid.
    pub overlay_position: Option<OverlayPosition>,
    /// If true, the embedded object is put on a new sheet whose ID is chosen for you.
    pub new_sheet: Option<bool>,
}

/// The position of an embedded object within a sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayPosition {
    /// The cell the object is anchored to.
    pub anchor_cell: Option<GridCoordinate>,
    /// The horizontal offset, in pixels, that the object is offset from the anchor cell.
    pub offset_x_pixels: Option<i32>,
    /// The vertical offset, in pixels, that the object is offset from the anchor cell.
    pub offset_y_pixels: Option<i32>,
    /// The width of the object, in pixels.
    pub width_pixels: Option<i32>,
    /// The height of the object, in pixels.
    pub height_pixels: Option<i32>,
}

/// A coordinate in a sheet.
/// All indexes are zero-based.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridCoordinate {
    /// The sheet this coordinate is on.
    pub sheet_id: Option<i32>,
    /// The row index of the coordinate.
    pub row_index: Option<i32>,
    /// The column index of the coordinate.
    pub column_index: Option<i32>,
}

/// The data execution status.
/// Used by Sheets API data source objects to indicate status of data execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataExecutionStatus {
    /// The state of the data execution.
    pub state: Option<DataExecutionState>,
    /// The error code.
    pub error_code: Option<DataExecutionErrorCode>,
    /// The error message, which may be displayed to a user.
    pub error_message: Option<String>,
    /// The last time the data was refreshed, in RFC 3339 format.
    pub last_refresh_time: Option<String>,
}

/// The state of the data execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataExecutionState {
    /// Default value.
    Unspecified,
    /// The data execution has not started.
    NotStarted,
    /// The data execution is currently running.
    Running,
    /// The data execution is currently cancelling.
    Cancelling,
    /// The data execution has completed successfully.
    Succeeded,
    /// The data execution has completed with errors.
    Failed,
}

/// Error code for data execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataExecutionErrorCode {
    /// Default value.
    Unspecified,
    /// The data execution timed out.
    TimedOut,
    /// The data execution returned more rows than allowed.
    TooManyRows,
    /// The data execution returned more columns than allowed.
    TooManyColumns,
    /// The data execution returned more cells than allowed.
    TooManyCells,
    /// An error occurred in the data execution engine.
    Engine,
    /// The data execution request contained an invalid parameter.
    ParameterInvalid,
    /// The data execution request contained an unsupported data type.
    UnsupportedDataType,
    /// The data execution request contained duplicate column names.
    DuplicateColumnNames,
    /// The data execution was interrupted.
    Interrupted,
    /// The data execution failed due to a concurrent query.
    ConcurrentQuery,
    /// An unspecified error occurred.
    Other,
    /// The data execution returned a cell with too many characters.
    TooManyCharsPerCell,
    /// The requested data was not found.
    DataNotFound,
    /// The user does not have permission to access the requested data.
    PermissionDenied,
    /// The data execution request is missing a column alias.
    MissingColumnAlias,
    /// The requested object was not found.
    ObjectNotFound,
    /// The requested object is in an error state.
    ObjectInErrorState,
    /// The data execution request contained an invalid object specification.
    ObjectSpecInvalid,
    /// The data execution was cancelled.
    DataExecutionCancelled,
}

/// A pair mapping a theme color type to the concrete color it represents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColorPair {
    /// The type of the theme color.
    pub color_type: Option<ThemeColorType>,
    /// The concrete color corresponding to the theme color type.
    pub color: Option<ColorStyle>,
}

/// Settings for iterative calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IterativeCalculationSettings {
    /// When iterative calculation is enabled, the maximum number of calculation rounds to perform.
    pub max_iterations: Option<i32>,
    /// When iterative calculation is enabled and successive results differ by less than this threshold value, the calculation rounds stop.
    pub convergence_threshold: Option<f64>,
}

/// How often to recalculate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecalculationInterval {
    /// Default value. This value must not be used.
    Unspecified,
    /// Volatile functions are updated on every change.
    OnChange,
    /// Volatile functions are updated on every change and every minute.
    Minute,
    /// Volatile functions are updated on every change and hourly.
    Hour,
}

/// A run of rich text and associated semantic chips.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChipRun {
    /// The zero-based character index where this run starts.
    pub start_index: Option<i32>,
    /// The chip associated with this run.
    pub chip: Option<Chip>,
}

/// A chip is a UI element that represents a person or a rich link.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chip {
    /// Properties of a person chip.
    pub person_properties: Option<PersonProperties>,
    /// Properties of a rich link chip.
    pub rich_link_properties: Option<RichLinkProperties>,
}

/// Properties of a person chip.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonProperties {
    /// The email address of the person.
    pub email: Option<String>,
    /// The display format of the person.
    pub display_format: Option<super::formatting::DisplayFormat>,
}

/// Properties of a rich link chip.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichLinkProperties {
    /// The URI of the rich link.
    pub uri: Option<String>,
    /// The MIME type of the rich link.
    pub mime_type: Option<String>,
}

/// Represents a color in the RGBA color space.
/// This representation is designed for simplicity of conversion to/from color representations in various languages over compactness.
/// For example, the fields of this representation can be trivially provided to the constructor of "java.awt.Color" in Java; it can also be trivially provided to UIColor's "+colorWithRed:green:blue:alpha" method in iOS; and, with just a little work, it can be easily formatted into a CSS "rgba()" string in JavaScript.
/// This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space.
/// When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue and alpha values each differ by at most 1e-5.
/// Example (Java):
/// import com.google.type.Color;
/// // ...
/// public static java.awt.Color fromProto(Color protocolor) {
///   float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0;
///   return new java.awt.Color(
///       protocolor.getRed(),
///       protocolor.getGreen(),
///       protocolor.getBlue(),
///       alpha);
/// }
/// public static Color toProto(java.awt.Color color) {
///   float red = (float) color.getRed();
///   float green = (float) color.getGreen();
///   float blue = (float) color.getBlue();
///   float denominator = 255.0;
///   Color.Builder resultBuilder = Color.newBuilder()
///       .setRed(red / denominator)
///       .setGreen(green / denominator)
///       .setBlue(blue / denominator);
///   int alpha = color.getAlpha();
///   if (alpha != 255) {
///     result.setAlpha(FloatValue.newBuilder().setValue(((float) alpha) / denominator).build());
///   }
///   return resultBuilder.build();
/// }
/// // ...
/// Example (iOS / Obj-C):
/// // ...
/// static UIColor* fromProto(Color* protocolor) {
///    float red = [protocolor red];
///    float green = [protocolor green];
///    float blue = [protocolor blue];
///    FloatValue* alpha_wrapper = [protocolor alpha];
///    float alpha = 1.0;
///    if (alpha_wrapper != nil) {
///      alpha = [alpha_wrapper value];
///    }
///    return [UIColor colorWithRed:red green:green blue:blue alpha:alpha];
/// }
/// static Color* toProto(UIColor* color) {
///     CGFloat red, green, blue, alpha;
///     if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) {
///       return nil;
///     }
///     Color* result = [[Color alloc] init];
///     [result setRed:red];
///     [result setGreen:green];
///     [result setBlue:blue];
///     if (alpha <= 0.9999) {
///         [result setAlpha:floatWrapperWithValue(alpha)];
///     }
///     [result setRed:red];
///     [result setGreen:green];
///     [result setBlue:blue];
///     [result setAlpha:floatWrapperWithValue(alpha)];
///     return result;
/// }
/// // ...
///  Example (JavaScript):
/// // ...
/// var protoToCssColor = function(rgb_color) {
///    var redFrac = rgb_color.red || 0.0;
///    var greenFrac = rgb_color.green || 0.0;
///    var blueFrac = rgb_color.blue || 0.0;
///    var red = Math.floor(redFrac * 255);
///    var green = Math.floor(greenFrac * 255);
///    var blue = Math.floor(blueFrac * 255);
///    if (!('alpha' in rgb_color)) {
///       return rgbToCssColor(red, green, blue);
///    }
///    var alphaFrac = rgb_color.alpha.value || 0.0;
///    var rgbParams = [red, green, blue].join(',');
///    return ['rgba(', rgbParams, ',', alphaFrac, ')'].join('');
/// };
/// var rgbToCssColor = function(red, green, blue) {
///   var rgbNumber = new Number((red << 16) | (green << 8) | blue);
///   var hexString = rgbNumber.toString(16);
///   var missingZeros = 6 - hexString.length;
///   var resultBuilder = ['#'];
///   for (var i = 0; i < missingZeros; i++) {
///      resultBuilder.push('0');
///   }
///   resultBuilder.push(hexString);
///   return resultBuilder.join('');
/// };
/// // ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    /// The amount of red in the color as a value in the interval [0, 1].
    pub red: Option<f64>,
    /// The amount of green in the color as a value in the interval [0, 1].
    pub green: Option<f64>,
    /// The amount of blue in the color as a value in the interval [0, 1].
    pub blue: Option<f64>,
    /// The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation:
    /// pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
    /// This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is to be rendered as a solid color (as if the alpha value had been explicitly given with a value of 1.0).
    pub alpha: Option<f64>,
}

/// A color value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorStyle {
    /// RGB color.
    pub rgb_color: Option<Color>,
    /// Theme color.
    pub theme_color: Option<ThemeColorType>,
}

/// Theme color types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThemeColorType {
    /// Unspecified theme color.
    Unspecified,
    /// Represents the primary text color.
    Text,
    /// Represents the primary background color.
    Background,
    /// Represents the first accent color.
    Accent1,
    /// Represents the second accent color.
    Accent2,
    /// Represents the third accent color.
    Accent3,
    /// Represents the fourth accent color.
    Accent4,
    /// Represents the fifth accent color.
    Accent5,
    /// Represents the sixth accent color.
    Accent6,
    /// Represents the hyperlink color.
    Link,
}

/// A placeholder for pivot table functionality.
/// This is not yet implemented.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotTable {
    // Placeholder
}
