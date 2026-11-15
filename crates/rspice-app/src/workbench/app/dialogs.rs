//! Modal dialogs.
//!
//! One module per dialog, each owning its own state struct and rendering.
//! Each feature keeps its dialogs' state in its slot
//! ([`crate::workbench::slots`]), and `AppState::dialog_modal_open` reads
//! them all so a single read answers whether any modal owns input this
//! frame; the dialogs themselves never inspect each other.

pub(crate) use rspice_app_project::workbench::app::dialogs::check_and_save_validation;
pub(in crate::workbench) mod help;
pub(in crate::workbench) mod state;
