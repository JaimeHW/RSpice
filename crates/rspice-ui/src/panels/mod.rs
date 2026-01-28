//! Panel Components for egui Application
//!
//! Side panels: Project Browser, Results Browser, and Properties Panel.

pub mod project_browser;
pub mod properties_panel;
pub mod results_browser;
pub mod script_console;
pub mod veriloga_dialog;
pub mod yield_panel;

pub use project_browser::render_project_browser;
pub use properties_panel::render_properties_panel;
pub use results_browser::render_results_browser;
pub use script_console::{render_script_console, ScriptConsoleState};
pub use veriloga_dialog::{
    render_veriloga_load_dialog, CompilationState, CompileErrorDisplay, CompiledModuleInfo,
    ParameterInfo, VerilogADialogOptions, VerilogADialogResult, VerilogALoadDialogState,
};
pub use yield_panel::render_yield_panel;
