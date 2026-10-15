//! Canonical state and behavior for the Code & Automation workspace.
//!
//! The visual surface composes these contracts; it does not own source text,
//! provenance, validation identity, or document transitions. Keeping those
//! concerns here makes native and browser behavior identical and prevents an
//! asynchronous file or validation response from being applied to newer text.

mod automation;

mod editor;
mod language_services;
mod page;
mod source_files;
mod source_navigation;
mod source_search;
mod veriloga;
mod veriloga_inspector;
mod veriloga_interface;
use rspice_simulation::project_veriloga::build_profile as veriloga_profile;

pub use super::{CodeSourceFileAction, CodeWorkspacePage};

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
    CodeLanguageToolsState, LanguageToolView, SourceCodeAction, SourceSymbol, SourceSymbolKind,
    commit_language_code_action, commit_language_completion, commit_language_rename,
    open_language_tools, open_language_tools_in_bundle,
};
pub(crate) use page::VerilogAHeaderView;
pub use page::{
    AutomationArtifactStore, AutomationBreakpointKind, AutomationDebugPhase,
    AutomationDispatchSnapshot, AutomationExceptionPolicy, AutomationExecutionState,
    AutomationRuntimeLaunchMode, AutomationValidationReceipt, AutomationWatch,
    CodeSourceFileDialogState, CodeSourceHistoryState, CodeSourceImportState,
    CodeSourceSearchScope, CodeSourceSearchState, CodeSourceWorkspaceDialogState,
    CodeWorkspaceRuntimeState, PendingVerilogACompile, SourceOperationToken,
    VerilogACompileOutcome, VerilogACompileReceipt, VerilogAFileSelection, VerilogAImportFormState,
    VerilogAPlacementHandoff, VerilogAReplaceReview, VerilogAReplaceTarget,
};
pub(crate) use source_files::{
    assign_automation_source_role, commit_source_file_dialog, commit_source_history_restore,
    commit_source_workspace_dialog, import_dropped_automation_source, open_source_file_dialog,
    open_source_file_dialog_in_bundle, open_source_history, open_source_workspace_dialog,
    poll_automation_source_import, request_automation_source_import,
    source_bundle_contains_document, source_bundle_document_is_editable,
    source_document_is_editable, source_file_mutation_block_reason,
};
pub(crate) use source_navigation::{
    SourceIncludeTarget, SourceIndexCache, SourceNavigation, SourceOutlineGroup, builtin_header,
    document_navigation,
};
pub(crate) use source_search::{
    CodeSourceSearchResults, SOURCE_SEARCH_RESULT_LIMIT, commit_source_search_replace,
    open_active_source_search, open_source_search, open_source_search_in_bundle,
    source_search_query_id, source_search_results,
};
#[cfg(test)]
pub(crate) use veriloga::complete_veriloga_file_pick;
pub(crate) use veriloga::{
    SelectedVerilogASource, active_veriloga_file_path, commit_veriloga_source_replacement,
    compile_project_bundle_receipt, compile_project_bundle_virtual_for_provenance,
    import_dropped_veriloga_source, replace_selected_veriloga_file, request_veriloga_compile,
    request_veriloga_source_replacement, selected_veriloga_editor_snapshot,
    selected_veriloga_source, viewed_veriloga_header, writable_libraries,
    write_veriloga_build_profile,
};
pub use veriloga::{consume_veriloga_compile_request, poll_veriloga_compile, poll_veriloga_import};
pub(crate) use veriloga_inspector::{
    VerilogAInspectorAction, show_veriloga_inspector, symbol_drift_text, veriloga_compile_status,
    veriloga_diagnostics, veriloga_receipt,
};
#[cfg(test)]
pub(crate) use veriloga_interface::VerilogAInterfaceOutcome;
pub(crate) use veriloga_interface::{
    VerilogACellSymbol, VerilogAInterfaceFinding, apply_receipt_interface, interface_check_finding,
    interface_finding, interface_mismatch_detail, reconciled_symbol, refresh_symbol_standing,
    symbol_standing,
};

/// Who owns the source bundle a language's editor is showing.
///
/// One owner for one fact, read by every host, command, dialog, and dock.
/// Every Verilog-A source is a cell view, so the Verilog-A source in front of
/// the reader is the active cell view's bundle, and there is none while the
/// active view is not a Verilog-A view. Automation is the project's
/// code-workspace bundle.
pub(crate) fn active_source_owner(
    workspace: &crate::state::ProjectWorkspace,
    language: crate::state::ProjectSourceLanguage,
) -> Option<crate::state::ProjectSourceOwner> {
    match language {
        crate::state::ProjectSourceLanguage::VerilogA => {
            (workspace.content.active_view_type() == crate::state::ViewType::VerilogA).then(|| {
                crate::state::ProjectSourceOwner::cell_view(workspace.content.active_view.clone())
            })
        }
        crate::state::ProjectSourceLanguage::RSpiceAutomation => {
            Some(crate::state::ProjectSourceOwner::code_workspace(language))
        }
    }
}

/// The bundle [`active_source_owner`] names, when the project holds one.
pub(crate) fn active_source_bundle(
    workspace: &crate::state::ProjectWorkspace,
    language: crate::state::ProjectSourceLanguage,
) -> Option<&crate::state::ProjectSourceBundle> {
    workspace
        .content
        .project_sources
        .bundle_for_owner(&active_source_owner(workspace, language)?)
}

/// The cell view whose Verilog-A source a deck's `.veriloga` locator names.
///
/// A deck reaches a module through the key its bundle derives, so the key is
/// read back to the bundle and the bundle to the view that owns it. A key
/// written before the module's bytes changed still names the view: it says
/// whose source the deck meant, which is what opening it needs.
pub(crate) fn veriloga_dependency_view(
    workspace: &crate::state::ProjectWorkspace,
    locator: &str,
) -> Option<crate::state::CellViewRef> {
    match workspace
        .content
        .project_sources
        .veriloga_bundle_for_source_key(workspace.content.project.id(), locator)?
        .owner()
    {
        crate::state::ProjectSourceOwner::CellView { reference } => Some(reference.clone()),
        crate::state::ProjectSourceOwner::CodeWorkspace { .. } => None,
    }
}

/// Whether the Design workspace is showing a Verilog-A cell view, which is
/// where that view's editor lives.
pub(crate) fn design_hosts_veriloga(
    shown: crate::workbench::state::Workspace,
    workspace: &crate::state::ProjectWorkspace,
) -> bool {
    shown == crate::workbench::state::Workspace::Design
        && workspace.content.active_view_type() == crate::state::ViewType::VerilogA
}

/// The source language of the code document commands, dialogs, and labels act
/// on, or `None` for the Netlist page's deck, which is not a project source
/// bundle at all.
///
/// A Verilog-A cell view the Design workspace is showing is that document;
/// everywhere else it is the page the Netlist workspace has selected.
pub(crate) fn active_source_language(
    shown: crate::workbench::state::Workspace,
    workspace: &crate::state::ProjectWorkspace,
    page: CodeWorkspacePage,
) -> Option<crate::state::ProjectSourceLanguage> {
    if design_hosts_veriloga(shown, workspace) {
        return Some(crate::state::ProjectSourceLanguage::VerilogA);
    }
    match page {
        CodeWorkspacePage::Netlist => None,
        CodeWorkspacePage::Automation => {
            Some(crate::state::ProjectSourceLanguage::RSpiceAutomation)
        }
    }
}

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
        CodeWorkspacePage::Automation => source_bundle_document_label(
            workspace,
            code,
            messages,
            crate::state::ProjectSourceLanguage::RSpiceAutomation,
            crate::workbench::MessageId::CodePageAutomation,
        ),
    }
}

/// The bundle file a language page is showing.
///
/// Selection is optional state: a page that has never had a file chosen shows
/// the bundle root. The navigator, the document well, and the lifecycle dialog
/// must agree on which file that is, so they all read it here.
///
/// A Verilog-A selection names its bundle, so a file chosen in one cell view
/// is never read as the selection of another that happens to share the path.
pub(crate) fn active_bundle_path(
    workspace: &crate::state::ProjectWorkspace,
    code: &CodeWorkspaceRuntimeState,
    language: crate::state::ProjectSourceLanguage,
) -> Option<String> {
    let bundle = active_source_bundle(workspace, language);
    let explicit = match language {
        crate::state::ProjectSourceLanguage::VerilogA => code
            .veriloga
            .selected_file
            .as_ref()
            .filter(|selection| {
                bundle.is_some_and(|bundle| {
                    selection.bundle_id == bundle.id()
                        && bundle.contains_file(&selection.logical_path)
                })
            })
            .map(|selection| selection.logical_path.clone()),
        crate::state::ProjectSourceLanguage::RSpiceAutomation => {
            code.automation.selected_file.clone()
        }
    };
    explicit.or_else(|| bundle.map(|bundle| bundle.root().logical_path().to_owned()))
}

/// The id the document well gives the editor showing this bundle file.
///
/// It has to match what the well constructs, including the bundle identity the
/// Verilog-A well mixes in, or a reveal request lands on an editor that is not
/// on screen and the caller silently does nothing. One owner, so a navigator
/// row and a search result cannot disagree about where they are pointing.
pub(crate) fn source_editor_id(
    language: crate::state::ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    logical_path: &str,
) -> egui::Id {
    match language {
        crate::state::ProjectSourceLanguage::VerilogA => {
            egui::Id::new(("workbench.veriloga.source", bundle_id, logical_path))
        }
        crate::state::ProjectSourceLanguage::RSpiceAutomation => {
            egui::Id::new(("workbench.automation.source", logical_path))
        }
    }
}

/// Choose `logical_path` of the bundle in front of the reader, and report the
/// bundle it belongs to so a caller can address the editor that will show it.
///
/// A built-in header open in the editor well gives way to it: choosing a file
/// is choosing to edit. `None` means this project holds no active bundle for
/// `language`, which is the honest answer when a finding outlived its source.
pub(crate) fn select_source_document(
    workspace: &crate::state::ProjectWorkspace,
    code: &mut CodeWorkspaceRuntimeState,
    language: crate::state::ProjectSourceLanguage,
    logical_path: &str,
) -> Option<crate::state::ProjectSourceId> {
    use crate::state::ProjectSourceLanguage;

    let bundle_id =
        active_source_bundle(workspace, language).map(crate::state::ProjectSourceBundle::id)?;
    match language {
        ProjectSourceLanguage::VerilogA => {
            code.veriloga.viewed_header = None;
            code.veriloga.selected_file = Some(page::VerilogAFileSelection {
                bundle_id,
                logical_path: logical_path.to_owned(),
            });
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            code.automation.selected_file = Some(logical_path.to_owned());
        }
    }
    Some(bundle_id)
}

/// The netlist page names the artifact it is editing. A generated primary has
/// no file of its own, so it is named after the cell that produces it and
/// marked as generated rather than borrowing a filename it does not have.
fn netlist_document_label(workspace: &crate::state::ProjectWorkspace) -> String {
    workspace
        .content
        .netlist_descriptor
        .as_ref()
        .map(|descriptor| descriptor.artifact_name.clone())
        .or_else(|| {
            workspace
                .content
                .netlist_source_path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| format!("{}.sp · generated", workspace.content.project.top_cell))
}

/// The selected file in a language bundle, or its root when nothing is
/// explicitly selected. A starter filename is never treated as identity: with
/// no bundle at all the page is named after itself.
fn source_bundle_document_label(
    workspace: &crate::state::ProjectWorkspace,
    code: &CodeWorkspaceRuntimeState,
    messages: crate::workbench::MessageCatalog,
    language: crate::state::ProjectSourceLanguage,
    fallback: crate::workbench::MessageId,
) -> String {
    active_bundle_path(workspace, code, language)
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

    #[test]
    fn browser_worker_script_has_a_separate_compile_protocol() {
        let source = include_str!("../../../web/simulation-worker.js");
        assert!(source.contains("compile-veriloga"));
        assert!(source.contains("runRspiceUiVerilogACompileRequest"));
        assert!(source.contains("veriloga-result"));
        assert!(source.contains("veriloga-error"));
    }

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
        workspace.content.project.top_cell = "afe".to_owned();

        code.page = CodeWorkspacePage::Netlist;
        assert_eq!(
            active_document_label(&workspace, &code, messages()),
            "afe.sp · generated"
        );

        workspace.content.netlist_source_path =
            Some(std::path::PathBuf::from("/p/top_override.sp"));
        assert_eq!(
            active_document_label(&workspace, &code, messages()),
            "top_override.sp"
        );

        code.page = CodeWorkspacePage::Automation;
        assert_ne!(
            active_document_label(&workspace, &code, messages()),
            "top_override.sp",
            "the Automation page must never be named after the SPICE deck"
        );

        // A Verilog-A cell view is a Design document with a tab of its own.
        // Opening one never renames the Netlist workspace's document: the
        // deck stays named after itself, not after the module's source file.
        code.page = CodeWorkspacePage::Netlist;
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        workspace
            .content
            .project_sources
            .insert_bundle(
                crate::state::ProjectSourceBundle::try_new(
                    crate::state::ProjectSourceOwner::cell_view(reference.clone()),
                    crate::state::ProjectSourceLanguage::VerilogA,
                    "models/gain_cell.va",
                    "module gain_cell(p, n); inout p, n; electrical p, n; endmodule\n",
                    [],
                    [],
                )
                .unwrap(),
            )
            .unwrap();
        workspace.open_view(reference, crate::state::ViewType::VerilogA);
        assert_eq!(
            active_document_label(&workspace, &code, messages()),
            "top_override.sp"
        );
    }

    /// Verilog-A has no source of its own outside a cell view: with no
    /// Verilog-A view active there is no Verilog-A bundle in front of the
    /// reader, and with one active it is that view's.
    #[test]
    fn the_veriloga_source_in_front_is_the_active_cell_views_or_none() {
        let (mut workspace, _) = fixture();
        let language = crate::state::ProjectSourceLanguage::VerilogA;
        assert_eq!(active_source_owner(&workspace, language), None);
        assert!(active_source_bundle(&workspace, language).is_none());

        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        workspace.open_view(reference.clone(), crate::state::ViewType::VerilogA);
        assert_eq!(
            active_source_owner(&workspace, language),
            Some(crate::state::ProjectSourceOwner::cell_view(reference))
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
