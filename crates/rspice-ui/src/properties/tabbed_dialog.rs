//! Schematic component editor
//!
//! Schema-driven typed instance editing in the dedicated two-pane mockup shell.

mod editors;
mod render;
mod state;

pub(crate) use editors::parse_expression_source;
pub use render::render_tabbed_property_dialog;
pub use state::{
    ComponentEditorContext, ComponentModelContext, ComponentOperatingPointContext,
    ComponentPropertySession, ComponentTerminalContext, TabbedDialogResult,
    TabbedPropertyDialogState,
};
