//! Cross-document design undo/redo.
//!
//! Ordinary schematic history intentionally owns one buffer. Hierarchy
//! extraction also creates a library cell, a child buffer, a generated symbol,
//! and a navigation transition, so it is retained here as one guarded project
//! transaction. Guards fail closed if either document or the target cell has
//! changed; no snapshot ever overwrites later work.

use std::collections::BTreeMap;

use crate::product::ObjectRevision;
#[cfg(test)]
use crate::state::model_library::{
    CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationSuite, ModelCorrelationState,
    ModelSourceEvidenceBinding, ProjectModelDefinition, ProjectModelRevisionDefinition,
};
use crate::state::model_library::{ModelLibrary, ModelSourceAuthority, ProjectModelCommit};
use crate::state::{
    Cell, CellViewRef, ComponentType, DesignManagementCatalog, LibraryManager, ModelLibraryManager,
    OpenCellView, SchematicSnapshot, SchematicState,
};

use crate::workbench::app_state::AppState;

const MAX_PROJECT_DESIGN_STEPS: usize = 32;

#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectDesignHistory {
    undo: Vec<ProjectDesignRecord>,
    redo: Vec<ProjectDesignRecord>,
}

#[derive(Debug, Clone)]
enum ProjectDesignRecord {
    HierarchyExtraction(Box<HierarchyExtractionRecord>),
    DesignManagement(Box<DesignManagementRecord>),
    SymbolDefinition(Box<SymbolDefinitionRecord>),
    ModelDefinition(Box<ModelDefinitionRecord>),
    ModelLibraries(Box<ModelLibrariesRecord>),
    ModelResolutions(Box<ModelResolutionRecordsRecord>),
}

#[derive(Debug, Clone)]
struct HierarchyExtractionRecord {
    description: String,
    parent_ref: CellViewRef,
    target_schematic_ref: CellViewRef,
    target_open_ref: CellViewRef,
    before_parent: SchematicSnapshot,
    after_parent: SchematicSnapshot,
    child: SchematicSnapshot,
    child_template: SchematicState,
    target_cell: Cell,
    open_views_before: Vec<OpenCellView>,
    hierarchy_stack_before: Vec<CellViewRef>,
    hierarchy_instances_before: Vec<String>,
    open_views_after: Vec<OpenCellView>,
    hierarchy_stack_after: Vec<CellViewRef>,
    hierarchy_instances_after: Vec<String>,
}

/// One guarded, project-scoped design-management transaction. The catalogs
/// are immutable before/after values; logical project revisions are guards,
/// not values to restore, so undo and redo each advance revision monotonically.
#[derive(Debug, Clone)]
struct DesignManagementRecord {
    description: String,
    owner: CellViewRef,
    before: DesignManagementCatalog,
    after: DesignManagementCatalog,
    before_schematics: BTreeMap<String, SchematicSnapshot>,
    after_schematics: BTreeMap<String, SchematicSnapshot>,
    undo_guard_revision: ObjectRevision,
    redo_guard_revision: Option<ObjectRevision>,
}

/// One guarded project-library symbol definition mutation.  Only the affected
/// cell is retained: importing a new definition and publishing a parameter
/// form are project-scoped edits, but neither operation owns unrelated
/// schematics or libraries.
#[derive(Debug, Clone)]
struct SymbolDefinitionRecord {
    description: String,
    library: String,
    cell: String,
    before: Option<Cell>,
    after: Option<Cell>,
    undo_guard_revision: ObjectRevision,
    redo_guard_revision: Option<ObjectRevision>,
    fixture: Option<SymbolDefinitionFixtureRecord>,
}

#[derive(Debug, Clone)]
struct SymbolDefinitionFixtureRecord {
    reference: CellViewRef,
    before: Option<SchematicState>,
    after: Option<SchematicState>,
}

/// One guarded project-owned model-source publication. Only the affected
/// model library is retained; manager filters, selection, and unrelated
/// libraries are presentation or independent domain state.
#[derive(Debug, Clone)]
struct ModelDefinitionRecord {
    description: String,
    library: String,
    model: String,
    before: Option<ModelLibrary>,
    after: Option<ModelLibrary>,
    undo_guard_revision: ObjectRevision,
    redo_guard_revision: Option<ObjectRevision>,
}

/// One guarded replacement of the complete loaded model-library set.
#[derive(Debug, Clone)]
struct ModelLibrariesRecord {
    description: String,
    before: Vec<ModelLibrary>,
    after: Vec<ModelLibrary>,
    undo_guard_revision: ObjectRevision,
    redo_guard_revision: Option<ObjectRevision>,
}

#[derive(Debug, Clone)]
struct ModelResolutionRecordsRecord {
    description: String,
    before: Vec<crate::state::model_library::ModelResolutionRecord>,
    after: Vec<crate::state::model_library::ModelResolutionRecord>,
    undo_guard_revision: ObjectRevision,
    redo_guard_revision: Option<ObjectRevision>,
}

/// Optional real schematic-buffer delta published together with a symbol
/// definition. Create Symbol uses this for its generated simulation fixture;
/// import and parameter-form edits pass `None`.
#[derive(Debug, Clone)]
pub(crate) struct SymbolDefinitionFixtureDelta {
    pub(crate) reference: CellViewRef,
    pub(crate) before: Option<SchematicState>,
    pub(crate) after: Option<SchematicState>,
}

pub(crate) struct SymbolDefinitionHistoryEntry {
    pub(crate) description: String,
    pub(crate) library: String,
    pub(crate) cell: String,
    pub(crate) before: Option<Cell>,
    pub(crate) after: Option<Cell>,
    pub(crate) committed_revision: ObjectRevision,
    fixture: Option<SymbolDefinitionFixtureRecord>,
}

fn require_unchanged_valid_model_resolution_ledger(
    state: &AppState,
    candidate: &ModelLibraryManager,
    transaction: &str,
) -> Result<(), String> {
    let expected = state.model_library_manager.owned_model_resolution_records();
    if candidate.owned_model_resolution_records() != expected {
        return Err(format!(
            "{transaction} cannot change model provider decisions; publish that ledger as a separate guarded transaction"
        ));
    }
    let mut validated = candidate.clone();
    validated
        .restore_model_resolution_records(expected)
        .map_err(|error| {
            format!(
                "{transaction} would invalidate an existing model provider decision: {error}. Clear or replace that decision first"
            )
        })
}

/// Publish an already validated model-manager candidate at the project
/// revision boundary and add one guarded global undo step.
pub(crate) fn publish_model_definition_candidate(
    state: &mut AppState,
    mut candidate: ModelLibraryManager,
    commit: ProjectModelCommit,
    description: impl Into<String>,
) -> Result<ObjectRevision, String> {
    if !state.project_lifecycle.project_open {
        return Err("Model definitions require an open project.".to_owned());
    }
    if state.workbench.safe_mode.project_read_only() {
        return Err("Model definitions cannot change while the project is read-only.".to_owned());
    }
    let observed = state
        .model_library_manager
        .get_library(&commit.library_name);
    if !model_library_semantics_match(observed, commit.before.as_ref()) {
        return Err(format!(
            "Model library '{}' changed before the candidate could be published.",
            commit.library_name
        ));
    }
    let candidate_library = candidate.get_library(&commit.library_name).ok_or_else(|| {
        format!(
            "Candidate model library '{}' is unavailable.",
            commit.library_name
        )
    })?;
    if !model_library_semantics_match(Some(candidate_library), Some(&commit.after)) {
        return Err("Candidate model manager does not contain the validated commit.".to_owned());
    }
    let before_set = state.model_library_manager.library_snapshot();
    let after_set = candidate.library_snapshot();
    let unrelated_before = before_set
        .iter()
        .filter(|library| library.name != commit.library_name)
        .cloned()
        .collect::<Vec<_>>();
    let unrelated_after = after_set
        .iter()
        .filter(|library| library.name != commit.library_name)
        .cloned()
        .collect::<Vec<_>>();
    if !model_library_snapshots_match(&unrelated_before, &unrelated_after) {
        return Err(format!(
            "Candidate model manager changes libraries outside '{}'",
            commit.library_name
        ));
    }
    require_unchanged_valid_model_resolution_ledger(
        state,
        &candidate,
        "The model-definition transaction",
    )?;
    if !matches!(
        commit.after.source_authority,
        ModelSourceAuthority::ProjectOwned { .. }
    ) {
        return Err("Only project-owned model definitions can be published.".to_owned());
    }
    if model_library_semantics_match(commit.before.as_ref(), Some(&commit.after)) {
        return Err("The model definition has no changes to publish.".to_owned());
    }

    let expected_revision = state
        .workspace
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    let committed_revision = state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    debug_assert_eq!(committed_revision, expected_revision);
    candidate.invalidate_model_validation_receipt();
    state.model_library_manager = candidate;
    state.workspace.project_metadata_dirty = true;
    if commit.affects_execution {
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        state.ui.netlist.current_generation_input_digest = None;
    }
    state.record_model_definition_transaction(ModelDefinitionRecord {
        description: description.into(),
        library: commit.library_name,
        model: commit.model_name,
        before: commit.before,
        after: Some(commit.after),
        undo_guard_revision: committed_revision,
        redo_guard_revision: None,
    });
    Ok(committed_revision)
}

/// Publish one validated model-library attach, detach, or refresh as a guarded
/// project transaction. Unlike a model-definition edit, the affected library
/// may be externally sourced; its authenticated retained bytes and authority
/// still participate in undo conflict checks and execution invalidation.
pub(crate) fn publish_model_library_candidate(
    state: &mut AppState,
    mut candidate: ModelLibraryManager,
    library_name: &str,
    description: impl Into<String>,
) -> Result<ObjectRevision, String> {
    if !state.project_lifecycle.project_open {
        return Err("Model libraries require an open project.".to_owned());
    }
    if state.workbench.safe_mode.project_read_only() {
        return Err("Model libraries cannot change while the project is read-only.".to_owned());
    }
    let before = state
        .model_library_manager
        .get_library(library_name)
        .cloned();
    let after = candidate.get_library(library_name).cloned();
    if model_library_semantics_match(before.as_ref(), after.as_ref()) {
        return Err("The model-library transaction has no changes to publish.".to_owned());
    }
    let before_set = state.model_library_manager.library_snapshot();
    let after_set = candidate.library_snapshot();
    let unrelated_before = before_set
        .iter()
        .filter(|library| library.name != library_name)
        .cloned()
        .collect::<Vec<_>>();
    let unrelated_after = after_set
        .iter()
        .filter(|library| library.name != library_name)
        .cloned()
        .collect::<Vec<_>>();
    if !model_library_snapshots_match(&unrelated_before, &unrelated_after) {
        return Err(format!(
            "Candidate model manager changes libraries outside '{library_name}'"
        ));
    }
    require_unchanged_valid_model_resolution_ledger(
        state,
        &candidate,
        "The model-library transaction",
    )?;
    let committed_revision = state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    candidate.invalidate_model_validation_receipt();
    state.model_library_manager = candidate;
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist.current_generation_input_digest = None;
    state.record_model_definition_transaction(ModelDefinitionRecord {
        description: description.into(),
        library: library_name.to_owned(),
        model: "*".to_owned(),
        before,
        after,
        undo_guard_revision: committed_revision,
        redo_guard_revision: None,
    });
    Ok(committed_revision)
}

/// Publish one validated multi-library replacement as a single project
/// revision and one guarded global Undo step.
pub(crate) fn publish_model_library_set_candidate(
    state: &mut AppState,
    mut candidate: ModelLibraryManager,
    description: impl Into<String>,
) -> Result<Option<ObjectRevision>, String> {
    if !state.project_lifecycle.project_open {
        return Err("Model libraries require an open project.".to_owned());
    }
    if state.workbench.safe_mode.project_read_only() {
        return Err("Model libraries cannot change while the project is read-only.".to_owned());
    }
    let before = state.model_library_manager.library_snapshot();
    let after = candidate.library_snapshot();
    require_unchanged_valid_model_resolution_ledger(
        state,
        &candidate,
        "The model-library-set transaction",
    )?;
    if model_library_snapshots_match(&before, &after) {
        return Ok(None);
    }
    let committed_revision = state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    candidate.invalidate_model_validation_receipt();
    state.model_library_manager = candidate;
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist.current_generation_input_digest = None;
    state.record_model_libraries_transaction(ModelLibrariesRecord {
        description: description.into(),
        before,
        after,
        undo_guard_revision: committed_revision,
        redo_guard_revision: None,
    });
    Ok(Some(committed_revision))
}

/// Publish source-qualified contested-definition decisions as one guarded
/// project transaction. The loaded libraries must remain byte-for-byte the
/// same; only the durable provider ledger may change.
pub(crate) fn publish_model_resolution_candidate(
    state: &mut AppState,
    mut candidate: ModelLibraryManager,
    description: impl Into<String>,
) -> Result<ObjectRevision, String> {
    if !state.project_lifecycle.project_open {
        return Err("Model provider decisions require an open project.".to_owned());
    }
    if state.workbench.safe_mode.project_read_only() {
        return Err(
            "Model provider decisions cannot change while the project is read-only.".to_owned(),
        );
    }
    let before_libraries = state.model_library_manager.library_snapshot();
    let after_libraries = candidate.library_snapshot();
    if !model_library_snapshots_match(&before_libraries, &after_libraries) {
        return Err("A provider-decision candidate cannot change model libraries.".to_owned());
    }
    let before = state.model_library_manager.owned_model_resolution_records();
    let after = candidate.owned_model_resolution_records();
    if before == after {
        return Err("The model provider decision has no changes to publish.".to_owned());
    }
    // Re-run record/catalog validation at the publication boundary.
    let mut validated = candidate.clone();
    validated.restore_model_resolution_records(after.clone())?;
    let committed_revision = state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    candidate.invalidate_model_validation_receipt();
    state.model_library_manager = candidate;
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist.current_generation_input_digest = None;
    state.record_model_resolution_transaction(ModelResolutionRecordsRecord {
        description: description.into(),
        before,
        after,
        undo_guard_revision: committed_revision,
        redo_guard_revision: None,
    });
    Ok(committed_revision)
}

/// Atomically publish a fully validated candidate library manager and record
/// the exact affected-cell transition in project Undo/Redo. Callers perform
/// all format, symbol, pin, form and netlist validation on `candidate` first;
/// this boundary owns authority/read-only checks and the monotonic project
/// revision.
pub(crate) fn publish_symbol_definition_candidate(
    state: &mut AppState,
    candidate: LibraryManager,
    library_name: &str,
    cell_name: &str,
    description: impl Into<String>,
) -> Result<ObjectRevision, String> {
    publish_symbol_definition_candidate_with_fixture(
        state,
        candidate,
        library_name,
        cell_name,
        description,
        None,
    )
}

pub(crate) fn publish_symbol_definition_candidate_with_fixture(
    state: &mut AppState,
    candidate: LibraryManager,
    library_name: &str,
    cell_name: &str,
    description: impl Into<String>,
    fixture: Option<SymbolDefinitionFixtureDelta>,
) -> Result<ObjectRevision, String> {
    if !state.project_lifecycle.project_open {
        return Err("Symbol definitions require an open project.".to_owned());
    }
    if state.workbench.safe_mode.project_read_only() {
        return Err("Symbol definitions cannot change while the project is read-only.".to_owned());
    }
    let source_library = state
        .library_manager
        .get_library(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?;
    if source_library.read_only {
        return Err(format!("Library '{library_name}' is read-only."));
    }
    let candidate_library = candidate
        .get_library(library_name)
        .ok_or_else(|| format!("Candidate library '{library_name}' is unavailable."))?;
    if candidate_library.read_only {
        return Err(format!("Candidate library '{library_name}' is read-only."));
    }

    let before = source_library.get_cell(cell_name).cloned();
    let after = candidate_library.get_cell(cell_name).cloned();
    if after.is_none() {
        return Err(format!(
            "Candidate symbol cell '{library_name}/{cell_name}' is unavailable."
        ));
    }
    if cell_semantics_match(before.as_ref(), after.as_ref())
        && fixture.as_ref().is_none_or(|fixture| {
            schematic_option_matches(fixture.before.as_ref(), fixture.after.as_ref())
        })
    {
        return Err("The symbol definition has no changes to publish.".to_owned());
    }
    if let Some(fixture) = fixture.as_ref() {
        if !fixture.reference.library.eq_ignore_ascii_case(library_name)
            || !fixture.reference.cell.eq_ignore_ascii_case(cell_name)
        {
            return Err(
                "The generated fixture must belong to the published symbol cell.".to_owned(),
            );
        }
        let observed = schematic_for_reference(state, &fixture.reference);
        if !schematic_option_matches(observed, fixture.before.as_ref()) {
            return Err(
                "The generated fixture target changed before symbol publication.".to_owned(),
            );
        }
    }

    // `next_revision` proves the only fallible mutation before either live
    // owner changes. `advance_revision` cannot then fail in this transaction.
    let expected_revision = state
        .workspace
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    let committed_revision = state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    debug_assert_eq!(committed_revision, expected_revision);
    state.library_manager = candidate;
    if let Some(fixture) = fixture.as_ref() {
        apply_symbol_fixture(state, &fixture.reference, fixture.after.as_ref());
    }
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.record_symbol_definition_transaction(SymbolDefinitionHistoryEntry {
        description: description.into(),
        library: library_name.to_owned(),
        cell: cell_name.to_owned(),
        before,
        after,
        committed_revision,
        fixture: fixture.map(|fixture| SymbolDefinitionFixtureRecord {
            reference: fixture.reference,
            before: fixture.before,
            after: fixture.after,
        }),
    });
    Ok(committed_revision)
}

pub(crate) struct DesignManagementHistoryEntry {
    pub(crate) description: String,
    pub(crate) owner: CellViewRef,
    pub(crate) before: DesignManagementCatalog,
    pub(crate) after: DesignManagementCatalog,
    pub(crate) before_schematics: BTreeMap<String, SchematicState>,
    pub(crate) after_schematics: BTreeMap<String, SchematicState>,
    pub(crate) committed_revision: ObjectRevision,
}

pub(crate) struct DesignManagementSchematicTransaction {
    pub(crate) before: BTreeMap<String, SchematicState>,
    pub(crate) after: BTreeMap<String, SchematicState>,
}

pub(crate) struct HierarchyExtractionHistoryEntry {
    pub(crate) parent_ref: CellViewRef,
    pub(crate) target_schematic_ref: CellViewRef,
    pub(crate) target_open_ref: CellViewRef,
    pub(crate) before_parent: SchematicState,
    pub(crate) after_parent: SchematicState,
    pub(crate) child: SchematicState,
    pub(crate) target_cell: Cell,
    pub(crate) open_views_before: Vec<OpenCellView>,
    pub(crate) hierarchy_stack_before: Vec<CellViewRef>,
    pub(crate) hierarchy_instances_before: Vec<String>,
    pub(crate) open_views_after: Vec<OpenCellView>,
    pub(crate) hierarchy_stack_after: Vec<CellViewRef>,
    pub(crate) hierarchy_instances_after: Vec<String>,
}

impl AppState {
    /// Atomically publish a fully validated project-owned model candidate and
    /// record the change in the project-wide undo/redo history.
    pub fn publish_project_model_candidate(
        &mut self,
        candidate: ModelLibraryManager,
        commit: ProjectModelCommit,
        description: impl Into<String>,
    ) -> Result<ObjectRevision, String> {
        publish_model_definition_candidate(self, candidate, commit, description)
    }

    /// Preflight the component-name changes represented by the candidate's
    /// effective annotation journal. Every scoped object must still exist and
    /// retain either the journal's old or already-applied reference; a
    /// conflicting external edit blocks the whole project transaction.
    pub(crate) fn prepare_design_management_schematic_transaction(
        &self,
        candidate: &DesignManagementCatalog,
    ) -> Result<DesignManagementSchematicTransaction, String> {
        let active_key = self.workspace.active_schematic_reference().key();
        let mut projected = self.workspace.schematic_buffers.clone();
        projected.insert(active_key, self.schematic.clone());
        let mut before = BTreeMap::new();

        let existing_mappings = self
            .workspace
            .design_management
            .annotation()
            .effective_mappings();
        for (object, mapping) in candidate.annotation().effective_mappings() {
            if existing_mappings.get(&object) == Some(&mapping) {
                continue;
            }
            let key = object.cell_view_key();
            let existing_key = projected
                .keys()
                .find(|candidate| candidate.eq_ignore_ascii_case(key))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "Annotation cannot be published because schematic '{}' is unavailable.",
                        key
                    )
                })?;
            let schematic = projected
                .get_mut(&existing_key)
                .expect("the resolved schematic key remains present");
            let component_index = schematic
                .components
                .iter()
                .position(|component| component.id == object.object_id())
                .ok_or_else(|| {
                    format!(
                        "Annotation cannot be published because object {} no longer exists in '{}'.",
                        object.object_id(),
                        key
                    )
                })?;
            let current_reference = schematic.components[component_index].name.clone();
            if current_reference == mapping.new_reference {
                continue;
            }
            if current_reference != mapping.old_reference {
                return Err(format!(
                    "Annotation cannot be published because {} in '{}' changed from '{}' to '{}'.",
                    current_reference, key, mapping.old_reference, mapping.new_reference
                ));
            }
            before
                .entry(existing_key.clone())
                .or_insert_with(|| schematic.clone());
            schematic.components[component_index]
                .name
                .clone_from(&mapping.new_reference);
            schematic.is_dirty = true;
        }

        let after = before
            .keys()
            .map(|key| {
                (
                    key.clone(),
                    projected
                        .get(key)
                        .expect("changed schematic remains projected")
                        .clone(),
                )
            })
            .collect();
        Ok(DesignManagementSchematicTransaction { before, after })
    }

    pub(crate) fn apply_design_management_schematic_transaction(
        &mut self,
        transaction: &DesignManagementSchematicTransaction,
    ) {
        let active_key = self.workspace.active_schematic_reference().key();
        for (key, schematic) in &transaction.after {
            if key.eq_ignore_ascii_case(&active_key) {
                self.schematic = schematic.clone();
                self.workspace
                    .schematic_buffers
                    .insert(active_key.clone(), schematic.clone());
            } else if let Some(existing_key) = self
                .workspace
                .schematic_buffers
                .keys()
                .find(|candidate| candidate.eq_ignore_ascii_case(key))
                .cloned()
            {
                self.workspace
                    .schematic_buffers
                    .insert(existing_key, schematic.clone());
            }
            if let Some(open) = self
                .workspace
                .open_views
                .iter_mut()
                .find(|open| open.reference.key().eq_ignore_ascii_case(key))
            {
                open.dirty = true;
            }
        }
    }

    pub(crate) fn clear_project_design_history(&mut self) {
        self.project_design_history = ProjectDesignHistory::default();
    }

    pub(crate) fn record_hierarchy_extraction(&mut self, entry: HierarchyExtractionHistoryEntry) {
        self.project_design_history
            .undo
            .push(ProjectDesignRecord::HierarchyExtraction(Box::new(
                HierarchyExtractionRecord {
                    description: "create hierarchical cell".to_owned(),
                    parent_ref: entry.parent_ref,
                    target_schematic_ref: entry.target_schematic_ref,
                    target_open_ref: entry.target_open_ref,
                    before_parent: SchematicSnapshot::capture(&entry.before_parent),
                    after_parent: SchematicSnapshot::capture(&entry.after_parent),
                    child: SchematicSnapshot::capture(&entry.child),
                    child_template: entry.child,
                    target_cell: entry.target_cell,
                    open_views_before: entry.open_views_before,
                    hierarchy_stack_before: entry.hierarchy_stack_before,
                    hierarchy_instances_before: entry.hierarchy_instances_before,
                    open_views_after: entry.open_views_after,
                    hierarchy_stack_after: entry.hierarchy_stack_after,
                    hierarchy_instances_after: entry.hierarchy_instances_after,
                },
            )));
        if self.project_design_history.undo.len() > MAX_PROJECT_DESIGN_STEPS {
            self.project_design_history.undo.remove(0);
        }
        self.project_design_history.redo.clear();
    }

    pub(crate) fn record_design_management_transaction(
        &mut self,
        entry: DesignManagementHistoryEntry,
    ) {
        if entry.before == entry.after {
            return;
        }
        self.project_design_history
            .undo
            .push(ProjectDesignRecord::DesignManagement(Box::new(
                DesignManagementRecord {
                    description: entry.description,
                    owner: entry.owner,
                    before: entry.before,
                    after: entry.after,
                    before_schematics: capture_schematic_map(entry.before_schematics),
                    after_schematics: capture_schematic_map(entry.after_schematics),
                    undo_guard_revision: entry.committed_revision,
                    redo_guard_revision: None,
                },
            )));
        if self.project_design_history.undo.len() > MAX_PROJECT_DESIGN_STEPS {
            self.project_design_history.undo.remove(0);
        }
        self.project_design_history.redo.clear();
    }

    pub(crate) fn record_symbol_definition_transaction(
        &mut self,
        entry: SymbolDefinitionHistoryEntry,
    ) {
        if cell_semantics_match(entry.before.as_ref(), entry.after.as_ref())
            && entry.fixture.as_ref().is_none_or(|fixture| {
                schematic_option_matches(fixture.before.as_ref(), fixture.after.as_ref())
            })
        {
            return;
        }
        self.project_design_history
            .undo
            .push(ProjectDesignRecord::SymbolDefinition(Box::new(
                SymbolDefinitionRecord {
                    description: entry.description,
                    library: entry.library,
                    cell: entry.cell,
                    before: entry.before,
                    after: entry.after,
                    undo_guard_revision: entry.committed_revision,
                    redo_guard_revision: None,
                    fixture: entry.fixture,
                },
            )));
        if self.project_design_history.undo.len() > MAX_PROJECT_DESIGN_STEPS {
            self.project_design_history.undo.remove(0);
        }
        self.project_design_history.redo.clear();
    }

    fn record_model_definition_transaction(&mut self, record: ModelDefinitionRecord) {
        if model_library_semantics_match(record.before.as_ref(), record.after.as_ref()) {
            return;
        }
        self.project_design_history
            .undo
            .push(ProjectDesignRecord::ModelDefinition(Box::new(record)));
        if self.project_design_history.undo.len() > MAX_PROJECT_DESIGN_STEPS {
            self.project_design_history.undo.remove(0);
        }
        self.project_design_history.redo.clear();
    }

    fn record_model_libraries_transaction(&mut self, record: ModelLibrariesRecord) {
        if model_library_snapshots_match(&record.before, &record.after) {
            return;
        }
        self.project_design_history
            .undo
            .push(ProjectDesignRecord::ModelLibraries(Box::new(record)));
        if self.project_design_history.undo.len() > MAX_PROJECT_DESIGN_STEPS {
            self.project_design_history.undo.remove(0);
        }
        self.project_design_history.redo.clear();
    }

    fn record_model_resolution_transaction(&mut self, record: ModelResolutionRecordsRecord) {
        if record.before == record.after {
            return;
        }
        self.project_design_history
            .undo
            .push(ProjectDesignRecord::ModelResolutions(Box::new(record)));
        if self.project_design_history.undo.len() > MAX_PROJECT_DESIGN_STEPS {
            self.project_design_history.undo.remove(0);
        }
        self.project_design_history.redo.clear();
    }

    pub(crate) fn can_undo_project_design(&self) -> bool {
        self.project_design_history
            .undo
            .last()
            .is_some_and(|record| {
                record.after_design_matches(self)
                    && record.validate_mutation(self, "undone").is_ok()
            })
    }

    pub(crate) fn can_redo_project_design(&self) -> bool {
        self.project_design_history
            .redo
            .last()
            .is_some_and(|record| {
                record.before_design_matches(self)
                    && record.validate_mutation(self, "redone").is_ok()
            })
    }

    pub(crate) fn project_undo_owns_active_document(&self) -> bool {
        self.project_design_history
            .undo
            .last()
            .is_some_and(|record| record.owns_active_document(self))
    }

    pub(crate) fn project_redo_owns_active_document(&self) -> bool {
        self.project_design_history
            .redo
            .last()
            .is_some_and(|record| record.owns_active_document(self))
    }

    pub(crate) fn undo_project_design(&mut self) -> Result<Option<String>, String> {
        let Some(record) = self.project_design_history.undo.last() else {
            return Ok(None);
        };
        if !record.after_design_matches(self) {
            return Ok(None);
        }
        record.validate_mutation(self, "undone")?;
        let mut record = self
            .project_design_history
            .undo
            .pop()
            .expect("the guarded project transaction remains present");
        record.apply_before(self)?;
        let description = record.description().to_owned();
        self.project_design_history.redo.push(record);
        Ok(Some(description))
    }

    pub(crate) fn redo_project_design(&mut self) -> Result<Option<String>, String> {
        let Some(record) = self.project_design_history.redo.last() else {
            return Ok(None);
        };
        if !record.before_design_matches(self) {
            return Ok(None);
        }
        record.validate_mutation(self, "redone")?;
        let record = self
            .project_design_history
            .redo
            .pop()
            .expect("the guarded project transaction remains present");
        let mut record = record;
        record.apply_after(self)?;
        let description = record.description().to_owned();
        self.project_design_history.undo.push(record);
        Ok(Some(description))
    }
}

impl ProjectDesignRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        match self {
            Self::HierarchyExtraction(record) => record.after_design_matches(state),
            Self::DesignManagement(record) => record.after_design_matches(state),
            Self::SymbolDefinition(record) => record.after_design_matches(state),
            Self::ModelDefinition(record) => record.after_design_matches(state),
            Self::ModelLibraries(record) => record.after_design_matches(state),
            Self::ModelResolutions(record) => record.after_design_matches(state),
        }
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        match self {
            Self::HierarchyExtraction(record) => record.before_design_matches(state),
            Self::DesignManagement(record) => record.before_design_matches(state),
            Self::SymbolDefinition(record) => record.before_design_matches(state),
            Self::ModelDefinition(record) => record.before_design_matches(state),
            Self::ModelLibraries(record) => record.before_design_matches(state),
            Self::ModelResolutions(record) => record.before_design_matches(state),
        }
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        match self {
            Self::HierarchyExtraction(record) => record.validate_mutation(state, operation),
            Self::DesignManagement(record) => record.validate_mutation(state, operation),
            Self::SymbolDefinition(record) => record.validate_mutation(state, operation),
            Self::ModelDefinition(record) => record.validate_mutation(state, operation),
            Self::ModelLibraries(record) => record.validate_mutation(state, operation),
            Self::ModelResolutions(record) => record.validate_mutation(state, operation),
        }
    }

    fn owns_active_document(&self, state: &AppState) -> bool {
        let active = state.workspace.active_schematic_reference();
        match self {
            Self::HierarchyExtraction(record) => {
                active == record.parent_ref || active == record.target_schematic_ref
            }
            Self::DesignManagement(record) => active == record.owner,
            Self::SymbolDefinition(record) => {
                active.library.eq_ignore_ascii_case(&record.library)
                    && active.cell.eq_ignore_ascii_case(&record.cell)
            }
            Self::ModelDefinition(_) => false,
            Self::ModelLibraries(_) => false,
            Self::ModelResolutions(_) => false,
        }
    }

    fn description(&self) -> &str {
        match self {
            Self::HierarchyExtraction(record) => &record.description,
            Self::DesignManagement(record) => &record.description,
            Self::SymbolDefinition(record) => &record.description,
            Self::ModelDefinition(record) => &record.description,
            Self::ModelLibraries(record) => &record.description,
            Self::ModelResolutions(record) => &record.description,
        }
    }

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        match self {
            Self::HierarchyExtraction(record) => record.apply_before(state),
            Self::DesignManagement(record) => record.apply_before(state),
            Self::SymbolDefinition(record) => record.apply_before(state),
            Self::ModelDefinition(record) => record.apply_before(state),
            Self::ModelLibraries(record) => record.apply_before(state),
            Self::ModelResolutions(record) => record.apply_before(state),
        }
    }

    fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        match self {
            Self::HierarchyExtraction(record) => record.apply_after(state),
            Self::DesignManagement(record) => record.apply_after(state),
            Self::SymbolDefinition(record) => record.apply_after(state),
            Self::ModelDefinition(record) => record.apply_after(state),
            Self::ModelLibraries(record) => record.apply_after(state),
            Self::ModelResolutions(record) => record.apply_after(state),
        }
    }
}

impl ModelLibrariesRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        model_library_snapshots_match(&state.model_library_manager.library_snapshot(), &self.after)
            && state.workspace.project.revision() == self.undo_guard_revision
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        model_library_snapshots_match(
            &state.model_library_manager.library_snapshot(),
            &self.before,
        ) && self
            .redo_guard_revision
            .is_some_and(|revision| state.workspace.project.revision() == revision)
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        if !state.project_lifecycle.project_open {
            return Err(format!(
                "Model libraries cannot be {operation} without an open project."
            ));
        }
        if state.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "Model libraries cannot be {operation} while the project is read-only."
            ));
        }
        Ok(())
    }

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "Model libraries cannot be undone because their sources or project revision changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        self.redo_guard_revision = Some(replace_model_library_snapshot(state, &self.before)?);
        Ok(())
    }

    fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Model libraries cannot be redone because their sources or project revision changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        self.undo_guard_revision = replace_model_library_snapshot(state, &self.after)?;
        Ok(())
    }
}

impl ModelResolutionRecordsRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        state.model_library_manager.owned_model_resolution_records() == self.after
            && state.workspace.project.revision() == self.undo_guard_revision
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        state.model_library_manager.owned_model_resolution_records() == self.before
            && self
                .redo_guard_revision
                .is_some_and(|revision| state.workspace.project.revision() == revision)
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        if !state.project_lifecycle.project_open {
            return Err(format!(
                "Model provider decisions cannot be {operation} without an open project."
            ));
        }
        if state.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "Model provider decisions cannot be {operation} while the project is read-only."
            ));
        }
        Ok(())
    }

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "Model provider decisions cannot be undone because their catalog or project revision changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        self.redo_guard_revision = Some(replace_model_resolution_records(state, &self.before)?);
        Ok(())
    }

    fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Model provider decisions cannot be redone because their catalog or project revision changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        self.undo_guard_revision = replace_model_resolution_records(state, &self.after)?;
        Ok(())
    }
}

impl ModelDefinitionRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        model_library_semantics_match(
            state.model_library_manager.get_library(&self.library),
            self.after.as_ref(),
        ) && state.workspace.project.revision() == self.undo_guard_revision
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        model_library_semantics_match(
            state.model_library_manager.get_library(&self.library),
            self.before.as_ref(),
        ) && self
            .redo_guard_revision
            .is_some_and(|revision| state.workspace.project.revision() == revision)
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        if !state.project_lifecycle.project_open {
            return Err(format!(
                "Model definition cannot be {operation} without an open project."
            ));
        }
        if state.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "Model definition cannot be {operation} while the project is read-only."
            ));
        }
        Ok(())
    }

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(format!(
                "Model '{}' cannot be undone because its source or project revision changed.",
                self.model
            ));
        }
        self.validate_mutation(state, "undone")?;
        self.redo_guard_revision = Some(replace_model_library(
            state,
            &self.library,
            self.before.as_ref(),
        )?);
        Ok(())
    }

    fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(format!(
                "Model '{}' cannot be redone because its source or project revision changed.",
                self.model
            ));
        }
        self.validate_mutation(state, "redone")?;
        self.undo_guard_revision =
            replace_model_library(state, &self.library, self.after.as_ref())?;
        Ok(())
    }
}

fn model_library_semantics_match(
    observed: Option<&ModelLibrary>,
    expected: Option<&ModelLibrary>,
) -> bool {
    match (observed, expected) {
        (None, None) => true,
        (Some(observed), Some(expected)) => {
            observed.name == expected.name
                && observed.pdk_name == expected.pdk_name
                && observed.technology_node == expected.technology_node
                && observed.root_path == expected.root_path
                && observed.source_authority == expected.source_authority
                && observed.source_closure == expected.source_closure
                && observed.source_contents == expected.source_contents
                && observed.source_edges == expected.source_edges
                && observed.models == expected.models
                && observed.model_definition_metadata == expected.model_definition_metadata
                && observed.model_qualification == expected.model_qualification
                && observed.model_correlation == expected.model_correlation
                && observed.corners == expected.corners
                && observed.selected_corner == expected.selected_corner
                && observed.version == expected.version
        }
        _ => false,
    }
}

fn model_library_snapshots_match(observed: &[ModelLibrary], expected: &[ModelLibrary]) -> bool {
    observed.len() == expected.len()
        && observed.iter().zip(expected).all(|(observed, expected)| {
            observed.name == expected.name
                && model_library_semantics_match(Some(observed), Some(expected))
        })
}

fn replace_model_library_snapshot(
    state: &mut AppState,
    replacement: &[ModelLibrary],
) -> Result<ObjectRevision, String> {
    let revision = state
        .workspace
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    state
        .model_library_manager
        .replace_library_snapshot(replacement.to_vec())?;
    state
        .model_library_manager
        .invalidate_model_validation_receipt();
    state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    debug_assert_eq!(state.workspace.project.revision(), revision);
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist.current_generation_input_digest = None;
    Ok(revision)
}

fn replace_model_resolution_records(
    state: &mut AppState,
    replacement: &[crate::state::model_library::ModelResolutionRecord],
) -> Result<ObjectRevision, String> {
    let revision = state
        .workspace
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    state
        .model_library_manager
        .restore_model_resolution_records(replacement.to_vec())?;
    state
        .model_library_manager
        .invalidate_model_validation_receipt();
    state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    debug_assert_eq!(state.workspace.project.revision(), revision);
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist.current_generation_input_digest = None;
    Ok(revision)
}

fn replace_model_library(
    state: &mut AppState,
    library_name: &str,
    replacement: Option<&ModelLibrary>,
) -> Result<ObjectRevision, String> {
    let revision = state
        .workspace
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    match replacement {
        Some(replacement) => {
            let mut replacement = replacement.clone();
            if let Some(current) = state.model_library_manager.get_library(library_name) {
                replacement.expanded = current.expanded;
            }
            state.model_library_manager.add_library(replacement);
            if state.model_library_manager.selected_library.as_deref() == Some(library_name)
                && state
                    .workbench
                    .selected_model
                    .as_ref()
                    .is_some_and(|model| {
                        state
                            .model_library_manager
                            .get_library(library_name)
                            .is_none_or(|library| !library.models.contains_key(model))
                    })
            {
                state.workbench.selected_model = None;
            }
        }
        None => {
            if state
                .model_library_manager
                .remove_library(library_name)
                .is_none()
            {
                return Err(format!("Model library '{library_name}' no longer exists."));
            }
            if state.model_library_manager.selected_library.as_deref() == Some(library_name) {
                state.model_library_manager.selected_library = None;
                state.workbench.selected_model = None;
            }
        }
    }
    state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    debug_assert_eq!(state.workspace.project.revision(), revision);
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist.current_generation_input_digest = None;
    Ok(revision)
}

impl SymbolDefinitionRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        cell_semantics_match(
            symbol_cell(state, &self.library, &self.cell),
            self.after.as_ref(),
        ) && self.fixture.as_ref().is_none_or(|fixture| {
            schematic_option_matches(
                schematic_for_reference(state, &fixture.reference),
                fixture.after.as_ref(),
            )
        }) && state.workspace.project.revision() == self.undo_guard_revision
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        cell_semantics_match(
            symbol_cell(state, &self.library, &self.cell),
            self.before.as_ref(),
        ) && self.fixture.as_ref().is_none_or(|fixture| {
            schematic_option_matches(
                schematic_for_reference(state, &fixture.reference),
                fixture.before.as_ref(),
            )
        }) && self
            .redo_guard_revision
            .is_some_and(|revision| state.workspace.project.revision() == revision)
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        if !state.project_lifecycle.project_open {
            return Err(format!(
                "Symbol definition cannot be {operation} without an open project."
            ));
        }
        if state.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "Symbol definition cannot be {operation} while the project is read-only."
            ));
        }
        let library = state
            .library_manager
            .get_library(&self.library)
            .ok_or_else(|| format!("Library '{}' no longer exists.", self.library))?;
        if library.read_only {
            return Err(format!(
                "Symbol definition cannot be {operation} because library '{}' is read-only.",
                self.library
            ));
        }
        if self.before.is_none()
            && state.workspace.open_views.iter().any(|open| {
                open.reference.library.eq_ignore_ascii_case(&self.library)
                    && open.reference.cell.eq_ignore_ascii_case(&self.cell)
            })
        {
            return Err(format!(
                "Imported symbol cannot be {operation} while a view from '{}/{}' is open.",
                self.library, self.cell
            ));
        }
        let fixture_removed = self
            .fixture
            .as_ref()
            .is_some_and(|fixture| match operation {
                "undone" => fixture.before.is_none(),
                "redone" => fixture.after.is_none(),
                _ => false,
            });
        if fixture_removed
            && self.fixture.as_ref().is_some_and(|fixture| {
                state.workspace.active_schematic_reference() == fixture.reference
                    || state
                        .workspace
                        .open_views
                        .iter()
                        .any(|open| open.reference == fixture.reference)
            })
        {
            let fixture = self.fixture.as_ref().expect("fixture removal was checked");
            return Err(format!(
                "Generated fixture '{}' cannot be {operation} while it is open.",
                fixture.reference.display_path()
            ));
        }
        Ok(())
    }

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "Symbol definition cannot be undone because the target cell or project revision changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        let revision = replace_symbol_cell(state, &self.library, &self.cell, self.before.as_ref())?;
        if let Some(fixture) = &self.fixture {
            apply_symbol_fixture(state, &fixture.reference, fixture.before.as_ref());
        }
        self.redo_guard_revision = Some(revision);
        Ok(())
    }

    fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Symbol definition cannot be redone because the target cell or project revision changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        self.undo_guard_revision =
            replace_symbol_cell(state, &self.library, &self.cell, self.after.as_ref())?;
        if let Some(fixture) = &self.fixture {
            apply_symbol_fixture(state, &fixture.reference, fixture.after.as_ref());
        }
        Ok(())
    }
}

fn symbol_cell<'a>(state: &'a AppState, library: &str, cell: &str) -> Option<&'a Cell> {
    state
        .library_manager
        .get_library(library)
        .and_then(|library| library.get_cell(cell))
}

fn cell_semantics_match(left: Option<&Cell>, right: Option<&Cell>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            matches!(
                (serde_json::to_vec(left), serde_json::to_vec(right)),
                (Ok(left), Ok(right)) if left == right
            )
        }
        _ => false,
    }
}

fn schematic_for_reference<'a>(
    state: &'a AppState,
    reference: &CellViewRef,
) -> Option<&'a SchematicState> {
    if state.workspace.active_schematic_reference() == *reference {
        Some(&state.schematic)
    } else {
        state.workspace.schematic_buffers.get(&reference.key())
    }
}

fn schematic_option_matches(
    observed: Option<&SchematicState>,
    expected: Option<&SchematicState>,
) -> bool {
    match (observed, expected) {
        (None, None) => true,
        (Some(observed), Some(expected)) => {
            SchematicSnapshot::capture(expected).is_equal_state(observed)
        }
        _ => false,
    }
}

fn apply_symbol_fixture(
    state: &mut AppState,
    reference: &CellViewRef,
    replacement: Option<&SchematicState>,
) {
    let key = reference.key();
    match replacement {
        Some(schematic) => {
            state
                .workspace
                .schematic_buffers
                .insert(key.clone(), schematic.clone());
            if state.workspace.active_schematic_reference() == *reference {
                state.schematic = schematic.clone();
            }
            if let Some(open) = state
                .workspace
                .open_views
                .iter_mut()
                .find(|open| open.reference == *reference)
            {
                open.dirty = schematic.is_dirty;
            }
        }
        None => {
            state.workspace.schematic_buffers.remove(&key);
        }
    }
}

fn replace_symbol_cell(
    state: &mut AppState,
    library_name: &str,
    cell_name: &str,
    replacement: Option<&Cell>,
) -> Result<ObjectRevision, String> {
    let revision = state
        .workspace
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    let library = state
        .library_manager
        .get_library_mut(library_name)
        .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?;
    if library.read_only {
        return Err(format!("Library '{library_name}' is read-only."));
    }
    match replacement {
        Some(cell) => {
            library.add_cell(cell.clone());
        }
        None => {
            if !library.remove_cell(cell_name) {
                return Err(format!(
                    "Cell '{library_name}/{cell_name}' no longer exists."
                ));
            }
            if state.library_manager.selected_library.as_deref() == Some(library_name)
                && state.library_manager.selected_cell.as_deref() == Some(cell_name)
            {
                state.library_manager.selected_cell = None;
                state.library_manager.selected_view = None;
            }
        }
    }
    state
        .workspace
        .project
        .advance_revision()
        .map_err(|error| error.to_string())?;
    debug_assert_eq!(state.workspace.project.revision(), revision);
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    Ok(revision)
}

impl DesignManagementRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        design_management_semantics_match(&state.workspace.design_management, &self.after)
            && schematic_map_matches(state, &self.after_schematics)
            && state.workspace.project.revision() == self.undo_guard_revision
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        design_management_semantics_match(&state.workspace.design_management, &self.before)
            && schematic_map_matches(state, &self.before_schematics)
            && self
                .redo_guard_revision
                .is_some_and(|revision| state.workspace.project.revision() == revision)
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        if !state.project_lifecycle.project_open {
            return Err(format!(
                "Design management cannot be {operation} without an open project."
            ));
        }
        if state.workbench.safe_mode.project_read_only()
            || state.schematic.read_only
            || state.active_view_read_only()
        {
            return Err(format!(
                "Design management cannot be {operation} while the active design is read-only."
            ));
        }
        if state.workspace.active_schematic_reference() != self.owner {
            return Err(format!(
                "Design management cannot be {operation} because '{}' is no longer the active schematic.",
                self.owner.display_path()
            ));
        }
        Ok(())
    }

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "Design management cannot be undone because project configuration changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        let revision = state
            .workspace
            .replace_design_management(self.before.clone())
            .map_err(|error| error.to_string())?;
        apply_schematic_map(state, &self.before_schematics)?;
        self.redo_guard_revision = Some(revision);
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        Ok(())
    }

    fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Design management cannot be redone because project configuration changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        let revision = state
            .workspace
            .replace_design_management(self.after.clone())
            .map_err(|error| error.to_string())?;
        apply_schematic_map(state, &self.after_schematics)?;
        self.undo_guard_revision = revision;
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        Ok(())
    }
}

fn capture_schematic_map(
    schematics: BTreeMap<String, SchematicState>,
) -> BTreeMap<String, SchematicSnapshot> {
    schematics
        .into_iter()
        .map(|(key, schematic)| (key, SchematicSnapshot::capture(&schematic)))
        .collect()
}

fn schematic_map_matches(state: &AppState, expected: &BTreeMap<String, SchematicSnapshot>) -> bool {
    let active_key = state.workspace.active_schematic_reference().key();
    expected.iter().all(|(key, snapshot)| {
        if key.eq_ignore_ascii_case(&active_key) {
            snapshot.is_equal_state(&state.schematic)
        } else {
            state
                .workspace
                .schematic_buffers
                .iter()
                .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
                .is_some_and(|(_, schematic)| snapshot.is_equal_state(schematic))
        }
    })
}

fn apply_schematic_map(
    state: &mut AppState,
    snapshots: &BTreeMap<String, SchematicSnapshot>,
) -> Result<(), String> {
    let active_key = state.workspace.active_schematic_reference().key();
    for (key, snapshot) in snapshots {
        if key.eq_ignore_ascii_case(&active_key) {
            snapshot.apply(&mut state.schematic);
            state
                .workspace
                .schematic_buffers
                .insert(active_key.clone(), state.schematic.clone());
            continue;
        }
        let Some(existing_key) = state
            .workspace
            .schematic_buffers
            .keys()
            .find(|candidate| candidate.eq_ignore_ascii_case(key))
            .cloned()
        else {
            return Err(format!(
                "Design management cannot restore schematic '{key}' because it is no longer open."
            ));
        };
        let schematic = state
            .workspace
            .schematic_buffers
            .get_mut(&existing_key)
            .expect("the retained schematic key remains present");
        snapshot.apply(schematic);
    }
    Ok(())
}

fn design_management_semantics_match(
    left: &DesignManagementCatalog,
    right: &DesignManagementCatalog,
) -> bool {
    matches!(
        (left.semantic_digest(), right.semantic_digest()),
        (Ok(left), Ok(right)) if left == right
    )
}

/// Creating a previously absent master can silently resolve an authored
/// dangling instance in any open schematic. That is outside the extraction
/// transaction, so fail before publishing the new cell.
pub(crate) fn validate_hierarchy_target_unreferenced(
    state: &AppState,
    target: &CellViewRef,
) -> Result<(), String> {
    let references_target = |schematic: &SchematicState| {
        schematic_references_master(schematic, &target.library, &target.cell)
    };
    if references_target(&state.schematic)
        || state
            .workspace
            .schematic_buffers
            .values()
            .any(references_target)
    {
        return Err(format!(
            "Cell '{}/{}' is already referenced by an open schematic and cannot be created implicitly by hierarchy extraction.",
            target.library, target.cell
        ));
    }
    Ok(())
}

impl HierarchyExtractionRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        state.workspace.active_view == self.target_open_ref
            && schematic_matches(state, &self.parent_ref, &self.after_parent)
            && schematic_matches(state, &self.target_schematic_ref, &self.child)
            && target_cell_matches(state, &self.target_schematic_ref, &self.target_cell)
            && navigation_matches(
                state,
                &self.open_views_after,
                &self.hierarchy_stack_after,
                &self.hierarchy_instances_after,
            )
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        state.workspace.active_view == self.parent_ref
            && schematic_matches(state, &self.parent_ref, &self.before_parent)
            && state
                .library_manager
                .get_library(&self.target_schematic_ref.library)
                .and_then(|library| library.get_cell(&self.target_schematic_ref.cell))
                .is_none()
            && !state
                .workspace
                .schematic_buffers
                .contains_key(&self.target_schematic_ref.key())
            && navigation_matches(
                state,
                &self.open_views_before,
                &self.hierarchy_stack_before,
                &self.hierarchy_instances_before,
            )
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        if state.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "Create hierarchy cannot be {operation} while the project is open read-only."
            ));
        }
        for library_name in [&self.parent_ref.library, &self.target_schematic_ref.library] {
            let library = state
                .library_manager
                .get_library(library_name)
                .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?;
            if library.read_only {
                return Err(format!(
                    "Create hierarchy cannot be {operation} because library '{library_name}' is read-only."
                ));
            }
        }
        if schematic_read_only(state, &self.parent_ref)
            || (self.after_design_matches(state)
                && schematic_read_only(state, &self.target_schematic_ref))
        {
            return Err(format!(
                "Create hierarchy cannot be {operation} because an affected schematic is read-only."
            ));
        }
        if has_external_master_reference(state, self) {
            return Err(format!(
                "Create hierarchy cannot be {operation} because another schematic now references {}/{}.",
                self.target_schematic_ref.library, self.target_schematic_ref.cell
            ));
        }
        Ok(())
    }

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err("Create hierarchy cannot be undone because its parent, child, or generated symbol changed.".to_owned());
        }
        self.validate_mutation(state, "undone")?;
        self.child_template = schematic_clone(state, &self.target_schematic_ref)
            .ok_or_else(|| "The generated child schematic is no longer open.".to_owned())?;
        self.target_cell = state
            .library_manager
            .get_library(&self.target_schematic_ref.library)
            .and_then(|library| library.get_cell(&self.target_schematic_ref.cell))
            .cloned()
            .ok_or_else(|| "The generated hierarchy cell no longer exists.".to_owned())?;
        let current_open_views = state.workspace.open_views.clone();
        let library = state
            .library_manager
            .get_library_mut(&self.target_schematic_ref.library)
            .ok_or_else(|| "The target library no longer exists.".to_owned())?;
        if !library.remove_cell(&self.target_schematic_ref.cell) {
            return Err("The generated hierarchy cell no longer exists.".to_owned());
        }
        state
            .workspace
            .schematic_buffers
            .remove(&self.target_schematic_ref.key());
        apply_design_snapshot(state, &self.parent_ref, &self.before_parent)?;
        state.workspace.open_views = restored_open_views(
            &self.open_views_before,
            &current_open_views,
            [(&self.parent_ref, true)],
        );
        state
            .workspace
            .hierarchy_stack
            .clone_from(&self.hierarchy_stack_before);
        state
            .workspace
            .hierarchy_instances
            .clone_from(&self.hierarchy_instances_before);
        state.workspace.active_view = self.parent_ref.clone();
        state.restore_active_schematic_from_workspace();
        Ok(())
    }

    fn apply_after(&self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Create hierarchy cannot be redone because the source or destination changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        let current_open_views = state.workspace.open_views.clone();
        let library = state
            .library_manager
            .get_library_mut(&self.target_schematic_ref.library)
            .ok_or_else(|| "The target library no longer exists.".to_owned())?;
        library.add_cell(self.target_cell.clone());
        apply_design_snapshot(state, &self.parent_ref, &self.after_parent)?;
        state
            .workspace
            .schematic_buffers
            .insert(self.target_schematic_ref.key(), self.child_template.clone());
        state.workspace.open_views = restored_open_views(
            &self.open_views_after,
            &current_open_views,
            [(&self.parent_ref, true), (&self.target_open_ref, true)],
        );
        state
            .workspace
            .hierarchy_stack
            .clone_from(&self.hierarchy_stack_after);
        state
            .workspace
            .hierarchy_instances
            .clone_from(&self.hierarchy_instances_after);
        state.workspace.active_view = self.target_open_ref.clone();
        state.restore_active_schematic_from_workspace();
        Ok(())
    }
}

fn schematic_matches(
    state: &AppState,
    reference: &CellViewRef,
    expected: &SchematicSnapshot,
) -> bool {
    if state.workspace.active_schematic_reference() == *reference {
        expected.is_equal_state(&state.schematic)
    } else {
        state
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .is_some_and(|schematic| expected.is_equal_state(schematic))
    }
}

fn apply_design_snapshot(
    state: &mut AppState,
    reference: &CellViewRef,
    snapshot: &SchematicSnapshot,
) -> Result<(), String> {
    if state.workspace.active_schematic_reference() == *reference {
        snapshot.apply(&mut state.schematic);
        state
            .workspace
            .schematic_buffers
            .insert(reference.key(), state.schematic.clone());
        return Ok(());
    }
    let schematic = state
        .workspace
        .schematic_buffers
        .get_mut(&reference.key())
        .ok_or_else(|| {
            format!(
                "Schematic '{}' is no longer open.",
                reference.display_path()
            )
        })?;
    snapshot.apply(schematic);
    Ok(())
}

fn schematic_read_only(state: &AppState, reference: &CellViewRef) -> bool {
    if state.workspace.active_schematic_reference() == *reference {
        state.schematic.read_only
    } else {
        state
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .is_some_and(|schematic| schematic.read_only)
    }
}

fn schematic_clone(state: &AppState, reference: &CellViewRef) -> Option<SchematicState> {
    if state.workspace.active_schematic_reference() == *reference {
        Some(state.schematic.clone())
    } else {
        state
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .cloned()
    }
}

fn restored_open_views<const N: usize>(
    expected: &[OpenCellView],
    current: &[OpenCellView],
    dirty_overrides: [(&CellViewRef, bool); N],
) -> Vec<OpenCellView> {
    expected
        .iter()
        .cloned()
        .map(|mut restored| {
            if let Some(actual) = current
                .iter()
                .find(|actual| actual.reference == restored.reference)
            {
                restored.dirty = actual.dirty;
            }
            if let Some((_, dirty)) = dirty_overrides
                .iter()
                .find(|(reference, _)| **reference == restored.reference)
            {
                restored.dirty = *dirty;
            }
            restored
        })
        .collect()
}

fn navigation_matches(
    state: &AppState,
    expected_views: &[OpenCellView],
    expected_stack: &[CellViewRef],
    expected_instances: &[String],
) -> bool {
    state.workspace.open_views.len() == expected_views.len()
        && state
            .workspace
            .open_views
            .iter()
            .zip(expected_views)
            .all(|(actual, expected)| {
                actual.reference == expected.reference && actual.view_type == expected.view_type
            })
        && state.workspace.hierarchy_stack == expected_stack
        && state.workspace.hierarchy_instances == expected_instances
}

fn has_external_master_reference(state: &AppState, record: &HierarchyExtractionRecord) -> bool {
    let target_library = record.target_schematic_ref.library.as_str();
    let target_cell = record.target_schematic_ref.cell.as_str();
    let is_external =
        |key: &str| key != record.parent_ref.key() && key != record.target_schematic_ref.key();
    let references_target = |schematic: &SchematicState| {
        schematic_references_master(schematic, target_library, target_cell)
    };

    state
        .workspace
        .schematic_buffers
        .iter()
        .any(|(key, schematic)| is_external(key) && references_target(schematic))
        || (is_external(&state.workspace.active_schematic_reference().key())
            && references_target(&state.schematic))
}

fn schematic_references_master(
    schematic: &SchematicState,
    target_library: &str,
    target_cell: &str,
) -> bool {
    schematic.components.iter().any(|component| {
        component.kind == ComponentType::CellInstance
            && component.library_cell.as_ref().is_some_and(|binding| {
                binding.library == target_library && binding.cell == target_cell
            })
    })
}

fn target_cell_matches(state: &AppState, reference: &CellViewRef, expected: &Cell) -> bool {
    let Some(actual) = state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
    else {
        return false;
    };
    normalized_cell(actual) == normalized_cell(expected)
}

fn normalized_cell(cell: &Cell) -> Option<serde_json::Value> {
    let mut cell = cell.clone();
    cell.expanded = false;
    for view in cell.views.values_mut() {
        view.modified = false;
        view.is_open = false;
        view.modified_time = None;
        view.file_path = None;
    }
    serde_json::to_value(cell).ok()
}

#[cfg(test)]
mod tests;
