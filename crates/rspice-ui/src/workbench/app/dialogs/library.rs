//! Design library dialogs.
//!
//! Creating and renaming cells and views, and the cell-level operations
//! (copy, delete, rename) that change library membership rather than the
//! contents of a single design.

use egui::Context;

use crate::workbench::app::{RSpiceApp, VERILOGA_LIBRARY_NAME, save_global_veriloga_library};

mod cell_ops_dialogs;
mod new_cell_dialog;
mod new_view_dialog;
mod pending_deletions;
mod shared;
