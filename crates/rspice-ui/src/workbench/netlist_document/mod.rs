//! Netlist document engine.
//!
//! This module owns editor diagnostics, syntax highlighting, completion,
//! parameter tuning, and run-to-run diff state. It has no application-chrome
//! chrome; the canonical Netlist workspace composes it as a document surface.

use std::collections::{HashMap, HashSet};

use crate::common::AppState;

mod baseline;
mod completion;
pub mod diagnostics;
mod editor;
mod highlight;
mod summary;
mod tuner;

pub use diagnostics::{Diagnostic, DiagnosticSeverity};
pub use editor::show as show_editor;
pub use tuner::right_panel as show_parameter_inspector;

/// Stable identity for the exact UTF-8 source bytes visible in the editor.
pub fn source_content_digest(source: &str) -> crate::product::ContentDigest {
    super::code_workspace::content_digest(source)
}

/// Invalidate byte-bound review evidence after any ownership or source edit.
pub fn invalidate_source_evidence(document: &mut NetlistDocumentState) {
    document.validation = None;
    document.validation_error = None;
}

/// Atomically replace the exact project-owned source across the canonical
/// document, persisted project projection, and visible editor buffer.
pub fn replace_owned_source(state: &mut AppState, source: String) -> bool {
    if state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource {
        return false;
    }
    let next_document = if let Some(document) = &state.ui.netlist.owned_document {
        let mut next = document.clone();
        if next
            .replace_editable_source(next.content_digest(), source.as_bytes().to_vec())
            .is_err()
        {
            return false;
        }
        Some(next)
    } else {
        None
    };
    if !state
        .workspace
        .replace_editable_netlist_source(source.clone())
    {
        return false;
    }
    state.simulation.netlist_content = source;
    if let Some(document) = next_document {
        state.workspace.netlist_document = Some(document.clone());
        state.ui.netlist.owned_document = Some(document);
    }
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

/// Select the immutable generated primary without deleting or changing an
/// owned source or generated-diff document. This is an explicit document
/// transition; navigating among Code workspace pages must not call it.
pub fn open_generated_primary(state: &mut AppState) -> bool {
    if state.ui.netlist.active_document == ActiveNetlistDocument::Generated {
        return false;
    }
    state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = state.ui.netlist.generated_source.clone();
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

/// Open a separate immutable `generated.diff` document comparing the newest
/// retained predecessor with the current generated artifact.
pub fn compare_latest_generated_revision(state: &mut AppState) -> Result<(), String> {
    let index = state
        .ui
        .netlist
        .generated_history
        .len()
        .checked_sub(1)
        .ok_or_else(|| "No prior generated revision is retained for comparison.".to_owned())?;
    compare_generated_revision(state, index)
}

pub fn compare_generated_revision(state: &mut AppState, index: usize) -> Result<(), String> {
    let previous = state
        .ui
        .netlist
        .generated_history
        .get(index)
        .ok_or_else(|| "The selected generated revision is no longer retained.".to_owned())?;
    let current = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .map(super::code_workspace::NetlistDocument::generated_artifact)
        .ok_or_else(|| "No current generated artifact is available.".to_owned())?;
    let previous_label = format!("generated-{}", short_digest(previous.content_digest()));
    let current_label = format!("generated-{}", short_digest(current.content_digest()));
    let diff = similar::TextDiff::from_lines(previous.source(), current.source())
        .unified_diff()
        .context_radius(3)
        .header(&previous_label, &current_label)
        .to_string();
    state.ui.netlist.generated_diff_source = if diff.is_empty() {
        format!("--- {previous_label}\n+++ {current_label}\n No source changes\n")
    } else {
        diff
    };
    state.ui.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = state.ui.netlist.generated_diff_source.clone();
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    invalidate_source_evidence(&mut state.ui.netlist);
    Ok(())
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    digest.to_string().chars().take(12).collect()
}

/// Runtime evidence that the exact visible deck passed the same preparation
/// contract used by execution. The receipt is invalid as soon as the visible
/// content digest changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistValidationReceipt {
    pub visible_content_digest: crate::product::ContentDigest,
    pub executable_source_digest: crate::product::ContentDigest,
    pub prepared_snapshot_digest: crate::product::ContentDigest,
    pub project_revision: u64,
    pub task_count: usize,
    pub advisory_count: usize,
}

/// Code-workspace document currently projected into the central editor.
/// Generated and owned source are independent retained documents; switching
/// between them never deletes either artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveNetlistDocument {
    #[default]
    Generated,
    OwnedSource,
    GeneratedDiff,
}

/// User-selected reach of the Code workspace find surface. Replacement is
/// intentionally limited to project-owned source documents; project-reference
/// search is always find-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetlistFindScope {
    #[default]
    CurrentDocument,
    AllOwnedSources,
    ProjectReferences,
}

/// Persistent, keyboard-reachable state for the mockup's Find and replace
/// surface. Match rows are derived from the current exact source every frame,
/// so they cannot become stale after an edit.
#[derive(Debug, Clone, Default)]
pub struct NetlistFindState {
    pub open: bool,
    pub find: String,
    pub replacement: String,
    pub match_case: bool,
    pub whole_symbol: bool,
    pub regular_expression: bool,
    pub scope: NetlistFindScope,
    pub selected_match: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NetlistOwnershipDialogState {
    pub open: bool,
    pub artifact_name: String,
    pub strategy: crate::state::OwnedNetlistEditStrategy,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct NetlistComparisonDialogState {
    pub open: bool,
    pub selected_history_index: usize,
}

#[derive(Debug, Clone)]
pub struct NetlistSaveDialogState {
    pub open: bool,
    pub message: String,
    pub error: Option<String>,
}

impl Default for NetlistSaveDialogState {
    fn default() -> Self {
        Self {
            open: false,
            message: "Update owned SPICE source".to_owned(),
            error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetlistExportDialogState {
    pub open: bool,
    pub format: crate::io::NetlistFormat,
    pub bundle_dependencies: bool,
    pub include_source_map: bool,
    pub error: Option<String>,
}

impl Default for NetlistExportDialogState {
    fn default() -> Self {
        Self {
            open: false,
            format: crate::io::NetlistFormat::Spice,
            bundle_dependencies: true,
            include_source_map: true,
            error: None,
        }
    }
}

impl Default for NetlistOwnershipDialogState {
    fn default() -> Self {
        Self {
            open: false,
            artifact_name: "top_override.sp".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            error: None,
        }
    }
}

/// Transient state for one netlist document.
#[derive(Debug, Clone, Default)]
pub struct NetlistDocumentState {
    /// Canonical immutable document, including generated provenance,
    /// dependency identities, source map, outline, and validation state.
    pub generated_document: Option<super::code_workspace::NetlistDocument>,
    /// Canonical project-owned document retained independently from the
    /// generated primary.
    pub owned_document: Option<super::code_workspace::NetlistDocument>,
    /// Prior immutable artifacts retained for deterministic revision compare.
    pub generated_history: Vec<super::code_workspace::GeneratedArtifact>,
    /// Read-only unified comparison document (`generated.diff`).
    pub generated_diff_source: String,
    /// Runtime-generated primary artifact retained independently from any
    /// project-owned source document.
    pub generated_source: String,
    /// Active central document.
    pub active_document: ActiveNetlistDocument,
    /// Whether initial active-document selection has been reconciled with a
    /// just-opened project's persisted owned source.
    pub active_document_initialized: bool,
    /// Exact project-input digest used to produce the retained generated
    /// artifact. `None` means the current bytes have no generated authority.
    pub generated_input_digest: Option<crate::product::ContentDigest>,
    /// Input digest observed this frame. A mismatch with
    /// `generated_input_digest` makes the retained artifact stale and blocks
    /// execution until generation succeeds.
    pub current_generation_input_digest: Option<crate::product::ContentDigest>,
    /// Exact diagnostic from the latest failed generation attempt.
    pub generation_error: Option<String>,
    /// Receipt for the latest exact visible source validation.
    pub validation: Option<NetlistValidationReceipt>,
    /// Exact validation failure retained until the bytes change or validation
    /// succeeds.
    pub validation_error: Option<String>,
    /// Content digest most recently published through Save Source/Save As.
    pub externally_saved_content_digest: Option<crate::product::ContentDigest>,
    /// Find/replace surface and selection state.
    pub find: NetlistFindState,
    pub ownership_dialog: NetlistOwnershipDialogState,
    pub comparison_dialog: NetlistComparisonDialogState,
    pub save_dialog: NetlistSaveDialogState,
    pub export_dialog: NetlistExportDialogState,
    /// One-based source line requested by outline, diagnostics, or find. The
    /// editor consumes this exactly once and places the caret there.
    pub requested_line: Option<usize>,
    /// Buffer revision, bumped on every edit.
    pub revision: u64,
    /// Revision the diagnostics were parsed for.
    diag_revision: Option<u64>,
    /// `ui.input(..).time` of the last edit, for parse debounce.
    last_edit_time: f64,
    /// Current parse diagnostics.
    pub diagnostics: Vec<Diagnostic>,
    /// Zero-based lines edited since the last completed manual-deck run.
    pub edited_lines: HashSet<usize>,
    /// Result data version last reconciled with the editor baseline.
    seen_data_version: u64,
    /// Exact editor buffer from the last successful manual-deck run.
    pub last_run_buffer: Option<String>,
    /// Numeric `.param` values captured from `last_run_buffer`.
    pub last_run_params: HashMap<String, f64>,
    /// Editor buffer captured when the current manual-deck run started.
    pub pending_run_buffer: Option<String>,
    /// Run id associated with `pending_run_buffer`.
    pub pending_manual_run_id: Option<u64>,
    /// Zero-based line containing the caret.
    pub cursor_line: usize,
    /// Whether parameter tuning re-runs on every slider movement.
    pub tuner_live: bool,
    /// A re-run requested while the engine was busy.
    pub rerun_queued: bool,
    /// Stable parameter slider ranges.
    pub param_ranges: HashMap<String, (f64, f64)>,
    /// Whether the completion popover was open last frame.
    pub completion_open: bool,
    /// Selected completion row.
    pub completion_index: usize,
    /// Revision at which completion was dismissed.
    pub completion_dismissed_at: Option<u64>,
    /// Harvested `.model` and `.subckt` symbols.
    symbols: Vec<completion::SymbolEntry>,
}

impl NetlistDocumentState {
    /// Whether a retained Code-workspace transaction owns exclusive input.
    /// This is queried before painting, so keyboard shortcuts cannot mutate
    /// the document behind a modal during its opening frame.
    pub(crate) fn application_modal_open(&self) -> bool {
        self.find.open
            || self.ownership_dialog.open
            || self.comparison_dialog.open
            || self.save_dialog.open
            || self.export_dialog.open
    }
}

/// Reconcile queued execution and diff state before rendering the document.
pub fn prepare(state: &mut AppState) {
    if state.ui.netlist.rerun_queued && !state.simulation.is_running {
        state.ui.netlist.rerun_queued = false;
        if let Some(reason) = state.manual_deck_run_block_reason() {
            state.push_user_message(crate::common::ConsoleMessage::warning(format!(
                "Queued netlist re-run cancelled: {reason}"
            )));
        } else {
            state.request_netlist_manual_deck_run();
        }
    }

    let data_version = state.simulation.data_version;
    if state.ui.netlist.seen_data_version == data_version {
        return;
    }
    state.ui.netlist.seen_data_version = data_version;
    if let Some(baseline) = state.ui.netlist.last_run_buffer.as_deref() {
        state.ui.netlist.edited_lines =
            baseline::changed_lines_against_baseline(&state.simulation.netlist_content, baseline);
    }
}

pub(super) fn request_run(state: &mut AppState) {
    if let Some(reason) = state.manual_deck_run_block_reason() {
        state.push_user_message(crate::common::ConsoleMessage::warning(format!(
            "Netlist run blocked: {reason}"
        )));
    } else {
        state.request_netlist_manual_deck_run();
    }
}

pub(super) fn refresh_diff_pips_from_baseline(state: &mut AppState) {
    let Some(baseline) = state.ui.netlist.last_run_buffer.as_deref() else {
        return;
    };
    state.ui.netlist.edited_lines =
        baseline::changed_lines_against_baseline(&state.simulation.netlist_content, baseline);
}

/// Numeric assignments in the active source, used by automation and tests.
pub fn buffer_assignments(buffer: &str) -> Vec<(String, usize, usize)> {
    tuner::buffer_assignments(buffer)
}
