use super::common::Editors;
use super::grid::GridRange;
use serde::{Deserialize, Serialize};

/// A named range.
/// Named ranges are ranges that have associated names.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRange {
    /// The ID of the named range.
    pub named_range_id: Option<String>,
    /// The name of the named range.
    pub name: Option<String>,
    /// The range this represents.
    pub range: Option<GridRange>,
}

/// A protected range.
/// Protected ranges restrict editing to specific users or groups.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtectedRange {
    /// The ID of the protected range.
    pub protected_range_id: Option<i32>,
    /// The range that is protected.
    pub range: Option<GridRange>,
    /// The ID of the named range the protected range is backed by, if any.
    pub named_range_id: Option<String>,
    /// The ID of the table the protected range is backed by, if any.
    pub table_id: Option<String>,
    /// The description of the protected range.
    pub description: Option<String>,
    /// True if the protected range is only a warning and not enforced.
    pub warning_only: Option<bool>,
    /// True if the user who requested this protected range can edit the protected area.
    pub requesting_user_can_edit: Option<bool>,
    /// The list of unprotected ranges within the protected range.
    pub unprotected_ranges: Option<Vec<GridRange>>,
    /// The users and groups with edit access to the protected range.
    pub editors: Option<Editors>,
}
