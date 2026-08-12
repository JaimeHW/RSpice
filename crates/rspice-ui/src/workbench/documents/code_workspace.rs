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
pub(crate) use language_services::{
    CodeLanguageToolsState, LanguageToolView, SourceIndexCache, SourceOutlineRow, document_outline,
    open_language_tools,
};
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
    assign_automation_source_role, commit_source_file_dialog, commit_source_history_restore,
    commit_source_workspace_dialog, import_dropped_automation_source, open_source_file_dialog,
    open_source_history, open_source_workspace_dialog, request_automation_source_import,
    poll_automation_source_import, source_bundle_contains_document,
    source_bundle_document_is_editable, source_document_is_editable,
    source_file_mutation_block_reason,
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
pub(crate) fn active_document_label(
    workspace: &crate::state::ProjectWorkspace,
    code: &CodeWorkspaceRuntimeState,
    messages: crate::workbench::MessageCatalog,
) -> String {
    match code.page {
        CodeWorkspacePage::Netlist => netlist_document_label(workspace),
        CodeWorkspacePage::VerilogA => source_bundle_document_label(
            workspace,
            messages,
            crate::state::ProjectSourceLanguage::VerilogA,
            code.veriloga
                .selected_file
                .as_ref()
                .map(|selection| selection.logical_path.as_str()),
            crate::workbench::MessageId::VerilogASourceDefault,
        ),
        CodeWorkspacePage::Automation => source_bundle_document_label(
            workspace,
            messages,
            crate::state::ProjectSourceLanguage::RSpiceAutomation,
            code.automation.selected_file.as_deref(),
            crate::workbench::MessageId::CodePageAutomation,
        ),
    }
}

/// The bundle language a page edits, or `None` for the netlist page, whose
/// deck is not a project source bundle at all.
pub(crate) const fn page_source_language(
    page: CodeWorkspacePage,
) -> Option<crate::state::ProjectSourceLanguage> {
    match page {
        CodeWorkspacePage::Netlist => None,
        CodeWorkspacePage::VerilogA => Some(crate::state::ProjectSourceLanguage::VerilogA),
        CodeWorkspacePage::Automation => {
            Some(crate::state::ProjectSourceLanguage::RSpiceAutomation)
        }
    }
}

/// The bundle file a language page is showing.
///
/// Selection is optional state: a page that has never had a file chosen shows
/// the bundle root. The navigator, the document well, and the lifecycle dialog
/// must agree on which file that is, so they all read it here.
pub(crate) fn active_bundle_path(
    workspace: &crate::state::ProjectWorkspace,
    code: &CodeWorkspaceRuntimeState,
    language: crate::state::ProjectSourceLanguage,
) -> Option<String> {
    let explicit = match language {
        crate::state::ProjectSourceLanguage::VerilogA => code
            .veriloga
            .selected_file
            .as_ref()
            .map(|selection| selection.logical_path.clone()),
        crate::state::ProjectSourceLanguage::RSpiceAutomation => {
            code.automation.selected_file.clone()
        }
    };
    explicit.or_else(|| {
        let owner = crate::state::ProjectSourceOwner::code_workspace(language);
        workspace
            .project_sources
            .bundle_for_owner(&owner)
            .map(|bundle| bundle.root().logical_path().to_owned())
    })
}

/// The netlist page names the artifact it is editing. A generated primary has
/// no file of its own, so it is named after the cell that produces it and
/// marked as generated rather than borrowing a filename it does not have.
fn netlist_document_label(workspace: &crate::state::ProjectWorkspace) -> String {
    workspace
        .netlist_descriptor
        .as_ref()
        .map(|descriptor| descriptor.artifact_name.clone())
        .or_else(|| {
            workspace
                .netlist_source_path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| format!("{}.sp · generated", workspace.project.top_cell))
}

/// The selected file in a language bundle, or its root when nothing is
/// explicitly selected. A starter filename is never treated as identity: with
/// no bundle at all the page is named after itself.
fn source_bundle_document_label(
    workspace: &crate::state::ProjectWorkspace,
    messages: crate::workbench::MessageCatalog,
    language: crate::state::ProjectSourceLanguage,
    selected: Option<&str>,
    fallback: crate::workbench::MessageId,
) -> String {
    let owner = crate::state::ProjectSourceOwner::code_workspace(language);
    let bundle = workspace.project_sources.bundle_for_owner(&owner);
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
        .unwrap_or_else(|| messages.text(fallback))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The function takes exactly the state it reads, so the fixture is that
    /// state and not a whole application.
    fn fixture() -> (crate::state::ProjectWorkspace, CodeWorkspaceRuntimeState) {
        (
            crate::state::ProjectWorkspace::default(),
            CodeWorkspaceRuntimeState::default(),
        )
    }

    fn messages() -> crate::workbench::MessageCatalog {
        crate::workbench::MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
    }

    /// The title bar used to state a hardcoded `top.sp · generated` for the
    /// whole workspace, so it named a netlist while the user edited Verilog-A
    /// and kept naming it after the deck was given a filename. One owner, and
    /// it follows the page.
    #[test]
    fn the_active_document_label_follows_the_visible_page() {
        let (mut workspace, mut code) = fixture();
        workspace.project.top_cell = "afe".to_owned();

        code.page = CodeWorkspacePage::Netlist;
        assert_eq!(
            active_document_label(&workspace, &code, messages()),
            "afe.sp · generated"
        );

        workspace.netlist_source_path = Some(std::path::PathBuf::from("/p/top_override.sp"));
        assert_eq!(
            active_document_label(&workspace, &code, messages()),
            "top_override.sp"
        );

        code.page = CodeWorkspacePage::VerilogA;
        assert_ne!(
            active_document_label(&workspace, &code, messages()),
            "top_override.sp",
            "the Verilog-A page must never be named after the SPICE deck"
        );
    }

    /// A logical path is a path; the tab shows the file at the end of it.
    #[test]
    fn a_selected_bundle_file_is_named_by_its_leaf() {
        let (workspace, mut code) = fixture();
        code.page = CodeWorkspacePage::Automation;
        code.automation.selected_file = Some("workflows/characterize.py".to_owned());

        assert_eq!(
            active_document_label(&workspace, &code, messages()),
            "characterize.py"
        );
    }
}
