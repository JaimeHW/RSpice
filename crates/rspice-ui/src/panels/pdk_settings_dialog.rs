//! PDK Settings Dialog
//!
//! Professional interface for configuring PDK (Process Design Kit) library paths,
//! environment variables, and model file discovery settings.
//!
//! ## Features
//!
//! - Library path management (add/remove/enable/disable)
//! - Environment variable configuration (e.g., $PDK_HOME)
//! - Model file discovery with preview
//! - Recent files tracking
//! - Persistent configuration

mod model;
mod render;
mod tabs;

pub use model::{PdkSettingsDialogResult, PdkSettingsDialogState, PdkSettingsTab};
pub use render::render_pdk_settings_dialog;
