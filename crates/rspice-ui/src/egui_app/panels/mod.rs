//! Panel Components for egui Application
//!
//! Side panels: Project Browser and Properties Panel.

pub mod project_browser;
pub mod properties_panel;

pub use project_browser::render_project_browser;
pub use properties_panel::render_properties_panel;
