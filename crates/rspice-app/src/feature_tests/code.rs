//! The code feature's behaviour, observed through a whole application.

// The flow tests' bodies reach the interface tests' editing helper as
// `super::veriloga_interface_tests`, as they did beside it under app/.
use crate::workbench::app::veriloga_interface_tests;

#[cfg(not(target_arch = "wasm32"))]
mod netlist_validation;
mod source_editor_commands;
mod veriloga_build_settings;
mod veriloga_flow;
mod veriloga_panel;
