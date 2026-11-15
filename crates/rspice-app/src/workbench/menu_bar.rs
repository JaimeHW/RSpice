//! Menu action layer.
//!
//! These modules implement the *behavior* behind the application menus —
//! file/project workflows with dirty-state confirmation, exports, simulation
//! control, design checks, Verilog-A cache management, examples and help.
//! Rendering lives in the workbench title-bar menu module, which dispatches
//! into the functions re-exported here.

mod file_menu;
use rspice_app_results::workbench::menu_bar::waveform_export;

pub(crate) use file_menu::{FileMenuAction, dispatch_file_menu_action};
#[cfg(test)]
pub(crate) use rspice_app_code::workbench::menu_bar::netlist_export::build_menu_netlist_in;
pub(crate) use waveform_export::action_export_result_selection_with_io;
#[cfg(test)]
pub(crate) use waveform_export::{
    DatasetExportChoice, DatasetExportPicker, action_export_dataset_with_io,
};

use crate::workbench::app_state::AppState;

/// Open the dataset export picker over the view that is showing, or say why
/// no view can be exported.
pub(crate) fn open_dataset_export_picker(state: &mut AppState) {
    match waveform_export::dataset_export_picker(state.results_reads()) {
        Ok(picker) => state.results.dataset_export = Some(picker),
        Err(message) => state.prompt(message),
    }
}
