//! Properties Module
//!
//! Component property editing: the tabbed dialog is the single editing
//! surface (opened by `E`, double-click, or the Inspector), backed by the
//! property registry and the bidirectional property bridge.
//!
//! - `engineering` - Engineering-notation value parsing/formatting
//! - `tabbed_dialog` - The tabbed property dialog
//! - `model_browser` - Model library browser
//! - `property_bridge` - Bidirectional property synchronization
//! - `pwl_editor` - Piecewise-linear waveform editor

pub mod engineering;
pub mod model_browser;
pub mod property_bridge;
pub mod pwl_editor;
pub mod tabbed_dialog;

// Re-export main types
pub use engineering::{format_engineering_value, parse_engineering_value};
pub use property_bridge::parse_params_string;
pub use tabbed_dialog::{
    TabInfo, TabbedDialogResult, TabbedPropertyDialogState, render_tabbed_property_dialog,
};
