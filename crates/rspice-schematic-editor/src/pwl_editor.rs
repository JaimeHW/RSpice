//! PWL (Piecewise Linear) Editor Module
//!
//! Retained point drafts and table controls shared by source-property surfaces.

mod data;
mod render;
mod state;

pub use data::PwlData;
pub use render::{PwlEditorResult, render_pwl_editor};
pub use state::PwlEditorState;
