//! The Overview's snapshot: what the page shows, read once per frame from the
//! project, its lifecycle, the checks, the run history, the catalogue and the
//! checkpoints before anything is drawn.
//!
//! # Derived on change, not on a frame
//!
//! What has to walk the design or the catalogues to be known — the cells and
//! their instances, the activity register, the model sources, the deck's line
//! count, the blocking findings — is derived once and kept in the frame
//! context beside the identity of what it was derived from: a content stamp,
//! a catalogue revision, the run history's revision, the diagnostic store's
//! generation. A frame in which none of those moved derives nothing, so the
//! page costs the same on a project of eight thousand instances as on an
//! empty one.
//!
//! Each status row's words and mark are its subject's, from
//! [`crate::workbench::project::health`]; the row adds only the action that
//! addresses it.

use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};
use std::sync::Arc;

use crate::state::netlist_document::DiagnosticSeverity;
use crate::state::{CellViewRef, ViewType};
use crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState;
use crate::workbench::app_state::design_checks::finding_identities;
use crate::workbench::app_state::{DesignCheckStatus, Model};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::lifecycle::project_lifecycle::dirty_document_count;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::health::{self, DesignSubject, SubjectStatus};
use crate::workbench::project::schematic_row_key;
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::project::{ProjectFeature, lifecycle_view};
use crate::workbench::state::Workspace;
use rspice_design::drc::DrcSeverity;

use super::super::derived::derived;
use super::activity::{run_title, unix_seconds_to_millis};
use super::{
    OverviewIntent, Tone, command_gate, lifecycle_label, lifecycle_tone, run_destination,
    singular_or_plural,
};

/// A day, in milliseconds: inside one an age is told, past one a date.
const DAY_MS: u64 = 86_400_000;

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

/// One finding that stops a run, as the strip under the status rows lists
/// it: what is wrong, then where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblemSnapshot {
    pub message: String,
    /// Where it is: an object or a net of the design, a line of the deck.
    pub path: String,
    pub tone: Tone,
    /// Its identity in the Problems list, which the link stands on; `None`
    /// for a deck's diagnostic, which its editor keeps.
    pub finding: Option<uuid::Uuid>,
}

/// One row of the Cells table: a cell of the project's libraries, or the
/// deck a netlist-first project is made of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellRowSnapshot {
    pub name: String,
    pub library: String,
    /// What the cell is to the project: its top, a testbench a configuration
    /// runs, its deck; empty for any other cell.
    pub role: &'static str,
    pub views: String,
    /// The instances on its schematic, where the project holds one.
    pub instances: Option<usize>,
    /// One of its views has unsaved changes.
    pub unsaved: bool,
    /// What opening the row does: its schematic, else the view it has.
    pub intent: OverviewIntent,
}

/// One row of the Activity table: a run, a validated revision, a library
/// publication or a checkpoint, and when it happened.
#[derive(Debug, Clone, PartialEq)]
pub struct OperationSnapshot {
    /// When it happened; `None` where its record keeps no time.
    pub timestamp_ms: Option<u64>,
    /// When it happened, as a date and a time.
    pub(super) stamp: String,
    pub event: String,
    /// How long a run took, in seconds.
    pub elapsed: Option<f64>,
    /// The rest, least needed last: a run's engine ends it, so a narrow
    /// table gives that up before anything else.
    pub detail: String,
    pub(crate) tone: Tone,
    pub intent: Option<OverviewIntent>,
}

impl OperationSnapshot {
    /// A record's time, and its date and time as the pointer reads them,
    /// written once when the register is derived.
    pub(super) fn dated(timestamp_ms: Option<u64>) -> (Option<u64>, String) {
        (
            timestamp_ms,
            timestamp_ms.map_or_else(String::new, crate::time_compat::utc_stamp),
        )
    }

    /// When it happened as the table says it, and in full for the pointer:
    /// how long ago inside a day, the date and time past one. `None` where
    /// its record keeps no time.
    pub fn when(&self, now_ms: u64) -> Option<(String, &str)> {
        let at = self.timestamp_ms?;
        let short = match now_ms.checked_sub(at) {
            Some(ago) if ago < DAY_MS => {
                crate::time_compat::relative_age(std::time::Duration::from_millis(ago))
            }
            _ => self.stamp.clone(),
        };
        Some((short, &self.stamp))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceDeckSnapshot {
    pub(super) name: String,
    pub(super) line_count: usize,
    pub(super) dependency_count: usize,
    pub(super) dependencies_sealed: bool,
    /// The deck's current errors, which the strip under the status rows
    /// names. A deck not validated, or changed since, is its status row's to
    /// say.
    problems: Vec<ProblemSnapshot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverviewSnapshot {
    pub(super) project_name: String,
    pub(super) project_revision: u64,
    /// Where the project file is, or `None` before it is first saved.
    pub(crate) location: Option<String>,
    pub descriptor_root: String,
    pub netlist_first: bool,
    pub statuses: Vec<StatusSnapshot>,
    /// The findings that stop a run, worst first: none while the checks
    /// pass, have not run or are out of date, which their row says.
    pub problems: Vec<ProblemSnapshot>,
    /// The deck a netlist-first project is made of, listed before its cells.
    pub deck: Option<CellRowSnapshot>,
    /// Every cell of the project's libraries: the top cell, the testbenches,
    /// those with unsaved changes, then the rest by library and name.
    pub cells: Arc<[CellRowSnapshot]>,
    /// Every operation the project keeps, newest first.
    pub operations: Arc<[OperationSnapshot]>,
    /// The time the operations' ages are told against.
    pub now_ms: u64,
}

impl OverviewSnapshot {
    /// The Cells table's rows in the order it lists them.
    pub fn cell_rows(&self) -> impl Iterator<Item = &CellRowSnapshot> {
        self.deck.iter().chain(self.cells.iter())
    }

    /// The Cells table's row `index`.
    pub fn cell_row(&self, index: usize) -> Option<&CellRowSnapshot> {
        match (&self.deck, index) {
            (Some(deck), 0) => Some(deck),
            (Some(_), index) => self.cells.get(index - 1),
            (None, index) => self.cells.get(index),
        }
    }

    pub fn cell_row_count(&self) -> usize {
        usize::from(self.deck.is_some()) + self.cells.len()
    }
}

fn source_deck_snapshot(ctx: &egui::Context, model: &Model) -> Option<SourceDeckSnapshot> {
    let document = model.workspace.content.netlist_document.as_ref()?;
    let name = health::deck_name(model)?;
    let line_count = derived(
        ctx,
        "overview.deck lines",
        document.content_digest(),
        || document.source().lines().count(),
    );
    let current = document
        .validation()
        .filter(|report| report.content_digest() == document.content_digest());
    // Its errors only: a warning does not stop a run, and the Source deck
    // row says how the deck validated.
    let problems = current
        .map(|report| {
            report
                .diagnostics()
                .iter()
                .filter(|diagnostic| diagnostic.severity() == DiagnosticSeverity::Error)
                .map(|diagnostic| {
                    let position = diagnostic.position();
                    ProblemSnapshot {
                        message: diagnostic.message().to_owned(),
                        path: format!("{}:{}:{}", name, position.line(), position.column()),
                        tone: Tone::Error,
                        finding: None,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    Some(SourceDeckSnapshot {
        name,
        line_count,
        dependency_count: document.dependencies().len(),
        dependencies_sealed: document.dependency_graph_is_sealed(),
        problems,
    })
}

/// The deck as the first row of a netlist-first project's Cells table.
fn deck_row(deck: &SourceDeckSnapshot) -> CellRowSnapshot {
    CellRowSnapshot {
        name: deck.name.clone(),
        library: String::new(),
        role: "Deck",
        // What it is and how long, then what it includes when it includes
        // anything: how it validated is the Source deck row's to say.
        views: format!(
            "SPICE \u{b7} {} {}{}",
            deck.line_count,
            singular_or_plural(deck.line_count, "line", "lines"),
            match (deck.dependency_count, deck.dependencies_sealed) {
                (0, _) => String::new(),
                (count, sealed) => format!(
                    " \u{b7} {count} {}{}",
                    singular_or_plural(count, "include", "includes"),
                    if sealed { "" } else { ", not all found" }
                ),
            }
        ),
        instances: None,
        unsaved: false,
        intent: OverviewIntent::Command(ids::open_workspace(Workspace::Netlist)),
    }
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

        let location = health::location(cx.model, cx.platform);
        let statuses = status_rows(ctx, cx, netlist_first, dirty_documents, location.is_some());
        let problems = if netlist_first {
            source_deck
                .as_ref()
                .map(|deck| deck.problems.clone())
                .unwrap_or_default()
        } else {
            blocking_findings(ctx, cx)
        };
        let deck = if netlist_first {
            source_deck.as_ref().map(deck_row)
        } else {
            None
        };
        let cells = derived(
            ctx,
            "overview.cells",
            CellsKey::of(cx.model, netlist_first),
            || cell_rows(cx.model, netlist_first),
        );

        let now_ms =
            u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX);
        let recovery = &cx.state.project_checkpoint_recovery;
        let operations = derived(
            ctx,
            "overview.operations",
            OperationsKey::of(cx.model, recovery),
            || retained_operations(cx.model, recovery),
        );

        Self {
            project_name: project.display_name().to_owned(),
            project_revision: project.revision().get(),
            location,
            descriptor_root: format!("{}/{}", project.root_library, project.top_cell),
            netlist_first,
            statuses,
            problems,
            deck,
            cells,
            operations,
            now_ms,
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
            // The Activity table, beside the row, lists this run with its
            // analyses, measurements and engine; the row says how it ended.
            let status = SubjectStatus {
                detail: String::new(),
                ..health::latest_run(model)
            };
            StatusSnapshot::of(
                "Latest run",
                status,
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

/// What the Cells table is read from, by identity: the catalogue, every
/// schematic (whose instances it counts), the configurations (whose roots
/// are the testbenches), the documents with unsaved changes and the top.
#[derive(Clone, PartialEq)]
struct CellsKey {
    catalog: u64,
    schematics: (usize, u64, u64),
    configurations: u64,
    unsaved: u64,
    top: u64,
    netlist_first: bool,
}

impl CellsKey {
    fn of(model: &Model, netlist_first: bool) -> Self {
        let content = &model.workspace.content;
        let mut configurations = DefaultHasher::new();
        for configuration in content.configuration_sets.configurations() {
            configuration.id().hash(&mut configurations);
            configuration.revision().hash(&mut configurations);
        }
        let mut unsaved = DefaultHasher::new();
        for document in content.open_views.iter().filter(|document| document.dirty) {
            document.reference.library.hash(&mut unsaved);
            document.reference.cell.hash(&mut unsaved);
        }
        let mut top = DefaultHasher::new();
        content.project.root_library.hash(&mut top);
        content.project.top_cell.hash(&mut top);
        Self {
            catalog: model.library_manager.catalog().stamp(),
            schematics: schematic_stamps(model),
            configurations: configurations.finish(),
            unsaved: unsaved.finish(),
            top: top.finish(),
            netlist_first,
        }
    }
}

/// Which of a cell's views a row opens: its schematic, else its testbench,
/// else its code, else its symbol, else whatever it has.
const fn opening_rank(view_type: ViewType) -> u8 {
    match view_type {
        ViewType::Schematic => 0,
        ViewType::Testbench => 1,
        ViewType::VerilogA | ViewType::VerilogAms | ViewType::Verilog | ViewType::Spice => 2,
        ViewType::Symbol => 3,
        _ => 4,
    }
}

/// Every cell of the project's libraries, in the order the Cells table lists
/// them: the top cell, the cells a configuration runs as its testbench, the
/// cells with unsaved changes, then the rest by library and name. The cell
/// a netlist-first project keeps pristine is not part of the design the
/// reader made, so it is not listed.
fn cell_rows(model: &Model, netlist_first: bool) -> Arc<[CellRowSnapshot]> {
    let content = &model.workspace.content;
    let project = &content.project;
    let testbenches = content
        .configuration_sets
        .configurations()
        .iter()
        .map(|configuration| {
            (
                configuration.root().library.as_str(),
                configuration.root().cell.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    let unsaved = content
        .open_views
        .iter()
        .filter(|document| document.dirty)
        .map(|document| {
            (
                document.reference.library.as_str(),
                document.reference.cell.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    let components = |reference: &CellViewRef| {
        if content.active_view == *reference {
            Some(
                model
                    .schematic
                    .editor_ref()
                    .design
                    .document()
                    .components
                    .len(),
            )
        } else {
            content
                .schematic_buffers
                .get(&reference.key())
                .map(|schematic| schematic.document().components.len())
        }
    };
    let mut rows = Vec::new();
    for library in model.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            let top = library.name == project.root_library && cell.name == project.top_cell;
            if top && netlist_first {
                continue;
            }
            let named = (library.name.as_str(), cell.name.as_str());
            let testbench = testbenches.contains(&named);
            let unsaved = unsaved.contains(&named);
            let views = cell.views_sorted();
            let reference = |view: &str| CellViewRef::new(&library.name, &cell.name, view);
            let instances = views
                .iter()
                .find(|view| view.view_type == ViewType::Schematic)
                .and_then(|view| components(&reference(&view.name)));
            let intent = views
                .iter()
                .min_by_key(|view| {
                    (
                        opening_rank(view.view_type),
                        view.name != crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
                    )
                })
                .map_or_else(
                    || OverviewIntent::ShowInLibrary {
                        library: library.name.clone(),
                        cell: Some(cell.name.clone()),
                    },
                    |view| OverviewIntent::OpenView(reference(&view.name), view.view_type),
                );
            let rank = if top {
                0
            } else if testbench {
                1
            } else if unsaved {
                2
            } else {
                3
            };
            rows.push((
                rank,
                CellRowSnapshot {
                    name: cell.name.clone(),
                    library: library.display_name().to_owned(),
                    role: if top {
                        "Top"
                    } else if testbench {
                        "Testbench"
                    } else {
                        ""
                    },
                    views: views
                        .iter()
                        .map(|view| view.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    instances,
                    unsaved,
                    intent,
                },
            ));
        }
    }
    // Stable, so each rank keeps the catalogue's library and name order.
    rows.sort_by_key(|(rank, _)| *rank);
    rows.into_iter().map(|(_, row)| row).collect()
}

/// What the activity register is read from, by identity: the run history,
/// every schematic (whose validated revisions it lists), the project's
/// publications and its checkpoints.
#[derive(Clone, PartialEq)]
struct OperationsKey {
    runs: crate::state::RunHistoryRevision,
    schematics: (usize, u64, u64),
    /// The newest library publication's receipt digest. The receipts are
    /// hash-chained, so it names every publication before it as well.
    publications: Option<crate::product::ContentDigest>,
    /// How many checkpoints are listed, and the first and last of them.
    checkpoints: (usize, Option<uuid::Uuid>, Option<uuid::Uuid>),
}

impl OperationsKey {
    fn of(model: &Model, recovery: &ProjectCheckpointRecoveryState) -> Self {
        let checkpoints = &recovery.checkpoints;
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
            checkpoints: (
                checkpoints.len(),
                checkpoints
                    .first()
                    .map(|checkpoint| checkpoint.checkpoint_id()),
                checkpoints
                    .last()
                    .map(|checkpoint| checkpoint.checkpoint_id()),
            ),
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

/// The runs, validated revisions, library publications and checkpoints the
/// project keeps, newest first: every one of them, since the table scrolls.
/// A plain save keeps no record of its own, so none is listed for it.
fn retained_operations(
    model: &Model,
    recovery: &ProjectCheckpointRecoveryState,
) -> Arc<[OperationSnapshot]> {
    let mut operations = model
        .simulation
        .retained
        .runs
        .iter()
        .enumerate()
        .map(|(index, run)| {
            let measurement_count = run
                .analyses
                .iter()
                .map(|analysis| analysis.measurements.len())
                .sum();
            let (timestamp_ms, stamp) =
                OperationSnapshot::dated(unix_seconds_to_millis(run.timestamp));
            OperationSnapshot {
                timestamp_ms,
                stamp,
                event: format!(
                    "{} \u{b7} {}",
                    run_title(run.id, &run.label),
                    lifecycle_label(run.lifecycle, run.success)
                ),
                elapsed: Some(run.elapsed_time),
                detail: format!(
                    "{} {} \u{b7} {measurement_count} {} \u{b7} {}",
                    run.analyses.len(),
                    singular_or_plural(run.analyses.len(), "analysis", "analyses"),
                    singular_or_plural(measurement_count, "measurement", "measurements"),
                    run.execution_target
                        .map_or("engine not recorded", |target| target.label()),
                ),
                tone: lifecycle_tone(run.lifecycle, run.success),
                intent: Some(OverviewIntent::OpenRun(index)),
            }
        })
        .collect::<Vec<_>>();
    let mut retained_revision_ids = BTreeSet::new();
    for schematic in std::iter::once(model.schematic.editor_ref().design)
        .chain(model.workspace.content.schematic_buffers.values())
    {
        for revision in schematic.document().validated_revisions.records() {
            if !retained_revision_ids.insert(revision.id().as_uuid()) {
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
            let (timestamp_ms, stamp) = OperationSnapshot::dated(Some(revision.created_unix_ms()));
            operations.push(OperationSnapshot {
                timestamp_ms,
                stamp,
                event: format!("{} validated", revision.view_identity()),
                elapsed: None,
                detail: format!(
                    "{} \u{b7} project revision {} \u{b7} {}",
                    revision.revision_note(),
                    revision.project_revision(),
                    revision.author()
                ),
                tone,
                intent: Some(OverviewIntent::OpenRevision(schematic_row_key(
                    revision.id(),
                ))),
            });
        }
    }
    // Published library snapshots carry their own creation time. The
    // hash-chained library *mutation* receipts beside them are deliberately
    // absent: they record a sequence and no timestamp, so this time-ordered
    // register could only show them by inventing a time for them.
    for publication in model.workspace.content.project.library_publications() {
        let (timestamp_ms, stamp) = OperationSnapshot::dated(Some(publication.created_unix_ms()));
        operations.push(OperationSnapshot {
            timestamp_ms,
            stamp,
            event: format!("Library published \u{b7} {}", publication.label()),
            elapsed: None,
            detail: format!(
                "#{:04} \u{b7} library revision {} \u{b7} {} \u{b7} {}",
                publication.sequence(),
                publication.library_revision(),
                publication.reason(),
                publication.actor_id()
            ),
            tone: Tone::Neutral,
            intent: Some(OverviewIntent::Command(
                ProjectCommand::Page(ProjectPage::Library).key(),
            )),
        });
    }
    for checkpoint in &recovery.checkpoints {
        let (timestamp_ms, stamp) = OperationSnapshot::dated(Some(checkpoint.created_unix_ms()));
        operations.push(OperationSnapshot {
            timestamp_ms,
            stamp,
            event: checkpoint.reason().label().to_owned(),
            elapsed: None,
            detail: format!("Revision {}", checkpoint.project_revision()),
            tone: Tone::Neutral,
            intent: Some(OverviewIntent::OpenCheckpoint(
                checkpoint.checkpoint_id().to_string(),
            )),
        });
    }
    // Newest first; a record with no time after every dated one.
    operations.sort_by(|left, right| right.timestamp_ms.cmp(&left.timestamp_ms));
    operations.into()
}

/// The findings of the cell view runs start from that stop a run, worst
/// first, each with its identity in the Problems list. Checks not run yet,
/// out of date or unavailable are the Schematic checks row's to say: the
/// strip names findings.
pub fn blocking_findings(ctx: &egui::Context, cx: &Cx<'_, ProjectFeature>) -> Vec<ProblemSnapshot> {
    let model = &*cx.model;
    let DesignCheckStatus::Current(receipt) = model.simulation_root_design_check_status(
        cx.runs.slot_facts(cx.peers.design().review_markers()),
    ) else {
        return Vec::new();
    };
    let root = model.workspace.content.simulation_root_reference();
    // A published check moves the store's generation, so the findings are
    // listed again only when a check is.
    derived(
        ctx,
        "overview.blocking findings",
        (model.diagnostics.generation(), root.key()),
        || {
            let result = &receipt.result;
            let mut findings = result
                .violations()
                .iter()
                .zip(finding_identities(&root, result))
                .filter(|(violation, _)| {
                    matches!(
                        violation.severity,
                        DrcSeverity::Critical | DrcSeverity::Error
                    )
                })
                .map(|(violation, finding)| {
                    (
                        violation.severity == DrcSeverity::Critical,
                        ProblemSnapshot {
                            message: violation.message.clone(),
                            path: violation.location.display(),
                            tone: Tone::Error,
                            finding: Some(finding),
                        },
                    )
                })
                .collect::<Vec<_>>();
            // Critical first, each severity in the order the checks found them.
            findings.sort_by_key(|(critical, _)| !*critical);
            findings.into_iter().map(|(_, finding)| finding).collect()
        },
    )
}
