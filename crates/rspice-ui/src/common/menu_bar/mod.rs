//! Menu action layer.
//!
//! These modules implement the *behavior* behind the application menus —
//! file/project workflows with dirty-state confirmation, exports, simulation
//! control, design checks, Verilog-A cache management, examples and help.
//! Rendering lives in [`crate::workbench::menubar`], which dispatches into the
//! functions re-exported here.

mod examples_menu;
mod export_actions;
mod file_menu;
mod netlist_compat;
mod simulate_menu;
mod tools_menu;
mod waveform_export;

pub(crate) use examples_menu::load_named_example;
pub(crate) use export_actions::{action_export_netlist_with_io, action_view_netlist};
pub(crate) use file_menu::{FileMenuAction, dispatch_file_menu_action};
pub(crate) use simulate_menu::open_simulation_options;
pub(crate) use tools_menu::run_design_rule_check;
pub(crate) use waveform_export::action_export_csv_with_io;
