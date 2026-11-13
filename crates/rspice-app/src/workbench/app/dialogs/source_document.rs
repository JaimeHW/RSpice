//! Ownership, persistence, history, and file operations for one project-owned
//! source document.
//!
//! Every operation here is a revision-bound transaction the Code Workspace
//! already implements. This module is the surface that reaches them. Before it
//! existed the whole lifecycle — new, rename, move, duplicate, delete, import,
//! role reassignment, and revision restore — was written, tested, and
//! unreachable, and the "Create source workspace" buttons on the empty
//! Verilog-A and Automation pages opened a transaction that nothing ever drew.
//!
//! A draft is never applied by closing the dialog. Committing is an explicit
//! act, because the operations here rewrite a source graph.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, Grid, RichText, Ui};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{ProjectSourceBundle, ProjectSourceLanguage};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, CommandForm, DIALOG_BODY_INSET, Dialog, DialogChoice, DialogInitialFocus, DialogSize,
    DialogTransactionTone, Problem, band_header, paint_problem_mark,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::{
    LifecycleAction, NetlistLifecycleAction, SourceDocumentRequest,
};
use crate::workbench::design_system::inspector::{muted_inspector_copy, toned_inspector_copy};
use crate::workbench::design_system::{
    PANEL_INSET, code_inspector_section, panel_bleed, panel_body, panel_inset, property_row,
    property_row_height, property_row_input, property_row_problem, property_row_wrapped,
    section_block_gap, subsection_header,
};
use crate::workbench::documents::code_workspace::{
    self, CodeSourceFileAction, CodeWorkspaceParts, CodeWorkspaceRuntimeState,
};
use crate::workbench::documents::netlist_document::{self, NetlistDocumentState};
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::{MessageCatalog, MessageId};

/// Side of a leading problem mark on an include row, and the advance from its
/// left edge to the text behind it. Read from the kit rather than restated
/// here, so an include row's mark is the mark every other panel sets.
use crate::ui::widgets::{
    PROBLEM_MARK_ADVANCE as INCLUDE_MARK_ADVANCE, PROBLEM_MARK_SIDE as INCLUDE_MARK_SIDE,
};

/// The shell's `open_source_document_dialog`, over the workspace in front, the model,
/// the Code workspace's runtime state, the netlist editor's state and the
/// platform's state.
pub(crate) fn open_source_document_dialog_in(
    workspace: Workspace,
    model: &Model,
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
) -> Result<(), String> {
    let Some(language) = visible_language_in(workspace, model, code) else {
        if !platform.project_lifecycle.is_open() {
            return Err("Open a project before managing its top decks.".to_owned());
        }
        if let Some(refusal) = generated_deck_refusal(model, netlist) {
            return Err(refusal.to_owned());
        }
        code.source_document_dialog = true;
        return Ok(());
    };
    if model.workspace.active_source_bundle(language).is_none() {
        return Err(format!(
            "This project owns no {} source workspace yet.",
            language.label()
        ));
    }
    code.source_document_dialog = true;
    Ok(())
}

/// The netlist page's New source document and New top deck: the dialog on
/// its review of a new top deck. Unlike Manage source document it opens on
/// a project that owns no deck yet, the one [`generated_deck_refusal`]
/// sends here. It reads the model and the platform's state, and writes the
/// Code workspace's runtime state and the netlist editor's state.
pub(crate) fn begin_new_top_deck_in(
    model: &Model,
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
    platform: &PlatformState,
) -> Result<(), String> {
    if !platform.project_lifecycle.is_open() {
        return Err("Open a project before creating its top deck.".to_owned());
    }
    crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action_in(
        model,
        netlist,
        CodeSourceFileAction::New,
    )?;
    code.source_document_dialog = true;
    Ok(())
}

/// Whether the lifecycle dialog can be opened for the visible page.
pub(crate) fn source_document_dialog_is_available(
    workspace: Workspace,
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
) -> bool {
    match visible_language_in(workspace, model, code) {
        None => {
            platform.project_lifecycle.is_open() && generated_deck_refusal(model, netlist).is_none()
        }
        Some(language) => model.workspace.active_source_bundle(language).is_some(),
    }
}

/// Why the netlist page has no source document to manage: the deck on it is
/// generated from the schematic and read-only, and the project owns no deck
/// or include of its own. The page's Create source makes one.
pub(crate) fn generated_deck_refusal(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<&'static str> {
    let content = &model.workspace.content;
    let owned = content.netlist_descriptor.is_some()
        || !content.retained_netlist_decks.is_empty()
        || netlist_document::active_dependency_in(model, netlist).is_some();
    (!owned).then_some("The generated netlist is read-only. Create a source deck to manage one.")
}

/// The source language of the document the dialog manages: a Verilog-A cell
/// view open in Design, else the Netlist workspace's page, where `None` is the
/// deck. It reads the workspace in front, the model and the Code workspace's
/// runtime state.
fn visible_language_in(
    workspace: Workspace,
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
) -> Option<ProjectSourceLanguage> {
    code_workspace::active_source_language(workspace, &model.workspace, code.page)
}

/// What the dialog states about the visible document.
///
/// Read once per frame into owned values so applying a transaction cannot
/// invalidate a borrow the body is still rendering from, and so every row is
/// measured from the same closure.
pub(crate) struct DocumentFacts {
    pub(crate) language: ProjectSourceLanguage,
    pub(crate) logical_path: String,
    leaf: String,
    role: String,
    is_entry: bool,
    is_environment_lock: bool,
    editable: bool,
    pub(crate) is_root: bool,
    line_endings: MessageId,
    validation_current: bool,
    imports: Vec<String>,
    imported_by: Vec<String>,
    history: Vec<RevisionRow>,
    block_reason: Option<&'static str>,
}

struct RevisionRow {
    revision: u64,
    digest: String,
    files: usize,
    current: bool,
}

pub(crate) struct NetlistLifecycleFacts {
    pub(crate) deck_id: Option<uuid::Uuid>,
    logical_path: String,
    line_endings: MessageId,
    /// The file on disk the deck or include was saved to or opened from;
    /// `None` for one that lives only inside the project.
    native_path: Option<String>,
    history_count: usize,
    pub(crate) decks: Vec<(uuid::Uuid, String)>,
    operations_blocked: Option<String>,
    include_identity: Option<String>,
    include_owned: bool,
}

pub(crate) fn netlist_lifecycle_facts(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<NetlistLifecycleFacts> {
    let descriptor = model.workspace.content.netlist_descriptor.as_ref();
    let document = model.workspace.content.netlist_document.as_ref();
    if descriptor.is_some() != document.is_some() {
        return None;
    }
    let mut decks = Vec::with_capacity(
        model.workspace.content.retained_netlist_decks.len() + usize::from(descriptor.is_some()),
    );
    if let Some(descriptor) = descriptor {
        decks.push((descriptor.deck_id, descriptor.artifact_name.clone()));
    }
    decks.extend(
        model
            .workspace
            .content
            .retained_netlist_decks
            .iter()
            .map(|deck| {
                (
                    deck.descriptor.deck_id,
                    deck.descriptor.artifact_name.clone(),
                )
            }),
    );
    decks.sort_by(|left, right| {
        left.1
            .to_ascii_lowercase()
            .cmp(&right.1.to_ascii_lowercase())
    });
    let active_dependency = netlist_document::active_dependency_in(model, netlist);
    let native_path = active_dependency.map_or_else(
        || {
            model
                .workspace
                .content
                .netlist_source_path
                .as_ref()
                .map(|path| path.display().to_string())
        },
        |dependency| dependency.locator().native_origin().map(str::to_owned),
    );
    let include_owned = active_dependency.is_some()
        && netlist_document::active_dependency_is_owned_in(model, netlist);
    let root_path = descriptor.map_or_else(
        || "No top deck".to_owned(),
        |descriptor| descriptor.artifact_name.clone(),
    );
    Some(NetlistLifecycleFacts {
        deck_id: descriptor.map(|descriptor| descriptor.deck_id),
        logical_path: active_dependency.map_or_else(
            || root_path.clone(),
            |dependency| dependency.locator().display_name().to_owned(),
        ),
        line_endings: active_dependency
            .and_then(crate::state::DependencyMetadata::source)
            .map_or_else(
                || {
                    document.map_or(MessageId::CodeSourceLineEndingsNone, |document| {
                        line_ending_summary(document.source())
                    })
                },
                line_ending_summary,
            ),
        native_path,
        history_count: descriptor.map_or(0, |descriptor| descriptor.revision_history.len()),
        decks,
        operations_blocked: active_dependency.and_then(|_| {
            (!include_owned).then(|| {
                "Copy this include into the project before changing its document ownership."
                    .to_owned()
            })
        }),
        include_identity: active_dependency
            .map(|dependency| dependency.locator().logical_identity().to_owned()),
        include_owned,
    })
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    digest.to_string().chars().take(12).collect()
}

pub(in crate::workbench) fn leaf_name(logical_path: &str) -> String {
    logical_path
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(logical_path)
        .to_owned()
}

/// Line endings as they actually are in the retained bytes, not as a policy
/// the writer intends to apply. A mixed document is worth saying out loud
/// because a whole-file rewrite silently normalizes it.
fn line_ending_summary(content: &str) -> MessageId {
    let carriage_return_line_feed = content.matches("\r\n").count();
    let line_feed = content.matches('\n').count() - carriage_return_line_feed;
    match (carriage_return_line_feed, line_feed) {
        (0, 0) => MessageId::CodeSourceLineEndingsNone,
        (0, _) => MessageId::CodeSourceLineEndingsLf,
        (_, 0) => MessageId::CodeSourceLineEndingsCrLf,
        _ => MessageId::CodeSourceLineEndingsMixed,
    }
}

fn document_facts(
    workspace: Workspace,
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
    messages: MessageCatalog,
) -> Option<DocumentFacts> {
    let language = visible_language_in(workspace, model, code)?;
    let logical_path = code_workspace::active_bundle_path(&model.workspace, code, language)?;
    let bundle = model.workspace.active_source_bundle(language)?;
    let content = bundle.file_content(&logical_path).unwrap_or_default();
    let history = bundle
        .revision_history()
        .iter()
        .map(|snapshot| RevisionRow {
            revision: snapshot.revision().get(),
            digest: short_digest(snapshot.closure_digest()),
            // A closure is its root plus the files beside it, and `files()`
            // holds only the latter.
            files: snapshot.files().len() + 1,
            current: snapshot.closure_digest() == bundle.closure_digest(),
        })
        .collect();
    Some(DocumentFacts {
        language,
        leaf: leaf_name(&logical_path),
        role: role_label(messages, bundle.role_for_path(&logical_path)),
        is_entry: bundle.role_for_path(&logical_path)
            == Some(crate::state::ProjectSourceRole::AutomationEntry),
        is_environment_lock: bundle.role_for_path(&logical_path)
            == Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock),
        editable: code_workspace::source_bundle_document_is_editable_in(
            model,
            code,
            platform,
            language,
            bundle.id(),
            &logical_path,
        ),
        is_root: crate::state::project_source_paths_equal(
            &logical_path,
            bundle.root().logical_path(),
        ),
        line_endings: line_ending_summary(content),
        validation_current: bundle.validation_is_current(),
        imports: dependency_paths(bundle, &logical_path, true),
        imported_by: dependency_paths(bundle, &logical_path, false),
        history,
        block_reason: code_workspace::source_file_mutation_block_reason_in(
            model, code, platform, language,
        ),
        logical_path,
    })
}

/// The include edges the bundle records for this document, in the requested
/// direction. These are persisted dependency records, never a re-parse of the
/// text, so the dialog cannot claim an edge the project does not hold.
fn dependency_paths(
    bundle: &ProjectSourceBundle,
    logical_path: &str,
    as_importer: bool,
) -> Vec<String> {
    bundle
        .dependencies()
        .iter()
        .filter_map(|dependency| {
            let (matched, other) = if as_importer {
                (dependency.importer(), dependency.imported())
            } else {
                (dependency.imported(), dependency.importer())
            };
            crate::state::project_source_paths_equal(matched, logical_path)
                .then(|| other.to_owned())
        })
        .collect()
}

/// Which section states a failure, chosen by the action that caused it.
#[derive(Clone, Copy)]
enum Report {
    FileTransaction,
    History,
    Console,
}

/// The roles an Automation document can be given here.
///
/// The entry role is the bundle's own identity, bound by the closure rather
/// than reassigned per file, so it is not offered. Everything else is a
/// project decision the workspace is allowed to change.
const ASSIGNABLE_AUTOMATION_ROLES: [Option<crate::state::ProjectSourceRole>; 4] = [
    None,
    Some(crate::state::ProjectSourceRole::AutomationRunPlan),
    Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock),
    Some(crate::state::ProjectSourceRole::AutomationPermissionManifest),
];

/// One spelling of a document's role, so the property row and the picker can
/// never disagree about which entry is currently selected.
///
/// The picker resolves a selection by matching the label it drew, so the label
/// has to come from the same locale the row was painted in.
fn role_label(messages: MessageCatalog, role: Option<crate::state::ProjectSourceRole>) -> String {
    messages.text(
        role.map_or(MessageId::CodeSourceRoleOrdinary, |role| match role {
            crate::state::ProjectSourceRole::VerilogABuildProfile => {
                MessageId::CodeSourceRoleBuildProfile
            }
            crate::state::ProjectSourceRole::AutomationEntry => {
                MessageId::CodeSourceRoleAutomationEntry
            }
            crate::state::ProjectSourceRole::AutomationRunPlan => MessageId::CodeSourceRoleRunPlan,
            crate::state::ProjectSourceRole::AutomationEnvironmentLock => {
                MessageId::CodeSourceRoleEnvironmentLock
            }
            crate::state::ProjectSourceRole::AutomationPermissionManifest => {
                MessageId::CodeSourceRolePermissionManifest
            }
        }),
    )
}

/// Close the document lifecycle dialog and the drafts it opened.
pub(in crate::workbench) fn close_document_lifecycle(code: &mut CodeWorkspaceRuntimeState) {
    code.source_document_dialog = false;
    code.source_file_dialog = None;
    code.source_history = None;
}

/// Close the netlist lifecycle dialog and the transactions it opened.
pub(crate) fn close_netlist_lifecycle(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
) {
    code.source_document_dialog = false;
    netlist.lifecycle_dialog.transaction = None;
    netlist.lifecycle_dialog.include_transaction = None;
}

/// Draw the source document dialogs over the Code workspace's runtime state,
/// the netlist editor's state, the model, the workbench session and the
/// platform's state: the Automation workspace's draft while one is open, else
/// the lifecycle of the document on the page in front. What a choice still
/// needs of the whole application is returned for the shell to carry out.
pub(in crate::workbench) fn render_source_document_dialog(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
    ctx: &Context,
) -> Option<SourceDocumentRequest> {
    if code.source_workspace_dialog.is_some() {
        return render_source_workspace_dialog(code, wb.messages(), ctx);
    }
    if !code.source_document_dialog {
        return None;
    }
    if visible_language_in(wb.workspace, model, code).is_none() {
        return render_netlist_source_document_dialog(code, netlist, model, wb, ctx);
    }
    let messages = wb.messages();
    let Some(facts) = document_facts(wb.workspace, model, code, platform, messages) else {
        // The bundle or the page went away underneath an open dialog.
        code.source_document_dialog = false;
        return None;
    };

    let mut dialog = Dialog::new(
        messages.text(MessageId::CodeSourceLifecycleTitle),
        messages.text(MessageId::CommonClose),
    )
    .description(messages.format(
        MessageId::CodeSourceLifecycleDescription,
        &[("document", &facts.leaf)],
    ))
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl);
    if let Some(reason) = facts.block_reason {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Progress,
            messages.text(MessageId::CodeSourceOperationsHeld),
            reason,
        );
    } else if !facts.editable {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Progress,
            messages.text(MessageId::CodeSourceReadOnlyDocument),
            messages.text(MessageId::CodeSourceReadOnlyDocumentDetail),
        );
    }

    let mut action = LifecycleAction::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        action = lifecycle_body(ui, code, &facts, messages);
        None
    });
    let close = matches!(
        choice,
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
    );
    match action {
        LifecycleAction::None => {}
        LifecycleAction::Cancel => code.source_file_dialog = None,
        LifecycleAction::SelectRevision(revision) => {
            if let Some(history) = code.source_history.as_mut() {
                history.selected_revision = Some(revision);
                history.error = None;
            }
        }
        LifecycleAction::CloseHistory => code.source_history = None,
        // The rest write the project's sources, so the shell carries them
        // out, and then the close this frame asked for.
        action => {
            return Some(SourceDocumentRequest::Document {
                action,
                language: facts.language,
                logical_path: facts.logical_path,
                close,
            });
        }
    }
    if close {
        close_document_lifecycle(code);
    }
    None
}

/// Carry out a lifecycle choice on the Verilog-A or Automation document at
/// `logical_path` that the dialog handed the shell.
pub(crate) fn apply_lifecycle_action(
    parts: &mut CodeWorkspaceParts<'_>,
    action: LifecycleAction,
    language: ProjectSourceLanguage,
    logical_path: &str,
) {
    // Where a failure is stated is part of the action, not a guess made
    // afterwards: a rejected rename belongs beside the path field, and a
    // rejected restore beside the revision list. Deciding by whichever
    // draft happens to be open would put one inside the other's section.
    let (outcome, report) = match action {
        // Nothing to carry out, or a choice the form applies over the Code
        // workspace's own state (Cancel, SelectRevision, CloseHistory):
        // the form never hands these over.
        LifecycleAction::None
        | LifecycleAction::Cancel
        | LifecycleAction::SelectRevision(_)
        | LifecycleAction::CloseHistory => return,
        LifecycleAction::Begin(file_action) => (
            code_workspace::open_source_file_dialog(
                parts,
                language,
                file_action,
                logical_path,
                logical_path,
            ),
            Report::Console,
        ),
        LifecycleAction::Commit => (
            code_workspace::commit_source_file_dialog(parts).map(|_| ()),
            Report::FileTransaction,
        ),
        LifecycleAction::OpenHistory => (
            code_workspace::open_source_history(parts, language, logical_path),
            Report::Console,
        ),
        LifecycleAction::Restore => (
            code_workspace::commit_source_history_restore(parts).map(|_| ()),
            Report::History,
        ),
        LifecycleAction::Import => (
            code_workspace::request_automation_source_import(parts, logical_path),
            Report::Console,
        ),
        LifecycleAction::ReplaceFromFile => {
            let outcome = code_workspace::request_veriloga_source_replacement(parts, logical_path);
            if outcome.is_ok() {
                // The picker and then the replacement's own confirmation
                // take over; this dialog would only sit behind them.
                parts.code.source_document_dialog = false;
            }
            (outcome, Report::Console)
        }
        LifecycleAction::AssignRole(role) => (
            code_workspace::assign_automation_source_role(parts, logical_path, role).map(|_| ()),
            Report::Console,
        ),
    };
    let Err(error) = outcome else {
        return;
    };
    let stated = match report {
        Report::FileTransaction => parts
            .code
            .source_file_dialog
            .as_mut()
            .map(|draft| draft.error = Some(error.clone())),
        Report::History => parts
            .code
            .source_history
            .as_mut()
            .map(|history| history.error = Some(error.clone())),
        Report::Console => None,
    };
    // Nothing owns the failure when the draft it belonged to was cleared
    // by the same call, so the console states it instead of losing it.
    if stated.is_none() {
        parts
            .model
            .record(LogSeverity::Error, LogSource::Studio, error);
    }
}

fn render_netlist_source_document_dialog(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    ctx: &Context,
) -> Option<SourceDocumentRequest> {
    let messages = wb.messages();
    let Some(facts) = netlist_lifecycle_facts(model, netlist) else {
        close_netlist_lifecycle(code, netlist);
        return None;
    };
    if facts.decks.is_empty() && facts.include_identity.is_none() {
        return render_new_top_deck_form(code, netlist, ctx, messages);
    }
    let mut dialog = Dialog::new(
        messages.text(MessageId::CodeSourceLifecycleTitle),
        messages.text(MessageId::CommonClose),
    )
    .description(messages.format(
        MessageId::CodeSourceLifecycleDescription,
        &[("document", &facts.logical_path)],
    ))
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    // The rows fit at every desktop size; a bar would only reserve a
    // gutter that left the right edge wider than the left.
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl);
    if let Some(reason) = facts.operations_blocked.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Progress,
            messages.text(MessageId::CodeSourceOperationsHeld),
            reason,
        );
    }

    let mut action = NetlistLifecycleAction::None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        action = netlist_lifecycle_body(ui, netlist, code, model, &facts, messages);
        None
    });
    let close = matches!(
        choice,
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
    );
    match action {
        NetlistLifecycleAction::None => {}
        NetlistLifecycleAction::Cancel => {
            netlist.lifecycle_dialog.transaction = None;
        }
        NetlistLifecycleAction::OpenHistory => {
            netlist.comparison_dialog.open = true;
            netlist.comparison_dialog.selected_history_index =
                facts.history_count.saturating_sub(1);
            code.source_document_dialog = false;
        }
        NetlistLifecycleAction::CancelInclude => {
            netlist.lifecycle_dialog.include_transaction = None;
        }
        NetlistLifecycleAction::SetIncludeSearchPaths(paths) => {
            if let Err(error) = model
                .workspace
                .content
                .project
                .set_include_search_paths(paths)
                .map_err(|error| error.to_string())
            {
                wb.prompt(error);
            } else {
                model.workspace.content.project_metadata_dirty = true;
                // The chain decides which file every relative include
                // resolves to, so the deck's diagnostics are no longer
                // evidence about the chain that is now in force.
                model.invalidate_source_evidence();
            }
        }
        // The rest write the project's decks and includes, so the shell
        // carries them out, and then the close this frame asked for.
        action => return Some(SourceDocumentRequest::Netlist { action, close }),
    }
    if close {
        close_netlist_lifecycle(code, netlist);
    }
    None
}

/// The transaction that creates the Automation workspace a project has
/// none of. It is opened straight from the empty state, so it renders
/// whenever the draft exists rather than waiting for the lifecycle dialog
/// to be asked for.
fn render_source_workspace_dialog(
    code: &mut CodeWorkspaceRuntimeState,
    messages: MessageCatalog,
    ctx: &Context,
) -> Option<SourceDocumentRequest> {
    let draft = code.source_workspace_dialog.clone()?;
    let language = ProjectSourceLanguage::RSpiceAutomation.label();
    let mut dialog = Dialog::new(
        messages.text(MessageId::CodeSourceWorkspaceTitle),
        messages.text(MessageId::CodeSourceWorkspacePrimary),
    )
    .description(messages.format(
        MessageId::CodeSourceWorkspaceDescription,
        &[("language", language)],
    ))
    .size(DialogSize::Transaction)
    .ghost(messages.text(MessageId::CommonCancel));
    if let Some(error) = draft.error.as_deref() {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            messages.text(MessageId::CodeSourceWorkspaceRefused),
            error,
        );
    }

    let mut edited = draft.clone();
    let choice = dialog.show(ctx, |ui| {
        code_inspector_section(
            ui,
            &messages.text(MessageId::CodeSourceWorkspaceClosure),
            None,
            None,
            |ui| {
                property_row_input(
                    ui,
                    &messages.text(MessageId::CodeSourceWorkspaceRootPath),
                    &mut edited.root_path,
                    false,
                );
                property_row_input(
                    ui,
                    &messages.text(MessageId::CodeSourceWorkspaceRunPlanPath),
                    &mut edited.run_plan_path,
                    false,
                );
                property_row_input(
                    ui,
                    &messages.text(MessageId::CodeSourceWorkspaceEnvironmentLockPath),
                    &mut edited.environment_lock_path,
                    false,
                );
                property_row_input(
                    ui,
                    &messages.text(MessageId::CodeSourceWorkspacePermissionPath),
                    &mut edited.permission_manifest_path,
                    false,
                );
                property_row(
                    ui,
                    &messages.text(MessageId::CodeSourceWorkspaceBoundDeck),
                    &edited.netlist_source_path,
                );
            },
        );
    });
    if let Some(state) = code.source_workspace_dialog.as_mut() {
        state.root_path = edited.root_path;
        state.run_plan_path = edited.run_plan_path;
        state.environment_lock_path = edited.environment_lock_path;
        state.permission_manifest_path = edited.permission_manifest_path;
    }
    match choice {
        // Creating the workspace writes the project's sources, so the
        // shell carries it out.
        DialogChoice::Primary => Some(SourceDocumentRequest::CreateWorkspace),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            code.source_workspace_dialog = None;
            None
        }
        DialogChoice::None | DialogChoice::Secondary => None,
    }
}

/// New source document on a project that owns no deck: a form that makes
/// one. There is no current document to state and nothing to operate on,
/// so the lifecycle's sections would be a page of disabled controls.
fn render_new_top_deck_form(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
    ctx: &Context,
    messages: MessageCatalog,
) -> Option<SourceDocumentRequest> {
    let Some(transaction) = netlist
        .lifecycle_dialog
        .transaction
        .as_mut()
        .filter(|transaction| transaction.action == CodeSourceFileAction::New)
    else {
        code.source_document_dialog = false;
        netlist.lifecycle_dialog.transaction = None;
        return None;
    };
    let title = messages.text(MessageId::CodeSourceCreateTitle);
    let primary = messages.text(MessageId::CodeSourceCreatePrimary);
    let path_label = messages.text(MessageId::CodeSourceLogicalPath);
    let against_label = messages.text(MessageId::CodeSourceAgainst);
    let against = messages.text(MessageId::CodeSourceAgainstEmptyNetlist);
    let needs_path = messages.text(MessageId::CodeSourceCreateNeedsPath);
    let error = transaction.error.clone();
    let path_missing = transaction.proposed_path.trim().is_empty();
    let status = match (error.as_deref(), path_missing) {
        (Some(error), _) => Err(error),
        (None, true) => Ok(Some(needs_path.as_str())),
        (None, false) => Ok(None),
    };
    let choice = CommandForm::new(&title, &primary)
        .primary_enabled(!path_missing)
        .status(status)
        .show(ctx, |rows| {
            let field = rows.text(
                &path_label,
                egui::Id::new("netlist-new-top-deck-path"),
                &mut transaction.proposed_path,
                "",
            );
            if field.changed() {
                // A refusal names the path it refused.
                transaction.error = None;
            }
            rows.value(&against_label, &against);
            Some(field.id)
        });
    match choice {
        DialogChoice::Primary => return Some(SourceDocumentRequest::CreateTopDeck),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            code.source_document_dialog = false;
            netlist.lifecycle_dialog.transaction = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    None
}

/// The sections of a Manage source document dialog, stacked down its flush
/// body.
fn sections<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        add(ui)
    })
    .inner
}

/// One section of a Manage source document dialog: the dialog's band across
/// its flush body, then `body` on the panel rhythm with every row on the left
/// edge of the dialog's title and footer.
fn dialog_section(ui: &mut Ui, title: &str, meta: Option<&str>, body: impl FnOnce(&mut Ui)) {
    band_header(ui, title, meta);
    // The panel body insets its rows by its own inset; this frame makes up
    // the difference to the dialog's.
    let edge = (DIALOG_BODY_INSET - PANEL_INSET) as i8;
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(edge, 0))
        .show(ui, |ui| {
            panel_body(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                body(ui);
            });
        });
}

fn lifecycle_body(
    ui: &mut Ui,
    code: &mut CodeWorkspaceRuntimeState,
    facts: &DocumentFacts,
    messages: MessageCatalog,
) -> LifecycleAction {
    let mut action = LifecycleAction::None;
    // The dialog's body scrolls when the sections outgrow it; a scroll area
    // of its own here held the dialog at full height and withheld a gutter.
    sections(ui, |ui| {
        if let Some(chosen) = current_document(ui, facts, messages) {
            action = chosen;
        }
        operations(ui, facts, &mut action, messages);
        if let Some(chosen) = operation_review(ui, code, messages) {
            action = chosen;
        }
        includes(ui, facts, messages);
        if let Some(chosen) = revision_history(ui, code, facts, messages) {
            action = chosen;
        }
    });
    action
}

fn netlist_lifecycle_body(
    ui: &mut Ui,
    netlist: &mut NetlistDocumentState,
    code: &mut CodeWorkspaceRuntimeState,
    model: &Model,
    facts: &NetlistLifecycleFacts,
    messages: MessageCatalog,
) -> NetlistLifecycleAction {
    let mut action = NetlistLifecycleAction::None;
    // The dialog's body scrolls when a review opened below the rows outgrows
    // it; a scroll area of its own here only withheld a gutter on the right.
    sections(ui, |ui| {
        dialog_section(
            ui,
            &messages.text(MessageId::CodeSourceCurrentDocument),
            None,
            |ui| {
                if facts.include_identity.is_some() {
                    property_row(
                        ui,
                        &messages.text(MessageId::NetlistLogicalIdentity),
                        facts.include_identity.as_deref().unwrap_or_default(),
                    );
                } else if facts.decks.len() <= 1 {
                    // One deck is nothing to choose between.
                    property_row(
                        ui,
                        &messages.text(MessageId::NetlistTopDeck),
                        &facts.logical_path,
                    );
                } else {
                    let choices = facts
                        .decks
                        .iter()
                        .map(|(_, path)| (path.clone(), path.clone()))
                        .collect::<Vec<_>>();
                    let mut selected = facts.logical_path.clone();
                    if crate::workbench::design_system::property_row_combo(
                        ui,
                        &messages.text(MessageId::NetlistTopDeck),
                        "netlist-top-deck-selector",
                        &mut selected,
                        &choices,
                        facts.operations_blocked.is_none()
                            && netlist.lifecycle_dialog.transaction.is_none(),
                    ) && let Some((deck_id, _)) =
                        facts.decks.iter().find(|(_, path)| *path == selected)
                        && Some(*deck_id) != facts.deck_id
                    {
                        action = NetlistLifecycleAction::SelectDeck(*deck_id);
                    }
                }
                // The deck's name inside the project is the row above;
                // a path is stated only where it says something new,
                // the file on disk it was saved to or opened from.
                if facts.include_identity.is_some() {
                    property_row(
                        ui,
                        &messages.text(MessageId::CodeSourcePath),
                        facts.native_path.as_deref().unwrap_or(&facts.logical_path),
                    );
                } else if let Some(path) = facts.native_path.as_deref() {
                    property_row(ui, &messages.text(MessageId::CodeSourcePath), path);
                }
                property_row_wrapped(
                    ui,
                    &messages.text(MessageId::CodeSourceLineEndings),
                    &messages.text(facts.line_endings),
                );
            },
        );
        dialog_section(
            ui,
            &messages.text(MessageId::CodeSourceFileOperations),
            None,
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                    let transaction_open = netlist.lifecycle_dialog.transaction.is_some()
                        || netlist.lifecycle_dialog.include_transaction.is_some();
                    if facts.include_identity.is_some() {
                        if Button::new(&messages.text(MessageId::NetlistCopyProject))
                            .enabled(!facts.include_owned && !transaction_open)
                            .show(ui)
                            .clicked()
                        {
                            action = NetlistLifecycleAction::CopyIncludeOwnership;
                        }
                        for (label, candidate) in [
                            (MessageId::CodeSourceRename, CodeSourceFileAction::Rename),
                            (MessageId::CodeSourceMove, CodeSourceFileAction::Move),
                        ] {
                            let response = Button::new(&messages.text(label))
                                .enabled(facts.include_owned && !transaction_open)
                                .show(ui);
                            if response.clicked() {
                                action = NetlistLifecycleAction::BeginInclude(candidate);
                            }
                        }
                        let response =
                            Button::new(&messages.text(MessageId::NetlistReleaseProjectOwnership))
                                .enabled(facts.include_owned && !transaction_open)
                                .show(ui);
                        if response.clicked() {
                            action = NetlistLifecycleAction::ReleaseIncludeOwnership;
                        }
                        if !facts.include_owned {
                            response.on_disabled_hover_text(
                                facts.operations_blocked.as_deref().unwrap_or(
                                    "This include is already a read-only referenced source.",
                                ),
                            );
                        }
                        ui.label(messages.text(MessageId::NetlistReleaseOwnershipDescription));
                    } else {
                        if Button::new(&messages.text(if cfg!(target_arch = "wasm32") {
                            MessageId::NetlistDownloadSourceCopy
                        } else {
                            MessageId::NetlistSaveSourceAs
                        }))
                        .enabled(facts.deck_id.is_some() && !transaction_open)
                        .show(ui)
                        .clicked()
                        {
                            action = NetlistLifecycleAction::SaveSourceAs;
                        }
                        for (label, candidate) in FILE_OPERATIONS {
                            let enabled = facts.operations_blocked.is_none()
                                && !transaction_open
                                && (facts.deck_id.is_some()
                                    || candidate == CodeSourceFileAction::New);
                            let response = Button::new(&messages.text(label))
                                .enabled(enabled)
                                .destructive(candidate == CodeSourceFileAction::Delete)
                                .show(ui);
                            if response.clicked() {
                                action = NetlistLifecycleAction::Begin(candidate);
                            }
                            if let Some(reason) = facts.operations_blocked.as_deref() {
                                response.on_disabled_hover_text(reason);
                            }
                        }
                        if Button::new(&messages.text(MessageId::CodeSourceRevisionHistory))
                            .enabled(
                                facts.history_count > 0
                                    && netlist.lifecycle_dialog.transaction.is_none(),
                            )
                            .show(ui)
                            .clicked()
                        {
                            action = NetlistLifecycleAction::OpenHistory;
                        }
                    }
                });
            },
        );
        if let Some(chosen) = include_search_paths(ui, code, model, messages) {
            action = chosen;
        }
        if let Some(chosen) = netlist_operation_review(ui, netlist, messages) {
            action = chosen;
        }
        if let Some(chosen) = netlist_include_operation_review(ui, netlist, messages) {
            action = chosen;
        }
    });
    action
}

/// The project's ordered include search chain.
///
/// Order is the whole setting: the resolver walks it front to back, so which
/// of two files of the same relative name a deck gets is decided here. Every
/// row states whether its directory is present, in place, because a chain
/// entry that silently does not exist is the failure this list exists to make
/// visible.
fn include_search_paths(
    ui: &mut Ui,
    code: &mut CodeWorkspaceRuntimeState,
    model: &Model,
    messages: MessageCatalog,
) -> Option<NetlistLifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let chain = rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
        model.workspace.content.project.include_search_paths(),
        model.workspace.content.project.data_root(),
    );
    let authored = model
        .workspace
        .content
        .project
        .include_search_paths()
        .to_vec();
    let unsaved = model.workspace.content.project.data_root().is_none();
    let mut action = None;
    dialog_section(
        ui,
        &messages.text(MessageId::NetlistIncludeSearchPaths),
        Some(&messages.format(
            MessageId::NetlistIncludeSearchPathCount,
            &[("count", &authored.len().to_string())],
        )),
        |ui| {
            // The chain goes on into the library folders set in Preferences,
            // so a list that ended here would misstate where a name is found.
            let library_folders = !model.library_folders().is_empty();
            if authored.is_empty() {
                muted_inspector_copy(
                    ui,
                    &messages.text(if library_folders {
                        MessageId::NetlistIncludeSearchPathEmptyWithLibraryFolders
                    } else {
                        MessageId::NetlistIncludeSearchPathEmpty
                    }),
                );
            }
            let inset = panel_inset(ui);
            for (index, entry) in chain.entries().iter().enumerate() {
                let selected = code.include_search_selected == Some(index);
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), property_row_height(ui)),
                    egui::Sense::click(),
                );
                // The row is a painted galley, so nothing here reaches a screen
                // reader on its own, and a clickable sense makes it a tab stop
                // that would otherwise take focus silently and invisibly.
                response.widget_info(|| {
                    egui::WidgetInfo::selected(
                        egui::WidgetType::SelectableLabel,
                        ui.is_enabled(),
                        selected,
                        format!("{}. {}", index + 1, entry.authored().display()),
                    )
                });
                if ui.is_rect_visible(rect) {
                    if selected {
                        ui.painter()
                            .rect_filled(panel_bleed(ui, rect), 0.0, t.color.accent_dim);
                    }
                    // A path that resolves is every path's normal state, so
                    // only one that does not is marked and named.
                    let missing =
                        rspice_simulation::netlist_preparation::IncludeSearchChain::states_presence(
                        ) && !entry.exists();
                    let state_text = if missing {
                        messages.text(MessageId::NetlistIncludeSearchPathMissing)
                    } else {
                        String::new()
                    };
                    let state_font = theme::sans(tokens::FS_0, FontWeight::Regular);
                    let mut state_width = ui
                        .painter()
                        .layout_no_wrap(state_text.clone(), state_font.clone(), t.color.text_dim)
                        .size()
                        .x;
                    ui.painter().text(
                        egui::pos2(rect.right() - inset, rect.center().y),
                        egui::Align2::RIGHT_CENTER,
                        &state_text,
                        state_font,
                        t.color.text_dim,
                    );
                    if missing {
                        paint_problem_mark(
                            ui.painter(),
                            egui::Rect::from_center_size(
                                egui::pos2(
                                    rect.right() - inset - state_width - INCLUDE_MARK_ADVANCE
                                        + INCLUDE_MARK_SIDE * 0.5,
                                    rect.center().y,
                                ),
                                egui::Vec2::splat(INCLUDE_MARK_SIDE),
                            ),
                            Problem::Warning,
                        );
                        state_width += INCLUDE_MARK_ADVANCE;
                    }
                    ui.painter()
                        .with_clip_rect(egui::Rect::from_x_y_ranges(
                            (rect.left() + inset)..=(rect.right() - inset - 8.0 - state_width),
                            rect.y_range(),
                        ))
                        .text(
                            egui::pos2(rect.left() + inset, rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            format!("{}. {}", index + 1, entry.authored().display()),
                            theme::mono(tokens::FS_0, FontWeight::Regular),
                            t.color.text,
                        );
                }
                theme::paint_focus_ring(ui, &response, rect);
                let response = response.on_hover_text(entry.resolved().display().to_string());
                if response.clicked() {
                    code.include_search_selected = Some(index);
                }
            }
            if library_folders && !authored.is_empty() {
                muted_inspector_copy(
                    ui,
                    &messages.text(MessageId::NetlistIncludeSearchPathThenLibraryFolders),
                );
            }
            if unsaved && !authored.is_empty() {
                muted_inspector_copy(
                    ui,
                    &messages.text(MessageId::NetlistIncludeSearchPathUnsaved),
                );
            }
            section_block_gap(ui);
            let selected = code
                .include_search_selected
                .filter(|index| *index < authored.len());
            // The field spans the body; the commands under it wrap.
            let draft = code.include_search_draft.clone();
            let mut edited = draft.clone();
            ui.add(
                egui::TextEdit::singleline(&mut edited)
                    .font(theme::field_mono())
                    .hint_text(messages.text(MessageId::NetlistIncludeSearchPathHint)),
            );
            if edited != draft {
                code.include_search_draft = edited.clone();
            }
            section_block_gap(ui);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                if Button::new(&messages.text(MessageId::NetlistIncludeSearchPathAdd))
                    .enabled(!edited.trim().is_empty())
                    .show(ui)
                    .clicked()
                {
                    let mut next = authored.clone();
                    next.push(std::path::PathBuf::from(edited.trim()));
                    code.include_search_draft.clear();
                    action = Some(NetlistLifecycleAction::SetIncludeSearchPaths(next));
                }
                #[cfg(not(target_arch = "wasm32"))]
                if Button::new(&messages.text(MessageId::NetlistIncludeSearchPathBrowse))
                    .show(ui)
                    .clicked()
                    && let Some(picked) = rfd::FileDialog::new()
                        .set_title(messages.text(MessageId::NetlistIncludeSearchPaths))
                        .pick_folder()
                {
                    let mut next = authored.clone();
                    next.push(picked);
                    action = Some(NetlistLifecycleAction::SetIncludeSearchPaths(next));
                }
                if Button::new(&messages.text(MessageId::NetlistIncludeSearchPathMoveUp))
                    .enabled(selected.is_some_and(|index| index > 0))
                    .show(ui)
                    .clicked()
                    && let Some(index) = selected
                {
                    let mut next = authored.clone();
                    next.swap(index - 1, index);
                    code.include_search_selected = Some(index - 1);
                    action = Some(NetlistLifecycleAction::SetIncludeSearchPaths(next));
                }
                if Button::new(&messages.text(MessageId::NetlistIncludeSearchPathMoveDown))
                    .enabled(selected.is_some_and(|index| index + 1 < authored.len()))
                    .show(ui)
                    .clicked()
                    && let Some(index) = selected
                {
                    let mut next = authored.clone();
                    next.swap(index, index + 1);
                    code.include_search_selected = Some(index + 1);
                    action = Some(NetlistLifecycleAction::SetIncludeSearchPaths(next));
                }
                if Button::new(&messages.text(MessageId::NetlistIncludeSearchPathRemove))
                    .enabled(selected.is_some())
                    .destructive(true)
                    .show(ui)
                    .clicked()
                    && let Some(index) = selected
                {
                    let mut next = authored.clone();
                    next.remove(index);
                    code.include_search_selected = None;
                    action = Some(NetlistLifecycleAction::SetIncludeSearchPaths(next));
                }
            });
        },
    );
    action
}

fn netlist_operation_review(
    ui: &mut Ui,
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
) -> Option<NetlistLifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let transaction = netlist.lifecycle_dialog.transaction.as_mut()?;
    let (title, commit) = match transaction.action {
        CodeSourceFileAction::New => (
            MessageId::CodeSourceCreateTitle,
            MessageId::CodeSourceCreatePrimary,
        ),
        CodeSourceFileAction::Rename => (
            MessageId::CodeSourceRenameTitle,
            MessageId::CodeSourceRenamePrimary,
        ),
        CodeSourceFileAction::Move => (
            MessageId::CodeSourceMoveTitle,
            MessageId::CodeSourceMovePrimary,
        ),
        CodeSourceFileAction::Duplicate => (
            MessageId::CodeSourceDuplicateTitle,
            MessageId::CodeSourceDuplicatePrimary,
        ),
        CodeSourceFileAction::Delete => (
            MessageId::CodeSourceDeleteTitle,
            MessageId::CodeSourceDeletePrimary,
        ),
    };
    let deleting = transaction.action == CodeSourceFileAction::Delete;
    let mut action = None;
    dialog_section(ui, &messages.text(title), None, |ui| {
        if deleting {
            property_row(
                ui,
                &messages.text(MessageId::CodeSourceRemoving),
                &transaction.source_path,
            );
        } else {
            property_row_input(
                ui,
                &messages.text(MessageId::CodeSourceLogicalPath),
                &mut transaction.proposed_path,
                transaction.error.is_some(),
            );
        }
        let against = messages.text(MessageId::CodeSourceAgainst);
        match (transaction.document_revision, transaction.content_digest) {
            (Some(revision), Some(digest)) => {
                property_row(
                    ui,
                    &against,
                    &format!(
                        "doc rev {} · {}",
                        revision.get(),
                        digest.to_string().chars().take(12).collect::<String>()
                    ),
                );
            }
            _ => {
                property_row_wrapped(
                    ui,
                    &against,
                    &messages.text(MessageId::CodeSourceAgainstEmptyNetlist),
                );
            }
        }
        if let Some(error) = transaction.error.as_deref() {
            toned_inspector_copy(ui, error, t.color.err);
        }
        section_block_gap(ui);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            if Button::new(&messages.text(commit))
                .accent()
                .destructive(deleting)
                .enabled(deleting || !transaction.proposed_path.trim().is_empty())
                .show(ui)
                .clicked()
            {
                action = Some(NetlistLifecycleAction::Commit);
            }
            if Button::new(&messages.text(MessageId::CommonCancel))
                .ghost()
                .show(ui)
                .clicked()
            {
                action = Some(NetlistLifecycleAction::Cancel);
            }
        });
        if deleting {
            muted_inspector_copy(ui, &messages.text(MessageId::CodeSourceDeleteWarning));
        }
    });
    action
}

fn netlist_include_operation_review(
    ui: &mut Ui,
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
) -> Option<NetlistLifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let transaction = netlist.lifecycle_dialog.include_transaction.as_mut()?;
    let (title, commit) = if transaction.action == CodeSourceFileAction::Rename {
        (
            MessageId::CodeSourceRenameTitle,
            MessageId::CodeSourceRenamePrimary,
        )
    } else {
        (
            MessageId::CodeSourceMoveTitle,
            MessageId::CodeSourceMovePrimary,
        )
    };
    let mut action = None;
    dialog_section(ui, &messages.text(title), None, |ui| {
        property_row(
            ui,
            &messages.text(MessageId::NetlistRequestedAs),
            &transaction.logical_identity,
        );
        property_row_input(
            ui,
            &messages.text(MessageId::CodeSourceLogicalPath),
            &mut transaction.proposed_identity,
            transaction.error.is_some(),
        );
        property_row(
            ui,
            &messages.text(MessageId::CodeSourceAgainst),
            &format!("root rev {}", transaction.root_revision.get()),
        );
        if let Some(error) = transaction.error.as_deref() {
            toned_inspector_copy(ui, error, t.color.err);
        }
        section_block_gap(ui);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            if Button::new(&messages.text(commit))
                .accent()
                .enabled(!transaction.proposed_identity.trim().is_empty())
                .show(ui)
                .clicked()
            {
                action = Some(NetlistLifecycleAction::CommitInclude);
            }
            if Button::new(&messages.text(MessageId::CommonCancel))
                .ghost()
                .show(ui)
                .clicked()
            {
                action = Some(NetlistLifecycleAction::CancelInclude);
            }
        });
    });
    action
}

fn current_document(
    ui: &mut Ui,
    facts: &DocumentFacts,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    // Every project document is editable; only one that is not is news.
    let ownership = (!facts.editable).then(|| messages.text(MessageId::CodeSourceOwnedReadOnly));
    let mut action = None;
    dialog_section(
        ui,
        &messages.text(MessageId::CodeSourceCurrentDocument),
        ownership.as_deref(),
        |ui| {
            property_row(
                ui,
                &messages.text(MessageId::CodeSourcePath),
                &facts.logical_path,
            );
            if let Some(chosen) = role_control(ui, facts, messages) {
                action = Some(chosen);
            }
            property_row_wrapped(
                ui,
                &messages.text(MessageId::CodeSourceLineEndings),
                &messages.text(facts.line_endings),
            );
            // A current validation is the state this row is in nearly always,
            // and saying so is the absence of a problem.
            if !facts.validation_current {
                property_row_problem(
                    ui,
                    &messages.text(MessageId::CodeSourceValidation),
                    &messages.text(MessageId::CodeSourceValidationStale),
                    Some(Problem::Warning),
                );
            }
        },
    );
    action
}

/// The recorded role, editable where the project can actually change it.
///
/// An Automation environment lock refuses graph mutation and the refusal tells
/// the user to reassign its role first. Without this control that instruction
/// could not be followed, so the lock was a document nothing could ever move,
/// rename, or delete.
fn role_control(
    ui: &mut Ui,
    facts: &DocumentFacts,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    let row_label = messages.text(MessageId::CodeSourceRecordedRole);
    if facts.language != ProjectSourceLanguage::RSpiceAutomation || facts.is_entry {
        property_row(ui, &row_label, &facts.role);
        return None;
    }
    let options = ASSIGNABLE_AUTOMATION_ROLES
        .iter()
        .map(|role| (role_label(messages, *role), role_label(messages, *role)))
        .collect::<Vec<_>>();
    let mut selected = facts.role.clone();
    let changed = crate::workbench::design_system::property_row_combo(
        ui,
        &row_label,
        "source-document-role",
        &mut selected,
        &options,
        facts.block_reason.is_none(),
    );
    changed
        .then(|| {
            ASSIGNABLE_AUTOMATION_ROLES
                .iter()
                .find(|role| role_label(messages, **role) == selected)
                .map(|role| LifecycleAction::AssignRole(*role))
        })
        .flatten()
}

/// The file operations, in the order they are offered.
const FILE_OPERATIONS: [(MessageId, CodeSourceFileAction); 5] = [
    (MessageId::CodeSourceNewFile, CodeSourceFileAction::New),
    (MessageId::CodeSourceRename, CodeSourceFileAction::Rename),
    (MessageId::CodeSourceMove, CodeSourceFileAction::Move),
    (
        MessageId::CodeSourceDuplicate,
        CodeSourceFileAction::Duplicate,
    ),
    (MessageId::CodeSourceDelete, CodeSourceFileAction::Delete),
];

fn operations(
    ui: &mut Ui,
    facts: &DocumentFacts,
    action: &mut LifecycleAction,
    messages: MessageCatalog,
) {
    dialog_section(
        ui,
        &messages.text(MessageId::CodeSourceFileOperations),
        None,
        |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
                for (label, candidate) in FILE_OPERATIONS {
                    let refusal = refusal_reason(facts, candidate);
                    let response = Button::new(&messages.text(label))
                        .enabled(refusal.is_none())
                        .destructive(candidate == CodeSourceFileAction::Delete)
                        .show(ui);
                    if response.clicked() {
                        *action = LifecycleAction::Begin(candidate);
                    }
                    if let Some(reason) = refusal {
                        response.on_disabled_hover_text(reason.resolve(messages));
                    }
                }
                if Button::new(&messages.text(MessageId::CodeSourceRevisionHistory))
                    .show(ui)
                    .clicked()
                {
                    *action = LifecycleAction::OpenHistory;
                }
                if facts.language == ProjectSourceLanguage::RSpiceAutomation {
                    let response =
                        Button::new(&messages.text(MessageId::CodeSourceImportDependency))
                            .enabled(facts.block_reason.is_none())
                            .show(ui);
                    if response.clicked() {
                        *action = LifecycleAction::Import;
                    }
                    if let Some(reason) = facts.block_reason {
                        response.on_disabled_hover_text(reason);
                    }
                }
                if facts.language == ProjectSourceLanguage::VerilogA {
                    let response =
                        Button::new(&messages.text(MessageId::CodeSourceReplaceFromFile))
                            .enabled(facts.editable)
                            .show(ui);
                    if response.clicked() {
                        *action = LifecycleAction::ReplaceFromFile;
                    }
                    if let Some(reason) = facts.block_reason {
                        response.on_disabled_hover_text(reason);
                    }
                }
            });
        },
    );
}

/// Why an operation is unavailable, or `None` when it is.
///
/// These are the transaction's own rules restated, not a stricter house rule.
/// Content editability deliberately does not appear: a document can be
/// read-only to type into and still be renamed, moved, or deleted, and gating
/// on it would disable buttons the service would have honoured.
fn refusal_reason(facts: &DocumentFacts, candidate: CodeSourceFileAction) -> Option<Refusal> {
    if let Some(reason) = facts.block_reason {
        return Some(Refusal::Service(reason));
    }
    let rewrites_graph = matches!(
        candidate,
        CodeSourceFileAction::New
            | CodeSourceFileAction::Rename
            | CodeSourceFileAction::Move
            | CodeSourceFileAction::Delete
    );
    if rewrites_graph && facts.is_environment_lock {
        return Some(Refusal::Stated(
            MessageId::CodeSourceEnvironmentLockMutationBlocked,
        ));
    }
    // The root is the closure's entry document. Removing it would leave a
    // bundle with nothing to enter through, so the service refuses.
    (candidate == CodeSourceFileAction::Delete && facts.is_root)
        .then_some(Refusal::Stated(MessageId::CodeSourceRootDeleteBlocked))
}

/// Why an operation is refused, and who owns the wording.
///
/// A rule this surface restates is catalog copy. A rule the source-graph
/// service already answered with is that service's sentence, passed through
/// unaltered so the two can never disagree about the reason.
#[derive(Clone, Copy)]
enum Refusal {
    Stated(MessageId),
    Service(&'static str),
}

impl Refusal {
    fn resolve(self, messages: MessageCatalog) -> String {
        match self {
            Self::Stated(id) => messages.text(id),
            Self::Service(reason) => reason.to_owned(),
        }
    }
}

/// The open file transaction, drawn where the operation buttons are so the
/// proposed path and its failure reason stay together.
fn operation_review(
    ui: &mut Ui,
    code: &mut CodeWorkspaceRuntimeState,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    let t = Tokens::get(ui.ctx());
    let draft = code.source_file_dialog.as_mut()?;
    let (title, commit) = match draft.action {
        CodeSourceFileAction::New => (
            MessageId::CodeSourceCreateTitle,
            MessageId::CodeSourceCreatePrimary,
        ),
        CodeSourceFileAction::Rename => (
            MessageId::CodeSourceRenameTitle,
            MessageId::CodeSourceRenamePrimary,
        ),
        CodeSourceFileAction::Move => (
            MessageId::CodeSourceMoveTitle,
            MessageId::CodeSourceMovePrimary,
        ),
        CodeSourceFileAction::Duplicate => (
            MessageId::CodeSourceDuplicateTitle,
            MessageId::CodeSourceDuplicatePrimary,
        ),
        CodeSourceFileAction::Delete => (
            MessageId::CodeSourceDeleteTitle,
            MessageId::CodeSourceDeletePrimary,
        ),
    };
    let deleting = draft.action == CodeSourceFileAction::Delete;
    let mut action = None;
    dialog_section(ui, &messages.text(title), None, |ui| {
        if deleting {
            property_row(
                ui,
                &messages.text(MessageId::CodeSourceRemoving),
                &draft.source_path,
            );
        } else {
            property_row_input(
                ui,
                &messages.text(MessageId::CodeSourceLogicalPath),
                &mut draft.proposed_path,
                draft.error.is_some(),
            );
        }
        property_row(
            ui,
            &messages.text(MessageId::CodeSourceAgainst),
            &messages.format(
                MessageId::CodeSourceAgainstRevision,
                &[
                    ("revision", &draft.bundle_revision.to_string()),
                    ("digest", &short_digest(draft.closure_digest)),
                ],
            ),
        );
        if let Some(error) = draft.error.as_deref() {
            toned_inspector_copy(ui, error, t.color.err);
        }
        section_block_gap(ui);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            if Button::new(&messages.text(commit))
                .accent()
                .destructive(deleting)
                .enabled(deleting || !draft.proposed_path.trim().is_empty())
                .show(ui)
                .clicked()
            {
                action = Some(LifecycleAction::Commit);
            }
            if Button::new(&messages.text(MessageId::CommonCancel))
                .ghost()
                .show(ui)
                .clicked()
            {
                action = Some(LifecycleAction::Cancel);
            }
        });
        if deleting {
            muted_inspector_copy(ui, &messages.text(MessageId::CodeSourceDeleteWarning));
        }
    });
    action
}

fn includes(ui: &mut Ui, facts: &DocumentFacts, messages: MessageCatalog) {
    let total = facts.imports.len() + facts.imported_by.len();
    dialog_section(
        ui,
        &messages.text(MessageId::CodeSourceRecordedIncludes),
        Some(&total.to_string()),
        |ui| {
            if total == 0 {
                muted_inspector_copy(ui, &messages.text(MessageId::CodeSourceNoIncludes));
                return;
            }
            let imports = messages.text(MessageId::CodeSourceImports);
            let imported_by = messages.text(MessageId::CodeSourceImportedBy);
            for imported in &facts.imports {
                property_row(ui, &imports, imported);
            }
            for importer in &facts.imported_by {
                property_row(ui, &imported_by, importer);
            }
        },
    );
}

fn revision_history(
    ui: &mut Ui,
    code: &mut CodeWorkspaceRuntimeState,
    facts: &DocumentFacts,
    messages: MessageCatalog,
) -> Option<LifecycleAction> {
    let history = code.source_history.as_ref()?;
    let t = Tokens::get(ui.ctx());
    let selected = history.selected_revision;
    let error = history.error.clone();
    let mut action = None;
    dialog_section(
        ui,
        &messages.text(MessageId::CodeSourceRetainedRevisions),
        Some(&facts.history.len().to_string()),
        |ui| {
            if facts.history.is_empty() {
                muted_inspector_copy(ui, &messages.text(MessageId::CodeSourceNoRetainedRevision));
                return;
            }
            subsection_header(
                ui,
                &format!(
                    "{} \u{b7} {}",
                    messages.text(MessageId::CodeSourceRetainedClosures),
                    messages.text(MessageId::CodeSourceNewestLast)
                ),
            );
            Grid::new("source-document-revisions")
                .num_columns(4)
                .spacing(egui::vec2(14.0, 3.0))
                .striped(true)
                .show(ui, |ui| {
                    for row in &facts.history {
                        let documents = messages.format(
                            if row.files == 1 {
                                MessageId::CodeSourceDocumentSingular
                            } else {
                                MessageId::CodeSourceDocuments
                            },
                            &[("count", &row.files.to_string())],
                        );
                        // Restoring a closure is chosen from this row, and the
                        // row's digest, size, and currency live in cells that
                        // never take focus. The selectable cell says all of it.
                        let response = ui.selectable_label(
                            selected == Some(row.revision),
                            RichText::new(messages.format(
                                MessageId::CodeSourceRevision,
                                &[("revision", &row.revision.to_string())],
                            ))
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium)),
                        );
                        let announced = messages.format(
                            if row.current {
                                MessageId::CodeSourceRevisionAccessibleCurrent
                            } else {
                                MessageId::CodeSourceRevisionAccessible
                            },
                            &[
                                ("revision", &row.revision.to_string()),
                                ("digest", &row.digest),
                                ("documents", &documents),
                            ],
                        );
                        let chosen = selected == Some(row.revision);
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::SelectableLabel,
                                true,
                                chosen,
                                &announced,
                            )
                        });
                        if response.clicked() {
                            action = Some(LifecycleAction::SelectRevision(row.revision));
                        }
                        ui.label(
                            RichText::new(&row.digest)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            RichText::new(&documents)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            RichText::new(if row.current {
                                messages.text(MessageId::CodeSourceCurrentClosure)
                            } else {
                                String::new()
                            })
                            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_dim),
                        );
                        ui.end_row();
                    }
                });
            if let Some(error) = error.as_deref() {
                toned_inspector_copy(ui, error, t.color.err);
            }
            section_block_gap(ui);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let response =
                    Button::new(&messages.text(MessageId::CodeSourceRestoreAsNewRevision))
                        .accent()
                        .enabled(selected.is_some() && facts.block_reason.is_none())
                        .show(ui);
                if response.clicked() {
                    action = Some(LifecycleAction::Restore);
                }
                if let Some(reason) = facts.block_reason {
                    response.on_disabled_hover_text(reason);
                }
                if Button::new(&messages.text(MessageId::CodeSourceCloseHistory))
                    .ghost()
                    .show(ui)
                    .clicked()
                {
                    action = Some(LifecycleAction::CloseHistory);
                }
            });
        },
    );
    action
}

#[cfg(test)]
pub(crate) mod tests;

/// Offscreen renders of the include search chain.
#[cfg(test)]
mod include_search_raster {
    use super::*;
    use crate::workbench::code::tests::{CodeHarness, harness};

    fn chain_state(entries: &[&str], root: Option<&str>) -> CodeHarness {
        let mut harness = harness();
        harness.model.workspace.content.project.path =
            root.map(|root| std::path::PathBuf::from(root).join("x.rspiceproj"));
        harness
            .model
            .workspace
            .content
            .project
            .set_include_search_paths(entries.iter().map(std::path::PathBuf::from).collect())
            .expect("fixture chain is valid");
        harness
    }

    fn render(width: f32, height: f32, harness: &mut CodeHarness) -> crate::ui::raster::Canvas {
        crate::ui::raster::render(egui::vec2(width, height), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    let messages = crate::workbench::MessageCatalog::new(
                        crate::workbench::UiTextLocale::EnglishUnitedStates,
                    );
                    let _ = include_search_paths(
                        ui,
                        &mut harness.state.code_workspace,
                        &harness.model,
                        messages,
                    );
                });
        })
    }

    /// An empty chain and a populated one cannot look alike: the empty state
    /// says what resolution falls back to, and a populated one lists the order.
    #[test]
    fn an_empty_chain_and_a_populated_one_paint_differently() {
        let empty = render(1024.0, 640.0, &mut chain_state(&[], None));
        let populated = render(
            1024.0,
            640.0,
            &mut chain_state(&["models", "corners", "/opt/pdk/lib"], None),
        );
        assert!(
            populated.content_height() > empty.content_height(),
            "three ordered rows must occupy more of the section than the empty state"
        );
    }
}
