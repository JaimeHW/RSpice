//! Ownership, persistence, history, and file operations for one project-owned
//! source document.
//!
//! Every operation here is a revision-bound transaction the Code Workspace
//! already implements. This module is the surface that reaches them. Before it
//! existed the whole lifecycle — new, rename, move, duplicate, delete and
//! revision restore — was written, tested, and unreachable.
//!
//! A draft is never applied by closing the dialog. Committing is an explicit
//! act, because the operations here rewrite a source graph.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, Grid, RichText, Ui};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::ProjectSourceBundle;
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, DIALOG_BODY_INSET, Dialog, DialogChoice, DialogInitialFocus, DialogSize,
    DialogTransactionTone, IconButton, PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem,
    band_header, button_grid, control_height, name_control, paint_problem_mark,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::{LifecycleAction, SourceDocumentRequest};
use crate::workbench::design_system::inspector::{muted_inspector_copy, toned_inspector_copy};
use crate::workbench::design_system::{
    Fact, PANEL_INSET, PANEL_SECTION_GAP, fact_rows, panel_body, property_row_height,
    section_block_gap, subsection_header,
};
use crate::workbench::documents::code_workspace::{
    self, CodeSourceFileAction, CodeWorkspaceParts, CodeWorkspaceRuntimeState,
};
use crate::workbench::documents::netlist_document::{self, NetlistDocumentState};
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::{MessageCatalog, MessageId};

/// The gap between two buttons of the operations grid, across and down, and
/// between the add row's field and its buttons.
const CONTROL_GAP: f32 = 6.0;

/// The shell's `open_source_document_dialog`, over the workspace in front, the model,
/// the Code workspace's runtime state, the netlist editor's state and the
/// platform's state.
pub fn open_source_document_dialog_in(
    workspace: Workspace,
    model: &Model,
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
    messages: MessageCatalog,
) -> Result<(), String> {
    if !veriloga_is_visible_in(workspace, model) {
        if !platform.project_lifecycle.is_open() {
            return Err(messages.text(MessageId::NetlistManageNeedsProject));
        }
        if let Some(refusal) = generated_deck_refusal(model, netlist) {
            return Err(messages.text(refusal));
        }
        code.source_document_dialog = true;
        return Ok(());
    }
    if model.workspace.active_source_bundle().is_none() {
        return Err(messages.text(MessageId::VerilogAManageNoSources));
    }
    code.source_document_dialog = true;
    Ok(())
}

/// Whether the lifecycle dialog can be opened for the visible page.
pub fn source_document_dialog_is_available(
    workspace: Workspace,
    model: &Model,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
) -> bool {
    if veriloga_is_visible_in(workspace, model) {
        model.workspace.active_source_bundle().is_some()
    } else {
        platform.project_lifecycle.is_open() && generated_deck_refusal(model, netlist).is_none()
    }
}

/// Why the netlist page has no source document to manage: the deck on it is
/// generated from the schematic and read-only, and no deck is open.
pub(crate) fn generated_deck_refusal(
    _model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<MessageId> {
    netlist
        .open_deck
        .is_none()
        .then_some(MessageId::NetlistManageGenerated)
}

/// Whether the document the dialog manages is a Verilog-A cell view open in
/// Design; otherwise it is the Netlist workspace's deck.
fn veriloga_is_visible_in(workspace: Workspace, model: &Model) -> bool {
    code_workspace::design_hosts_veriloga(workspace, &model.workspace)
}

/// What the dialog states about the visible document.
///
/// Read once per frame into owned values so applying a transaction cannot
/// invalidate a borrow the body is still rendering from, and so every row is
/// measured from the same closure.
pub struct DocumentFacts {
    pub logical_path: String,
    leaf: String,
    /// The role a file plays, when it plays one: an ordinary source says
    /// nothing here.
    role: Option<String>,
    editable: bool,
    pub is_root: bool,
    line_endings: MessageId,
    validation_current: bool,
    imports: Vec<String>,
    imported_by: Vec<String>,
    history: Vec<RevisionRow>,
    block_reason: Option<String>,
    /// Why the document cannot be deleted, when it cannot.
    delete_refusal: Option<String>,
}

struct RevisionRow {
    revision: u64,
    digest: String,
    files: usize,
    current: bool,
}

pub struct NetlistLifecycleFacts {
    /// The file's name.
    logical_path: String,
    /// Where the file is in the project's folder, or on disk when it is
    /// outside it.
    project_path: String,
    line_endings: MessageId,
    /// Where the file is on disk.
    native_path: String,
    /// Whether the file is an include the deck reads.
    include: bool,
}

/// What the dialog states of the file in front: the open deck, or an
/// include open beside it.
pub fn netlist_lifecycle_facts(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<NetlistLifecycleFacts> {
    let file = netlist_document::active_deck_file_in(netlist)?;
    let native_path = file.path.display().to_string();
    let project_path = model
        .workspace
        .content
        .project
        .data_root()
        .and_then(|root| file.path.strip_prefix(root).ok())
        .map(|relative| {
            relative
                .components()
                .map(|part| part.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        })
        .filter(|relative| !relative.is_empty())
        .unwrap_or_else(|| native_path.clone());
    Some(NetlistLifecycleFacts {
        logical_path: file.name(),
        project_path,
        line_endings: line_ending_summary(&file.line_ending.write(&file.text)),
        native_path,
        include: netlist.front.include().is_some(),
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
    if !veriloga_is_visible_in(workspace, model) {
        return None;
    }
    let logical_path = code_workspace::active_bundle_path(&model.workspace, code)?;
    let bundle = model.workspace.active_source_bundle()?;
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
        leaf: leaf_name(&logical_path),
        role: bundle
            .is_build_profile(&logical_path)
            .then(|| role_label(messages, true)),
        editable: code_workspace::source_bundle_document_is_editable_in(
            model,
            code,
            platform,
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
        delete_refusal: super::source_file_form::delete_refusal(bundle, &logical_path, messages),
        history,
        block_reason: code_workspace::source_file_mutation_block_reason_in(
            model, code, platform, messages,
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

/// One spelling of a document's role, so the property row and the picker can
/// never disagree about which entry is currently selected.
///
/// The picker resolves a selection by matching the label it drew, so the label
/// has to come from the same locale the row was painted in.
pub(super) fn role_label(messages: MessageCatalog, build_profile: bool) -> String {
    messages.text(if build_profile {
        MessageId::CodeSourceRoleBuildProfile
    } else {
        MessageId::CodeSourceRoleOrdinary
    })
}

/// Close the document lifecycle dialog and the drafts it opened.
pub fn close_document_lifecycle(code: &mut CodeWorkspaceRuntimeState) {
    code.source_document_dialog = false;
    code.source_file_dialog = None;
    code.source_history = None;
}

/// Close the netlist lifecycle dialog.
pub fn close_netlist_lifecycle(code: &mut CodeWorkspaceRuntimeState) {
    code.source_document_dialog = false;
}

/// Draw the source document dialogs over the Code workspace's runtime state,
/// the netlist editor's state, the model, the workbench session and the
/// platform's state: the lifecycle of the document on the page in front. What a choice still
/// needs of the whole application is returned for the shell to carry out.
pub(in crate::workbench) fn render_source_document_dialog(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
    ctx: &Context,
) -> Option<SourceDocumentRequest> {
    if code.source_file_dialog.is_some() {
        return super::source_file_form::render(code, model, platform, wb.messages(), ctx);
    }
    if !code.source_document_dialog {
        return None;
    }
    if !veriloga_is_visible_in(wb.workspace, model) {
        render_netlist_source_document_dialog(code, netlist, model, wb, ctx);
        return None;
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
    if let Some(reason) = facts.block_reason.clone() {
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

/// Carry out a lifecycle choice on the Verilog-A document at
/// `logical_path` that the dialog handed the shell.
pub fn apply_lifecycle_action(
    parts: &mut CodeWorkspaceParts<'_>,
    action: LifecycleAction,
    logical_path: &str,
) {
    // Where a failure is stated is part of the action, not a guess made
    // afterwards: a rejected rename belongs beside the path field, and a
    // rejected restore beside the revision list. Deciding by whichever
    // draft happens to be open would put one inside the other's section.
    let (outcome, report) = match action {
        // Nothing to carry out, or a choice the form applies over the Code
        // workspace's own state (SelectRevision, CloseHistory): the form
        // never hands these over.
        LifecycleAction::None
        | LifecycleAction::SelectRevision(_)
        | LifecycleAction::CloseHistory => return,
        LifecycleAction::Begin(file_action) => {
            let outcome = code_workspace::open_source_file_dialog(
                parts,
                file_action,
                logical_path,
                logical_path,
            );
            if outcome.is_ok() {
                // The transaction's own form takes over; this dialog would
                // only sit behind it.
                parts.code.source_document_dialog = false;
            }
            (outcome, Report::Console)
        }
        LifecycleAction::Commit => (
            code_workspace::commit_source_file_dialog(parts).map(|_| ()),
            Report::FileTransaction,
        ),
        LifecycleAction::OpenHistory => (
            code_workspace::open_source_history(parts, logical_path),
            Report::Console,
        ),
        LifecycleAction::Restore => (
            code_workspace::commit_source_history_restore(parts).map(|_| ()),
            Report::History,
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
    netlist: &NetlistDocumentState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    ctx: &Context,
) {
    let messages = wb.messages();
    let Some(facts) = netlist_lifecycle_facts(model, netlist) else {
        close_netlist_lifecycle(code);
        return;
    };
    let dialog = Dialog::new(
        messages.text(MessageId::NetlistManageDeckTitle),
        messages.text(MessageId::CommonClose),
    )
    .description(messages.format(
        MessageId::NetlistManageDeckDescription,
        &[("document", &facts.logical_path)],
    ))
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    // The rows fit at every desktop size; a bar would only reserve a
    // gutter that left the right edge wider than the left.
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl);

    let mut search_paths = None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        search_paths = netlist_lifecycle_body(ui, code, model, &facts, messages);
        None
    });
    if let Some(paths) = search_paths {
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
    if matches!(
        choice,
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
    ) {
        close_netlist_lifecycle(code);
    }
}

/// The sections of a Files and includes dialog, stacked down its flush
/// body.
fn sections<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        add(ui)
    })
    .inner
}

/// One section of a Files and includes dialog: the dialog's band across
/// its flush body, then `body` on the dialog's inset.
fn dialog_section(ui: &mut Ui, title: &str, meta: Option<&str>, body: impl FnOnce(&mut Ui)) {
    band_header(ui, title, meta);
    dialog_inset(ui, body);
}

/// `body` on the panel rhythm with every row on the left edge of the
/// dialog's title and footer.
fn dialog_inset(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
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
        current_document(ui, facts, messages);
        operations(ui, facts, &mut action, messages);
        includes(ui, facts, messages);
        if let Some(chosen) = revision_history(ui, code, facts, messages) {
            action = chosen;
        }
    });
    action
}

/// The file in front, then the include search chain; the chain as its rows
/// now read it, when one of them changed it.
fn netlist_lifecycle_body(
    ui: &mut Ui,
    code: &mut CodeWorkspaceRuntimeState,
    model: &Model,
    facts: &NetlistLifecycleFacts,
    messages: MessageCatalog,
) -> Option<Vec<std::path::PathBuf>> {
    // The dialog's body scrolls when the rows outgrow it; a scroll area of
    // its own here only withheld a gutter on the right.
    sections(ui, |ui| {
        dialog_section(
            ui,
            &messages.text(MessageId::CodeSourceCurrentDocument),
            None,
            |ui| {
                let file = messages.text(if facts.include {
                    MessageId::NetlistLogicalIdentity
                } else {
                    MessageId::NetlistTopDeck
                });
                let line_endings = messages.text(MessageId::CodeSourceLineEndings);
                let line_ending = messages.text(facts.line_endings);
                fact_rows(
                    ui,
                    &[
                        Fact::path(&file, &facts.project_path).whole(&facts.native_path),
                        Fact::new(&line_endings, &line_ending),
                    ],
                );
            },
        );
        include_search_paths(ui, code, model, messages)
    })
}

/// What a press on an include folder's row asked for.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FolderAction {
    Select,
    MoveUp,
    MoveDown,
    Remove,
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
) -> Option<Vec<std::path::PathBuf>> {
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
    let selected = code
        .include_search_selected
        .filter(|index| *index < authored.len());
    let mut action = None;
    band_header(
        ui,
        &messages.text(MessageId::NetlistIncludeSearchPaths),
        (!authored.is_empty())
            .then(|| messages.text(MessageId::NetlistIncludeSearchPathCount))
            .as_deref(),
    );
    // The rows run edge to edge, as the band over them does; their words
    // keep the dialog's inset.
    for (index, entry) in chain.entries().iter().enumerate() {
        let Some(pressed) = include_folder_row(
            ui,
            index,
            authored.len(),
            entry,
            selected == Some(index),
            messages,
        ) else {
            continue;
        };
        let mut next = authored.clone();
        match pressed {
            FolderAction::Select => code.include_search_selected = Some(index),
            FolderAction::MoveUp => {
                next.swap(index - 1, index);
                code.include_search_selected = Some(index - 1);
                action = Some(next);
            }
            FolderAction::MoveDown => {
                next.swap(index, index + 1);
                code.include_search_selected = Some(index + 1);
                action = Some(next);
            }
            FolderAction::Remove => {
                next.remove(index);
                code.include_search_selected = None;
                action = Some(next);
            }
        }
    }
    dialog_inset(ui, |ui| {
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
        } else if library_folders {
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
        if let Some(folder) = include_folder_field(ui, code, messages) {
            let mut next = authored.clone();
            next.push(folder);
            action = Some(next);
        }
    });
    action
}

/// One folder of the chain, flush to the dialog's edges: its place and its
/// path, whether it is missing, and, while the row is hovered or selected,
/// the three things that act on it alone.
fn include_folder_row(
    ui: &mut Ui,
    index: usize,
    count: usize,
    entry: &rspice_simulation::netlist_preparation::IncludeSearchEntry,
    selected: bool,
    messages: MessageCatalog,
) -> Option<FolderAction> {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let height = property_row_height(ui);
    let shown = format!("{}. {}", index + 1, entry.authored().display());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    // The row is a painted galley, so nothing here reaches a screen reader
    // on its own, and a clickable sense makes it a tab stop that would
    // otherwise take focus silently and invisibly.
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &shown,
        )
    });
    let mut pressed = response.clicked().then_some(FolderAction::Select);
    let acting = selected || ui.rect_contains_pointer(rect);
    // The row's actions, right to left from the inset; laid out only while
    // they show, so a resting list is its paths and nothing else.
    let mut right = rect.right() - DIALOG_BODY_INSET;
    if acting {
        let side = (height - 4.0).max(1.0);
        let actions = [
            (
                Icon::Trash,
                MessageId::NetlistIncludeSearchPathRemove,
                true,
                FolderAction::Remove,
            ),
            (
                Icon::ChevronDown,
                MessageId::NetlistIncludeSearchPathMoveDown,
                index + 1 < count,
                FolderAction::MoveDown,
            ),
            (
                Icon::ChevronUp,
                MessageId::NetlistIncludeSearchPathMoveUp,
                index > 0,
                FolderAction::MoveUp,
            ),
        ];
        for (icon, label, enabled, action) in actions {
            let slot = egui::Rect::from_center_size(
                egui::pos2(right - side * 0.5, rect.center().y),
                egui::Vec2::splat(side),
            );
            right -= side + 2.0;
            let label = messages.text(label);
            // A child of its own, so the row below starts where this one
            // ends rather than where its last action does.
            let mut slot_ui = ui.new_child(egui::UiBuilder::new().max_rect(slot));
            let button = IconButton::new(icon)
                .side(side)
                .enabled(enabled)
                .tooltip(&label)
                .show(&mut slot_ui);
            if button.clicked() {
                pressed = Some(action);
            }
        }
        right -= CONTROL_GAP - 2.0;
    }
    if ui.is_rect_visible(rect) {
        if selected {
            ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        }
        // A path that resolves is every path's normal state, so only one
        // that does not is marked and named.
        let missing = rspice_simulation::netlist_preparation::IncludeSearchChain::states_presence()
            && !entry.exists();
        if missing {
            let state = ui.painter().text(
                egui::pos2(right, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                messages.text(MessageId::NetlistIncludeSearchPathMissing),
                font.clone(),
                t.color.text_dim,
            );
            right = state.left() - PROBLEM_MARK_ADVANCE;
            paint_problem_mark(
                ui.painter(),
                egui::Rect::from_min_size(
                    egui::pos2(right, rect.center().y - PROBLEM_MARK_SIDE * 0.5),
                    egui::Vec2::splat(PROBLEM_MARK_SIDE),
                ),
                Problem::Warning,
            );
            right -= CONTROL_GAP;
        }
        let left = rect.left() + DIALOG_BODY_INSET;
        ui.painter()
            .with_clip_rect(egui::Rect::from_x_y_ranges(
                left..=right.max(left),
                rect.y_range(),
            ))
            .text(
                egui::pos2(left, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &shown,
                font,
                t.color.text,
            );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(entry.resolved().display().to_string());
    pressed
}

/// The add row under the chain: the path field across the row, then
/// Browse… and Add at the field's height. Returns the folder it adds.
fn include_folder_field(
    ui: &mut Ui,
    code: &mut CodeWorkspaceRuntimeState,
    messages: MessageCatalog,
) -> Option<std::path::PathBuf> {
    let height = control_height(ui);
    let add_label = messages.text(MessageId::NetlistIncludeSearchPathAdd);
    let add_named = messages.text(MessageId::NetlistIncludeSearchPathAddNamed);
    let field_name = messages.text(MessageId::NetlistIncludeSearchPathField);
    let typed = !code.include_search_draft.trim().is_empty();
    let add = Button::new(&add_label)
        .accessible_label(&add_named)
        .enabled(typed);
    #[cfg(not(target_arch = "wasm32"))]
    let browse_label = messages.text(MessageId::NetlistIncludeSearchPathBrowse);
    #[cfg(not(target_arch = "wasm32"))]
    let browse = Button::new(&browse_label);
    #[cfg(not(target_arch = "wasm32"))]
    let buttons = browse.measured_width(ui) + add.measured_width(ui) + 2.0 * CONTROL_GAP;
    #[cfg(target_arch = "wasm32")]
    let buttons = add.measured_width(ui) + CONTROL_GAP;
    let mut added = None;
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = CONTROL_GAP;
            let field = ui.add_sized(
                egui::vec2((ui.available_width() - buttons).max(1.0), height),
                egui::TextEdit::singleline(&mut code.include_search_draft)
                    .font(theme::field_mono())
                    .hint_text(messages.text(MessageId::NetlistIncludeSearchPathHint))
                    .vertical_align(egui::Align::Center)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            name_control(ui, &field, &field_name);
            let entered = typed
                && field.lost_focus()
                && ui.input(|input| input.key_pressed(egui::Key::Enter));
            #[cfg(not(target_arch = "wasm32"))]
            if browse.show(ui).clicked()
                && let Some(picked) = rfd::FileDialog::new()
                    .set_title(messages.text(MessageId::NetlistIncludeSearchPaths))
                    .pick_folder()
            {
                added = Some(picked);
            }
            if add.show(ui).clicked() || entered {
                added = Some(std::path::PathBuf::from(code.include_search_draft.trim()));
                code.include_search_draft.clear();
            }
        },
    );
    added
}

fn current_document(ui: &mut Ui, facts: &DocumentFacts, messages: MessageCatalog) {
    // Every project document is editable; only one that is not is news.
    let ownership = (!facts.editable).then(|| messages.text(MessageId::CodeSourceOwnedReadOnly));
    dialog_section(
        ui,
        &messages.text(MessageId::CodeSourceCurrentDocument),
        ownership.as_deref(),
        |ui| {
            let path = messages.text(MessageId::CodeSourcePath);
            let role = messages.text(MessageId::CodeSourceRecordedRole);
            let line_endings = messages.text(MessageId::CodeSourceLineEndings);
            let line_ending = messages.text(facts.line_endings);
            let compile = messages.text(MessageId::CodeSourceCompile);
            // The editor's header says it in these words.
            let edited = messages.text(MessageId::VerilogAStatusModified);
            let mut rows = vec![Fact::path(&path, &facts.logical_path)];
            if let Some(role_text) = &facts.role {
                rows.push(Fact::new(&role, role_text));
            }
            rows.push(Fact::new(&line_endings, &line_ending));
            // A current compile is the state this row is in nearly always,
            // and saying so is the absence of a problem.
            if !facts.validation_current {
                rows.push(Fact::new(&compile, &edited).problem(Problem::Warning));
            }
            fact_rows(ui, &rows);
        },
    );
}

/// One of the document's operations.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    File(CodeSourceFileAction),
    History,
    ReplaceFromFile,
}

/// The document's operations, in the order the grid offers them: the three
/// that give it another name or place, then its history and its source, and
/// delete last. A new file is not an operation on this document; the Files
/// rail makes one.
const OPERATIONS: [(MessageId, Operation); 6] = [
    (
        MessageId::CodeSourceRename,
        Operation::File(CodeSourceFileAction::Rename),
    ),
    (
        MessageId::CodeSourceMove,
        Operation::File(CodeSourceFileAction::Move),
    ),
    (
        MessageId::CodeSourceDuplicate,
        Operation::File(CodeSourceFileAction::Duplicate),
    ),
    (MessageId::CodeSourceRevisionHistory, Operation::History),
    (
        MessageId::CodeSourceReplaceFromFile,
        Operation::ReplaceFromFile,
    ),
    (
        MessageId::CodeSourceDelete,
        Operation::File(CodeSourceFileAction::Delete),
    ),
];

/// Columns of the operations grid.
const OPERATION_COLUMNS: usize = 3;

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
            let labels = OPERATIONS.map(|(label, _)| messages.text(label));
            let refusals = OPERATIONS.map(|(_, operation)| match operation {
                Operation::File(candidate) => {
                    refusal_reason(facts, candidate).map(Refusal::resolve)
                }
                Operation::History => None,
                // Replacing the source rewrites the document, so it waits on
                // what holds every file operation, and on a document that
                // cannot be edited.
                Operation::ReplaceFromFile => facts.block_reason.clone().or_else(|| {
                    (!facts.editable)
                        .then(|| messages.text(MessageId::CodeSourceReadOnlyDocumentDetail))
                }),
            });
            let buttons = OPERATIONS
                .iter()
                .zip(&labels)
                .zip(&refusals)
                .map(|(((_, operation), label), refusal)| {
                    Button::new(label)
                        .enabled(refusal.is_none())
                        .destructive(*operation == Operation::File(CodeSourceFileAction::Delete))
                })
                .collect();
            // The body leaves this much under the grid; the grid stands as
            // far below the band.
            ui.add_space(PANEL_SECTION_GAP);
            let pressed = button_grid(ui, OPERATION_COLUMNS, CONTROL_GAP, buttons);
            for ((response, refusal), (_, operation)) in
                pressed.into_iter().zip(refusals).zip(OPERATIONS)
            {
                if response.clicked() {
                    *action = match operation {
                        Operation::File(candidate) => LifecycleAction::Begin(candidate),
                        Operation::History => LifecycleAction::OpenHistory,
                        Operation::ReplaceFromFile => LifecycleAction::ReplaceFromFile,
                    };
                }
                if let Some(reason) = refusal {
                    response.on_disabled_hover_text(reason);
                }
            }
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
    if let Some(reason) = &facts.block_reason {
        return Some(Refusal::Service(reason.clone()));
    }
    // The main file, a file a role binds and a file that still includes
    // another: the bundle refuses to delete each, and says why first.
    if candidate == CodeSourceFileAction::Delete {
        return facts.delete_refusal.clone().map(Refusal::Delete);
    }
    None
}

/// Why an operation is refused, and who owns the wording: the source-graph
/// service's sentence, passed through unaltered so the two can never
/// disagree about the reason.
#[derive(Clone)]
enum Refusal {
    Service(String),
    /// The delete form's own refusal, already worded.
    Delete(String),
}

impl Refusal {
    fn resolve(self) -> String {
        match self {
            Self::Service(reason) | Self::Delete(reason) => reason,
        }
    }
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
            let rows: Vec<Fact<'_>> = facts
                .imports
                .iter()
                .map(|imported| Fact::path(&imports, imported))
                .chain(
                    facts
                        .imported_by
                        .iter()
                        .map(|importer| Fact::path(&imported_by, importer)),
                )
                .collect();
            fact_rows(ui, &rows);
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
                if let Some(reason) = &facts.block_reason {
                    response.on_disabled_hover_text(reason.as_str());
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

#[cfg(any(test, feature = "test-support"))]
pub mod fixtures;
#[cfg(test)]
mod tests;
