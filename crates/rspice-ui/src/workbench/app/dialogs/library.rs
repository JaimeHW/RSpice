//! Design library membership dialogs.
//!
//! Every dialog here changes what the project's library catalog contains —
//! a library, a cell, or a view — rather than the contents of one design, so
//! each is bound to the exact catalog revision it opened against and refuses
//! to commit against a catalog that moved underneath it.

use egui::Context;

use crate::workbench::app::{RSpiceApp, VERILOGA_LIBRARY_NAME, save_global_veriloga_library};

mod cell_ops_dialogs;
mod library_ops_dialogs;
mod new_cell_dialog;
mod new_view_dialog;
mod pending_deletions;
mod rename_view_dialog;
mod shared;
