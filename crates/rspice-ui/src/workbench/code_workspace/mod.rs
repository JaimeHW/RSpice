//! Canonical state and behavior for the Code & Automation workspace.
//!
//! The visual surface composes these contracts; it does not own source text,
//! provenance, validation identity, or document transitions. Keeping those
//! concerns here makes native and browser behavior identical and prevents an
//! asynchronous file or validation response from being applied to newer text.

mod automation;
mod document;
mod editor;
mod outline;
mod page;
mod search;
mod veriloga;
#[cfg(any(target_arch = "wasm32", test))]
mod veriloga_worker;
#[cfg(target_arch = "wasm32")]
pub(crate) use veriloga_worker::run_worker_request_value as run_veriloga_worker_request_value;

pub use automation::{
    export_automation_artifact, poll_automation_workflow, start_automation_workflow,
};
pub use document::{
    DependencyMetadata, DependencyResolution, DiagnosticSeverity, DocumentError, DocumentOwnership,
    GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
    ImportedProvenance, NetlistDocument, NetlistDocumentId, SaveAcknowledgement, SourceLocator,
    SourcePosition, SourceProvenance, TransitionReceipt, ValidationDiagnostic, ValidationReport,
    content_digest,
};
pub use editor::{CodeEditorDiagnostic, CodeEditorLanguage, CodeEditorSeverity, show_code_editor};
pub(crate) use outline::parse_include_directives;
pub use outline::{
    IncludeDirective, IncludeKind, NetlistOutline, OutlineEntry, OutlineEntryKind, OutlineSection,
    OutlineSectionKind,
};
pub(crate) use page::compile_project_source_bundle_runtime;
pub use page::{
    AutomationDispatchSnapshot, AutomationExecutionState, AutomationValidationReceipt,
    CodeWorkspacePage, CodeWorkspaceRuntimeState, PendingVerilogACompile, PreparedVerilogARuntime,
    PreparedVerilogARuntimeSet, SourceOperationToken, TargetQualification, VerilogACompileOutcome,
    VerilogACompileReceipt, VerilogAFileEditorKind, VerilogAFileEditorState, VerilogAFileSelection,
    VerilogAImportTarget, VerilogASourceOperationToken, append_project_veriloga_directive,
    project_veriloga_directive,
};
pub use search::{
    FindDirection, FindError, FindMatch, FindOptions, ReplaceOutcome, ReplaceScope,
    find_all_in_source, find_in_source, replace_in_source,
};
pub(crate) use veriloga::{
    SelectedVerilogASource, active_veriloga_file_path, compile_project_bundle_receipt,
    project_bundle_as_virtual, project_virtual_compile_limits, replace_selected_veriloga_file,
    selected_veriloga_source,
};
pub use veriloga::{poll_veriloga_compile, poll_veriloga_import, start_veriloga_compile};
