//! Properties Module
//!
//! Component property editing: the tabbed dialog is the single editing
//! surface (opened by `E`, double-click, or the Inspector), backed by the
//! property registry and the bidirectional property bridge.
//!

//! - `tabbed_dialog` - The tabbed property dialog
//! - `model_browser` - Model library browser
//! - `property_bridge` - Bidirectional property synchronization
//! - `pwl_editor` - Piecewise-linear waveform editor


pub(crate) mod model_browser;
pub(crate) mod property_bridge;
pub(crate) mod pwl_editor;
pub(crate) mod tabbed_dialog;

// Re-export main types


pub use tabbed_dialog::{
    ComponentPropertySession, TabInfo, TabbedDialogResult, TabbedPropertyDialogState,
    render_tabbed_property_dialog,
};
