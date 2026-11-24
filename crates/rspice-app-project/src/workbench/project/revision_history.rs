//! The project's revision journal, which the History page lists beside its
//! checkpoints: every record the project's owners keep, built when the page
//! shows and again only when what they were built from moves; and a
//! schematic revision compared with its view's drawing as it stands, and put
//! back.
//!
//! Validated schematic revisions own exact restorable snapshots. Simulation
//! plans, model/PDK bindings, and governed design-management receipts expose
//! their retained identities and metadata without inventing missing actors,
//! timestamps, or restoration authority.

use std::sync::Arc;

use crate::product::ContentDigest;
use crate::state::model_library::ModelSourceAuthority;
use crate::state::{
    CellViewRef, ValidatedRevisionJournal, ValidatedRevisionSemanticDelta,
    ValidatedSchematicRevision, ValidatedSchematicRevisionId, ViewType,
};
use crate::ui::widgets::table::{RowKey, TableState};
use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Times this thread built the journal's rows.
    pub static AUDIT_ROW_BUILDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Between the parts of a list in one cell.
const SEPARATOR: &str = " \u{b7} ";

/// One record of the revision journal, as the History page lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectAuditRow {
    pub(crate) key: String,
    pub(crate) kind: ProjectAuditArtifactKind,
    pub(crate) revision: String,
    /// When the record was made, as a date and time; empty for a record
    /// that keeps no time.
    pub(crate) time: String,
    /// The same moment in milliseconds since the epoch, which the table
    /// sorts by.
    pub(crate) made_unix_ms: Option<u64>,
    pub(crate) actor: String,
    pub(crate) change: String,
    pub(crate) affected_evidence: String,
    pub(crate) status: String,
    pub(crate) schematic_revision: Option<ValidatedSchematicRevisionId>,
    /// The schematic view a revision belongs to, which restoring it opens.
    pub(crate) view: Option<CellViewRef>,
    /// The view's drawing is this revision's as it stands.
    pub(crate) current: bool,
}

/// What the journal's rows were built from: the schematic in front, which a
/// restore changes and opens, and the project revision, which covers a
/// governed change. The page builds them again when it next shows, too.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProjectAuditRowsSource {
    pub(crate) journal_len: usize,
    pub(crate) journal_head: Option<ValidatedSchematicRevisionId>,
    pub(crate) project_revision: u64,
    pub(crate) active_schematic_epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProjectAuditArtifactKind {
    Schematic,
    SimulationPlan,
    ModelOrPdk,
    Annotation,
    HierarchyCheck,
}

/// What a History row is, which its Kind column says and its kind filter
/// picks: a checkpoint, a damaged one set aside, or a record of the journal
/// by what it records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HistoryKind {
    Checkpoint,
    Schematic,
    SimulationPlan,
    ModelsAndPdk,
    AnnotationAndChecks,
    SetAside,
}

impl HistoryKind {
    /// Every kind, in the order the kind filter offers them.
    pub const ALL: [Self; 6] = [
        Self::Checkpoint,
        Self::Schematic,
        Self::SimulationPlan,
        Self::ModelsAndPdk,
        Self::AnnotationAndChecks,
        Self::SetAside,
    ];

    /// The kind as a row's Kind column says it.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Checkpoint => "Checkpoint",
            Self::Schematic => "Schematic",
            Self::SimulationPlan => "Simulation plan",
            Self::ModelsAndPdk => "Models and PDK",
            Self::AnnotationAndChecks => "Annotation and checks",
            Self::SetAside => "Set aside",
        }
    }

    /// The kind as the kind filter offers it: the rows of that kind.
    pub const fn plural(self) -> &'static str {
        match self {
            Self::Checkpoint => "Checkpoints",
            Self::Schematic => "Schematics",
            Self::SimulationPlan => "Simulation plans",
            Self::ModelsAndPdk => "Models and PDK",
            Self::AnnotationAndChecks => "Annotation and checks",
            Self::SetAside => "Set aside",
        }
    }
}

impl ProjectAuditArtifactKind {
    /// The History kind a journal record of this kind is listed under.
    pub(crate) const fn history_kind(self) -> HistoryKind {
        match self {
            Self::Schematic => HistoryKind::Schematic,
            Self::SimulationPlan => HistoryKind::SimulationPlan,
            Self::ModelOrPdk => HistoryKind::ModelsAndPdk,
            Self::Annotation | Self::HierarchyCheck => HistoryKind::AnnotationAndChecks,
        }
    }

    /// What the record is, in the inspector's words.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Schematic => "Schematic revision",
            Self::SimulationPlan => "Simulation plan",
            Self::ModelOrPdk => "Model or PDK",
            Self::Annotation => "Annotation",
            Self::HierarchyCheck => "Hierarchy check",
        }
    }
}

/// A restore that opened its revision's view and waits for it to be in
/// front before it puts the revision back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PendingRestore {
    pub(crate) view: CellViewRef,
    pub(crate) revision: ValidatedSchematicRevisionId,
}

/// A schematic revision's restore, asked and waiting for its answer: the
/// record, and the schematic in front as it stood when it was asked, which
/// the answer is checked against.
#[derive(Debug, Clone)]
pub(crate) struct RevisionRestore {
    pub(crate) key: String,
    pub(crate) authority: SchematicEditAuthority,
    pub(crate) expected_journal: ValidatedRevisionJournal,
    /// Why the answer could not be carried out.
    pub(crate) error: Option<String>,
}

/// What Compare… found for a schematic revision: the two drawings, in
/// words, and what changed from the one to the other.
#[derive(Debug, Clone)]
pub(crate) struct RevisionComparison {
    pub(crate) key: String,
    pub(crate) sides: String,
    pub(crate) delta: ValidatedRevisionSemanticDelta,
}

/// The History page between frames: what its filter and kind let through,
/// its table, the journal's rows and the union of them with the checkpoints,
/// each built only when what it was built from moves; and the revision
/// restore or comparison it is asking about.
#[derive(Debug, Clone, Default)]
pub struct HistoryPageState {
    pub(crate) filter: String,
    /// The one kind the table lists; none for every kind.
    pub(crate) kind: Option<HistoryKind>,
    /// The table's selection, sort, widths and sideways scroll.
    pub(crate) table: TableState,
    /// The table had the keyboard on its last frame.
    pub(crate) focused: bool,
    /// The pass the page last drew on: a page that was not drawn on the pass
    /// before builds its journal rows again.
    pub(crate) drawn_on: Option<u64>,
    pub(crate) rows: Arc<[ProjectAuditRow]>,
    pub(crate) rows_source: Option<ProjectAuditRowsSource>,
    /// The table's rows and what they were built from.
    pub(crate) listed: Option<(HistoryListSource, HistoryList)>,
    pub(crate) revision_restore: Option<RevisionRestore>,
    pub(crate) revision_comparison: Option<RevisionComparison>,
    pub(crate) pending_restore: Option<PendingRestore>,
}

/// What the table's rows were built from: the journal's rows, the catalog
/// as its fingerprint, the filter and the kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HistoryListSource {
    pub(crate) journal: Option<ProjectAuditRowsSource>,
    pub(crate) catalog: u64,
    pub(crate) filter: String,
    pub(crate) kind: Option<HistoryKind>,
}

/// One row of the History table, newest first: a checkpoint, a damaged one
/// set aside, or a record of the journal, with what its cells say.
#[derive(Debug, Clone)]
pub(crate) struct HistoryRow {
    pub(crate) key: String,
    pub(crate) kind: HistoryKind,
    pub(crate) made_unix_ms: Option<u64>,
    pub(crate) when: String,
    pub(crate) change: String,
    pub(crate) subject: String,
    pub(crate) revision: String,
    pub(crate) pinned: bool,
    pub(crate) note: String,
    /// Why a damaged checkpoint was set aside, in a few words.
    pub(crate) status: String,
    /// The same, in a sentence that says what to do about it.
    pub(crate) detail: String,
}

/// Every row the page has and the ones its filter and kind let through, by
/// index, with their keys; and the kinds the page has rows of.
#[derive(Debug, Clone)]
pub(crate) struct HistoryList {
    pub(crate) rows: Arc<[HistoryRow]>,
    pub(crate) visible: Arc<[usize]>,
    pub(crate) keys: Arc<[RowKey]>,
    pub(crate) kinds: Arc<[HistoryKind]>,
    /// Moves whenever a row's words do, so the table reads its rows once
    /// per change rather than on every frame it is sorted.
    pub(crate) revision: u64,
}

impl HistoryList {
    pub(crate) fn row(&self, key: &str) -> Option<&HistoryRow> {
        self.rows.iter().find(|row| row.key == key)
    }
}

/// The key a damaged checkpoint's row goes by: its label, which no
/// checkpoint identity or journal key can be.
pub(crate) fn set_aside_key(label: &str) -> String {
    format!("set-aside:{label}")
}

/// The journal's rows, built again only when what they were built from has
/// moved since, or when the page shows after a pass it did not draw on.
pub(crate) fn kept_audit_rows(
    history: &mut HistoryPageState,
    model: &Model,
    pass: u64,
) -> Arc<[ProjectAuditRow]> {
    if history.drawn_on.is_none_or(|drawn| drawn + 1 < pass) {
        history.rows_source = None;
    }
    history.drawn_on = Some(pass);
    let journal = &model.schematic.document().validated_revisions;
    let source = ProjectAuditRowsSource {
        journal_len: journal.records().len(),
        journal_head: journal.records().last().map(ValidatedSchematicRevision::id),
        project_revision: model.workspace.content.project.revision().get(),
        active_schematic_epoch: model.active_schematic_epoch,
    };
    if history.rows_source != Some(source) {
        history.rows = project_audit_rows(model).into();
        history.rows_source = Some(source);
    }
    Arc::clone(&history.rows)
}

/// The key of the newest revision of the schematic in front, which File >
/// Revision history… stands the History page on.
pub(crate) fn newest_front_revision(model: &Model) -> Option<String> {
    model
        .schematic
        .document()
        .validated_revisions
        .records()
        .last()
        .map(|record| schematic_row_key(record.id()))
}

/// What Compare… finds for the schematic revision `row`: what changed in
/// its view's drawing since it was recorded.
pub(crate) fn revision_comparison(
    row: &ProjectAuditRow,
    model: &Model,
) -> Result<RevisionComparison, String> {
    let (record, delta) = changes_since(row, model).map_err(str::to_owned)?;
    let path = row
        .view
        .as_ref()
        .map(CellViewRef::display_path)
        .unwrap_or_default();
    Ok(RevisionComparison {
        key: row.key.clone(),
        sides: format!(
            "From revision {} to the drawing of {path} as it stands.",
            revision_label(record)
        ),
        delta,
    })
}

/// Ask whether to restore the schematic revision `row`, holding the
/// schematic in front as it stands for the answer to be checked against.
pub(crate) fn ask_revision_restore(history: &mut HistoryPageState, model: &Model, key: String) {
    history.revision_restore = Some(RevisionRestore {
        key,
        authority: SchematicEditAuthority::capture(model),
        expected_journal: model.schematic.document().validated_revisions.clone(),
        error: None,
    });
}

/// Carry out the revision restore the reader confirmed: in place when its
/// view is in front, and otherwise once the view it opens is. Answers what
/// was done, or why nothing was, which the question then states.
pub(crate) fn restore_revision(
    cx: &mut Cx<'_, ProjectFeature>,
    row: &ProjectAuditRow,
) -> Result<(), String> {
    let Some(asked) = cx.state.history.revision_restore.clone() else {
        return Ok(());
    };
    asked
        .authority
        .validate_presentation(&*cx.model, "Restore")?;
    if asked.expected_journal != cx.model.schematic.document().validated_revisions {
        return Err(
            "The schematic's revisions changed since Restore was asked. Ask again.".to_owned(),
        );
    }
    match restore_target(row, cx.model)? {
        (revision, None) => {
            restore_in_place(cx, revision)?;
            restored(cx, row);
        }
        (revision, Some(view)) => {
            // The shell brings the view to the front once this frame's
            // dialogs have drawn; the restore waits for it there.
            cx.wb
                .outbox
                .push(ProjectRequest::OpenView(view.clone(), ViewType::Schematic));
            cx.state.history.pending_restore = Some(PendingRestore { view, revision });
        }
    }
    cx.state.history.revision_restore = None;
    Ok(())
}

/// Carry out a restore that opened its view, now that the view is in front;
/// a view that did not open is said not to have, and nothing is restored.
pub(crate) fn finish_pending_restore(cx: &mut Cx<'_, ProjectFeature>) {
    let Some(pending) = cx.state.history.pending_restore.take() else {
        return;
    };
    if cx.model.workspace.content.active_view != pending.view {
        cx.model.report_failure(
            REVISION_NOT_RESTORED,
            format!(
                "{} did not open, so its revision was not restored.",
                pending.view.display_path()
            ),
        );
        return;
    }
    let row = cx
        .state
        .history
        .rows
        .iter()
        .find(|row| row.schematic_revision == Some(pending.revision))
        .cloned();
    match restore_in_place(cx, pending.revision) {
        Ok(()) => {
            if let Some(row) = row {
                restored(cx, &row);
            }
        }
        Err(error) => cx.model.report_failure(REVISION_NOT_RESTORED, error),
    }
}

/// The notice a revision restore that could not be made raises.
const REVISION_NOT_RESTORED: &str = "Revision not restored";

/// Say on the status line what a restore put back.
fn restored(cx: &mut Cx<'_, ProjectFeature>, row: &ProjectAuditRow) {
    let path = row
        .view
        .as_ref()
        .map(CellViewRef::display_path)
        .unwrap_or_default();
    cx.wb.prompt(format!(
        "Restored revision {} of {path}. Undo takes it back.",
        row.revision
    ));
}

/// Put revision `id` of the schematic in front back, as one step of Undo.
fn restore_in_place(
    cx: &mut Cx<'_, ProjectFeature>,
    id: ValidatedSchematicRevisionId,
) -> Result<(), String> {
    if cx.model.safe_mode.project_read_only() || cx.model.schematic_edit_read_only() {
        return Err("The active schematic or project is read-only.".to_owned());
    }
    cx.model
        .schematic
        .restore_validated_revision(id)
        .map_err(|error| error.to_string())?;
    if let Some(prompt) = cx.model.sync_active_schematic_to_workspace() {
        cx.wb.prompt(prompt);
    }
    // What the run's preflight decided, the run prepared and the
    // netlist generated was for the design the restore replaced.
    // The next two calls are what RSpiceApp::invalidate_simulation_preflight does.
    cx.runs.preflight.invalidate();
    cx.handles.run_controller.clear_prepared_run();
    cx.model.netlist_artifacts.current_generation_input_digest = None;
    Ok(())
}

/// The revision `row` names, and what changed in its view's drawing since
/// it was recorded: the open schematic's, or the one the project keeps for
/// a view that is not in front.
fn changes_since<'a>(
    row: &ProjectAuditRow,
    model: &'a Model,
) -> Result<
    (
        &'a ValidatedSchematicRevision,
        ValidatedRevisionSemanticDelta,
    ),
    &'static str,
> {
    let (Some(id), Some(view)) = (row.schematic_revision, row.view.as_ref()) else {
        return Err(NOT_A_DRAWING);
    };
    let record = find_project_schematic_revision(model, id)
        .ok_or("The selected schematic revision no longer exists.")?;
    let delta = if *view == model.workspace.content.active_schematic_reference() {
        model
            .schematic
            .editor_ref()
            .design
            .semantic_delta_since(record)
    } else {
        model
            .workspace
            .content
            .schematic_buffers
            .get(&view.key())
            .ok_or("The selected revision's view is no longer in the project.")?
            .semantic_delta_since(record)
    };
    Ok((record, delta))
}

/// Why a record cannot be restored or compared with the current design.
const NOT_A_DRAWING: &str =
    "Only a schematic revision keeps a drawing to restore or to compare with the current design.";

/// The revision `row` restores and the view it opens first, when that view
/// is not the one in front; or why it cannot be restored.
pub(crate) fn restore_target(
    row: &ProjectAuditRow,
    model: &Model,
) -> Result<(ValidatedSchematicRevisionId, Option<CellViewRef>), String> {
    let (Some(revision), Some(view)) = (row.schematic_revision, row.view.as_ref()) else {
        return Err(NOT_A_DRAWING.to_owned());
    };
    let path = view.display_path();
    if row.current {
        return Err(format!("{path} already matches this revision."));
    }
    if model.safe_mode.project_read_only() {
        return Err("The project is open read-only.".to_owned());
    }
    if model
        .library_manager
        .get_library(&view.library)
        .is_some_and(|library| library.read_only)
    {
        return Err(format!("{path} is in a read-only library."));
    }
    let in_front = *view == model.workspace.content.active_view;
    if in_front && model.schematic_edit_read_only() {
        return Err(format!("{path} is open read-only."));
    }
    Ok((revision, (!in_front).then(|| view.clone())))
}

fn find_project_schematic_revision(
    model: &Model,
    id: ValidatedSchematicRevisionId,
) -> Option<&ValidatedSchematicRevision> {
    model
        .schematic
        .document()
        .validated_revisions
        .records()
        .iter()
        .find(|record| record.id() == id)
        .or_else(|| {
            model
                .workspace
                .content
                .schematic_buffers
                .values()
                .flat_map(|schematic| schematic.document().validated_revisions.records())
                .find(|record| record.id() == id)
        })
}

fn project_audit_rows(model: &Model) -> Vec<ProjectAuditRow> {
    #[cfg(any(test, feature = "test-support"))]
    AUDIT_ROW_BUILDS.with(|builds| builds.set(builds.get() + 1));
    let mut rows = Vec::new();
    // The schematic open in front, which is the symbol's own while a symbol
    // is, then every other schematic view the project keeps.
    let active = model.workspace.content.active_schematic_reference();
    let active_key = active.key();
    let design = model.schematic.editor_ref().design;
    append_schematic_rows(
        &mut rows,
        &model.schematic.document().validated_revisions,
        || design.validated_design_content_digest().ok(),
        Some(active),
    );
    let mut retained_schematics = model
        .workspace
        .content
        .schematic_buffers
        .iter()
        .filter(|entry| entry.0.as_str() != active_key.as_str())
        .collect::<Vec<_>>();
    retained_schematics.sort_by(|(left, _), (right, _)| left.cmp(right));
    for (key, schematic) in retained_schematics {
        append_schematic_rows(
            &mut rows,
            &schematic.document().validated_revisions,
            || schematic.validated_design_content_digest().ok(),
            CellViewRef::from_key(key),
        );
    }
    append_simulation_plan_rows(&mut rows, model);
    append_model_and_pdk_rows(&mut rows, model);
    append_governed_rows(&mut rows, model);
    rows
}

/// One row per revision of `view`'s journal, newest first. `drawn` is the
/// digest of the view's drawing as it stands, asked only of a view with
/// revisions to hold it against.
fn append_schematic_rows(
    rows: &mut Vec<ProjectAuditRow>,
    journal: &ValidatedRevisionJournal,
    drawn: impl FnOnce() -> Option<ContentDigest>,
    view: Option<CellViewRef>,
) {
    if journal.records().is_empty() {
        return;
    }
    let drawn = drawn();
    for record in journal.records().iter().rev() {
        let current = drawn == Some(record.design_content_digest());
        // The drawing as it stands is this revision's: what the reader most
        // needs to know of it, before which kind of revision it is.
        let status = if current {
            "Current"
        } else if record.is_accepted_baseline() {
            "Accepted"
        } else {
            "Validated"
        };
        let made = record.created_unix_ms();
        rows.push(ProjectAuditRow {
            key: schematic_row_key(record.id()),
            kind: ProjectAuditArtifactKind::Schematic,
            revision: short_digest(record),
            time: written_time(made),
            made_unix_ms: (made != 0).then_some(made),
            actor: record.author().to_owned(),
            change: if record.is_accepted_baseline() {
                "Schematic accepted".to_owned()
            } else {
                record.revision_note().to_owned()
            },
            affected_evidence: affected_schematic_evidence(record),
            status: status.to_owned(),
            schematic_revision: Some(record.id()),
            view: view.clone(),
            current,
        });
    }
}

fn append_simulation_plan_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    if let Some(plan) = model.sim_setup.analysis_plan.as_ref() {
        rows.push(simulation_plan_row(
            model.sim_setup.active_plan_name().as_str(),
            plan.id().to_string(),
            plan.revision().get().to_string(),
            plan.instances().len(),
            model.sim_setup.active_plan_lineage(),
            true,
        ));
    }
    for plan in model.sim_setup.inactive_plans().iter().rev() {
        rows.push(simulation_plan_row(
            plan.name().as_str(),
            plan.id().to_string(),
            plan.revision().get().to_string(),
            plan.analysis_plan().instances().len(),
            plan.lineage(),
            false,
        ));
    }
}

fn simulation_plan_row(
    name: &str,
    id: String,
    revision: String,
    analysis_count: usize,
    lineage: crate::workbench::app_state::SimulationPlanLineage,
    active: bool,
) -> ProjectAuditRow {
    let lineage_evidence = match (lineage.source_plan_id(), lineage.source_revision()) {
        (Some(source), Some(source_revision)) => {
            format!("Cloned from {source} at revision {}", source_revision.get())
        }
        _ => "Root plan".to_owned(),
    };
    ProjectAuditRow {
        key: format!("simulation-plan:{id}:{revision}"),
        kind: ProjectAuditArtifactKind::SimulationPlan,
        revision,
        time: String::new(),
        made_unix_ms: None,
        actor: String::new(),
        change: name.to_owned(),
        affected_evidence: format!(
            "{}{SEPARATOR}{lineage_evidence}",
            counted(analysis_count, "analysis", "analyses")
        ),
        status: if active { "Active" } else { "Retained" }.to_owned(),
        schematic_revision: None,
        view: None,
        current: false,
    }
}

fn append_model_and_pdk_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    if let Some(binding) = model.workspace.content.project.technology_binding() {
        let digest = binding
            .source_closure()
            .iter()
            .find(|source| source.path == binding.root_source())
            .map(|source| short_text(&source.digest.to_string()))
            .unwrap_or_else(|| model.workspace.content.project.revision().get().to_string());
        rows.push(ProjectAuditRow {
            key: format!("technology-binding:{}:{}", binding.model_library(), digest),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision: digest,
            time: String::new(),
            made_unix_ms: None,
            actor: String::new(),
            change: format!("Attached technology {}", binding.display_label()),
            affected_evidence: [
                counted(binding.model_count(), "model", "models"),
                counted(binding.source_closure().len(), "file", "files"),
                counted(
                    binding.process_sections().len(),
                    "process section",
                    "process sections",
                ),
            ]
            .join(SEPARATOR),
            status: "Attached".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }

    for library in model.model_library_manager.libraries_sorted() {
        // Where the library is read from, in the words the Dependencies
        // inspector uses, and the version of its files the project recorded.
        let (revision, status): (String, &str) = match library.source_authority {
            ModelSourceAuthority::ProjectOwned { revision, .. } => {
                (revision.get().to_string(), "Project-owned")
            }
            ModelSourceAuthority::External => {
                let root_digest = library
                    .root_path
                    .as_ref()
                    .and_then(|root| {
                        library
                            .source_closure
                            .iter()
                            .find(|source| &source.path == root)
                    })
                    .map(|source| short_text(&source.digest.to_string()));
                match root_digest {
                    Some(digest) => (digest, "Linked to its file"),
                    None => (
                        "Not recorded".to_owned(),
                        "Linked to its file, not read yet",
                    ),
                }
            }
            ModelSourceAuthority::RetainedImport { digest, .. } => {
                (short_text(&digest.to_string()), "Kept as imported")
            }
            ModelSourceAuthority::BuiltIn => (
                nonempty_or(&library.version, "Built-in"),
                "Ships with RSpice",
            ),
        };
        rows.push(ProjectAuditRow {
            key: format!("model-library:{}:{revision}", library.name),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision,
            time: String::new(),
            made_unix_ms: None,
            actor: String::new(),
            change: if library.pdk_name.trim().is_empty() {
                format!("Model library {}", library.name)
            } else {
                format!(
                    "Model library {}{SEPARATOR}{}",
                    library.name, library.pdk_name
                )
            },
            affected_evidence: [
                counted(library.model_count(), "model", "models"),
                counted(library.corner_count(), "corner", "corners"),
                counted(library.source_closure.len(), "file", "files"),
            ]
            .join(SEPARATOR),
            status: status.to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }
}

fn append_governed_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    let management = &model.workspace.content.design_management;
    for entry in management.annotation().journal().iter().rev() {
        rows.push(ProjectAuditRow {
            key: format!("annotation-journal:{}", entry.id()),
            kind: ProjectAuditArtifactKind::Annotation,
            revision: entry.sequence().to_string(),
            time: String::new(),
            made_unix_ms: None,
            actor: String::new(),
            change: format!("Annotation rules revision {}", entry.policy_revision()),
            affected_evidence: counted(entry.mappings().len(), "reference", "references"),
            status: "Committed".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }
    for receipt in management.hierarchy_audits().iter().rev() {
        rows.push(ProjectAuditRow {
            key: format!("hierarchy-audit:{}", receipt.id()),
            kind: ProjectAuditArtifactKind::HierarchyCheck,
            revision: receipt.sequence().to_string(),
            time: String::new(),
            made_unix_ms: None,
            actor: String::new(),
            change: "Checked the hierarchy".to_owned(),
            affected_evidence: [
                counted(receipt.resolved_subjects(), "instance", "instances"),
                counted(receipt.findings().len(), "finding", "findings"),
            ]
            .join(SEPARATOR),
            status: if receipt.passed() {
                "Passed"
            } else {
                "Has findings"
            }
            .to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }
}

/// The history table's key for a validated schematic revision, which its
/// selection holds.
pub(crate) fn schematic_row_key(id: ValidatedSchematicRevisionId) -> String {
    format!("schematic:{}", id.as_uuid())
}

fn short_digest(record: &ValidatedSchematicRevision) -> String {
    short_text(&record.revision_digest().to_string())
}

/// A schematic revision as a comparison names it: its digest and when it
/// was made.
fn revision_label(record: &ValidatedSchematicRevision) -> String {
    format!(
        "{} ({})",
        short_digest(record),
        crate::time_compat::local_stamp(record.created_unix_ms())
    )
}

/// When a record was made, as the list's Time column writes it; nothing
/// for the unknown time zero stands for.
fn written_time(made_unix_ms: u64) -> String {
    if made_unix_ms == 0 {
        String::new()
    } else {
        crate::time_compat::local_stamp(made_unix_ms)
    }
}

fn short_text(value: &str) -> String {
    value.chars().take(10).collect()
}

fn nonempty_or(value: &str, fallback: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        fallback.to_owned()
    } else {
        value.to_owned()
    }
}

/// `count` of a thing, named in the singular for one.
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

fn affected_schematic_evidence(record: &ValidatedSchematicRevision) -> String {
    let findings = record.finding_counts();
    let mut parts = vec![record.view_identity().to_owned()];
    if record.dependency_count() > 0 {
        parts.push(counted(
            record.dependency_count(),
            "dependency",
            "dependencies",
        ));
    }
    if findings.advisories > 0 {
        parts.push(counted(
            findings.advisories as usize,
            "advisory",
            "advisories",
        ));
    }
    if findings.blockers > 0 {
        parts.push(counted(findings.blockers as usize, "blocker", "blockers"));
    }
    if parts.len() == 1 {
        parts.push(format!("project revision {}", record.project_revision()));
    }
    parts.join(SEPARATOR)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProjectId;
    use crate::state::{
        ComponentType, Point, PreflightReport, ValidatedRevisionChangeKind,
        ValidatedRevisionDomain, ValidatedRevisionRequest, ValidationFindingCounts,
    };
    use crate::workbench::project::tests::harness;

    fn append(state: &mut crate::state::SchematicState, note: &str) {
        let digest = state
            .editor_ref()
            .design
            .validated_design_content_digest()
            .unwrap();
        state
            .append_validated_revision(ValidatedRevisionRequest {
                project_id: ProjectId::new().to_string(),
                project_revision: 1,
                view_identity: "user/top/schematic".to_owned(),
                revision_note: note.to_owned(),
                author: CURRENT_TEST_ACTOR.to_owned(),
                validation_receipt_digest: digest,
                finding_counts: ValidationFindingCounts::default(),
                dependencies: Vec::new(),
                advisory_dispositions: Vec::new(),
            })
            .unwrap();
    }

    const CURRENT_TEST_ACTOR: &str = "Test engineer";

    /// A row of a record that keeps no time and no drawing.
    fn metadata_row(key: &str, kind: ProjectAuditArtifactKind, change: &str) -> ProjectAuditRow {
        ProjectAuditRow {
            key: key.to_owned(),
            kind,
            revision: key.to_owned(),
            time: String::new(),
            made_unix_ms: None,
            actor: String::new(),
            change: change.to_owned(),
            affected_evidence: String::new(),
            status: "Retained".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        }
    }

    #[test]
    fn semantic_compare_and_restore_are_exact_and_undoable() {
        let mut state = crate::state::SchematicState::default();
        append(&mut state, "Empty baseline");
        let first = state.document().validated_revisions.records()[0].clone();
        state.add_component(ComponentType::Resistor, Point::new(10, 20));
        append(&mut state, "Add resistor");
        let second = state.document().validated_revisions.records()[1].clone();
        let delta = first.semantic_delta_to(&second);
        assert_eq!(
            delta.count(
                ValidatedRevisionDomain::Components,
                ValidatedRevisionChangeKind::Added
            ),
            1
        );

        state
            .restore_validated_revision(first.id())
            .expect("validated restore");
        assert!(state.document().components.is_empty());
        assert!(state.undo());
        assert_eq!(state.document().components.len(), 1);
    }

    /// Every record is listed under the kind that records it; an annotation
    /// and a hierarchy check share one.
    #[test]
    fn each_journal_record_is_listed_under_its_own_kind() {
        for (kind, listed) in [
            (ProjectAuditArtifactKind::Schematic, HistoryKind::Schematic),
            (
                ProjectAuditArtifactKind::SimulationPlan,
                HistoryKind::SimulationPlan,
            ),
            (
                ProjectAuditArtifactKind::ModelOrPdk,
                HistoryKind::ModelsAndPdk,
            ),
            (
                ProjectAuditArtifactKind::Annotation,
                HistoryKind::AnnotationAndChecks,
            ),
            (
                ProjectAuditArtifactKind::HierarchyCheck,
                HistoryKind::AnnotationAndChecks,
            ),
        ] {
            assert_eq!(kind.history_kind(), listed, "{kind:?}");
        }
    }

    /// Each revision states the date and time it was made; Compare… on one
    /// names it by that date and time and what changed in its view's
    /// drawing since, object by object.
    #[test]
    fn a_revision_states_when_it_was_made_and_what_changed_since() {
        let mut harness = harness();
        append(&mut harness.model.schematic, "Baseline");
        harness
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        append(&mut harness.model.schematic, "Add resistor");
        let records = harness
            .model
            .schematic
            .document()
            .validated_revisions
            .records()
            .to_vec();
        let rows = project_audit_rows(&harness.model);
        let made = crate::time_compat::local_stamp(records[1].created_unix_ms());
        assert!(
            rows.iter().any(|row| row.time == made),
            "{made:?} is not listed: {rows:?}"
        );

        // The baseline, which the resistor moved on from.
        let baseline = rows
            .iter()
            .find(|row| row.key == schematic_row_key(records[0].id()))
            .expect("the baseline is listed");
        let found = revision_comparison(baseline, &harness.model).expect("it is compared");
        let stamp = crate::time_compat::local_stamp(records[0].created_unix_ms());
        assert!(found.sides.contains(&stamp), "{:?}", found.sides);
        let added = found
            .delta
            .changes
            .iter()
            .find(|change| change.kind == ValidatedRevisionChangeKind::Added)
            .expect("the resistor is named as added");
        assert_eq!(added.domain, ValidatedRevisionDomain::Components);

        let plan = metadata_row(
            "simulation-plan:one",
            ProjectAuditArtifactKind::SimulationPlan,
            "Bias sweep",
        );
        assert_eq!(
            revision_comparison(&plan, &harness.model).map(|found| found.key),
            Err(NOT_A_DRAWING.to_owned())
        );
    }

    /// Restore says why it cannot act on a record: one that keeps no
    /// drawing, a revision the drawing already matches.
    #[test]
    fn restore_states_why_it_cannot_act() {
        let mut harness = harness();
        append(&mut harness.model.schematic, "Baseline");
        let rows = project_audit_rows(&harness.model);
        let revision = rows
            .iter()
            .find(|row| row.schematic_revision.is_some())
            .expect("the revision is listed");
        let plan = metadata_row(
            "simulation-plan:one",
            ProjectAuditArtifactKind::SimulationPlan,
            "Active",
        );
        let shown = revision
            .view
            .as_ref()
            .map(CellViewRef::display_path)
            .unwrap();
        assert!(revision.current, "nothing changed since the revision");
        assert_eq!(
            restore_target(revision, &harness.model),
            Err(format!("{shown} already matches this revision."))
        );
        assert_eq!(
            restore_target(&plan, &harness.model),
            Err(NOT_A_DRAWING.to_owned())
        );
    }

    /// A revision of a schematic that is not in front asks the shell to open
    /// it and waits; a view that did not open is said not to have, and
    /// nothing is restored.
    #[test]
    fn a_revision_of_a_view_not_in_front_opens_it_first() {
        let mut harness = harness();
        append(&mut harness.model.schematic, "Amplifier");
        // The drawing moved on since its revision.
        harness
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        let amp = CellViewRef::new("user", "amp", "schematic");
        let drawing = harness.model.schematic.editor_ref().design.clone();
        harness
            .model
            .workspace
            .content
            .schematic_buffers
            .insert(amp.key(), drawing);
        harness.model.schematic = crate::state::SchematicState::default();
        let rows = project_audit_rows(&harness.model);
        let row = rows
            .iter()
            .find(|row| row.view.as_ref() == Some(&amp))
            .expect("the other view's revision is listed")
            .clone();
        assert!(!row.current);
        assert_eq!(
            restore_target(&row, &harness.model),
            Ok((row.schematic_revision.unwrap(), Some(amp.clone())))
        );
        harness.state.history.rows = rows.into();
        ask_revision_restore(&mut harness.state.history, &harness.model, row.key.clone());

        assert_eq!(restore_revision(&mut harness.cx(), &row), Ok(()));

        let history = &harness.state.history;
        assert!(
            history.revision_restore.is_none(),
            "the question is answered"
        );
        let pending = history.pending_restore.clone().expect("the restore waits");
        assert_eq!(pending.view, amp);
        match harness.wb.outbox.take::<ProjectRequest>() {
            Some(ProjectRequest::OpenView(view, ViewType::Schematic)) => assert_eq!(view, amp),
            other => panic!("the view is not opened: {other:?}"),
        }

        // The shell did not bring it to the front.
        finish_pending_restore(&mut harness.cx());
        assert_eq!(harness.state.history.pending_restore, None);
        let notices = harness.model.notices.take();
        assert!(
            notices.iter().any(|notice| notice.detail
                == "user/amp/schematic did not open, so its revision was not restored."),
            "{notices:?}"
        );
        assert_eq!(
            harness.model.workspace.content.schematic_buffers[&amp.key()]
                .document()
                .components
                .len(),
            1
        );
    }

    /// A restore in the project feature's context does what the application
    /// did in place: the design is the revision's, and what was decided,
    /// prepared or generated for the replaced design is dropped. A restore
    /// refused, here because the journal moved since Restore was asked,
    /// leaves all of it alone and says why.
    #[test]
    fn a_revision_restored_in_the_project_context_ends_where_the_restore_in_place_did() {
        for journal_moved in [false, true] {
            let mut harness = harness();
            let schematic = &mut harness.model.schematic;
            let baseline_components = schematic.document().components.len();
            append(schematic, "Baseline");
            let baseline = schematic.document().validated_revisions.records()[0].id();
            let read_at_ask = schematic.document().validated_revisions.clone();
            schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
            append(schematic, "Add resistor");
            ask_revision_restore(
                &mut harness.state.history,
                &harness.model,
                schematic_row_key(baseline),
            );
            if journal_moved {
                harness
                    .state
                    .history
                    .revision_restore
                    .as_mut()
                    .unwrap()
                    .expected_journal = read_at_ask;
            }
            let digest = ContentDigest::from_bytes([7; 32]);
            harness
                .model
                .netlist_artifacts
                .current_generation_input_digest = Some(digest);
            harness.runs.preflight.report = Some(PreflightReport {
                project_revision: 1,
                topology_root: "user/top/schematic".to_owned(),
                topology_revision: 1,
                topology_closure: Vec::new(),
                simulation_plan_id: None,
                simulation_plan_revision: None,
                blockers: Vec::new(),
                advisories: Vec::new(),
                prepared: None,
            });

            let rows = project_audit_rows(&harness.model);
            let row = rows
                .iter()
                .find(|row| row.key == schematic_row_key(baseline))
                .expect("the baseline is listed")
                .clone();
            let answer = restore_revision(&mut harness.cx(), &row);

            let model = &harness.model;
            if journal_moved {
                assert_eq!(
                    answer,
                    Err(
                        "The schematic's revisions changed since Restore was asked. Ask again."
                            .to_owned()
                    )
                );
                assert_eq!(
                    model.schematic.document().components.len(),
                    baseline_components + 1
                );
                assert_eq!(
                    model.netlist_artifacts.current_generation_input_digest,
                    Some(digest)
                );
                assert!(harness.runs.preflight.report.is_some());
            } else {
                assert_eq!(answer, Ok(()));
                assert!(harness.state.history.revision_restore.is_none());
                assert_eq!(
                    model.schematic.document().components.len(),
                    baseline_components
                );
                assert_eq!(
                    model.netlist_artifacts.current_generation_input_digest,
                    None
                );
                assert!(harness.runs.preflight.report.is_none());
            }
        }
    }
}
