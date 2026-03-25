//! Panel Components for egui Application
//!
//! Side panels: Project Browser, Results Browser, and Properties Panel.

pub mod calculator;
pub mod log_panel;
pub mod pdk_settings_dialog;
pub mod project_browser;
pub mod properties_panel;
pub mod results_browser;
pub mod script_console;
pub mod veriloga_dialog;
pub mod yield_panel;

pub use log_panel::{LogBuffer, LogEntry, LogPanelState, LogSeverity, LogSource, render_log_panel};
pub use pdk_settings_dialog::{
    PdkSettingsDialogResult, PdkSettingsDialogState, PdkSettingsTab, render_pdk_settings_dialog,
};
pub use project_browser::render_project_browser;
pub use properties_panel::{render_properties_panel, render_property_dialog};
pub use results_browser::render_results_browser;
pub use script_console::{ScriptConsoleState, render_script_console};
pub use veriloga_dialog::{
    CompilationState, CompileErrorDisplay, CompiledModuleInfo, ParameterInfo,
    VerilogADialogOptions, VerilogADialogResult, VerilogALoadDialogState,
    render_veriloga_load_dialog,
};
pub use yield_panel::render_yield_panel;
