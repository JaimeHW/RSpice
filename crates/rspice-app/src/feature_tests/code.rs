//! The code feature's behaviour, observed through a whole application.

// The moved tests' bodies name these as `super::...`, as they did beside
// them under app/ or in the module they tested: the interface tests'
// editing helper, the source navigation and the New Verilog-A device
// form's renderer.
use crate::workbench::app::veriloga_interface_tests;
use rspice_app_code::workbench::app::dialogs::library::new_veriloga_device::render_new_veriloga_device_form;
use rspice_app_code::workbench::app::dialogs::source_language_tools::navigate_source_bundle;

mod generated_netlist_jumps;
mod netlist_cross_probe;
#[cfg(not(target_arch = "wasm32"))]
mod netlist_document;
pub(super) mod netlist_export;
mod netlist_first;
mod netlist_first_drawn;
mod netlist_generation;
mod netlist_search;
mod netlist_source_menu;
mod netlist_surface;
mod netlist_toolbar;
mod netlist_transfer;
mod netlist_validation;
mod netlist_workflow;
mod new_veriloga_device;
mod source_document;
mod source_editor_commands;
mod source_find;
mod source_language_tools;
mod veriloga_build_settings;
mod veriloga_cell_source;
mod veriloga_design;
mod veriloga_flow;
mod veriloga_import;
mod veriloga_interface;
mod veriloga_panel;
mod veriloga_parameter;
mod veriloga_placement;
mod veriloga_symbol;
