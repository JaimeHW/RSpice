//! The source feature's behaviour, observed through a whole application.

// The moved tests' bodies name these as `super::...`, as they did beside
// them under app/ or in the module they tested: the interface tests'
// editing helper, the source navigation and the New Verilog-A device
// form's renderer.
use crate::workbench::app::veriloga_interface_tests;
use rspice_app_code::workbench::app::dialogs::library::new_veriloga_device::render_new_veriloga_device_form;
use rspice_app_code::workbench::app::dialogs::source_language_tools::navigate_source_bundle;

mod deck_dialects;
mod deck_disk;
mod deck_disk_away;
#[cfg(not(target_arch = "wasm32"))]
mod deck_in_front;
mod deck_read_away;
mod deck_run_memory;
mod deck_run_preparing;
mod deck_save_as;
mod deck_setup;
mod generated_netlist_jumps;
mod netlist_cross_probe;
#[cfg(not(target_arch = "wasm32"))]
mod netlist_document;
pub(super) mod netlist_export;
mod netlist_first;
mod netlist_first_drawn;
mod netlist_generation;
mod netlist_language;
mod netlist_routes;
#[cfg(not(target_arch = "wasm32"))]
mod netlist_session;
mod netlist_source_menu;
mod netlist_surface;
mod netlist_tabs;
mod netlist_toolbar;
mod netlist_transfer;
mod netlist_workflow;
mod new_veriloga_device;
mod project_deck_run;
mod run_target;
mod source_document;
mod source_editor_commands;
mod source_language_tools;
mod stored_deck;
mod stored_deck_files;
mod stored_deck_typed;
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
