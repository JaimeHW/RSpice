//! UI Components Module
//!
//! Reusable, composable UI components following a consistent design system.

mod button;
pub mod component_edit;
pub mod component_library;
pub mod confirm_modal;
pub mod context_menu;
pub mod file_handlers;
mod icons;
mod panel;
pub mod simulation_dialog;
pub mod tab_bar;
mod tabs;
mod toolbar;
pub mod veriloga_dialog;
pub mod veriloga_inspector;

#[allow(unused_imports)]
pub use button::Button;
#[allow(unused_imports)]
pub use component_edit::ComponentEditModal;
#[allow(unused_imports)]
pub use component_library::ComponentLibrary;
#[allow(unused_imports)]
pub use context_menu::{ContextMenu, MenuAction, MenuItem};
pub use panel::Panel;
#[allow(unused_imports)]
pub use simulation_dialog::SimulationDialog;
#[allow(unused_imports)]
pub use tabs::{Tab, TabBar};
pub use toolbar::Toolbar;
