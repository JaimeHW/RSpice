//! The code feature's behaviour, observed through a whole application.

#[cfg(not(target_arch = "wasm32"))]
mod netlist_validation;
mod source_editor_commands;
mod veriloga_build_settings;
mod veriloga_panel;
