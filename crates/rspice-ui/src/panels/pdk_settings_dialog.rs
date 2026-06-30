//! Model Library Paths Dialog
//!
//! Master–detail interface for configuring PDK (Process Design Kit) library
//! paths, environment variables, and model file discovery.
//!
//! ## Features
//!
//! - Source management (add/remove/enable/disable, inline path editing)
//! - Environment variable overrides next to the paths that use them
//! - Discovered-file table with type badges, scoped to the selected source
//! - Persistent configuration; changes commit on Close

mod model;
mod render;

pub use model::{PdkSettingsDialogResult, PdkSettingsDialogState};
pub use render::render_pdk_settings_dialog;
