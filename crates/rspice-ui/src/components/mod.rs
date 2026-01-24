//! UI Components Module
//!
//! Reusable, composable UI components following a consistent design system.

pub mod annotation_overlay;
mod button;
pub mod component_library;
pub mod context_menu;
pub mod file_handlers;
mod icons;
mod panel;
pub mod project_browser;
pub mod tab_bar;
mod tabs;
mod toolbar;

#[allow(unused_imports)]
pub use button::{Button, ButtonVariant};
#[allow(unused_imports)]
pub use component_library::ComponentLibrary;
#[allow(unused_imports)]
pub use context_menu::{ContextMenu, MenuAction, MenuItem};
pub use panel::Panel;
pub use project_browser::ProjectBrowser;
#[allow(unused_imports)]
pub use tabs::{Tab, TabBar};
pub use toolbar::Toolbar;

// Re-export dialogs from their new locations
#[allow(unused_imports)]
pub use crate::dialogs::ComponentEditModal;
#[allow(unused_imports)]
pub use crate::dialogs::SimulationDialog;
