//! Native startup recovery and current-launch safe-mode transactions.
//!
//! Recovery checkpoints are discovered only from paths already owned by the
//! recent-file/session model. Every operation revalidates the derived sibling
//! path and reparses the checkpoint at commit time. Opening a checkpoint
//! creates an unsaved project containing the editable candidate and, when the
//! saved source is readable, a read-only baseline cellview. The source and
//! checkpoint files are never modified by that operation.

#[cfg(not(target_arch = "wasm32"))]
use std::collections::{BTreeMap, BTreeSet};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use serde::Serialize;

use crate::common::app::{AppState, ConsoleMessage, RSpiceApp, RecentKind};
use crate::common::file_workflow::FileWorkflowIo;
use crate::io::{ProjectExecutionContext, ProjectSimulationResults};
#[cfg(not(target_arch = "wasm32"))]
use crate::state::{
    Cell, CellViewRef, Library, LibraryManager, OpenCellView, ProjectWorkspace, SchematicState,
    View, ViewType,
};

use super::state::{LocalSafeModeOptions, Workspace};

#[cfg(not(target_arch = "wasm32"))]
use crate::common::recovery_checkpoint::{
    CheckpointBinding, CheckpointInspection, CheckpointOwnership, SourceSnapshotRelation,
    discard_bound_checkpoint, inspect_checkpoint, read_bound_checkpoint, read_source_snapshot,
};

// Browser builds retain the shared launcher model but intentionally construct
// no native filesystem candidates.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RecoveryIntegrity {
    Verified {
        baseline_available: bool,
        components: usize,
        wires: usize,
        changed_objects: Option<usize>,
        baseline_note: Option<String>,
    },
    Invalid(String),
}

impl RecoveryIntegrity {
    pub(crate) fn is_recoverable(&self) -> bool {
        matches!(self, Self::Verified { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecoveryCandidate {
    pub(crate) original: PathBuf,
    pub(crate) checkpoint: PathBuf,
    pub(crate) display_name: String,
    pub(crate) modified_unix_ms: u64,
    pub(crate) age: String,
    pub(crate) integrity: RecoveryIntegrity,
    #[cfg(not(target_arch = "wasm32"))]
    binding: Option<CheckpointBinding>,
}

impl RecoveryCandidate {
    pub(crate) fn can_discard(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.binding.as_ref().is_some_and(|binding| {
                matches!(binding.ownership, CheckpointOwnership::Managed { .. })
            })
        }
        #[cfg(target_arch = "wasm32")]
        false
    }

    pub(crate) fn is_legacy_checkpoint(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.binding
                .as_ref()
                .is_some_and(|binding| matches!(binding.ownership, CheckpointOwnership::Legacy))
        }
        #[cfg(target_arch = "wasm32")]
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecoveryNoticeTone {
    Info,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecoveryNotice {
    pub(crate) message: String,
    pub(crate) tone: RecoveryNoticeTone,
}

#[derive(Debug, Clone)]
pub(crate) struct RecoveryCatalog {
    pub(crate) candidates: Vec<RecoveryCandidate>,
    pub(crate) selected_checkpoint: Option<PathBuf>,
    pub(crate) pending_discard: Option<RecoveryCandidate>,
    pub(crate) notice: Option<RecoveryNotice>,
    refresh_requested: bool,
}

impl Default for RecoveryCatalog {
    fn default() -> Self {
        Self {
            candidates: Vec::new(),
            selected_checkpoint: None,
            pending_discard: None,
            notice: None,
            refresh_requested: true,
        }
    }
}

impl RecoveryCatalog {
    pub(crate) fn request_refresh(&mut self) {
        self.refresh_requested = true;
    }

    pub(crate) fn selected(&self) -> Option<&RecoveryCandidate> {
        let selected = self.selected_checkpoint.as_ref()?;
        self.candidates
            .iter()
            .find(|candidate| &candidate.checkpoint == selected)
    }

    pub(crate) fn select(&mut self, checkpoint: PathBuf) {
        if self
            .candidates
            .iter()
            .any(|candidate| candidate.checkpoint == checkpoint)
        {
            self.selected_checkpoint = Some(checkpoint);
        }
    }

    pub(crate) fn info(&mut self, message: impl Into<String>) {
        self.notice = Some(RecoveryNotice {
            message: message.into(),
            tone: RecoveryNoticeTone::Info,
        });
    }

    pub(crate) fn warning(&mut self, message: impl Into<String>) {
        self.notice = Some(RecoveryNotice {
            message: message.into(),
            tone: RecoveryNoticeTone::Warning,
        });
    }
}

/// Refresh once when the launcher opens, enters Recovery, or completes a
/// destructive checkpoint action. Parsing never occurs in every paint frame.
pub(crate) fn refresh_catalog_if_requested(app: &mut RSpiceApp) {
    if !app
        .state
        .workbench
        .project_launcher_recovery
        .refresh_requested
    {
        return;
    }

    let source_paths = app
        .state
        .recent_files
        .iter()
        .filter(|recent| recent.kind == RecentKind::Schematic)
        .map(|recent| recent.path.clone())
        .collect::<Vec<_>>();
    let mut live_paths = app
        .state
        .workspace
        .schematic_buffers
        .values()
        .filter_map(|schematic| schematic.current_file.clone())
        .collect::<Vec<_>>();
    if let Some(path) = app.state.schematic.current_file.clone() {
        live_paths.push(path);
    }

    let candidates = discover_candidates(&source_paths, &live_paths, app.file_workflow_io.as_ref());
    let catalog = &mut app.state.workbench.project_launcher_recovery;
    let previous = catalog.selected_checkpoint.clone();
    catalog.candidates = candidates;
    catalog.selected_checkpoint = previous
        .filter(|selected| {
            catalog
                .candidates
                .iter()
                .any(|candidate| &candidate.checkpoint == selected)
        })
        .or_else(|| {
            catalog
                .candidates
                .iter()
                .find(|candidate| candidate.integrity.is_recoverable())
                .or_else(|| catalog.candidates.first())
                .map(|candidate| candidate.checkpoint.clone())
        });
    catalog.refresh_requested = false;
}

#[cfg(not(target_arch = "wasm32"))]
fn discover_candidates(
    source_paths: &[PathBuf],
    _live_paths: &[PathBuf],
    io: &(impl FileWorkflowIo + ?Sized),
) -> Vec<RecoveryCandidate> {
    let mut sources = source_paths.to_vec();
    sources.sort();
    sources.dedup();

    let mut candidates = sources
        .into_iter()
        .flat_map(|original| {
            crate::common::recovery_checkpoint::checkpoint_paths_for_source(&original)
                .into_iter()
                .filter_map(move |checkpoint| discover_candidate(original.clone(), checkpoint, io))
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .modified_unix_ms
            .cmp(&left.modified_unix_ms)
            .then_with(|| left.checkpoint.cmp(&right.checkpoint))
    });
    candidates
}

#[cfg(target_arch = "wasm32")]
fn discover_candidates(
    _source_paths: &[PathBuf],
    _live_paths: &[PathBuf],
    _io: &(impl FileWorkflowIo + ?Sized),
) -> Vec<RecoveryCandidate> {
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn discover_candidate(
    original: PathBuf,
    checkpoint: PathBuf,
    _io: &(impl FileWorkflowIo + ?Sized),
) -> Option<RecoveryCandidate> {
    let checkpoint_meta = std::fs::metadata(&checkpoint).ok()?;
    let checkpoint_modified = checkpoint_meta.modified().ok()?;

    let display_name = document_name(&original);
    let age = checkpoint_age(checkpoint_modified);
    let inspection = inspect_checkpoint(&original, &checkpoint);
    if matches!(inspection, CheckpointInspection::ActiveOwner) {
        return None;
    }
    let modified_unix_ms = match &inspection {
        CheckpointInspection::Candidate(binding) => binding.identity.modified_unix_ms,
        CheckpointInspection::Unsafe {
            identity: Some(identity),
            ..
        } => identity.modified_unix_ms,
        _ => checkpoint_modified
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .and_then(|duration| duration.as_millis().try_into().ok())
            .unwrap_or(0),
    };

    let (binding, integrity) = match inspection {
        CheckpointInspection::ActiveOwner => unreachable!("active owners returned above"),
        CheckpointInspection::Unsafe { message, .. } => (None, RecoveryIntegrity::Invalid(message)),
        CheckpointInspection::Candidate(binding) => {
            let recovered = read_bound_checkpoint(&original, &checkpoint, &binding)
                .and_then(|bytes| parse_checkpoint_bytes(&bytes, &checkpoint));
            let integrity = match recovered {
                Err(error) => RecoveryIntegrity::Invalid(format!(
                    "Checkpoint failed integrity validation: {error}"
                )),
                Ok(recovered) => {
                    let components = recovered.components.len();
                    let wires = recovered.wires.len();
                    let baseline =
                        read_source_snapshot(&original, &binding).and_then(|(bytes, relation)| {
                            parse_checkpoint_bytes(&bytes, &original)
                                .map(|baseline| (baseline, relation))
                        });
                    match baseline {
                        Ok((baseline, relation)) => RecoveryIntegrity::Verified {
                            baseline_available: true,
                            components,
                            wires,
                            changed_objects: Some(structural_delta(&baseline, &recovered)),
                            baseline_note: match relation {
                                SourceSnapshotRelation::Exact => None,
                                SourceSnapshotRelation::Changed => Some(
                                    "Saved baseline changed after this checkpoint was committed"
                                        .to_owned(),
                                ),
                                SourceSnapshotRelation::Unrecorded => Some(
                                    "Legacy checkpoint has no recorded saved-source identity"
                                        .to_owned(),
                                ),
                            },
                        },
                        Err(error) => RecoveryIntegrity::Verified {
                            baseline_available: false,
                            components,
                            wires,
                            changed_objects: None,
                            baseline_note: Some(format!("Saved baseline unavailable: {error}")),
                        },
                    }
                }
            };
            (Some(binding), integrity)
        }
    };

    Some(RecoveryCandidate {
        original,
        checkpoint,
        display_name,
        modified_unix_ms,
        age,
        integrity,
        binding,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_checkpoint_bytes(bytes: &[u8], checkpoint: &Path) -> Result<SchematicState, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("checkpoint is not valid UTF-8 JSON: {error}"))?;
    crate::io::schematic_io::load_schematic_text(text, Some(checkpoint))
        .map_err(|error| error.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn structural_delta(baseline: &SchematicState, recovered: &SchematicState) -> usize {
    changed_objects_by_id(&baseline.components, &recovered.components, |item| item.id)
        + changed_objects_by_id(&baseline.wires, &recovered.wires, |item| item.id)
        + changed_objects_by_id(&baseline.net_labels, &recovered.net_labels, |item| item.id)
        + changed_objects_by_id(&baseline.junctions, &recovered.junctions, |item| item.id)
}

/// Count semantic additions, removals, and modifications while preserving a
/// stable object identity. Exact matches are paired first, which also keeps a
/// malformed duplicate-ID input conservative: one changed duplicate counts as
/// one change instead of disappearing behind a map overwrite.
#[cfg(not(target_arch = "wasm32"))]
fn changed_objects_by_id<T: Serialize>(
    baseline: &[T],
    recovered: &[T],
    id: impl Fn(&T) -> u64,
) -> usize {
    fn grouped<T: Serialize>(
        items: &[T],
        id: &impl Fn(&T) -> u64,
    ) -> BTreeMap<u64, Vec<Option<serde_json::Value>>> {
        let mut groups = BTreeMap::<u64, Vec<Option<serde_json::Value>>>::new();
        for item in items {
            // Loaded schematics originated as JSON and therefore normally
            // reserialize. Treat an unexpected failure as non-matching rather
            // than silently declaring two objects equal.
            groups
                .entry(id(item))
                .or_default()
                .push(serde_json::to_value(item).ok());
        }
        groups
    }

    let mut baseline = grouped(baseline, &id);
    let mut recovered = grouped(recovered, &id);
    let identities = baseline
        .keys()
        .chain(recovered.keys())
        .copied()
        .collect::<BTreeSet<_>>();

    identities
        .into_iter()
        .map(|identity| {
            let left = baseline.remove(&identity).unwrap_or_default();
            let mut right = recovered.remove(&identity).unwrap_or_default();
            let mut unmatched_left = 0usize;

            for value in left {
                let exact = value.as_ref().and_then(|value| {
                    right
                        .iter()
                        .position(|candidate| candidate.as_ref() == Some(value))
                });
                if let Some(index) = exact {
                    right.swap_remove(index);
                } else {
                    unmatched_left += 1;
                }
            }

            // Objects sharing an identity are paired as modifications after
            // exact matches; any remainder is an addition or removal.
            unmatched_left.max(right.len())
        })
        .sum()
}

#[cfg(not(target_arch = "wasm32"))]
fn document_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Recovered document")
        .to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
fn checkpoint_age(modified: std::time::SystemTime) -> String {
    let Some(elapsed) = modified.elapsed().ok() else {
        return "time unavailable".to_owned();
    };
    match elapsed.as_secs() {
        seconds if seconds < 60 => "moments ago".to_owned(),
        seconds if seconds < 3_600 => format!("{} min ago", seconds / 60),
        seconds if seconds < 86_400 => format!("{} h ago", seconds / 3_600),
        seconds => format!("{} days ago", seconds / 86_400),
    }
}

fn validate_candidate_path(candidate: &RecoveryCandidate) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let expected = crate::common::file_workflow::autosave_checkpoint_path(&candidate.original);
        if expected != candidate.checkpoint {
            return Err(
                "Recovery checkpoint identity no longer matches its saved source".to_owned(),
            );
        }
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = candidate;
        Err("Native recovery checkpoints are unavailable in this browser".to_owned())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ProjectOwnedDifferences {
    execution_or_models: bool,
    results: bool,
}

impl ProjectOwnedDifferences {
    const fn is_empty(self) -> bool {
        !self.execution_or_models && !self.results
    }
}

fn canonical_execution_context(
    context: &ProjectExecutionContext,
) -> Result<serde_json::Value, String> {
    serde_json::to_value(context)
        .map_err(|error| format!("simulation/model state could not be compared: {error}"))
}

fn pristine_execution_context() -> Result<ProjectExecutionContext, String> {
    ProjectExecutionContext::from_state(
        &crate::common::app::SimSetupState::new(),
        &crate::common::app::default_model_library_manager(),
    )
    .map_err(|error| format!("pristine simulation/model state is invalid: {error}"))
}

fn project_owned_differences(state: &AppState) -> Result<ProjectOwnedDifferences, String> {
    let current_execution =
        ProjectExecutionContext::from_state(&state.sim_setup, &state.model_library_manager)
            .map_err(|error| format!("current simulation/model state is invalid: {error}"))?;
    let current_execution = canonical_execution_context(&current_execution)?;

    let current_results = ProjectSimulationResults::from_state(&state.simulation);
    current_results
        .validate()
        .map_err(|error| format!("current simulation results are invalid: {error}"))?;

    let (saved_execution, saved_results) =
        if let Some(project_path) = state.workspace.project.path.as_deref() {
            let project = crate::io::load_project_file(project_path).map_err(|error| {
                format!(
                    "saved project snapshot '{}' could not be loaded: {error}",
                    project_path.display()
                )
            })?;
            if let Some(warning) = project.simulation_results_warning.as_deref() {
                return Err(format!(
                    "saved project result snapshot could not be verified: {warning}"
                ));
            }
            let execution = match project.execution_context.as_ref() {
                Some(context) => canonical_execution_context(context)?,
                None => canonical_execution_context(&pristine_execution_context()?)?,
            };
            (execution, project.simulation_results)
        } else {
            (
                canonical_execution_context(&pristine_execution_context()?)?,
                ProjectSimulationResults::default(),
            )
        };

    Ok(ProjectOwnedDifferences {
        execution_or_models: current_execution != saved_execution,
        results: current_results != saved_results,
    })
}

/// Explain why replacing the active project with a recovery comparison would
/// destroy state that cannot be reconstructed from its saved project (or from
/// the documented pristine state of an unsaved project).
pub(crate) fn recovery_replacement_block_reason(state: &AppState) -> Option<String> {
    if state.schematic.is_dirty || state.workspace.any_dirty() {
        return Some(
            "Recovery comparison is blocked because the current design has unsaved changes. Save or close the current project before opening recovery work."
                .to_owned(),
        );
    }
    if state.simulation.is_running {
        return Some(
            "Recovery comparison is blocked because a simulation run is active. Stop the run, then save or close the current project before opening recovery work."
                .to_owned(),
        );
    }

    let differences = match project_owned_differences(state) {
        Ok(differences) => differences,
        Err(error) => {
            return Some(format!(
                "Recovery comparison is blocked because the current project state could not be verified against its recovery boundary: {error}. Save or close the current project before opening recovery work."
            ));
        }
    };
    if differences.is_empty() {
        return None;
    }

    let mut changed = Vec::with_capacity(2);
    if differences.execution_or_models {
        changed.push("simulation plan or model-library bindings");
    }
    if differences.results {
        changed.push("simulation result history or selection");
    }
    let baseline = if state.workspace.project.path.is_some() {
        "the last saved project snapshot"
    } else {
        "pristine unsaved-project defaults"
    };
    Some(format!(
        "Recovery comparison is blocked because {} differ from {baseline}. Save or close the current project before opening recovery work.",
        changed.join(" and ")
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn open_comparison(
    app: &mut RSpiceApp,
    candidate: &RecoveryCandidate,
) -> Result<(), String> {
    validate_candidate_path(candidate)?;
    if let Some(reason) = recovery_replacement_block_reason(&app.state) {
        return Err(reason);
    }

    let recovered = reopen_candidate(candidate)?;
    let (baseline, baseline_notice) = match reopen_baseline(candidate) {
        Ok((baseline, SourceSnapshotRelation::Exact)) => (Some(baseline), None),
        Ok((baseline, SourceSnapshotRelation::Changed)) => (
            Some(baseline),
            Some(
                "The saved baseline changed after the recovery checkpoint was committed; the comparison uses the exact current saved bytes"
                    .to_owned(),
            ),
        ),
        Ok((baseline, SourceSnapshotRelation::Unrecorded)) => (
            Some(baseline),
            Some(
                "This legacy checkpoint has no recorded saved-source identity; the comparison uses the exact current saved bytes"
                    .to_owned(),
            ),
        ),
        Err(error) => (
            None,
            Some(format!("Saved recovery baseline is unavailable: {error}")),
        ),
    };
    let comparison = build_comparison_workspace(&candidate.display_name, baseline, recovered)?;

    app.state.clear_design_execution_context();
    app.state.library_manager = comparison.libraries;
    app.state.workspace = comparison.workspace;
    app.state.schematic = comparison.active;
    app.state.sim_setup = crate::common::app::SimSetupState::new();
    app.state.model_library_manager = crate::common::app::default_model_library_manager();
    app.state.workbench.activate(Workspace::Design);
    app.state.push_user_message(ConsoleMessage::warning(format!(
        "Opened recovery comparison for '{}'; the saved source and checkpoint remain unchanged",
        candidate.display_name
    )));
    if let Some(notice) = baseline_notice {
        app.state.push_user_message(ConsoleMessage::warning(notice));
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn open_comparison(
    _app: &mut RSpiceApp,
    _candidate: &RecoveryCandidate,
) -> Result<(), String> {
    Err("Native recovery checkpoints are unavailable in this browser".to_owned())
}

#[cfg(not(target_arch = "wasm32"))]
fn reopen_candidate(candidate: &RecoveryCandidate) -> Result<SchematicState, String> {
    let binding = candidate
        .binding
        .as_ref()
        .ok_or_else(|| "Recovery checkpoint has no verified content identity".to_owned())?;
    let bytes = read_bound_checkpoint(&candidate.original, &candidate.checkpoint, binding)
        .map_err(|error| format!("Recovery checkpoint could not be reopened: {error}"))?;
    parse_checkpoint_bytes(&bytes, &candidate.checkpoint)
        .map_err(|error| format!("Recovery checkpoint could not be reopened: {error}"))
}

#[cfg(not(target_arch = "wasm32"))]
fn reopen_baseline(
    candidate: &RecoveryCandidate,
) -> Result<(SchematicState, SourceSnapshotRelation), String> {
    let binding = candidate
        .binding
        .as_ref()
        .ok_or_else(|| "Recovery checkpoint has no verified content identity".to_owned())?;
    let (bytes, relation) = read_source_snapshot(&candidate.original, binding)?;
    let baseline = parse_checkpoint_bytes(&bytes, &candidate.original)
        .map_err(|error| format!("Saved recovery baseline could not be opened: {error}"))?;
    Ok((baseline, relation))
}

#[cfg(not(target_arch = "wasm32"))]
struct ComparisonWorkspace {
    libraries: LibraryManager,
    workspace: ProjectWorkspace,
    active: SchematicState,
}

#[cfg(not(target_arch = "wasm32"))]
fn build_comparison_workspace(
    source_name: &str,
    baseline: Option<SchematicState>,
    mut recovered: SchematicState,
) -> Result<ComparisonWorkspace, String> {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let project_name = format!("{source_name} recovery");
    workspace
        .project
        .rename(project_name)
        .or_else(|_| workspace.project.rename("Recovered work"))
        .map_err(|error| format!("Recovery project identity is invalid: {error}"))?;
    workspace.project.path = None;
    workspace.project.description =
        "Unsaved recovery comparison. Original source and checkpoint are retained.".to_owned();

    recovered.current_file = None;
    recovered.is_dirty = true;
    recovered.read_only = false;
    recovered.needs_history_reset = true;
    let candidate_reference = workspace.active_view.clone();
    workspace
        .schematic_buffers
        .insert(candidate_reference.key(), recovered.clone());
    workspace.set_active_dirty(true);

    if let Some(mut baseline) = baseline {
        // Library/cell/view identifiers are persisted slash-delimited keys and
        // therefore follow the same alphanumeric/underscore contract as the
        // library dialogs. Keep the human-readable role in metadata below.
        const BASELINE_LIBRARY: &str = "recovery_baseline";
        const BASELINE_CELL: &str = "saved_baseline";
        const BASELINE_VIEW: &str = "schematic";
        let mut cell = Cell::new(BASELINE_CELL);
        cell.add_view(View::new(BASELINE_VIEW, ViewType::Schematic));
        let mut library = Library::new(BASELINE_LIBRARY);
        library.add_cell(cell);
        library.read_only = true;
        library
            .metadata
            .insert("role".to_owned(), "immutable recovery baseline".to_owned());
        library
            .metadata
            .insert("display_name".to_owned(), "Recovery baseline".to_owned());
        libraries.add_library(library);

        baseline.is_dirty = false;
        baseline.read_only = true;
        baseline.needs_history_reset = true;
        let baseline_reference = CellViewRef::new(BASELINE_LIBRARY, BASELINE_CELL, BASELINE_VIEW);
        workspace
            .schematic_buffers
            .insert(baseline_reference.key(), baseline);
        workspace
            .open_views
            .push(OpenCellView::new(baseline_reference, ViewType::Schematic));
    }

    workspace.open_as_root(candidate_reference.clone(), ViewType::Schematic);
    workspace.set_active_dirty(true);
    libraries.select_view(
        &candidate_reference.library,
        &candidate_reference.cell,
        &candidate_reference.view,
    );

    Ok(ComparisonWorkspace {
        libraries,
        workspace,
        active: recovered,
    })
}

pub(crate) fn discard_checkpoint(candidate: &RecoveryCandidate) -> Result<String, String> {
    validate_candidate_path(candidate)?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let binding = candidate
            .binding
            .as_ref()
            .ok_or_else(|| "Recovery checkpoint has no verified content identity".to_owned())?;
        discard_bound_checkpoint(&candidate.original, &candidate.checkpoint, binding)
            .map_err(|error| format!("Recovery checkpoint could not be discarded: {error}"))?;
        Ok(format!(
            "Discarded recovery checkpoint for '{}'",
            candidate.display_name
        ))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = candidate;
        Err("Native recovery checkpoints are unavailable in this browser".to_owned())
    }
}

/// Enter the safe-mode subset the current runtime can enforce. The exact
/// pre-safe-mode session is serialized before any state is replaced and is
/// used by AppState persistence until this process exits.
pub(crate) fn start_local_safe_mode(
    app: &mut RSpiceApp,
    options: LocalSafeModeOptions,
) -> Result<(), String> {
    if app.state.workbench.safe_mode.active {
        return Err("Safe mode is already active for this launch".to_owned());
    }
    if !options.has_effect() {
        return Err("Select at least one safe-mode isolation option".to_owned());
    }

    app.state.sync_active_schematic_to_workspace();
    let preserved_session = serde_json::to_string(&app.state)
        .map_err(|error| format!("The current session could not be protected: {error}"))?;

    if options.isolate_prior_documents {
        crate::common::project_workflow::create_new_project(&mut app.state);
    }
    if options.reset_layout {
        app.state.workbench.reset_layout();
    }
    app.state
        .workbench
        .safe_mode
        .activate(options, preserved_session);
    app.state.workbench.activate(Workspace::Project);
    app.state.push_user_message(ConsoleMessage::warning(
        "Safe mode is active for this launch; the prior session is retained for the next normal launch",
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, Component, ComponentType, Junction, NetLabel, Point,
        SimulationRun, Wire,
    };

    #[test]
    fn comparison_workspace_keeps_recovery_editable_and_baseline_read_only() {
        let mut baseline = SchematicState::default();
        baseline.is_dirty = false;
        let mut recovered = baseline.clone();
        recovered.is_dirty = false;

        let comparison = build_comparison_workspace("amplifier", Some(baseline), recovered)
            .expect("comparison builds");

        assert!(comparison.workspace.project.path.is_none());
        assert!(comparison.active.is_dirty);
        assert!(comparison.active.current_file.is_none());
        assert_eq!(comparison.workspace.open_views.len(), 2);
        let baseline_library = comparison
            .libraries
            .get_library("recovery_baseline")
            .expect("baseline library exists");
        assert!(baseline_library.read_only);
        assert!(comparison.workspace.any_dirty());
    }

    #[test]
    fn comparison_without_a_saved_baseline_still_preserves_an_unsaved_candidate() {
        let comparison = build_comparison_workspace("orphan", None, SchematicState::default())
            .expect("checkpoint-only comparison builds");

        assert_eq!(comparison.workspace.open_views.len(), 1);
        assert!(comparison.active.is_dirty);
        assert!(comparison.workspace.project.path.is_none());
    }

    #[test]
    fn safe_mode_requires_a_real_local_effect() {
        assert!(
            !LocalSafeModeOptions {
                isolate_prior_documents: false,
                reset_layout: false,
            }
            .has_effect()
        );
        assert!(LocalSafeModeOptions::default().has_effect());
    }

    #[test]
    fn structural_delta_detects_equal_count_object_edits() {
        let mut baseline = SchematicState::default();
        baseline.components.push(
            Component::new(1, ComponentType::Resistor, Point::new(10, 10))
                .with_name_value("R1", "1k"),
        );
        baseline
            .wires
            .push(Wire::segment(2, Point::new(0, 0), Point::new(10, 0)));
        baseline
            .net_labels
            .push(NetLabel::new(3, Point::new(10, 0), "OUT"));
        baseline.junctions.push(Junction::new(4, Point::new(10, 0)));

        let mut recovered = baseline.clone();
        recovered.components[0].value = "2k".to_owned();
        recovered.wires[0].points[1] = Point::new(20, 0);
        recovered.net_labels[0].name = "SENSE".to_owned();
        recovered.junctions[0].pos = Point::new(20, 0);

        assert_eq!(structural_delta(&baseline, &recovered), 4);
    }

    #[test]
    fn replacement_guard_blocks_nondefault_unsaved_execution_and_results() {
        let pristine = AppState::default();
        assert_eq!(recovery_replacement_block_reason(&pristine), None);

        let mut changed_plan = pristine.clone();
        changed_plan.sim_setup.tran.stop = "25u".to_owned();
        let reason = recovery_replacement_block_reason(&changed_plan)
            .expect("changed unsaved simulation plan must block replacement");
        assert!(reason.contains("simulation plan or model-library bindings"));
        assert!(reason.contains("pristine unsaved-project defaults"));

        let mut changed_results = pristine;
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(
            1,
            AnalysisType::Transient,
            "TRAN recovery guard",
        ));
        changed_results.simulation.runs.push(run);
        changed_results.simulation.next_run_id = 1;
        let reason = recovery_replacement_block_reason(&changed_results)
            .expect("unsaved result history must block replacement");
        assert!(reason.contains("simulation result history or selection"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn replacement_guard_compares_project_owned_state_to_saved_snapshot() {
        let root = unique_temp_dir("recovery-project-boundary");
        let project_path = root.join("guard.rspiceproj");
        let mut state = AppState::default();
        assert!(crate::common::project_workflow::save_project_to_path(
            &mut state,
            &project_path
        ));
        assert_eq!(recovery_replacement_block_reason(&state), None);

        state.sim_setup.tran.stop = "17u".to_owned();
        let reason = recovery_replacement_block_reason(&state)
            .expect("execution state changed after save must block replacement");
        assert!(reason.contains("last saved project snapshot"));
        assert!(reason.contains("simulation plan or model-library bindings"));

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn discovery_retains_malformed_checkpoint_as_blocked_evidence() {
        let root = unique_temp_dir("recovery-discovery");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let checkpoint = crate::common::file_workflow::autosave_checkpoint_path(&source);
        std::fs::write(&checkpoint, "not schematic json").expect("write malformed checkpoint");

        let candidates = discover_candidates(
            &[source.clone()],
            &[],
            &crate::common::file_workflow::NativeFileWorkflowIo,
        );
        assert_eq!(candidates.len(), 1);
        assert!(matches!(
            candidates[0].integrity,
            RecoveryIntegrity::Invalid(_)
        ));

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn discovery_lists_every_generation_even_when_the_source_is_live() {
        let root = unique_temp_dir("recovery-generations");
        let source = root.join("design.rsch");
        crate::io::save_schematic(&SchematicState::default(), &source).expect("save source");
        let legacy = crate::common::file_workflow::autosave_checkpoint_path(&source);
        std::fs::write(&legacy, "not schematic json").expect("write malformed legacy point");
        let mut generated_name = legacy.as_os_str().to_owned();
        generated_name.push(format!(".generation-{}", uuid::Uuid::new_v4()));
        let generated = PathBuf::from(generated_name);
        let mut generated_bytes =
            serde_json::to_vec_pretty(&crate::io::SchematicFile::new(SchematicState::default()))
                .expect("serialize generated point");
        generated_bytes.push(b'\n');
        std::fs::write(&generated, generated_bytes).expect("write generated point");

        let candidates = discover_candidates(
            &[source.clone()],
            &[source],
            &crate::common::file_workflow::NativeFileWorkflowIo,
        );

        assert_eq!(candidates.len(), 2);
        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.checkpoint == legacy)
        );
        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.checkpoint == generated)
        );
        assert!(
            candidates
                .iter()
                .any(|candidate| candidate.integrity.is_recoverable())
        );
        assert!(
            candidates
                .iter()
                .any(|candidate| matches!(candidate.integrity, RecoveryIntegrity::Invalid(_)))
        );

        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_temp_dir(label: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("rspice-{label}-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&path).expect("create fixture directory");
        path
    }
}
