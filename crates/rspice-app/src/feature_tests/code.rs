//! The code feature's behaviour, observed through a whole application.

// The moved tests' bodies name these as `super::...`, as they did beside
// them under app/: the interface tests' editing helper, the dialogs
// module and the source navigation.
use crate::workbench::app::{dialogs, navigate_source_bundle, veriloga_interface_tests};

#[cfg(not(target_arch = "wasm32"))]
mod netlist_validation;
mod source_editor_commands;
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
