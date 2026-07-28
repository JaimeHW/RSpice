//! Canonical state and behavior for the Code & Automation workspace.
//!
//! The visual surface composes these contracts; it does not own source text,
//! provenance, validation identity, or document transitions. Keeping those
//! concerns here makes native and browser behavior identical and prevents an
//! asynchronous file or validation response from being applied to newer text.

mod automation;

mod editor;
mod page;
mod veriloga;
#[cfg(any(target_arch = "wasm32", test))]
mod veriloga_worker;
#[cfg(target_arch = "wasm32")]
pub(crate) use veriloga_worker::run_worker_request_value as run_veriloga_worker_request_value;

pub use automation::{
    export_automation_artifact, poll_automation_workflow, start_automation_workflow,
};
pub use editor::{CodeEditorDiagnostic, CodeEditorLanguage, CodeEditorSeverity, show_code_editor};
pub use page::{
    AutomationDispatchSnapshot, AutomationExecutionState, AutomationValidationReceipt,
    CodeWorkspacePage, CodeWorkspaceRuntimeState, PendingVerilogACompile, SourceOperationToken,
    TargetQualification, VerilogACompileOutcome, VerilogACompileReceipt, VerilogAFileEditorKind,
    VerilogAFileEditorState, VerilogAFileSelection, VerilogAImportTarget,
};
pub(crate) use veriloga::{
    SelectedVerilogASource, active_veriloga_file_path, compile_project_bundle_receipt,
    project_bundle_as_virtual, replace_selected_veriloga_file, selected_veriloga_source,
};
pub use veriloga::{poll_veriloga_compile, poll_veriloga_import, start_veriloga_compile};
