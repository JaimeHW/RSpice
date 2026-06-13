//! Panel Components for egui Application
//!
//! Dialog-hosted panels (PDK settings, Verilog-A, property dialog, script
//! console, calculator) and the structured log model consumed by the shell
//! console.

pub mod calculator;
pub mod log_panel;
pub mod pdk_settings_dialog;
pub mod properties_panel;
pub mod script_console;
pub mod veriloga_dialog;

pub use log_panel::{LogAnchor, LogBuffer, LogEntry, LogSeverity, LogSource};
pub use pdk_settings_dialog::{
    PdkSettingsDialogResult, PdkSettingsDialogState, render_pdk_settings_dialog,
};
pub use properties_panel::render_property_dialog;
pub use script_console::{ConsoleHistoryItem, ScriptConsoleState};
pub use veriloga_dialog::{
    CompilationState, CompileErrorDisplay, CompiledModuleInfo, ParameterInfo,
    VerilogADialogOptions, VerilogADialogResult, VerilogALoadDialogState,
    render_veriloga_load_dialog,
};
