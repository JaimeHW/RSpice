//! PWL (Piecewise Linear) Editor Module
//!
//! Commercial-grade graphical editor for PWL time-value pairs.
//! Matches Cadence Spectre's PWL source editing capabilities.

mod data;
mod render;
mod state;

pub use data::{PwlData, PwlPoint, PwlValidationError};
pub use render::{PwlEditorResult, render_pwl_editor};
pub use state::PwlEditorState;
