//! Tabbed Property Dialog
//!
//! Commercial-grade tabbed property editing dialog matching Cadence Virtuoso
//! Edit Instance Properties (q) dialog behavior.

mod editors;
mod render;
mod state;

pub use render::render_tabbed_property_dialog;
pub use state::{TabInfo, TabbedDialogResult, TabbedPropertyDialogState};
