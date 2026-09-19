//! Schematic component editor
//!
//! Schema-driven typed instance editing in the dedicated two-pane mockup shell.

mod editors;
mod render;
mod state;

pub(crate) use editors::parse_expression_source;
/// The one route that attaches a waveform data file to a project. The
/// component editor and the Stimulus Library both import through it, so a file
/// lands in the same place whichever surface asked for it.
pub(crate) use render::attach_data_file;
pub(crate) use render::provenance_colour;
pub use render::render_tabbed_property_dialog;
pub use state::{
    ComponentEditorContext, ComponentModelContext, ComponentOperatingPointContext,
    ComponentPropertySession, ComponentTerminalContext, StimulusEditorContext, TabbedDialogResult,
    TabbedPropertyDialogState,
};
