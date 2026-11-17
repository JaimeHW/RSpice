//! The Overview's snapshot: what the page shows, read once per frame from the
//! project, its lifecycle, the checks, the run history and the model catalog
//! before anything is drawn.
//!
//! # Derived on change, not on a frame
//!
//! What has to walk the design or the catalogues to be known — the net count,
//! the view count, the activity register, the model sources, the deck's line
//! count, the configuration's validity — is derived once and kept in the
//! frame context beside the identity of what it was derived from: a content
//! stamp, a catalogue revision, the run history's revision, a digest. A frame
//! in which none of those moved derives nothing, so the page costs the same on
//! a project of eight thousand instances as on an empty one.

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
struct ModelSnapshot {
    state: String,
    detail: String,
    tone: Tone,
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
    pub(super) validation_state: String,
    validation_detail: String,
    pub(super) validation_tone: Tone,
    /// The one finding the strip under the status register names, or nothing
    /// when there is no finding: a strip reading "no current diagnostics" is
    /// a report of the absence of a problem.
    problem: Option<ProblemSnapshot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverviewSnapshot {
    pub(super) project_name: String,
    pub(super) project_id: String,
    pub(super) project_revision: u64,
    pub(super) schema_version: u16,
    pub(crate) path: String,
    pub(super) persistence_bound: bool,
    pub descriptor_root: String,
    pub(super) simulation_root: String,
    pub(super) technology: String,
    pub(super) technology_attached: bool,
    pub(super) dirty_documents: usize,
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
    /// Every retained operation the project holds, counted from the records
    /// rather than from the truncated list above: reporting the shown count as
    /// the retained count would claim a project with thirty runs has five.
    pub operation_total: usize,
    pub(super) run_plan_name: String,
    pub(super) execution_target: String,
}

fn source_deck_snapshot(ctx: &egui::Context, model: &Model) -> Option<SourceDeckSnapshot> {
    let document = model.workspace.content.netlist_document.as_ref()?;
    let name = document
        .provenance()
        .imported()
        .map(|imported| imported.origin().display_name().to_owned())
        .or_else(|| {
            document
                .save_acknowledgement()
                .map(|saved| saved.origin().display_name().to_owned())
        })
        .or_else(|| {
            model
                .workspace
                .content
                .netlist_source_path
                .as_ref()
                .and_then(|path| path.file_name())
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "Untitled SPICE deck".to_owned());
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
    let dependency_count = document.dependencies().len();
    let dependencies_sealed = document.dependency_graph_is_sealed();
    let validation = document.validation();
    let validation_current =
        validation.is_some_and(|report| report.content_digest() == document.content_digest());

    let (validation_state, validation_detail, validation_tone, problem) = match validation {
        None => (
            "Not validated".to_owned(),
            "Validate the retained source before execution".to_owned(),
            Tone::Warn,
            Some(ProblemSnapshot {
                code: "NOT RUN".to_owned(),
                message: "Validate the project source deck".to_owned(),
                path: name.clone(),
                tone: Tone::Warn,
            }),
        ),
        Some(_) if !validation_current => (
            "Stale".to_owned(),
            "The source changed after the retained validation".to_owned(),
            Tone::Warn,
            Some(ProblemSnapshot {
                code: "STALE".to_owned(),
                message: "Source changed after the retained validation".to_owned(),
                path: name.clone(),
                tone: Tone::Warn,
            }),
        ),
        Some(report) => {
            let errors = report.error_count();
            let warnings = report.warning_count();
            let info = report.diagnostics().len().saturating_sub(errors + warnings);
            let tone = if errors > 0 {
                Tone::Error
            } else if warnings > 0 {
                Tone::Warn
            } else {
                Tone::Neutral
            };
            let state_label = if errors > 0 {
                format!(
                    "{errors} blocking {}",
                    singular_or_plural(errors, "diagnostic", "diagnostics")
                )
            } else {
                "Validated".to_owned()
            };
            let detail = format!(
                "{warnings} {} · {info} information",
                singular_or_plural(warnings, "advisory", "advisories")
            );
            let highest = report
                .diagnostics()
                .iter()
                .max_by_key(|diagnostic| diagnostic.severity());
            // Nothing at all when the deck validates: "No current source
            // diagnostics" was a strip the page painted to report an absence.
            let problem = highest.map(|diagnostic| {
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
            (state_label, detail, tone, problem)
        }
    };

    Some(SourceDeckSnapshot {
        name,
        ownership,
        revision: document.revision().get(),
        line_count,
        dependency_count,
        dependencies_sealed,
        validation_state,
        validation_detail,
        validation_tone,
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
        let configuration = configuration_snapshot(ctx, cx);
        let checks = checks_snapshot(cx);
        let models = model_snapshot(ctx, cx);
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

        let latest_run_status = cx.model.simulation.retained.runs.first().map_or_else(
            || {
                let command = ids::open_workspace(run_destination(false, netlist_first));
                let (enabled, disabled_reason) = command_gate(cx, command);
                StatusSnapshot {
                    area: "Latest run",
                    state: "No retained run".to_owned(),
                    detail: if netlist_first {
                        "Run the saved deck to produce results".to_owned()
                    } else {
                        "Run the active plan to produce results".to_owned()
                    },
                    action: if netlist_first {
                        "Open deck"
                    } else {
                        "Open analyses"
                    },
                    tone: Tone::Neutral,
                    intent: OverviewIntent::Command(command),
                    enabled,
                    disabled_reason,
                }
            },
            |run| {
                let measurement_count: usize = run
                    .analyses
                    .iter()
                    .map(|analysis| analysis.measurements.len())
                    .sum();
                let has_dataset = !run.analyses.is_empty();
                let destination = run_destination(has_dataset, netlist_first);
                let (enabled, disabled_reason) = command_gate(cx, ids::open_workspace(destination));
                StatusSnapshot {
                    area: "Latest run",
                    state: format!(
                        "Run {} · {}",
                        run.id,
                        lifecycle_label(run.lifecycle, run.success)
                    ),
                    detail: format!(
                        "{} {} · {measurement_count} {} · {}",
                        run.analyses.len(),
                        singular_or_plural(run.analyses.len(), "analysis", "analyses"),
                        singular_or_plural(measurement_count, "measurement", "measurements"),
                        run.execution_target.map_or_else(
                            || "execution target not retained".to_owned(),
                            |target| target.label().to_owned(),
                        )
                    ),
                    action: match (has_dataset, netlist_first) {
                        (true, _) => "Open results",
                        (false, true) => "Open deck",
                        (false, false) => "Open analyses",
                    },
                    tone: lifecycle_tone(run.lifecycle, run.success),
                    intent: OverviewIntent::OpenRun(0),
                    enabled,
                    disabled_reason,
                }
            },
        );

        let (configuration_enabled, configuration_disabled_reason) =
            command_gate(cx, ids::CONFIGURATION_SETS);
        let (models_enabled, models_disabled_reason) = command_gate(cx, ids::MODELS_CATALOG);
        let primary_status = if netlist_first {
            let (enabled, disabled_reason) =
                command_gate(cx, ids::open_workspace(Workspace::Netlist));
            source_deck.as_ref().map_or_else(
                || StatusSnapshot {
                    area: "Source deck",
                    state: "Unavailable".to_owned(),
                    detail: "The project has no saved source document".to_owned(),
                    action: "Open deck",
                    tone: Tone::Error,
                    intent: OverviewIntent::Command(ids::open_workspace(Workspace::Netlist)),
                    enabled,
                    disabled_reason,
                },
                |deck| StatusSnapshot {
                    area: "Source deck",
                    state: deck.validation_state.clone(),
                    detail: deck.validation_detail.clone(),
                    action: "Open deck",
                    tone: deck.validation_tone,
                    intent: OverviewIntent::Command(ids::open_workspace(Workspace::Netlist)),
                    enabled,
                    disabled_reason,
                },
            )
        } else {
            let enabled = cx.platform.project_lifecycle.is_open();
            StatusSnapshot {
                area: "Schematic checks",
                state: checks.state,
                detail: checks.detail,
                action: "Run checks",
                tone: checks.tone,
                intent: OverviewIntent::CheckProjectRoot,
                enabled,
                disabled_reason: (!enabled).then_some("no project is open"),
            }
        };
        let statuses = vec![
            primary_status,
            StatusSnapshot {
                area: "Configuration",
                state: configuration.state.clone(),
                detail: configuration.detail.clone(),
                action: "Manage",
                tone: configuration.tone,
                intent: OverviewIntent::Command(ids::CONFIGURATION_SETS),
                enabled: configuration_enabled,
                disabled_reason: configuration_disabled_reason,
            },
            latest_run_status,
            StatusSnapshot {
                area: "Model closure",
                state: models.state,
                detail: models.detail,
                action: "Inspect",
                tone: models.tone,
                intent: OverviewIntent::OpenProjectModels,
                enabled: models_enabled,
                disabled_reason: models_disabled_reason,
            },
        ];
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
            project_id: project.id().to_string(),
            project_revision: project.revision().get(),
            schema_version: project.schema_version(),
            path: project_persistence_location(cx),
            persistence_bound: cx.platform.project_lifecycle.binding().is_some(),
            descriptor_root: format!("{}/{}", project.root_library, project.top_cell),
            simulation_root: configuration.root.clone(),
            technology: project.technology_binding().map_or_else(
                || "Not attached".to_owned(),
                |binding| binding.display_label(),
            ),
            technology_attached: project.technology_binding().is_some(),
            dirty_documents,
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
            run_plan_name: cx.model.sim_setup.active_plan_name().as_str().to_owned(),
            execution_target: crate::state::ExecutionTarget::current().label().to_owned(),
        }
    }
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
                        run.execution_target.map_or_else(
                            || "execution target not retained".to_owned(),
                            |target| target.label().to_owned(),
                        ),
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

pub(super) struct HealthCopy {
    pub(super) state: String,
    pub(super) detail: String,
    pub(super) tone: Tone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DrcCounts {
    pub(super) critical: usize,
    pub(super) errors: usize,
    pub(super) warnings: usize,
    pub(super) info: usize,
}

fn project_persistence_location(cx: &Cx<'_, ProjectFeature>) -> String {
    let fallback = || {
        cx.model
            .workspace
            .content
            .project
            .path
            .as_ref()
            .map_or_else(
                || "Not yet saved".to_owned(),
                |path| path.display().to_string(),
            )
    };
    let Some(binding) = cx.platform.project_lifecycle.binding() else {
        return fallback();
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        binding.canonical_path.display().to_string()
    }
    #[cfg(target_arch = "wasm32")]
    {
        match binding {
            crate::workbench::lifecycle::project_lifecycle::PersistenceBinding::Browser {
                binding,
                ..
            } => {
                if binding.persisted_generation == Some(binding.receipt.accepted_generation) {
                    binding.display_name.clone()
                } else {
                    format!("{} · session-only browser binding", binding.display_name)
                }
            }
        }
    }
}

fn checks_snapshot(cx: &Cx<'_, ProjectFeature>) -> HealthCopy {
    match cx
        .model
        .simulation_root_design_check_status(cx.runs.slot_facts(cx.peers.design().review_markers()))
    {
        DesignCheckStatus::Current(receipt) => {
            let summary = receipt.result.summary();
            drc_copy(
                true,
                Some(DrcCounts {
                    critical: summary.critical,
                    errors: summary.errors,
                    warnings: summary.warnings,
                    info: summary.info,
                }),
            )
        }
        DesignCheckStatus::NotRun => HealthCopy {
            state: "Not run".to_owned(),
            detail: "Run checks for the project-root schematic".to_owned(),
            tone: Tone::Warn,
        },
        DesignCheckStatus::Stale(_) => HealthCopy {
            state: "Stale".to_owned(),
            detail: "Project-root inputs changed after the retained check".to_owned(),
            tone: Tone::Warn,
        },
        DesignCheckStatus::Unavailable { reason, .. } => HealthCopy {
            state: "Unavailable".to_owned(),
            detail: reason,
            tone: Tone::Error,
        },
    }
}

/// The one project-root finding the status card's strip names, or nothing.
pub fn problem_snapshot(cx: &Cx<'_, ProjectFeature>) -> Option<ProblemSnapshot> {
    let root_path = format!(
        "{}/{}",
        cx.model.workspace.content.project.root_library,
        cx.model.workspace.content.project.top_cell
    );
    let result = match cx
        .model
        .simulation_root_design_check_status(cx.runs.slot_facts(cx.peers.design().review_markers()))
    {
        DesignCheckStatus::Current(receipt) => &receipt.result,
        DesignCheckStatus::NotRun => {
            return Some(ProblemSnapshot {
                code: "NOT RUN".to_owned(),
                message: "Run schematic checks for the project root".to_owned(),
                path: root_path,
                tone: Tone::Warn,
            });
        }
        DesignCheckStatus::Stale(_) => {
            return Some(ProblemSnapshot {
                code: "STALE".to_owned(),
                message: "Project-root inputs changed after the retained check".to_owned(),
                path: root_path,
                tone: Tone::Warn,
            });
        }
        DesignCheckStatus::Unavailable { reason, .. } => {
            return Some(ProblemSnapshot {
                code: "UNAVAILABLE".to_owned(),
                message: reason,
                path: root_path,
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

pub(super) fn drc_copy(current: bool, summary: Option<DrcCounts>) -> HealthCopy {
    if !current {
        return HealthCopy {
            state: "Not current".to_owned(),
            detail: "No check result describes the active schematic topology".to_owned(),
            tone: Tone::Warn,
        };
    }
    let Some(summary) = summary else {
        return HealthCopy {
            state: "Not run".to_owned(),
            detail: "No retained schematic-check result".to_owned(),
            tone: Tone::Warn,
        };
    };
    let blockers = summary.critical + summary.errors;
    if blockers > 0 {
        HealthCopy {
            state: format!(
                "{blockers} blocking {}",
                singular_or_plural(blockers, "finding", "findings")
            ),
            detail: format!(
                "{} critical · {} errors · {} advisories",
                summary.critical,
                summary.errors,
                summary.warnings + summary.info
            ),
            tone: Tone::Error,
        }
    } else {
        HealthCopy {
            state: "Passed".to_owned(),
            detail: format!(
                "{} {}",
                summary.warnings + summary.info,
                singular_or_plural(summary.warnings + summary.info, "advisory", "advisories")
            ),
            tone: Tone::Neutral,
        }
    }
}

fn configuration_snapshot(
    ctx: &egui::Context,
    cx: &Cx<'_, ProjectFeature>,
) -> ConfigurationSnapshot {
    let catalog = &cx.model.workspace.content.configuration_sets;
    // With no configuration used for runs the project runs Default: its own
    // top cell through the built-in view list, with no DUT instance.
    let root = cx
        .model
        .workspace
        .content
        .simulation_root_reference()
        .display_path();
    // Validating proves every configuration's digest, so it is done once per
    // edit of the catalogue rather than on every frame.
    let validity = derived(
        ctx,
        "overview.configuration validity",
        catalog.stamp(),
        || catalog.validate().map_err(|error| error.to_string()),
    );
    if let Err(error) = validity {
        let active = catalog.active();
        return ConfigurationSnapshot {
            configured: active.is_some(),
            state: "Unreadable configurations".to_owned(),
            detail: error,
            root,
            dut: active.map_or_else(
                || "\u{2014}".to_owned(),
                |configuration| configuration.dut_path().to_owned(),
            ),
            tone: Tone::Error,
        };
    }
    let Some(active) = catalog.active() else {
        return ConfigurationSnapshot {
            configured: false,
            state: "Default".to_owned(),
            detail: "Runs use the built-in view list".to_owned(),
            root,
            dut: "\u{2014}".to_owned(),
            tone: Tone::Neutral,
        };
    };
    let projection = cx.model.workspace.configuration_execution_projection(
        &cx.model.library_manager,
        &cx.model.workspace.content.active_view,
        &cx.model.schematic,
    );
    let (detail, tone) = match projection {
        Ok(projection) => (
            format!(
                "{} instances bound · {} instance bindings",
                projection.plan().bindings().len(),
                active.overrides().len(),
            ),
            Tone::Neutral,
        ),
        Err(error) => (format!("Runs are blocked: {error}"), Tone::Error),
    };
    ConfigurationSnapshot {
        configured: true,
        state: active.name().to_owned(),
        detail,
        root: active.root().display_path(),
        dut: active.dut_path().to_owned(),
        tone,
    }
}

/// What the model catalogue holds and whether its sources can execute, as
/// the status row states it.
#[derive(Debug, Clone)]
struct CatalogFacts {
    library_count: usize,
    model_count: usize,
    source_count: usize,
    source_contract_error: Option<String>,
}

fn catalog_facts(manager: &crate::state::model_library::ModelLibraryManager) -> CatalogFacts {
    let libraries = manager.libraries_sorted();
    let mut sources = BTreeSet::new();
    for library in &libraries {
        for source in &library.source_closure {
            sources.insert(source.path.clone());
        }
    }
    let source_contract_error = libraries.into_iter().find_map(|library| {
        if !library.source_authority.has_execution_source() {
            return None;
        }
        if library.root_path.is_none() {
            return Some(format!(
                "Model library '{}' has no root source file",
                library.name
            ));
        }
        if library.source_closure.is_empty() {
            return Some(format!(
                "Model library '{}' has no imported source files",
                library.name
            ));
        }
        if library.source_authority.uses_retained_bytes()
            && library.source_closure.iter().any(|source| {
                !library
                    .source_contents
                    .iter()
                    .any(|content| content.path == source.path)
            })
        {
            return Some(format!(
                "Model library '{}' does not retain every pinned source",
                library.name
            ));
        }
        None
    });
    CatalogFacts {
        library_count: manager.library_count(),
        model_count: manager.total_model_count(),
        source_count: sources.len(),
        source_contract_error,
    }
}

fn model_snapshot(ctx: &egui::Context, cx: &Cx<'_, ProjectFeature>) -> ModelSnapshot {
    let manager = &cx.model.model_library_manager;
    let CatalogFacts {
        library_count,
        model_count,
        source_count,
        source_contract_error,
    } = derived(
        ctx,
        "overview.model catalog",
        manager.catalog_stamp(),
        || catalog_facts(manager),
    );
    let scan_errors = &cx.model.pdk_config.scan_errors;
    let binding = cx.model.workspace.content.project.technology_binding();
    let contract_error = binding.and_then(|_| cx.model.project_technology_verdict().err());

    let copy = if let Some(error) = contract_error {
        HealthCopy {
            state: "Contract stale".to_owned(),
            detail: error,
            tone: Tone::Error,
        }
    } else if let Some(error) = source_contract_error {
        HealthCopy {
            state: "Closure blocked".to_owned(),
            detail: error,
            tone: Tone::Error,
        }
    } else if let Some(first) = scan_errors.first() {
        HealthCopy {
            state: format!(
                "{} scan {}",
                scan_errors.len(),
                singular_or_plural(scan_errors.len(), "diagnostic", "diagnostics")
            ),
            detail: first.clone(),
            tone: Tone::Warn,
        }
    } else if library_count == 0 {
        HealthCopy {
            state: "No model libraries".to_owned(),
            detail: "No loaded executable model catalog".to_owned(),
            tone: Tone::Warn,
        }
    } else if binding.is_none() {
        HealthCopy {
            state: "Technology not attached".to_owned(),
            detail: format!(
                "{library_count} {} · {model_count} models · {source_count} source files",
                singular_or_plural(library_count, "library", "libraries")
            ),
            tone: Tone::Neutral,
        }
    } else {
        HealthCopy {
            state: "Contract current".to_owned(),
            detail: format!(
                "{library_count} {} · {model_count} models · {source_count} source files",
                singular_or_plural(library_count, "library", "libraries")
            ),
            tone: Tone::Neutral,
        }
    };

    ModelSnapshot {
        state: copy.state,
        detail: copy.detail,
        tone: copy.tone,
    }
}
