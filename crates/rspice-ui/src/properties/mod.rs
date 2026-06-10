//! Properties Module
//!
//! Property editing and design variable management.
//! Provides UI components for editing component properties and design parameters.
//!
//! - `dialog` - Component properties dialog (basic)
//! - `design_variables` - Design variable manager (parameterized values)
//! - `tabbed_dialog` - Commercial-grade tabbed property dialog (Virtuoso-style)
//! - `model_browser` - Model library browser
//! - `property_bridge` - Bidirectional property synchronization
//! - `pwl_editor` - Piecewise-linear waveform editor
//!
//! # Architecture
//!
//! Properties are displayed in context-sensitive dialogs and panels.
//! Design variables enable parameterized circuit design with expressions.

pub mod dialog;
pub mod model_browser;
pub mod property_bridge;
pub mod pwl_editor;
pub mod tabbed_dialog;

// Re-export main types
pub use dialog::{
    EditedProperties, PropertiesDialogResult, PropertyEditorState, format_engineering_value,
    parse_engineering_value, render_properties_dialog,
};
pub use property_bridge::parse_params_string;
pub use tabbed_dialog::{
    TabInfo, TabbedDialogResult, TabbedPropertyDialogState, render_tabbed_property_dialog,
};
