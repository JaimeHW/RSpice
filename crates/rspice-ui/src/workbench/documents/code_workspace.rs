//! Canonical state and behavior for the Code & Automation workspace.
//!
//! The visual surface composes these contracts; it does not own source text,
//! provenance, validation identity, or document transitions. Keeping those
//! concerns here makes native and browser behavior identical and prevents an
//! asynchronous file or validation response from being applied to newer text.

mod automation;

mod editor;
mod editor_buffer;
mod language_services;
mod page;
mod source_files;
mod source_search;
mod veriloga;
mod veriloga_profile;
#[cfg(any(target_arch = "wasm32", test))]
mod veriloga_worker;
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) use veriloga_worker::run_worker_request_value as run_veriloga_worker_request_value;

pub(crate) use automation::automation_task_count;
pub(crate) use automation::invalidate_automation_evidence;
pub use automation::{
    cancel_automation_workflow, continue_automation_debugger, dry_run_automation_workflow,
    export_automation_artifact, pause_automation_debugger, poll_automation_workflow,
    poll_managed_automation_runtime, refresh_automation_debugger, remove_automation_breakpoint,
    restart_automation_debugger, select_automation_debug_frame, settle_pending_python_execute,
    start_automation_debugger, start_automation_workflow, step_into_automation_debugger,
    step_out_automation_debugger, step_over_automation_debugger, stop_automation_debugger,
    sync_automation_debugger, toggle_automation_breakpoint, validate_automation_workspace,
};
pub use editor::{
    CodeDiagnosticCollection, CodeEditorDiagnostic, CodeEditorLanguage, CodeEditorSeverity,
    show_code_document_interaction_versioned, show_code_document_with_debug_versioned,
};
pub(crate) use language_services::{CodeLanguageToolsState, LanguageToolView, open_language_tools};
pub use page::{
    AutomationArtifactStore, AutomationBreakpoint, AutomationBreakpointKind, AutomationDebugPhase,
    AutomationDebugState, AutomationDispatchSnapshot, AutomationExceptionPolicy,
    AutomationExecutionState, AutomationRuntimeLaunchMode, AutomationValidationReceipt,
    AutomationWatch, CodeSourceFileAction, CodeSourceFileDialogState, CodeSourceHistoryState,
    CodeSourceImportState, CodeSourceSearchScope, CodeSourceSearchState,
    CodeSourceWorkspaceDialogState, CodeWorkspacePage, CodeWorkspaceRuntimeState,
    PendingVerilogACompile, SourceOperationToken, TargetQualification, VerilogACompileDialogState,
    VerilogACompileOutcome, VerilogACompileReceipt, VerilogAFileSelection,
    VerilogAQualificationHistoryRow, VerilogARootImportTarget,
};
pub(crate) use page::{CodeSourceEditorBufferCache, CodeSourceEditorBufferIdentity};
pub(crate) use source_files::{
    import_dropped_automation_source, open_source_workspace_dialog,
    source_bundle_contains_document, source_bundle_document_is_editable,
    source_document_is_editable, source_file_mutation_block_reason,
};
pub(crate) use source_search::open_source_search;
pub(crate) use veriloga::{
    SelectedVerilogASource, active_veriloga_file_path, commit_veriloga_compile_dialog,
    compile_project_bundle_receipt, compile_project_bundle_virtual_for_provenance,
    import_dropped_veriloga_source, open_veriloga_compile_dialog, replace_selected_veriloga_file,
    request_veriloga_root_import, selected_veriloga_editor_snapshot, selected_veriloga_source,
};
pub use veriloga::{poll_veriloga_compile, poll_veriloga_import};
