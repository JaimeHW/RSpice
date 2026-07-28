//! Panel Components for egui Application
//!
//! Dialog-hosted panels (PDK settings, Verilog-A, property dialog, script
//! console, calculator) and the structured log model consumed by the workbench
//! console.

pub(crate) mod calculator;
pub(crate) mod pdk_settings_dialog;
pub(crate) mod properties_panel;
pub(crate) mod script_console;

pub use pdk_settings_dialog::{
    PdkSettingsDialogResult, PdkSettingsDialogState, render_pdk_settings_dialog,
};
pub use properties_panel::render_property_dialog;
pub use script_console::{ConsoleHistoryItem, ScriptConsoleState};
