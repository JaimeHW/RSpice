//! Menu action layer.
//!
//! These modules implement the *behavior* behind the application menus —
//! file/project workflows with dirty-state confirmation, exports, simulation
//! control, design checks, Verilog-A cache management, examples and help.
//! Rendering lives in the workbench title-bar menu module, which dispatches
//! into the functions re-exported here.

mod file_menu;
mod netlist_compat;
mod netlist_export;
mod simulate_menu;
mod tools_menu;
mod waveform_export;

pub(crate) use file_menu::{FileMenuAction, dispatch_file_menu_action};
pub(crate) use netlist_export::{
    action_export_generated_netlist_with_options, action_export_netlist_with_io,
    action_view_netlist, build_menu_netlist, netlist_failure_text, record_netlist_warnings,
};
pub(crate) use simulate_menu::open_simulation_options;
pub(crate) use tools_menu::{check_cellview, check_design};
pub(crate) use waveform_export::{
    DatasetExportPicker, action_export_csv_with_io, action_export_result_selection_with_io,
    show_dataset_export_picker,
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
