//! Properties Module
//!
//! Property editing and design variable management.
//! Provides UI components for editing component properties and design parameters.
//!
//! - `dialog` - Component properties dialog
//! - `enhanced_editor` - Enhanced property editor with validation
//! - `design_variables` - Design variable manager (parameterized values)
//!
//! # Architecture
//!
//! Properties are displayed in context-sensitive dialogs and panels.
//! Design variables enable parameterized circuit design with expressions.

pub mod design_variables;
pub mod dialog;
pub mod enhanced_editor;

// Re-export main types
pub use design_variables::{DesignVariable, DesignVariablesState};
pub use dialog::{
    format_engineering_value, parse_engineering_value, render_properties_dialog, EditedProperties,
    PropertiesDialogResult, PropertyEditorState,
};
pub use enhanced_editor::{render_enhanced_property_editor, EnhancedPropertyEditorState};
