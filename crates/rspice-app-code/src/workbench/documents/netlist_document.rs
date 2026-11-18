//! Netlist document engine.
//!
//! This module owns editor diagnostics, syntax highlighting, completion and
//! run-to-run diff state. It has no application-chrome
//! chrome; the canonical Netlist workspace composes it as a document surface.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use std::collections::HashSet;

use crate::state::netlist_diagnostics::NetlistDiagnosticCollection;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;

const MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS: usize = 128;

mod baseline;
mod comparison;
mod completion;
pub mod cross_probe;
pub(crate) mod diagnostics;
pub(crate) mod dialect;
mod editor;
mod executed_deck;
mod highlight;
mod index_worker;
pub mod language;
mod navigation;
mod param_scan;
pub(crate) mod references;
mod source_index;
pub use navigation::{open_diagnostic_location, open_source_location};
pub use references::open_deck_place;

pub use comparison::compare_generated_revision;
pub use comparison::{close_revision_comparison, reveal_executed_deck};
pub use comparison::{
    close_run_deck_snapshot, compare_run_deck_snapshot_in, open_executed_deck_in,
    open_netlist_comparison, open_run_deck_snapshot_in, run_deck_snapshot_artifact_name_in,
    run_deck_snapshot_run_id_in,
};
pub use diagnostics::{DiagnosticSeverity, control_disposition_summary};
pub use editor::show as show_editor;
pub use editor::{
    commit_edit as commit_editor_edit, editor_id_in, open_editor_ids_in, release_closed_editors,
};
pub(crate) use executed_deck::run_snapshot_source_in;
pub use executed_deck::{ExecutedDeckSelection, ExecutedDeckVerification};
pub use source_index::{
    root_deck_source_index, root_deck_source_index_now, visible_source_index_in,
    visible_source_index_now,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum OwnedNetlistReplacementTarget {
    Root,
    Dependency(String),
}

/// One replacement in the open deck or an include file open beside it,
/// prepared before any file changes. The original digest prevents a stale
/// search result from changing newer text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedNetlistReplacement {
    target: OwnedNetlistReplacementTarget,
    original: String,
    expected_digest: crate::product::ContentDigest,
    replacement: String,
    replacement_count: usize,
}

impl OwnedNetlistReplacement {
    #[must_use]
    pub fn root(original: &str, replacement: String, replacement_count: usize) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::Root,
            original: original.to_owned(),
            expected_digest: crate::state::content_digest(original),
            replacement,
            replacement_count,
        }
    }

    #[must_use]
    pub fn dependency(
        logical_identity: impl Into<String>,
        original: &str,
        replacement: String,
        replacement_count: usize,
    ) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::Dependency(logical_identity.into()),
            original: original.to_owned(),
            expected_digest: crate::state::content_digest(original),
            replacement,
            replacement_count,
        }
    }
}

impl OwnedNetlistReplacement {
    /// The include the replacement edits, or none for the deck itself.
    fn include(&self) -> Option<&str> {
        match &self.target {
            OwnedNetlistReplacementTarget::Root => None,
            OwnedNetlistReplacementTarget::Dependency(key) => Some(key),
        }
    }

    fn reversed(&self) -> Self {
        Self {
            target: self.target.clone(),
            original: self.replacement.clone(),
            expected_digest: crate::state::content_digest(&self.replacement),
            replacement: self.original.clone(),
            replacement_count: self.replacement_count,
        }
    }
}

#[derive(Debug, Clone)]
struct NetlistEditJournalEntry {
    description: String,
    forward: Vec<OwnedNetlistReplacement>,
    reverse: Vec<OwnedNetlistReplacement>,
}

const MAX_NETLIST_EDIT_JOURNAL_ENTRIES: usize = 32;

fn record_netlist_edit(state: &mut NetlistDocumentState, entry: NetlistEditJournalEntry) {
    state.edit_redo.clear();
    state.edit_undo.push(entry);
    if state.edit_undo.len() > MAX_NETLIST_EDIT_JOURNAL_ENTRIES {
        state.edit_undo.remove(0);
    }
}

fn clear_netlist_edit_journal(state: &mut NetlistDocumentState) {
    state.edit_undo.clear();
    state.edit_redo.clear();
}

/// Apply a set of replacements to the open deck and its open includes, all
/// or none: every file's text is checked against the text the replacement
/// was prepared from before any of them changes.
pub fn replace_deck_files(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    replacements: Vec<OwnedNetlistReplacement>,
) -> Result<usize, String> {
    replace_deck_files_impl(model, netlist, replacements, true)
}

fn replace_deck_files_impl(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    replacements: Vec<OwnedNetlistReplacement>,
    record_edit: bool,
) -> Result<usize, String> {
    let applied = replacements
        .into_iter()
        .filter(|replacement| {
            replacement.replacement_count > 0 && replacement.original != replacement.replacement
        })
        .collect::<Vec<_>>();
    if applied.is_empty() {
        return Ok(0);
    }
    let deck = netlist
        .open_deck
        .as_mut()
        .ok_or_else(|| "No deck is open; nothing was changed.".to_owned())?;
    for replacement in &applied {
        let file = deck.file(replacement.include()).ok_or_else(|| {
            "A file the replacement names is no longer open; nothing was changed.".to_owned()
        })?;
        if crate::state::content_digest(&file.text) != replacement.expected_digest {
            return Err(
                "A file changed after the replacement was prepared; nothing was changed."
                    .to_owned(),
            );
        }
    }
    let changed_count = applied
        .iter()
        .map(|replacement| replacement.replacement_count)
        .sum::<usize>();
    for replacement in &applied {
        if let Some(file) = deck.file_mut(replacement.include()) {
            file.edit(replacement.replacement.clone());
        }
    }
    if let Some(file) = active_deck_file_in(netlist) {
        model.simulation.source.netlist_content = file.text.clone();
    }
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    if record_edit {
        let reverse = applied
            .iter()
            .rev()
            .map(OwnedNetlistReplacement::reversed)
            .collect();
        record_netlist_edit(
            netlist,
            NetlistEditJournalEntry {
                description: format!("replace {changed_count} netlist match(es)"),
                forward: applied,
                reverse,
            },
        );
    }
    Ok(changed_count)
}

/// Publish `replacements` as one all-or-nothing change that Undo takes back
/// one file at a time: each file's change is its own step, `description`.
pub(crate) fn replace_owned_sources_a_step_per_file(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    replacements: Vec<OwnedNetlistReplacement>,
    description: &str,
) -> Result<usize, String> {
    let steps = replacements
        .iter()
        .filter(|replacement| {
            replacement.replacement_count > 0 && replacement.original != replacement.replacement
        })
        .cloned()
        .collect::<Vec<_>>();
    let changed =
        replace_owned_sources_atomically_impl(model, netlist, open_documents, replacements, false)?;
    for step in steps {
        let reverse = vec![step.reversed()];
        record_netlist_edit(
            netlist,
            NetlistEditJournalEntry {
                description: description.to_owned(),
                forward: vec![step],
                reverse,
            },
        );
    }
    Ok(changed)
}

#[must_use]
pub fn can_undo_netlist_edit(netlist: &NetlistDocumentState) -> bool {
    !netlist.edit_undo.is_empty()
}

#[must_use]
pub fn can_redo_netlist_edit(netlist: &NetlistDocumentState) -> bool {
    !netlist.edit_redo.is_empty()
}

pub fn undo_netlist_edit(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
) -> Result<Option<String>, String> {
    let Some(entry) = netlist.edit_undo.pop() else {
        return Ok(None);
    };
    if let Err(error) = replace_deck_files_impl(model, netlist, entry.reverse.clone(), false) {
        netlist.edit_undo.push(entry);
        return Err(error);
    }
    let description = entry.description.clone();
    netlist.edit_redo.push(entry);
    Ok(Some(description))
}

pub fn redo_netlist_edit(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
) -> Result<Option<String>, String> {
    let Some(entry) = netlist.edit_redo.pop() else {
        return Ok(None);
    };
    if let Err(error) = replace_deck_files_impl(model, netlist, entry.forward.clone(), false) {
        netlist.edit_redo.push(entry);
        return Err(error);
    }
    let description = entry.description.clone();
    netlist.edit_undo.push(entry);
    Ok(Some(description))
}

/// Replace the text of the file in front, the open deck or an include open
/// beside it, with what the editor shows. Whether that changed it.
pub fn edit_deck_file(model: &mut Model, netlist: &mut NetlistDocumentState, text: String) -> bool {
    if netlist.active_document != ActiveNetlistDocument::OwnedSource {
        return false;
    }
    let include = netlist.active_dependency_identity.clone();
    let Some(file) = netlist
        .open_deck
        .as_mut()
        .and_then(|deck| deck.file_mut(include.as_deref()))
    else {
        return false;
    };
    if !file.edit(text.clone()) {
        return false;
    }
    model.simulation.source.netlist_content = text;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    clear_netlist_edit_journal(netlist);
    true
}

/// The canonical document a root names: the generated primary. A deck is a
/// file, not a document, and revision comparisons and run snapshots have
/// none. It reads the model and the netlist editor's state.
fn canonical_root_document_in<'a>(
    model: &'a Model,
    _netlist: &'a NetlistDocumentState,
    root: ActiveNetlistDocument,
) -> Option<&'a crate::state::NetlistDocument> {
    match root {
        ActiveNetlistDocument::Generated => model.netlist_artifacts.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource
        | ActiveNetlistDocument::GeneratedDiff
        | ActiveNetlistDocument::RunSnapshot => None,
    }
}

/// The file in front when the open deck or an include beside it is: the
/// file the editor edits and Save writes.
#[must_use]
pub fn active_deck_file_in(netlist: &NetlistDocumentState) -> Option<&super::deck_files::OpenFile> {
    if netlist.active_document != ActiveNetlistDocument::OwnedSource {
        return None;
    }
    netlist
        .open_deck
        .as_ref()?
        .file(netlist.active_dependency_identity.as_deref())
}

/// Exact retained dependency currently projected as a first-class source
/// document. The dependency source remains canonical inside its root
/// `NetlistDocument`; this function never synthesizes a second source store.
/// It reads the model and the netlist editor's state.
pub(crate) fn active_dependency_in<'a>(
    model: &'a Model,
    netlist: &'a NetlistDocumentState,
) -> Option<&'a crate::state::DependencyMetadata> {
    let identity = netlist.active_dependency_identity.as_deref()?;
    let root = netlist.active_dependency_root?;
    canonical_root_document_in(model, netlist, root)?
        .dependencies()
        .iter()
        .find(|dependency| dependency.locator().logical_identity() == identity)
}

/// Whether the editor edits what it shows: the open deck and the includes
/// open beside it, which save to their files. The generated netlist, its
/// includes, a comparison and a run's input are read-only. It reads the
/// netlist editor's state.
#[must_use]
pub fn active_netlist_source_is_editable_in(
    _model: &Model,
    netlist: &NetlistDocumentState,
) -> bool {
    active_deck_file_in(netlist).is_some()
}

/// Which netlist document a manual deck run would take its source from.
///
/// Before the session has activated a document, an open deck outranks the
/// generated primary; afterwards the active document is the answer and
/// nothing infers one from the visible buffer.
///
/// It reads the model and the netlist editor's state.
pub(crate) fn effective_active_document_in(
    netlist: &NetlistDocumentState,
) -> ActiveNetlistDocument {
    if netlist.active_document_initialized {
        netlist.active_document
    } else if netlist.open_deck.is_some() {
        ActiveNetlistDocument::OwnedSource
    } else {
        ActiveNetlistDocument::Generated
    }
}

/// The exact deck text a manual run would consume right now.
///
/// The run gate and the post-run strip both have to name the same bytes: one
/// decides whether the deck may run, the other whether the deck still matches
/// the run that already happened.
///
/// It reads the model and the netlist editor's state.
pub(crate) fn working_deck_source_in<'a>(
    model: &'a Model,
    netlist: &'a NetlistDocumentState,
) -> &'a str {
    match netlist.open_deck.as_ref() {
        Some(deck)
            if effective_active_document_in(netlist) == ActiveNetlistDocument::OwnedSource =>
        {
            deck.root.text.as_str()
        }
        _ => model.simulation.source.netlist_content.as_str(),
    }
}

/// Why Run on the Netlist workspace will not run the deck: what
/// [`manual_deck_run_block_reason_in`] finds, then a validation whose
/// authorization the run controller no longer holds, which is the run
/// tier's half of the answer.
pub fn manual_deck_run_block_reason_for_run(
    model: &Model,
    netlist: &NetlistDocumentState,
    messages: crate::workbench::MessageCatalog,
    controller: &rspice_app_runs::simulation::SimulationController,
) -> Option<String> {
    manual_deck_run_block_reason_in(model, netlist, messages).or_else(|| {
        rspice_app_runs::workbench::app_state::run_preflight::manual_deck_authorization_block_reason(
            model, controller,
        )
    })
}

/// User-facing reason the Netlist workspace cannot run the current deck. It
/// reads the model, the netlist editor's state and the message catalogue.
pub fn manual_deck_run_block_reason_in(
    model: &Model,
    netlist: &NetlistDocumentState,
    messages: crate::workbench::MessageCatalog,
) -> Option<String> {
    if model.simulation.execution.has_active_execution() {
        return Some("A simulation execution is already active".to_string());
    }
    let active_document = effective_active_document_in(netlist);
    if netlist.active_dependency_identity.is_some() {
        return Some(messages.text(crate::workbench::MessageId::NetlistDependencyCannotRun));
    }
    let source = working_deck_source_in(model, netlist);
    if source.trim().is_empty() {
        return Some("Enter a netlist before running".to_string());
    }
    if active_document == ActiveNetlistDocument::GeneratedDiff {
        return Some("Generated comparison documents cannot be executed".to_owned());
    }
    if active_document == ActiveNetlistDocument::RunSnapshot {
        return Some(
            "The deck a completed run used is sealed with that run; return to the working deck to run it again."
                .to_owned(),
        );
    }
    let current_digest = crate::state::content_digest(source);
    if active_document == ActiveNetlistDocument::Generated
        && (model.netlist_artifacts.generation_error.is_some()
            || model.netlist_artifacts.generated_input_digest
                != model.netlist_artifacts.current_generation_input_digest)
    {
        return Some(
            model
                .netlist_artifacts
                .generation_error
                .clone()
                .unwrap_or_else(|| "Regenerate the stale netlist before running".to_owned()),
        );
    }
    let validated = model
        .netlist_artifacts
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == current_digest
                && receipt.project_revision == model.workspace.content.project.revision().get()
        });
    if !validated {
        return Some("Validate the current source and project revision before running".to_owned());
    }
    None
}

/// Open a resolved direct or transitive dependency without changing its root
/// document selection or ownership. Unresolved and stale identities fail
/// closed and leave the current buffer unchanged. It works over the model, the
/// netlist editor's state, the include tabs the reader opened and the runs.
pub(crate) fn open_netlist_dependency_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    logical_identity: &str,
) -> Result<(), String> {
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    open_netlist_dependency_from_root_in(
        model,
        netlist,
        open_documents,
        runs,
        root,
        logical_identity,
    )
}

/// Open a dependency of the given root document, over the model, the netlist
/// editor's state, the include tabs the reader opened and the runs, whose
/// manual run forgets the lines edited in the deck it leaves.
pub fn open_netlist_dependency_from_root_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    root: ActiveNetlistDocument,
    logical_identity: &str,
) -> Result<(), String> {
    if root == ActiveNetlistDocument::OwnedSource {
        let path = std::path::PathBuf::from(logical_identity);
        return super::deck_files::show_include(model, netlist, open_documents, runs, &path);
    }
    if root == ActiveNetlistDocument::GeneratedDiff {
        return Err("Revision comparisons do not own dependency documents.".to_owned());
    }
    let document = canonical_root_document_in(model, netlist, root).ok_or_else(|| {
        "The selected dependency root is no longer available in this workspace.".to_owned()
    })?;
    let root_document_id = document.id();
    let source = document
        .dependencies()
        .iter()
        .find(|dependency| dependency.locator().logical_identity() == logical_identity)
        .and_then(crate::state::DependencyMetadata::source)
        .ok_or_else(|| {
            "The selected dependency is unresolved or no longer belongs to this source closure."
                .to_owned()
        })?
        .to_owned();
    put_dependency_in_front(
        model,
        netlist,
        open_documents,
        runs,
        root,
        root_document_id,
        logical_identity,
        source,
    )
}

/// Put the include `logical_identity` of the `root` document named
/// `root_document_id` in front on its own tab, showing `source`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn put_dependency_in_front(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    root: ActiveNetlistDocument,
    root_document_id: crate::state::NetlistDocumentId,
    logical_identity: &str,
    source: String,
) -> Result<(), String> {
    let tab = crate::workbench::state::WorkspaceDocumentId::NetlistDependency {
        root: root_document_id,
        logical_identity: logical_identity.to_owned(),
    };
    if !open_documents.contains(&tab)
        && open_documents.len() >= MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS
    {
        return Err(format!(
            "Close an include tab before opening another; the workspace retains at most {MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS} secondary netlist documents."
        ));
    }
    netlist.active_document = root;
    netlist.active_dependency_root = Some(root);
    netlist.active_dependency_identity = Some(logical_identity.to_owned());
    open_documents.insert(tab);
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = source;
    netlist.requested_line = None;
    netlist.cursor_line = 0;
    runs.manual_run.edited_lines.clear();
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    Ok(())
}

/// Return from an include document to its exact generated or owned root. It
/// writes the model, the netlist editor's state and the runs.
pub(crate) fn close_active_dependency_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &mut RunsState,
) -> bool {
    let Some(root) = netlist.active_dependency_root.take() else {
        return false;
    };
    netlist.active_dependency_identity = None;
    netlist.active_document = root;
    model.simulation.source.netlist_content = match root {
        ActiveNetlistDocument::Generated => model.netlist_artifacts.generated_source.clone(),
        ActiveNetlistDocument::OwnedSource => netlist
            .open_deck
            .as_ref()
            .map(|deck| deck.root.text.clone())
            .unwrap_or_default(),
        ActiveNetlistDocument::GeneratedDiff => netlist.generated_diff_source.clone(),
        ActiveNetlistDocument::RunSnapshot => run_snapshot_source_in(model, netlist, runs),
    };
    netlist.requested_line = None;
    netlist.cursor_line = 0;
    netlist.completion_open = false;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    refresh_diff_pips_from_baseline_in(model, runs);
    true
}

/// Take the reader to the generated deck from another workspace: the Netlist
/// workspace, with the generated primary in front. With nothing generated the
/// generated document still comes to the front, where the page states why
/// there is none, rather than leaving another deck under the command's name.
/// It works over the model, the netlist editor's state and the workbench.
pub fn show_generated_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) {
    if !open_generated_primary_in(model, netlist)
        && model.netlist_artifacts.generated_document.is_none()
    {
        netlist.active_dependency_identity = None;
        netlist.active_dependency_root = None;
        netlist.active_document = ActiveNetlistDocument::Generated;
        netlist.active_document_initialized = true;
    }
    if workbench.workspace != crate::workbench::state::Workspace::Netlist {
        workbench.activate(crate::workbench::state::Workspace::Netlist);
    }
}

/// Select the immutable generated primary without deleting or changing an
/// owned source or generated-diff document. This is an explicit document
/// transition; navigating among Code workspace pages must not call it. It
/// works over the model and the netlist editor's state.
pub fn open_generated_primary_in(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    if model.netlist_artifacts.generated_document.is_none()
        || model.netlist_artifacts.generated_source.trim().is_empty()
    {
        return false;
    }
    // A session that never chose a document is not showing the generated one
    // yet, whatever the default says: the page would put a retained deck in
    // front on its first frame.
    if netlist.active_document_initialized
        && netlist.active_document == ActiveNetlistDocument::Generated
        && netlist.active_dependency_identity.is_none()
    {
        return false;
    }
    netlist.active_dependency_identity = None;
    netlist.active_dependency_root = None;
    netlist.active_document = ActiveNetlistDocument::Generated;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = model.netlist_artifacts.generated_source.clone();
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    true
}

/// Code-workspace document currently projected into the central editor.
/// The generated netlist and the open deck are independent documents;
/// switching between them changes neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveNetlistDocument {
    #[default]
    Generated,
    /// The open deck, a file, or an include file open beside it.
    OwnedSource,
    GeneratedDiff,
    /// The exact deck a completed manual run consumed. Sealed with that run,
    /// so it is a viewer and never an editable deck.
    RunSnapshot,
}

/// User-selected reach of the Code workspace find surface. Replacement is
/// intentionally limited to project-owned source documents; project-reference
/// search is always find-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetlistFindScope {
    #[default]
    CurrentDocument,
    Selection,
    OpenDocuments,
    ActiveLanguageProject,
    ProjectReferences,
}

/// Persistent, keyboard-reachable state for the mockup's Find and replace
/// surface. Match rows are derived from the current exact source every frame,
/// so they cannot become stale after an edit.
#[derive(Debug, Clone)]
pub struct NetlistFindState {
    pub open: bool,
    pub find: String,
    pub replacement: String,
    pub match_case: bool,
    pub whole_symbol: bool,
    pub regular_expression: bool,
    pub include_comments: bool,
    pub include_generated_references: bool,
    pub scope: NetlistFindScope,
    pub selected_match: usize,
    pub error: Option<String>,
    /// A press the window made this frame, which the shell carries out once
    /// the page has drawn. The session does not store it.
    pub pending: Option<NetlistFindStep>,
}

impl Default for NetlistFindState {
    fn default() -> Self {
        Self {
            open: false,
            find: String::new(),
            replacement: String::new(),
            match_case: false,
            whole_symbol: false,
            regular_expression: false,
            include_comments: true,
            include_generated_references: false,
            scope: NetlistFindScope::CurrentDocument,
            selected_match: 0,
            error: None,
            pending: None,
        }
    }
}

/// A press in the find window: going to a match, replacing it, or replacing
/// every match. Going to a match opens its document and a replacement
/// commits through a candidate copy of the whole state, so the shell carries
/// it out once the page has drawn.
#[derive(Debug, Clone)]
pub struct NetlistFindStep {
    pub(crate) action: Option<NetlistFindAction>,
    /// The options the matches were found with.
    pub(crate) options: crate::state::FindOptions,
    /// The match the press names.
    pub(crate) target: Option<NetlistFindTarget>,
    /// What Replace all planned, against the scan the window drew.
    pub(crate) replacements: Option<Vec<OwnedNetlistReplacement>>,
}

/// What a press in the find window asks for.
#[derive(Debug, Clone, Copy)]
pub(crate) enum NetlistFindAction {
    Select(usize),
    ReplaceNext,
    ReplaceAll,
}

/// The match a find step names, held independently of the scan it came from.
#[derive(Debug, Clone)]
pub(crate) struct NetlistFindTarget {
    pub(crate) index: usize,
    pub(crate) active_document: ActiveNetlistDocument,
    pub(crate) dependency_identity: Option<String>,
    pub(crate) editable: bool,
    /// The document's bytes, present only for a replacement.
    pub(crate) source: Option<String>,
    pub(crate) line: usize,
    pub(crate) byte_range: std::ops::Range<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct NetlistComparisonDialogState {
    pub open: bool,
    pub selected_history_index: usize,
}

#[derive(Debug, Clone)]
pub struct NetlistExportDialogState {
    pub open: bool,
    pub bundle_dependencies: bool,
    pub include_source_map: bool,
    pub error: Option<String>,
}

impl Default for NetlistExportDialogState {
    fn default() -> Self {
        Self {
            open: false,
            bundle_dependencies: true,
            include_source_map: true,
            error: None,
        }
    }
}

/// Transient state for one netlist document.
#[derive(Debug, Clone, Default)]
pub struct NetlistDocumentState {
    /// The deck open in the workspace, a file, with the includes opened
    /// beside it.
    pub open_deck: Option<super::deck_files::OpenDeck>,
    /// The project deck whose file could not be read, so it is not tried
    /// again on every frame.
    pub unreadable_deck: Option<crate::state::DeckId>,
    /// Prior immutable artifacts retained for deterministic revision compare.
    pub generated_history: Vec<crate::state::GeneratedArtifact>,
    /// Read-only unified comparison document (`generated.diff`).
    pub generated_diff_source: String,
    /// Document selected before entering a read-only revision comparison.
    pub comparison_return_document: ActiveNetlistDocument,
    /// Active central document.
    pub active_document: ActiveNetlistDocument,
    /// Logical identity of the include projected into the editor, the path
    /// of a deck's include file, or `None` when the root is visible.
    pub active_dependency_identity: Option<String>,
    /// Root document the include in front belongs to. Stored separately so
    /// the generated deck's includes and the open deck's cannot be confused.
    pub active_dependency_root: Option<ActiveNetlistDocument>,
    /// Whether initial active-document selection has been reconciled with a
    /// just-opened project's deck.
    pub active_document_initialized: bool,
    /// Input digest of the latest generation that failed. The same inputs
    /// fail the same way, so they are not rebuilt, and the failure is not
    /// recorded again, until one of them changes.
    pub failed_generation_input_digest: Option<crate::product::ContentDigest>,
    /// Find/replace surface and selection state.
    pub find: NetlistFindState,
    /// Explicit multi-file replacement history. Ordinary editor keystrokes
    /// keep using the text editor's per-document history; these entries cover
    /// replacements that change one or more open files.
    edit_undo: Vec<NetlistEditJournalEntry>,
    edit_redo: Vec<NetlistEditJournalEntry>,
    pub signature_help: language::NetlistSignatureHelpState,
    pub comparison_dialog: NetlistComparisonDialogState,
    pub export_dialog: NetlistExportDialogState,
    /// One-based source line requested by outline, diagnostics, or find. The
    /// editor consumes this exactly once and places the caret there.
    pub requested_line: Option<usize>,
    /// One-based line selected through the current schematic's authoritative
    /// generated source map. Unlike the caret this can remain highlighted
    /// while another editor interaction has focus.
    pub cross_probe_line: Option<usize>,
    /// Buffer revision, bumped on every edit.
    pub revision: u64,
    /// Revision the diagnostics were parsed for.
    diag_revision: Option<u64>,
    /// Library folders the diagnostics resolved includes through; a change
    /// in Preferences stales them as an edit does.
    diag_library_folders: Vec<std::path::PathBuf>,
    /// `ui.input(..).time` of the last edit, for parse debounce.
    last_edit_time: f64,
    /// Revision the last edit typed in the editor left the buffer at. While
    /// the buffer stands there its indexes may trail it by a build; text put
    /// in the buffer any other way is indexed before a frame shows it.
    typed_revision: Option<u64>,
    /// Current parse diagnostics.
    pub diagnostics: std::sync::Arc<NetlistDiagnosticCollection>,
    /// The check of the shown source running away from the editor's
    /// thread, and what it found until a frame takes it.
    live_check: editor::LiveCheck,
    /// Result data version last reconciled with the editor baseline.
    seen_data_version: u64,
    /// Working deck the run snapshot was opened from, so its comparison knows
    /// which revision to diff against once the snapshot itself is active.
    pub(crate) run_snapshot_return_document: ActiveNetlistDocument,
    /// Which executed deck the run-snapshot document is showing.
    ///
    /// Absent means it is showing this session's manual-deck baseline: the
    /// deck somebody typed and ran, which is the only one there is a working
    /// copy to compare against. Present means it is showing the source one
    /// point of a completed run was actually handed — a different artifact,
    /// for a different question, with nothing to diff it against.
    pub executed_deck_view: Option<ExecutedDeckSelection>,
    /// Zero-based line containing the caret.
    pub cursor_line: usize,
    /// Unicode-scalar caret position in the visible source. Language actions
    /// use this exact editor result rather than guessing from the line.
    pub cursor_char_index: usize,
    /// Last non-empty editor selection expressed as an exact byte range in
    /// the visible source. Selection-scoped find never guesses from a line.
    pub selected_byte_range: Option<std::ops::Range<usize>>,
    /// Whether the completion popover was open last frame.
    pub completion_open: bool,
    /// Selected completion row.
    pub completion_index: usize,
    /// Revision at which completion was dismissed.
    pub completion_dismissed_at: Option<u64>,
    /// Harvested `.model` and `.subckt` symbols.
    symbols: Vec<completion::SymbolEntry>,
    /// The project index of the root the editor shows, keyed by the root
    /// document's exact revision and built away from the editor's thread.
    project_index: language::ProjectIndexSlot,
    /// Buffer-revision-keyed value spans behind the editor's value hover.
    value_index: Option<language::CachedValueIndex>,
    /// What the parse of one buffer revision left unread in the buffer: the
    /// lines of an untaken `.if` branch or an unselected inline `.lib`
    /// section. Absent when that parse failed, or read a deck other than the
    /// buffer as its own root.
    unread_source: Option<(
        u64,
        std::sync::Arc<[rspice_core::netlist::UnreadSourceRegion]>,
    )>,
    /// Outline and line offsets of the visible buffer. Read through
    /// [`visible_source_index_in`], which is what keeps it current.
    source_index: source_index::SourceIndexSlot,
    /// The same for the project's top-level deck while the editor shows other
    /// text. Read through [`root_deck_source_index`].
    root_deck_index: source_index::SourceIndexSlot,
    /// The name under the caret, as the frame last read it. Read through
    /// [`cross_probe::caret_name`].
    caret_name: cross_probe::CaretNameCache,
    /// Where the generated netlist writes the net last shown from the
    /// schematic. Drawn through [`cross_probe::net_marks`].
    net_marks: cross_probe::NetMarks,
}

impl NetlistDocumentState {
    /// Whether the buffer stands where an edit typed in the editor left it,
    /// so its indexes may trail it by a build while the writer types.
    fn is_being_typed(&self) -> bool {
        self.typed_revision == Some(self.revision)
    }

    /// Take the schematic cross-probe's line: withdraw the request it made
    /// before while nothing has asked for another line, mark `line`, and ask
    /// for it while the generated deck is the document shown.
    pub fn project_cross_probe_line(&mut self, released: Option<usize>, line: Option<usize>) {
        if released.is_some() && self.requested_line == released {
            self.requested_line = None;
        }
        self.cross_probe_line = line;
        if line.is_some() && self.active_document == ActiveNetlistDocument::Generated {
            self.requested_line = line;
        }
    }

    /// The buffer revision the findings this editor holds were validated
    /// against, or `None` where they are no validation: nothing has been
    /// parsed yet, or the comparison is showing, for which the editor clears
    /// its findings without parsing anything.
    pub(crate) fn validated_revision(&self) -> Option<u64> {
        if self.active_document == ActiveNetlistDocument::GeneratedDiff {
            return None;
        }
        self.diag_revision
    }

    /// Record a settled parse of `revision`, as the editor does once it has
    /// validated the buffer.
    #[cfg(any(test, feature = "test-support"))]
    pub fn mark_validated_for_test(&mut self, revision: u64) {
        self.diag_revision = Some(revision);
    }

    /// What the parse of `revision` left unread in the buffer, when that
    /// parse has run and read the buffer as its own root.
    pub(crate) fn unread_source_at(
        &self,
        revision: u64,
    ) -> Option<std::sync::Arc<[rspice_core::netlist::UnreadSourceRegion]>> {
        self.unread_source
            .as_ref()
            .filter(|(parsed, _)| *parsed == revision)
            .map(|(_, unread)| std::sync::Arc::clone(unread))
    }

    /// Whether a retained Code-workspace transaction owns exclusive input.
    /// This is queried before painting, so keyboard shortcuts cannot mutate
    /// the document behind a modal during its opening frame.
    pub fn application_modal_open(&self) -> bool {
        self.find.open || self.comparison_dialog.open || self.export_dialog.open
    }
}

/// Reconcile queued execution and diff state before rendering the document,
/// over the model, the netlist editor's state, the runs and the session.
pub fn prepare(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &mut RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) {
    if runs.manual_run.rerun_queued && !model.simulation.execution.has_active_execution() {
        runs.manual_run.rerun_queued = false;
        if let Some(reason) = manual_deck_run_block_reason_in(model, netlist, workbench.messages())
        {
            model.record(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Run,
                format!("Queued netlist re-run cancelled: {reason}"),
            );
        } else {
            request_netlist_manual_deck_run_in(model, runs, workbench);
        }
    }

    let data_version = model.simulation.view.data_version;
    if netlist.seen_data_version == data_version {
        return;
    }
    netlist.seen_data_version = data_version;
    if let Some(baseline) = runs.manual_run.last_run_buffer.as_deref() {
        runs.manual_run.edited_lines = baseline::changed_lines_against_baseline(
            &model.simulation.source.netlist_content,
            baseline,
        );
    }
}

/// Run the Netlist workspace's deck: remember the page Run was pressed on,
/// and start the run, or queue it behind the one going. It writes the model,
/// the runs and the session.
pub fn request_netlist_manual_deck_run_in(
    model: &mut Model,
    runs: &mut RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) {
    workbench.run_press_route = Some(workbench.current_route());
    model.simulation.execution.run_intent = crate::state::SimulationRunIntent::ManualDeck;
    if model.simulation.execution.has_active_execution() {
        runs.manual_run.rerun_queued = true;
    } else {
        model.simulation.execution.request_manual_deck_run();
    }
}

/// Mark the deck's lines that differ from the last manual run's buffer, over
/// the model and the runs.
fn refresh_diff_pips_from_baseline_in(model: &Model, runs: &mut RunsState) {
    let Some(baseline) = runs.manual_run.last_run_buffer.as_deref() else {
        return;
    };
    runs.manual_run.edited_lines = baseline::changed_lines_against_baseline(
        &model.simulation.source.netlist_content,
        baseline,
    );
}

#[cfg(any(test, feature = "test-support"))]
pub mod fixtures;
#[cfg(test)]
mod tests;
