use egui::Context;

use super::{save_global_veriloga_library, ConsoleMessage, RSpiceApp, VERILOGA_LIBRARY_NAME};

mod new_cell_dialog;
mod new_view_dialog;
mod pending_deletions;
mod shared;

#[cfg(test)]
mod tests;
