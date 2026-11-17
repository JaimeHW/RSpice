//! The Overview's snapshot: what the page shows, read once per frame from the
//! project, its lifecycle, the checks, the run history and the model catalog
//! before anything is drawn.
//!
//! # Derived on change, not on a frame
//!
//! What has to walk the design or the catalogues to be known â€” the net count,
//! the view count, the activity register, the model sources, the deck's line
//! count, the configuration's validity â€” is derived once and kept in the
//! frame context beside the identity of what it was derived from: a content
//! stamp, a catalogue revision, the run history's revision, a digest. A frame
//! in which none of those moved derives nothing, so the page costs the same on
//! a project of eight thousand instances as on an empty one.
//!
//! Each status row's words and mark are its subject's, from
//! [`crate::workbench::project::health`]; the row adds only the action that
//! addresses it.

use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};
use std::sync::Arc;

use crate::state::netlist_document::{DiagnosticSeverity, DocumentOwnership};
use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::{DesignCheckStatus, Model};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::lifecycle::project_lifecycle::dirty_document_count;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::health::{self, DesignSubject, SubjectStatus};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::project::{ProjectFeature, lifecycle_view};
use crate::workbench::state::Workspace;
use rspice_design::drc::DrcSeverity;

use super::super::derived::derived;
use super::activity::{relative_time, run_title, unix_seconds_to_millis};
use super::{
    OverviewIntent, Tone, command_gate, lifecycle_label, lifecycle_tone, run_destination,
    singular_or_plural,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusSnapshot {
    pub area: &'static str,
    pub(crate) state: String,
    pub(crate) detail: String,
    pub action: &'static str,
    pub(crate) tone: Tone,
    pub intent: OverviewIntent,
    pub(super) enabled: bool,
    pub(super) disabled_reason: Option<&'static str>,
}

impl StatusSnapshot {
    fn of(
        area: &'static str,
        status: SubjectStatus,
        action: &'static str,
        intent: OverviewIntent,
        (enabled, disabled_reason): (bool, Option<&'static str>),
    ) -> Self {
        Self {
            area,
            state: status.state,
            detail: status.detail,
            action,
            tone: status.tone,
            intent,
            enabled,
            disabled_reason,
        }
    }

    /// The row's words as the page paints them: the state, then the detail
    /// when there is one.
    pub fn text(&self) -> String {
        if self.detail.is_empty() {
            self.state.clone()
        } else {
            format!("{} \u{b7} {}", self.state, self.detail)
        }
    }

    /// The row's state alone, the words the chips and the inspector repeat.
    pub fn state(&self) -> &str {
        &self.state
    }

    pub const fn tone(&self) -> Tone {
        self.tone
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationSnapshot {
    pub configured: bool,
    pub state: String,
    pub(crate) detail: String,
    pub root: String,
    pub dut: String,
    pub tone: Tone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OpenDocumentSnapshot {
    pub(super) reference: CellViewRef,
    pub(super) view_type: ViewType,
    pub(super) active: bool,
    pub(super) dirty: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationSnapshot {
    pub when: String,
    pub event: String,
    pub detail: String,
    pub(crate) tone: Tone,
    pub intent: Option<OverviewIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblemSnapshot {
    pub code: String,
    pub message: String,
    pub path: String,
    pub tone: Tone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceDeckSnapshot {
    pub(super) name: String,
    pub(super) ownership: &'static str,
    pub(super) revision: u64,
    pub(super) line_count: usize,
    pub(super) dependency_count: usize,
    pub(super) dependencies_sealed: bool,
    pub(super) validation_tone: Tone,
    /// The deck's worst current diagnostic, which the strip under the status
    /// rows names; nothing while the deck has none. A deck not validated, or
    /// changed since, is its status row's to say.
    problem: Option<ProblemSnapshot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverviewSnapshot {
    pub(super) project_name: String,
    pub(super) project_revision: u64,
    /// Where the project file is, or `None` before it is first saved.
    pub(crate) location: Option<String>,
    pub descriptor_root: String,
    pub(super) simulation_root: String,
    pub(super) technology: String,
    pub(super) technology_attached: bool,
    pub(super) library_count: usize,
    pub(super) model_library_count: usize,
    pub(super) cell_count: usize,
    pub(super) view_count: usize,
    pub(super) root_sheet_count: usize,
    pub(super) root_instance_count: usize,
    pub(super) root_net_count: usize,
    pub netlist_first: bool,
    pub(super) source_deck: Option<SourceDeckSnapshot>,
    pub configuration: ConfigurationSnapshot,
    pub statuses: Vec<StatusSnapshot>,
    pub problem: Option<ProblemSnapshot>,
    pub(super) open_documents: Vec<OpenDocumentSnapshot>,
    pub operations: Vec<OperationSnapshot>,
    /// Every operation the project keeps, counted from the records rather
    /// than from the truncated list above: reporting the shown count as the
    /// kept count would claim a project with thirty runs has five.
    pub operation_total: usize,
}

fn source_deck_snapshot(ctx: &egui::Context, model: &Model) -> Option<SourceDeckSnapshot> {
    let document = model.workspace.content.netlist_document.as_ref()?;
    let name = health::deck_name(model)?;
    let ownership = match document.ownership() {
        DocumentOwnership::Generated => "generated",
        DocumentOwnership::Imported => "imported",
        DocumentOwnership::Editable => "editable",
    };
    let line_count = derived(
        ctx,
        "overview.deck lines",
        document.content_digest(),
        || document.source().lines().count(),
    );
    let current = document
        .validation()
        .filter(|report| report.content_digest() == document.content_digest());
    // Nothing at all when the deck validates: "No current source
    // diagnostics" was a strip the page painted to report an absence.
    let problem = current
        .and_then(|report| {
            report
                .diagnostics()
                .iter()
                .max_by_key(|diagnostic| diagnostic.severity())
        })
        .map(|diagnostic| {
            let position = diagnostic.position();
            ProblemSnapshot {
                code: diagnostic.code().map_or_else(
                    || match diagnostic.severity() {
                        DiagnosticSeverity::Info => "SPICE-INFO".to_owned(),
                        DiagnosticSeverity::Warning => "SPICE-WARN".to_owned(),
                        DiagnosticSeverity::Error => "SPICE-ERROR".to_owned(),
                    },
                    str::to_owned,
                ),
                message: diagnostic.message().to_owned(),
                path: format!("{}:{}:{}", name, position.line(), position.column()),
                tone: match diagnostic.severity() {
                    DiagnosticSeverity::Info => Tone::Neutral,
                    DiagnosticSeverity::Warning => Tone::Warn,
                    DiagnosticSeverity::Error => Tone::Error,
                },
            }
        });

    Some(SourceDeckSnapshot {
        name,
        ownership,
        revision: document.revision().get(),
        line_count,
        dependency_count: document.dependencies().len(),
        dependencies_sealed: document.dependency_graph_is_sealed(),
        validation_tone: health::source_deck(model).tone,
        problem,
    })
}

impl OverviewSnapshot {
    /// What the page shows now, with what has to walk the project to be
    /// known kept in `ctx` until what it was derived from changes.
    pub fn capture(ctx: &egui::Context, cx: &Cx<'_, ProjectFeature>) -> Self {
        let project = &cx.model.workspace.content.project;
        let dirty_documents =
            dirty_document_count(&lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers));
        let netlist_first = cx.model.is_netlist_first_without_schematic();
        let source_deck = source_deck_snapshot(ctx, cx.model);
        let configuration = configuration_snapshot(ctx, cx.model);
        let library_count = cx.model.library_manager.library_count();
        let model_library_count = cx.model.model_library_manager.library_count();
        let (cell_count, view_count) = derived(
            ctx,
            "overview.catalog counts",
            cx.model.library_manager.catalog().stamp(),
            || {
                (
                    cx.model.library_manager.total_cell_count(),
                    cx.model.library_manager.total_view_count(),
                )
            },
        );
        // A netlist-first project keeps its bootstrap cell pristine: it is
        // not part of the design the reader made, so it is not counted.
        let (library_count, cell_count, view_count) = if netlist_first {
            bootstrap_excluded(cx.model, library_count, cell_count, view_count)
        } else {
            (library_count, cell_count, view_count)
        };
        let root_reference = CellViewRef::new(
            &project.root_library,
            &project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let root_schematic = if cx.model.workspace.content.active_view == root_reference {
            Some(cx.model.schematic.editor_ref().design)
        } else {
            cx.model
                .workspace
                .content
                .schematic_buffers
                .get(&root_reference.key())
        };
        let root_sheet_count = cx
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog(&root_reference.key())
            .map_or_else(
                || usize::from(root_schematic.is_some()),
                |catalog| catalog.sheets().len(),
            );
        let root_instance_count =
            root_schematic.map_or(0, |schematic| schematic.document().components.len());
        let root_net_count = root_schematic.map_or(0, |schematic| {
            derived(ctx, "overview.root nets", schematic.content_stamp(), || {
                rspice_design::connectivity::summary::design_nets(schematic).len()
            })
        });

        let location = health::location(cx.model, cx.platform);
        let statuses = status_rows(ctx, cx, netlist_first, dirty_documents, location.is_some());
        let problem = if netlist_first {
            source_deck.as_ref().map_or_else(
                || {
                    Some(ProblemSnapshot {
                        code: "SOURCE".to_owned(),
                        message: "The project's source document is unavailable".to_owned(),
                        path: "Project source".to_owned(),
                        tone: Tone::Error,
                    })
                },
                |deck| deck.problem.clone(),
            )
        } else {
            problem_snapshot(cx)
        };

        let open_documents = cx
            .model
            .workspace
            .content
            .open_views
            .iter()
            .map(|document| OpenDocumentSnapshot {
                reference: document.reference.clone(),
                view_type: document.view_type,
                active: document.reference == cx.model.workspace.content.active_view,
                dirty: document.dirty,
            })
            .collect();

        let now_ms =
            u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX);
        let retained = derived(
            ctx,
            "overview.operations",
            OperationsKey::of(cx.model),
            || Arc::new(retained_operations(cx.model)),
        );
        let operations = retained
            .newest
            .iter()
            .map(|operation| operation.at(now_ms))
            .collect();
        let operation_total = retained.total;

        Self {
            project_name: project.display_name().to_owned(),
            project_revision: project.revision().get(),
            location,
            descriptor_root: format!("{}/{}", project.root_library, project.top_cell),
            simulation_root: configuration.root.clone(),
            technology: project.technology_binding().map_or_else(
                || "Not attached".to_owned(),
                |binding| binding.display_label(),
            ),
            technology_attached: project.technology_binding().is_some(),
            library_count,
            model_library_count,
            cell_count,
            view_count,
            root_sheet_count,
            root_instance_count,
            root_net_count,
            netlist_first,
            source_deck,
            configuration,
            statuses,
            problem,
            open_documents,
            operations,
            operation_total,
        }
    }
}

/// The status rows, one per subject, each with the action that addresses it.
fn status_rows(
    ctx: &egui::Context,
    cx: &Cx<'_, ProjectFeature>,
    netlist_first: bool,
    dirty_documents: usize,
    saved: bool,
) -> Vec<StatusSnapshot> {
    let model = &*cx.model;
    let facts = cx.runs.slot_facts(cx.peers.design().review_markers());
    let (subject, design) = health::design(model, facts);
    let design = match subject {
        DesignSubject::SourceDeck => {
            let command = ids::open_workspace(Workspace::Netlist);
            StatusSnapshot::of(
                subject.label(),
                design,
                "Open deck",
                OverviewIntent::Command(command),
                command_gate(cx, command),
            )
        }
        DesignSubject::SchematicChecks => {
            let enabled = cx.platform.project_lifecycle.is_open();
            StatusSnapshot::of(
                subject.label(),
                design,
                "Run checks",
                OverviewIntent::CheckSimulationRoot,
                (enabled, (!enabled).then_some("no project is open")),
            )
        }
    };

    let latest_run = match model.simulation.retained.runs.first() {
        None => {
            let command = ids::open_workspace(run_destination(false, netlist_first));
            StatusSnapshot::of(
                "Latest run",
                health::latest_run(model),
                if netlist_first {
                    "Open deck"
                } else {
                    "Open analyses"
                },
                OverviewIntent::Command(command),
                command_gate(cx, command),
            )
        }
        Some(run) => {
            let has_dataset = !run.analyses.is_empty();
            let destination = run_destination(has_dataset, netlist_first);
            StatusSnapshot::of(
                "Latest run",
                health::latest_run(model),
                match (has_dataset, netlist_first) {
                    (true, _) => "Open results",
                    (false, true) => "Open deck",
                    (false, false) => "Open analyses",
                },
                OverviewIntent::OpenRun(0),
                command_gate(cx, ids::open_workspace(destination)),
            )
        }
    };

    let verify = ids::open_workspace(Workspace::Verify);
    let dependencies = ProjectCommand::Page(ProjectPage::Dependencies).key();
    let recovery = ProjectCommand::Page(ProjectPage::Recovery).key();
    vec![
        design,
        StatusSnapshot::of(
            "Configuration",
            health::configuration(ctx, model),
            "Manage",
            OverviewIntent::Command(ids::CONFIGURATION_SETS),
            command_gate(cx, ids::CONFIGURATION_SETS),
        ),
        latest_run,
        StatusSnapshot::of(
            "Verification",
            health::verification(model),
            "Open Verify",
            OverviewIntent::Command(verify),
            command_gate(cx, verify),
        ),
        StatusSnapshot::of(
            "Model sources",
            health::model_sources(ctx, model),
            "Inspect",
            OverviewIntent::OpenProjectModels,
            command_gate(cx, ids::MODELS_CATALOG),
        ),
        StatusSnapshot::of(
            "Dependencies",
            health::dependencies(ctx, model),
            "Open dependencies",
            OverviewIntent::Command(dependencies),
            command_gate(cx, dependencies),
        ),
        StatusSnapshot::of(
            "Unsaved work",
            health::unsaved_work(
                dirty_documents,
                saved,
                &cx.state.project_checkpoint_recovery,
            ),
            "Open recovery",
            OverviewIntent::Command(recovery),
            command_gate(cx, recovery),
        ),
    ]
}

/// The catalogue's counts without the bootstrap cell a netlist-first project
/// leaves pristine, nor its library when it holds nothing else.
fn bootstrap_excluded(
    model: &Model,
    libraries: usize,
    cells: usize,
    views: usize,
) -> (usize, usize, usize) {
    let project = &model.workspace.content.project;
    let Some(library) = model.library_manager.get_library(&project.root_library) else {
        return (libraries, cells, views);
    };
    let Some(cell) = library.get_cell(&project.top_cell) else {
        return (libraries, cells, views);
    };
    let only_cell = library.cell_count() == 1;
    (
        libraries.saturating_sub(usize::from(only_cell)),
        cells.saturating_sub(1),
        views.saturating_sub(cell.view_count()),
    )
}

/// One retained operation, with the time it happened rather than a phrase
/// about it: how long ago it was is said again on every frame.
#[derive(Debug, Clone)]
struct RetainedOperation {
    timestamp_ms: Option<u64>,
    event: String,
    detail: String,
    tone: Tone,
    intent: Option<OverviewIntent>,
}

impl RetainedOperation {
    fn at(&self, now_ms: u64) -> OperationSnapshot {
        OperationSnapshot {
            when: self.timestamp_ms.map_or_else(
                || "time n/a".to_owned(),
                |value| relative_time(value, now_ms),
            ),
            event: self.event.clone(),
            detail: self.detail.clone(),
            tone: self.tone,
            intent: self.intent.clone(),
        }
    }
}

/// The activity register: the newest operations, and how many the project
/// retains.
#[derive(Debug, Clone, Default)]
struct RetainedOperations {
    newest: Vec<RetainedOperation>,
    total: usize,
}

/// What the activity register is read from, by identity: the run history,
/// every schematic (whose validated revisions it lists) and the project's
/// publications.
#[derive(Clone, PartialEq)]
struct OperationsKey {
    runs: crate::state::RunHistoryRevision,
    schematics: (usize, u64, u64),
    /// The newest library publication's receipt digest. The receipts are
    /// hash-chained, so it names every publication before it as well.
    publications: Option<crate::product::ContentDigest>,
}

impl OperationsKey {
    fn of(model: &Model) -> Self {
        Self {
            runs: model.simulation.retained.runs.revision(),
            schematics: schematic_stamps(model),
            publications: model
                .workspace
                .content
                .project
                .library_publications()
                .last()
                .map(|receipt| receipt.receipt_digest()),
        }
    }
}

/// The content stamps of the open editor's schematic and of every buffered
/// one, folded without regard to the buffers' order: an edit to any of them,
/// or a buffer added or dropped, moves the fold.
fn schematic_stamps(model: &Model) -> (usize, u64, u64) {
    let mut folded_xor = 0_u64;
    let mut folded_sum = 0_u64;
    let editor = std::iter::once((None, model.schematic.editor_ref().design.content_stamp()));
    let buffers = model
        .workspace
        .content
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (Some(key), schematic.content_stamp()));
    for (key, stamp) in editor.chain(buffers) {
        let mut entry = DefaultHasher::new();
        key.hash(&mut entry);
        stamp.hash(&mut entry);
        let entry = entry.finish();
        folded_xor ^= entry;
        folded_sum = folded_sum.wrapping_add(entry.rotate_left(17));
    }
    (
        model.workspace.content.schematic_buffers.len(),
        folded_xor,
        folded_sum,
    )
}

/// The runs, validated revisions and library publications the project
/// retains, newest first.
fn retained_operations(model: &Model) -> RetainedOperations {
    let mut operations = model
        .simulation
        .retained
        .runs
        .iter()
        .enumerate()
        .take(8)
        .map(|(index, run)| {
            let measurement_count = run
                .analyses
                .iter()
                .map(|analysis| analysis.measurements.len())
                .sum();
            let timestamp_ms = unix_seconds_to_millis(run.timestamp);
            (
                timestamp_ms.unwrap_or_default(),
                RetainedOperation {
                    timestamp_ms,
                    event: format!(
                        "{} · {}",
                        run_title(run.id, &run.label),
                        lifecycle_label(run.lifecycle, run.success)
                    ),
                    detail: format!(
                        "{} {} · {measurement_count} {} · {} · {}",
                        run.analyses.len(),
                        singular_or_plural(run.analyses.len(), "analysis", "analyses"),
                        singular_or_plural(measurement_count, "measurement", "measurements"),
                        run.execution_target
                            .map_or("engine not recorded", |target| target.label()),
                        crate::quantity::format_elapsed(run.elapsed_time),
                    ),
                    tone: lifecycle_tone(run.lifecycle, run.success),
                    intent: Some(OverviewIntent::OpenRun(index)),
                },
            )
        })
        .collect::<Vec<_>>();
    let mut retained_revision_ids = BTreeSet::new();
    for schematic in std::iter::once(model.schematic.editor_ref().design)
        .chain(model.workspace.content.schematic_buffers.values())
    {
        for revision in schematic
            .document()
            .validated_revisions
            .records()
            .iter()
            .rev()
            .take(8)
        {
            if !retained_revision_ids.insert(revision.id().as_uuid().to_string()) {
                continue;
            }
            let findings = revision.finding_counts();
            let tone = if findings.blockers > 0 {
                Tone::Error
            } else if findings.advisories > 0 {
                Tone::Warn
            } else {
                Tone::Neutral
            };
            operations.push((
                revision.created_unix_ms(),
                RetainedOperation {
                    timestamp_ms: Some(revision.created_unix_ms()),
                    event: format!("{} validated", revision.view_identity()),
                    detail: format!(
                        "{} · project revision {} · {}",
                        revision.revision_note(),
                        revision.project_revision(),
                        revision.author()
                    ),
                    tone,
                    intent: Some(OverviewIntent::Command(
                        ProjectCommand::RevisionHistory.key(),
                    )),
                },
            ));
        }
    }
    // Published library snapshots are the third class of retained project
    // operation that carries its own creation time. The hash-chained library
    // *mutation* receipts beside them are deliberately absent: they record a
    // sequence and no timestamp, so this time-ordered register could only
    // show them by inventing a `when` for them.
    for publication in model
        .workspace
        .content
        .project
        .library_publications()
        .iter()
        .rev()
        .take(8)
    {
        // A retained receipt cannot carry a zero creation time; the draft
        // validation rejects one before the receipt is ever chained.
        let created_unix_ms = publication.created_unix_ms();
        operations.push((
            created_unix_ms,
            RetainedOperation {
                timestamp_ms: Some(created_unix_ms),
                event: format!("Library published · {}", publication.label()),
                detail: format!(
                    "#{:04} · library revision {} · {} · {}",
                    publication.sequence(),
                    publication.library_revision(),
                    publication.reason(),
                    publication.actor_id()
                ),
                tone: Tone::Neutral,
                intent: Some(OverviewIntent::Command(
                    ProjectCommand::Page(ProjectPage::Library).key(),
                )),
            },
        ));
    }
    operations.sort_by(|left, right| right.0.cmp(&left.0));
    // Counted from the retained records themselves, not from what the loops
    // above collected: each of those is capped, so reporting their length
    // would describe the cap rather than the project.
    let retained_revision_total = std::iter::once(model.schematic.editor_ref().design)
        .chain(model.workspace.content.schematic_buffers.values())
        .flat_map(|schematic| schematic.document().validated_revisions.records().iter())
        .map(|revision| revision.id().as_uuid())
        .collect::<BTreeSet<_>>()
        .len();
    RetainedOperations {
        newest: operations
            .into_iter()
            .take(5)
            .map(|(_, operation)| operation)
            .collect(),
        total: model.simulation.retained.runs.len()
            + retained_revision_total
            + model.workspace.content.project.library_publications().len(),
    }
}

/// The one finding of the cell view runs start from that the status rows'
/// strip names, or nothing. Checks not run yet, or out of date, are the
/// Schematic checks row's to say: the strip names findings.
pub fn problem_snapshot(cx: &Cx<'_, ProjectFeature>) -> Option<ProblemSnapshot> {
    let result = match cx
        .model
        .simulation_root_design_check_status(cx.runs.slot_facts(cx.peers.design().review_markers()))
    {
        DesignCheckStatus::Current(receipt) => &receipt.result,
        DesignCheckStatus::NotRun | DesignCheckStatus::Stale(_) => return None,
        DesignCheckStatus::Unavailable { reason, .. } => {
            return Some(ProblemSnapshot {
                code: "UNAVAILABLE".to_owned(),
                message: reason,
                path: cx
                    .model
                    .workspace
                    .content
                    .simulation_root_reference()
                    .display_path(),
                tone: Tone::Error,
            });
        }
    };
    let highest_priority = result
        .violations()
        .iter()
        .max_by_key(|violation| drc_severity_priority(violation.severity));
    // Nothing when the root checks clean: "No current schematic findings" is
    // the absence of a finding dressed as one.
    highest_priority.map(|violation| ProblemSnapshot {
        code: format!("CHK-{:03}", violation.id),
        message: violation.message.clone(),
        path: violation.location.display(),
        tone: match violation.severity {
            DrcSeverity::Critical | DrcSeverity::Error => Tone::Error,
            DrcSeverity::Warning => Tone::Warn,
            DrcSeverity::Info => Tone::Neutral,
        },
    })
}

const fn drc_severity_priority(severity: DrcSeverity) -> u8 {
    match severity {
        DrcSeverity::Critical => 4,
        DrcSeverity::Error => 3,
        DrcSeverity::Warning => 2,
        DrcSeverity::Info => 1,
    }
}

/// The configuration runs use, as the Configuration row states it, with the
/// root and the design under test the design register names.
fn configuration_snapshot(ctx: &egui::Context, model: &Model) -> ConfigurationSnapshot {
    let status = health::configuration(ctx, model);
    let active = model.workspace.content.configuration_sets.active();
    ConfigurationSnapshot {
        configured: active.is_some(),
        state: status.state,
        detail: status.detail,
        // With no configuration used for runs the project runs Default: its
        // own top cell through the built-in view list, with no DUT instance.
        root: model
            .workspace
            .content
            .simulation_root_reference()
            .display_path(),
        dut: active.map_or_else(
            || "\u{2014}".to_owned(),
            |configuration| configuration.dut_path().to_owned(),
        ),
        tone: status.tone,
    }
}
