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

/// The document the Code & Automation workspace is showing right now.
///
/// One owner for one fact. The title bar, the document tab, and anything else
/// that names the active document read this: three chrome elements deriving
/// the same label separately is how the title bar came to state a filename
/// the workspace was not showing.
pub(crate) fn active_document_label(state: &crate::workbench::AppState) -> String {
    match state.ui.code_workspace.page {
        CodeWorkspacePage::Netlist => netlist_document_label(state),
        CodeWorkspacePage::VerilogA => source_bundle_document_label(
            state,
            crate::state::ProjectSourceLanguage::VerilogA,
            state
                .ui
                .code_workspace
                .veriloga
                .selected_file
                .as_ref()
                .map(|selection| selection.logical_path.as_str()),
            crate::workbench::MessageId::VerilogASourceDefault,
        ),
        CodeWorkspacePage::Automation => source_bundle_document_label(
            state,
            crate::state::ProjectSourceLanguage::RSpiceAutomation,
            state.ui.code_workspace.automation.selected_file.as_deref(),
            crate::workbench::MessageId::CodePageAutomation,
        ),
    }
}

/// The netlist page names the artifact it is editing. A generated primary has
/// no file of its own, so it is named after the cell that produces it and
/// marked as generated rather than borrowing a filename it does not have.
fn netlist_document_label(state: &crate::workbench::AppState) -> String {
    state
        .workspace
        .netlist_descriptor
        .as_ref()
        .map(|descriptor| descriptor.artifact_name.clone())
        .or_else(|| {
            state
                .workspace
                .netlist_source_path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| format!("{}.sp · generated", state.workspace.project.top_cell))
}

/// The selected file in a language bundle, or its root when nothing is
/// explicitly selected. A starter filename is never treated as identity: with
/// no bundle at all the page is named after itself.
fn source_bundle_document_label(
    state: &crate::workbench::AppState,
    language: crate::state::ProjectSourceLanguage,
    selected: Option<&str>,
    fallback: crate::workbench::MessageId,
) -> String {
    let owner = crate::state::ProjectSourceOwner::code_workspace(language);
    let bundle = state.workspace.project_sources.bundle_for_owner(&owner);
    selected
        .map(str::to_owned)
        .or_else(|| bundle.map(|bundle| bundle.root().logical_path().to_owned()))
        .map(|path| {
            path.rsplit('/')
                .next()
                .filter(|name| !name.is_empty())
                .unwrap_or(path.as_str())
                .to_owned()
        })
        .unwrap_or_else(|| state.ui.messages().text(fallback))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::AppState;

    /// The title bar used to state a hardcoded `top.sp · generated` for the
    /// whole workspace, so it named a netlist while the user edited Verilog-A
    /// and kept naming it after the deck was given a filename. One owner, and
    /// it follows the page.
    #[test]
    fn the_active_document_label_follows_the_visible_page() {
        let mut state = AppState::default();
        state.workspace.project.top_cell = "afe".to_owned();

        state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
        assert_eq!(active_document_label(&state), "afe.sp · generated");

        state.workspace.netlist_source_path = Some(std::path::PathBuf::from("/p/top_override.sp"));
        assert_eq!(active_document_label(&state), "top_override.sp");

        state.ui.code_workspace.page = CodeWorkspacePage::VerilogA;
        assert_ne!(
            active_document_label(&state),
            "top_override.sp",
            "the Verilog-A page must never be named after the SPICE deck"
        );
    }

    /// A logical path is a path; the tab shows the file at the end of it.
    #[test]
    fn a_selected_bundle_file_is_named_by_its_leaf() {
        let mut state = AppState::default();
        state.ui.code_workspace.page = CodeWorkspacePage::Automation;
        state.ui.code_workspace.automation.selected_file =
            Some("workflows/characterize.py".to_owned());

        assert_eq!(active_document_label(&state), "characterize.py");
    }
}
