//! # Models Module
//!
//! This module contains all the data models and structures used to represent
//! Google Sheets API responses and requests. These models closely mirror the
//! Google Sheets API JSON structures for type safety and ease of use.
//!
//! ## Organization
//!
//! The models are organized into logical groups:
//!
//! - **Core Structures**: [`Spreadsheet`], [`Sheet`], [`Cell`] - Main entities
//! - **Data Types**: [`ValueRange`], [`GridRange`] - Data containers
//! - **Formatting**: [`CellFormat`], [`TextFormat`] - Cell appearance
//! - **Conditions**: [`ConditionalFormatRule`], [`BooleanCondition`] - Conditional formatting
//! - **Charts**: [`EmbeddedChart`], [`ChartSpec`] - Chart representations
//! - **Filters**: [`FilterView`], [`BasicFilter`] - Data filtering
//! - **Common**: [`Color`], [`ColorStyle`] - Shared types
//!
//! ## Usage
//!
//! Most models derive `Serialize` and `Deserialize` for JSON conversion,
//! `Debug` for debugging, and `Clone` for easy duplication. They are designed
//! to be used directly with the `reqwest` HTTP client for API communication.
//!
//! ## Type Safety
//!
//! The models use strong typing with enums for restricted values (like
//! [`SheetType`], [`ValueRenderOption`]) to prevent invalid API requests
//! and provide better IDE support and compile-time validation.

pub mod cell;
pub mod charts;
pub mod common;
pub mod conditions;
pub mod data_source;
pub mod filters;
pub mod formatting;
pub mod grid;
pub mod range;
pub mod sheet;
pub mod spreadsheet;
pub mod value;

// Re-export for convenience
pub use cell::*;
pub use charts::*;
pub use common::*;
pub use conditions::*;
pub use data_source::*;
pub use filters::*;
pub use formatting::*;
pub use grid::*;
pub use range::*;
pub use sheet::*;
pub use spreadsheet::*;
pub use value::*;
