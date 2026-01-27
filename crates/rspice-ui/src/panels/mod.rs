//! Panel Components for egui Application
//!
//! Side panels: Project Browser and Properties Panel.

pub mod project_browser;
pub mod properties_panel;
pub mod script_console;
pub mod yield_panel;

pub use project_browser::render_project_browser;
pub use properties_panel::render_properties_panel;
pub use script_console::{render_script_console, ScriptConsoleState};
pub use yield_panel::render_yield_panel;
