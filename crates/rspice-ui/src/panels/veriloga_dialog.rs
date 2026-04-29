//! Verilog-A Model Loading Dialog
//!
//! Professional interface for loading, compiling, and registering Verilog-A
//! compact device models in the component library.

mod compile;
mod options;
mod render;
mod state;
mod types;

pub use options::VerilogADialogOptions;
pub use render::render_veriloga_load_dialog;
pub use state::VerilogALoadDialogState;
pub use types::{
    CompilationState, CompileErrorDisplay, CompileTask, CompileTaskResult, CompiledModuleInfo,
    ErrorSeverity, ParameterInfo, VerilogADialogResult,
};
