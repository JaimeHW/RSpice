//! Exact command context for the Netlist & Script Editor workspace.
//!
//! Menus, shortcuts, the command palette, and direct toolbar dispatch must
//! resolve the same project/document authority. The resolver deliberately
//! returns `None` for incomplete, stale, or internally inconsistent state;
//! callers fail closed instead of guessing from a visible label or file
//! extension.

use crate::product::{ContentDigest, ProjectId};
use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};
use crate::workbench::commands::vocabulary::CommandPlatform;
use crate::workbench::documents::code_workspace::CodeWorkspacePage;
use crate::workbench::{RSpiceApp, state::Workspace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CodeDocumentKind {
    GeneratedNetlist,
    OwnedNetlist,
    NetlistComparison,
    NetlistRunSnapshot,
    NetlistInclude,
    VerilogA,
    Automation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CodeDocumentOwnership {
    GeneratedReadOnly,
    ProjectOwned,
    ExternalReadOnly,
    ComparisonReadOnly,
    GovernedReadOnly,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct CodeCommandCapabilities {
    pub find: bool,
    pub validate: bool,
    pub execute: bool,
    pub compare_revisions: bool,
    pub save: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CodeCommandContext {
    pub project_id: ProjectId,
    pub page: CodeWorkspacePage,
    pub document_kind: CodeDocumentKind,
    pub ownership: CodeDocumentOwnership,
    pub document_identity: String,
    pub logical_path: String,
    pub revision: u64,
    pub content_digest: ContentDigest,
    pub capabilities: CodeCommandCapabilities,
    pub platform: CommandPlatform,
}

#[must_use]
pub(crate) fn resolve(app: &RSpiceApp) -> Option<CodeCommandContext> {
    if app.state.workbench.workspace != Workspace::Netlist
        || !app.state.project_lifecycle.project_open
    {
        return None;
    }
    match app.state.ui.code_workspace.page {
        CodeWorkspacePage::Netlist => resolve_netlist(app),
        CodeWorkspacePage::VerilogA => resolve_project_source(app, ProjectSourceLanguage::VerilogA),
        CodeWorkspacePage::Automation => {
            resolve_project_source(app, ProjectSourceLanguage::RSpiceAutomation)
        }
    }
}

#[must_use]
pub(crate) fn resolve_veriloga_compile(app: &RSpiceApp) -> Option<CodeCommandContext> {
    if app.state.workbench.safe_mode.project_read_only() {
        return None;
    }
    let context = resolve(app)?;
    if context.document_kind != CodeDocumentKind::VerilogA
        || app.state.ui.code_workspace.veriloga.pending.is_some()
    {
        return None;
    }
    let selected =
        crate::workbench::documents::code_workspace::selected_veriloga_editor_snapshot(app).ok()?;
    app.state
        .workspace
        .project_sources
        .get_bundle(selected.bundle_id())
        .filter(|bundle| !bundle.root().content().trim().is_empty())?;
    Some(context)
}

fn resolve_netlist(app: &RSpiceApp) -> Option<CodeCommandContext> {
    use crate::workbench::documents::netlist_document::{
        ActiveNetlistDocument, active_dependency, active_dependency_is_owned,
    };

    let state = &app.state;
    let page = CodeWorkspacePage::Netlist;
    let project_id = state.workspace.project.id();
    let active = state.ui.netlist.active_document;
    if let Some(dependency) = active_dependency(state) {
        let source = dependency.source()?;
        let owned = active_dependency_is_owned(state);
        let root = state.ui.netlist.active_dependency_root?;
        if root != active || state.simulation.netlist_content != source {
            return None;
        }
        let root_document = match root {
            ActiveNetlistDocument::Generated => state.ui.netlist.generated_document.as_ref(),
            ActiveNetlistDocument::OwnedSource => state.ui.netlist.owned_document.as_ref(),
            ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
        }?;
        let owned_descriptor = owned.then(|| {
            state
                .workspace
                .netlist_descriptor
                .as_ref()?
                .owned_include(dependency.locator().logical_identity())
        });
        let (document_identity, revision) = if let Some(descriptor) = owned_descriptor.flatten() {
            (descriptor.document_id.to_string(), descriptor.revision)
        } else {
            (
                format!(
                    "{}:{}",
                    root_document.id(),
                    dependency.locator().logical_identity()
                ),
                root_document.revision().get(),
            )
        };
        return Some(CodeCommandContext {
            project_id,
            page,
            document_kind: CodeDocumentKind::NetlistInclude,
            ownership: if owned {
                CodeDocumentOwnership::ProjectOwned
            } else {
                CodeDocumentOwnership::ExternalReadOnly
            },
            document_identity,
            logical_path: dependency.locator().logical_identity().to_owned(),
            revision,
            content_digest: crate::state::content_digest(source),
            capabilities: CodeCommandCapabilities {
                find: true,
                save: owned,
                ..CodeCommandCapabilities::default()
            },
            platform: runtime_platform(),
        });
    }

    let (document_kind, ownership, document_identity, logical_path, revision, source, compare) =
        match active {
            ActiveNetlistDocument::Generated => {
                let document = state.ui.netlist.generated_document.as_ref()?;
                if state.ui.netlist.generated_source != document.source()
                    || state.simulation.netlist_content != document.source()
                {
                    return None;
                }
                (
                    CodeDocumentKind::GeneratedNetlist,
                    CodeDocumentOwnership::GeneratedReadOnly,
                    document.id().to_string(),
                    "generated.sp".to_owned(),
                    document.revision().get(),
                    document.source(),
                    !state.ui.netlist.generated_history.is_empty(),
                )
            }
            ActiveNetlistDocument::OwnedSource => {
                let document = state.ui.netlist.owned_document.as_ref()?;
                let descriptor = state.workspace.netlist_descriptor.as_ref()?;
                let canonical = state.workspace.netlist_document.as_ref()?;
                if state.workspace.netlist_source.as_deref() != Some(document.source())
                    || state.simulation.netlist_content != document.source()
                    || canonical.id() != document.id()
                    || canonical.revision() != document.revision()
                    || canonical.source() != document.source()
                {
                    return None;
                }
                (
                    CodeDocumentKind::OwnedNetlist,
                    CodeDocumentOwnership::ProjectOwned,
                    document.id().to_string(),
                    descriptor.artifact_name.clone(),
                    document.revision().get(),
                    document.source(),
                    !descriptor.revision_history.is_empty(),
                )
            }
            ActiveNetlistDocument::GeneratedDiff => {
                let document = state.ui.netlist.generated_document.as_ref()?;
                if state.ui.netlist.generated_diff_source.is_empty() {
                    return None;
                }
                if state.simulation.netlist_content != state.ui.netlist.generated_diff_source {
                    return None;
                }
                (
                    CodeDocumentKind::NetlistComparison,
                    CodeDocumentOwnership::ComparisonReadOnly,
                    format!("{}:comparison", document.id()),
                    "generated.diff".to_owned(),
                    document.revision().get(),
                    state.ui.netlist.generated_diff_source.as_str(),
                    false,
                )
            }
            // Sealed with its run, exactly like the automation environment
            // lock is sealed with its runtime: read-only, and governed by a
            // record outside the document. Its revision is the run's, read off
            // the run's own receipt rather than restated here.
            ActiveNetlistDocument::RunSnapshot => {
                let run_id =
                    crate::workbench::documents::netlist_document::run_deck_snapshot_run_id(state)?;
                let snapshot = state.ui.netlist.last_run_buffer.as_deref()?;
                if state.simulation.netlist_content != snapshot {
                    return None;
                }
                let revision = state
                    .simulation
                    .run_by_sequence(run_id)?
                    .prepared_receipt()?
                    .project_revision()
                    .get();
                (
                    CodeDocumentKind::NetlistRunSnapshot,
                    CodeDocumentOwnership::GovernedReadOnly,
                    format!("run:{run_id}"),
                    crate::workbench::documents::netlist_document::run_deck_snapshot_artifact_name(
                        state,
                    ),
                    revision,
                    snapshot,
                    false,
                )
            }
        };
    let source_is_present = !source.trim().is_empty();
    let generated_current = active != ActiveNetlistDocument::Generated
        || (state.ui.netlist.generation_error.is_none()
            && state.ui.netlist.generated_input_digest
                == state.ui.netlist.current_generation_input_digest);
    let editable = ownership == CodeDocumentOwnership::ProjectOwned
        && !state.workbench.safe_mode.project_read_only();
    Some(CodeCommandContext {
        project_id,
        page,
        document_kind,
        ownership,
        document_identity,
        logical_path,
        revision,
        content_digest: crate::state::content_digest(source),
        capabilities: CodeCommandCapabilities {
            find: true,
            validate: source_is_present
                && generated_current
                && !matches!(
                    document_kind,
                    CodeDocumentKind::NetlistComparison | CodeDocumentKind::NetlistRunSnapshot
                ),
            execute: source_is_present
                && generated_current
                && !matches!(
                    document_kind,
                    CodeDocumentKind::NetlistComparison | CodeDocumentKind::NetlistRunSnapshot
                )
                && state.manual_deck_run_block_reason().is_none(),
            compare_revisions: compare,
            save: editable,
        },
        platform: runtime_platform(),
    })
}

fn resolve_project_source(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
) -> Option<CodeCommandContext> {
    let state = &app.state;
    let page = match language {
        ProjectSourceLanguage::VerilogA => CodeWorkspacePage::VerilogA,
        ProjectSourceLanguage::RSpiceAutomation => CodeWorkspacePage::Automation,
    };
    let selected_veriloga = (language == ProjectSourceLanguage::VerilogA)
        .then(|| {
            crate::workbench::documents::code_workspace::selected_veriloga_editor_snapshot(app).ok()
        })
        .flatten()
        .filter(|selected| !selected.selection_was_invalid());
    let bundle = match language {
        ProjectSourceLanguage::VerilogA => state
            .workspace
            .project_sources
            .get_bundle(selected_veriloga.as_ref()?.bundle_id())?,
        ProjectSourceLanguage::RSpiceAutomation => {
            let owner = ProjectSourceOwner::code_workspace(language);
            state.workspace.project_sources.bundle_for_owner(&owner)?
        }
    };
    let selected_path = match language {
        ProjectSourceLanguage::VerilogA => selected_veriloga.as_ref()?.active_path().to_owned(),
        ProjectSourceLanguage::RSpiceAutomation => {
            match state.ui.code_workspace.automation.selected_file.as_deref() {
                Some(path) if bundle.contains_file(path) => path.to_owned(),
                Some(_) => return None,
                None => bundle.root().logical_path().to_owned(),
            }
        }
    };
    let source = bundle.file_content(&selected_path)?;
    let document_identity = bundle.document_id(&selected_path)?.to_string();
    let revision = bundle.document_revision(&selected_path)?.get();
    let ownership = if language == ProjectSourceLanguage::RSpiceAutomation
        && bundle.role_for_path(&selected_path)
            == Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock)
    {
        CodeDocumentOwnership::GovernedReadOnly
    } else {
        CodeDocumentOwnership::ProjectOwned
    };
    let busy = match language {
        ProjectSourceLanguage::VerilogA => state.ui.code_workspace.veriloga.pending.is_some(),
        ProjectSourceLanguage::RSpiceAutomation => {
            let automation = &state.ui.code_workspace.automation;
            automation.pending_runtime_validation.is_some()
                || automation.runtime_execution.is_some()
                || automation.execution.is_active()
        }
    };
    let project_writable = !state.workbench.safe_mode.project_read_only();
    Some(CodeCommandContext {
        project_id: state.workspace.project.id(),
        page,
        document_kind: match language {
            ProjectSourceLanguage::VerilogA => CodeDocumentKind::VerilogA,
            ProjectSourceLanguage::RSpiceAutomation => CodeDocumentKind::Automation,
        },
        ownership,
        document_identity,
        logical_path: selected_path,
        revision,
        content_digest: crate::state::content_digest(source),
        capabilities: CodeCommandCapabilities {
            find: true,
            validate: !busy
                && !source.trim().is_empty()
                && (language != ProjectSourceLanguage::VerilogA || project_writable),
            execute: false,
            compare_revisions: false,
            save: ownership == CodeDocumentOwnership::ProjectOwned && !busy && project_writable,
        },
        platform: runtime_platform(),
    })
}

const fn runtime_platform() -> CommandPlatform {
    #[cfg(target_arch = "wasm32")]
    {
        CommandPlatform::Browser
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        CommandPlatform::Desktop
    }
}
