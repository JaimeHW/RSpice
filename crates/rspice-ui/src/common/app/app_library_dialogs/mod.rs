use egui::Context;

use super::{ConsoleMessage, RSpiceApp, VERILOGA_LIBRARY_NAME, save_global_veriloga_library};

mod new_cell_dialog;
mod new_view_dialog;
mod pending_deletions;
mod shared;

#[cfg(test)]
mod tests;
