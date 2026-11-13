//! The code feature's behaviour, observed through a whole application.

// The moved tests' bodies name these as `super::...`, as they did beside
// them under app/ or in the module they tested: the interface tests'
// editing helper, the dialogs module, the source navigation and the New
// Verilog-A device form's renderer.
use crate::workbench::app::dialogs::library::new_veriloga_device::render_new_veriloga_device_form;
use crate::workbench::app::{dialogs, navigate_source_bundle, veriloga_interface_tests};

#[cfg(not(target_arch = "wasm32"))]
mod netlist_document;
mod netlist_export;
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
