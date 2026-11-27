//! Run details…: one retained run, read the way a reader asks about a run.
//!
//! What the run was comes first, as one band of facts: its name, how it
//! ended, when it started, how long it took and on which engine. Under it
//! the analyses it ran, one row each with what it kept and how it ended; an
//! analysis opens on Results, and each of its signals on the schematic, in
//! one action. Then the inputs that produced it, in words, each one a link
//! to what it names while that exists; then the solver's statistics. The
//! identifiers and digests that prove where the run came from close the
//! dialog, folded until the reader asks for them.
//!
//! Every fact in the dialog stands on one label column, so the sections
//! read as one sheet; identifiers alone are set in mono.
//!
//! The run's manifest is projected once, when the dialog opens. Its dataset
//! digest reads every sample the run holds in memory (a run kept in a results
//! store is named by the store's own digest), so it is computed on a Results
//! job and its row fills in when it is known: the dialog shows at once.

mod executed_deck;

use egui::{Context, Ui};

use super::super::super::design_system::{
    FACT_MARK_ROOM, Fact, fact_label_width, fact_rows_on, panel_bleed, panel_body,
    property_row_wrapped, section_block_gap, section_header, section_header_action,
    subsection_header,
};
use super::result_authority::forced_lte_acceptances;
use crate::state::{SimulationRunLifecycle, SimulationState};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::app_state::Model;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::operational_state::run_state_words;
use rspice_results::number_format::fmt_si_significant;
use rspice_ui_kit::panels::Problem;
use rspice_ui_kit::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, RowKey, TABLE_EDGE_INSET, TableState, fitted_width,
};

/// The run Run details… is open on, with its manifest taken once when the
/// dialog opened, and its dataset digest once a Results job has computed it.
#[derive(Debug, Clone)]
pub struct RunDetailsDialogState {
    pub run_id: u64,
    pub manifest: rspice_results::manifest::ManifestViewModel,
    dataset_digest: std::sync::Arc<std::sync::OnceLock<String>>,
    /// The inventory's rows are the run's analyses, in order: the run kept
    /// a receipt that matches it, or none at all.
    rows_are_analyses: bool,
    /// The reader unfolded the Identifiers section.
    identifiers_shown: bool,
}

impl RunDetailsDialogState {
    /// Run details… on `run`: its manifest at once, and its dataset digest
    /// computed on a Results job.
    pub fn new(run: &crate::state::SimulationRun) -> Self {
        use crate::workbench::documents::result_document::{frame_work, manifest};

        let dataset_digest = std::sync::Arc::<std::sync::OnceLock<String>>::default();
        let (slot, retained) = (std::sync::Arc::clone(&dataset_digest), run.data.clone());
        let _ = crate::workbench::results_job::ResultsJob::start("run-digest", move |_| {
            frame_work::note(frame_work::DatasetWalk::DatasetDigest);
            let digest = crate::state::run_store::dataset_digest(&retained);
            let _ = slot.set(digest.to_string());
            Ok(())
        });
        Self {
            run_id: run.id,
            manifest: manifest::manifest_without_digest(run),
            dataset_digest,
            rows_are_analyses: run.prepared_receipt().is_none()
                || run.validate_provenance().is_ok(),
            identifiers_shown: false,
        }
    }

    /// The run's dataset digest; `None` while it is still being computed.
    pub fn dataset_digest(&self) -> Option<&str> {
        self.dataset_digest.get().map(String::as_str)
    }

    /// Unfold the Identifiers section, as its header's Show does.
    pub fn show_identifiers(&mut self) {
        self.identifiers_shown = true;
    }
}

/// At most this many significant digits for the times a convergence record
/// names.
const TIME_DIGITS: usize = 6;

/// Open Run details… on the retained run `run_id`. A run that is no longer
/// retained opens nothing.
pub fn open(
    run_details: &mut Option<Box<RunDetailsDialogState>>,
    simulation: &SimulationState,
    run_id: u64,
) {
    let Some(run) = simulation.retained.runs.iter().find(|run| run.id == run_id) else {
        return;
    };
    *run_details = Some(Box::new(RunDetailsDialogState::new(run)));
}

/// The routes out of Run details… the reader took this frame. Each is a
/// link or an inventory row of the dialog, and taking any closes it.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RunDetailsRoutes {
    /// Open the producing plan's Analyses page.
    pub(crate) open_plan: bool,
    /// Open the deck the engine executed for this run.
    pub(crate) open_deck: Option<u64>,
    /// Show this analysis on Results: the run, and the analysis's place in
    /// it.
    pub show_analysis: Option<(u64, usize)>,
    /// Show this signal on the schematic.
    pub show_signal: Option<String>,
}

/// Follow the inventory's route the reader took: the analysis shown on
/// Results, or the signal shown on the schematic, as the data browser shows
/// them. The status line says why when neither can be.
pub fn follow_inventory_route(state: &mut ResultsParts<'_>, routes: &RunDetailsRoutes) {
    use crate::workbench::commands::schematic_cross_probe::show_signal_on_schematic;
    use crate::workbench::docks::navigator::results_actions::select_result_analysis;

    if let Some((run_id, analysis)) = routes.show_analysis {
        let run = state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .position(|run| run.id == run_id);
        // Results shows a run only while it is the workspace in front, and
        // Run details opens from the status bar in any workspace.
        let from = state.workbench.workspace;
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        if !run.is_some_and(|run| select_result_analysis(state, run, analysis)) {
            state.workbench.activate(from);
            state
                .workbench
                .prompt("That analysis is no longer retained, so it cannot be shown.");
        }
    }
    if let Some(signal) = &routes.show_signal
        && let Err(reason) = show_signal_on_schematic(state, signal)
    {
        state.workbench.prompt(reason);
    }
}

/// One fact of the dialog, owned, so every section's labels are known
/// before any is drawn and all of them stand on one label column.
struct Line {
    label: String,
    value: String,
    /// An identifier: set in mono and shortened from the middle.
    code: bool,
    /// What the hover says in place of the value, of which it is the whole.
    whole: Option<String>,
    /// The problem the value states.
    problem: Option<Problem>,
    /// What following the value does, when it is a link.
    link: Option<&'static str>,
}

impl Line {
    fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            code: false,
            whole: None,
            problem: None,
            link: None,
        }
    }

    fn code(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            code: true,
            ..Self::new(label, value)
        }
    }

    fn whole(mut self, whole: impl Into<String>) -> Self {
        self.whole = Some(whole.into());
        self
    }

    const fn marked(mut self, problem: Option<Problem>) -> Self {
        self.problem = problem;
        self
    }

    const fn linked(mut self, action: &'static str) -> Self {
        self.link = Some(action);
        self
    }
}

/// `lines` on the dialog's one label column; the line whose link was
/// followed.
fn facts(ui: &mut Ui, lines: &[Line], label_width: f32) -> Option<usize> {
    let facts: Vec<Fact<'_>> = lines
        .iter()
        .map(|line| {
            let fact = if line.code {
                Fact::code(&line.label, &line.value)
            } else {
                Fact::new(&line.label, &line.value)
            };
            let fact = match &line.whole {
                Some(whole) => fact.whole(whole),
                None => fact,
            };
            fact.marked(line.problem).linked(line.link)
        })
        .collect();
    fact_rows_on(ui, &facts, label_width)
}

/// A rule across the dialog's body at `y`, from one edge to the other, over
/// whatever was painted there.
fn hairline(ui: &Ui, y: f32) {
    let edges = panel_bleed(ui, ui.max_rect()).x_range();
    let border = rspice_ui_kit::tokens::Tokens::get(ui.ctx()).color.border;
    ui.painter()
        .hline(edges, y + 0.5, egui::Stroke::new(1.0, border));
}

/// Draw Run details… while it is open, and return the routes the reader
/// took. Closing it, or taking any route, clears `run_details` once the frame
/// is done.
pub fn show(
    ctx: &Context,
    run_details: &mut Option<Box<RunDetailsDialogState>>,
    model: &Model,
) -> RunDetailsRoutes {
    let Some(dialog) = run_details.as_ref() else {
        return RunDetailsRoutes::default();
    };
    let run_id = dialog.run_id;
    let Some(run) = model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.id == run_id)
    else {
        // The run left the history while its details were open.
        *run_details = None;
        return RunDetailsRoutes::default();
    };
    let manifest = &dialog.manifest;
    let executed = model
        .simulation
        .retained
        .executed_decks
        .get(run_id)
        .map(crate::state::ExecutedDeck::model_sources);
    // The producing-plan route resolves against the active dataset, so it is
    // offered only for the run that is active.
    let plan_block = model
        .simulation
        .active_run()
        .is_some_and(|active| active.id == run_id)
        .then(|| {
            crate::workbench::state::plan_provenance::producing_plan_block(
                &model.simulation,
                model.sim_setup.stable_analysis_plan().ok(),
            )
        });
    let digits = usize::from(
        model
            .preferences
            .result_presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let mut identifiers_shown = dialog.identifiers_shown;
    let mut open_plan = false;
    let mut open_deck = None;
    let mut inventory_route = None;

    let choice = Dialog::new("Run details", "Close")
        .description(
            "One retained run: what it ran, the analyses and signals it kept, the inputs that \
             produced it, and the identifiers that prove where it came from.",
        )
        .size(DialogSize::AccountManager)
        .flush_body()
        .overlay_scrollbar()
        .show(ctx, |ui| {
            let summary = summary(ui, run, manifest);
            let inputs = inputs(model, run, manifest, plan_block, executed.as_deref());
            let identifiers = identifiers(run, manifest, dialog.dataset_digest());
            let convergence: Vec<(&str, ConvergenceLines<'_>)> = run
                .analyses
                .iter()
                .filter_map(|analysis| {
                    let quality = analysis.convergence.as_deref()?;
                    Some((analysis.label.as_str(), convergence_lines(quality)))
                })
                .collect();
            let lines = || {
                summary
                    .iter()
                    .chain(&inputs.lines)
                    .chain(&identifiers)
                    .chain(convergence.iter().flat_map(|(_, lines)| lines.all()))
            };
            let marked = lines().any(|line| line.problem.is_some());
            let label_width = fact_label_width(ui, lines().map(|line| line.label.as_str()))
                + if marked { FACT_MARK_ROOM } else { 0.0 };
            if dialog.dataset_digest().is_none() {
                // The thread computing it cannot wake the frame, so the
                // dialog looks again shortly.
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(100));
            }
            panel_body(ui, |ui| {
                let _ = facts(ui, &summary, label_width);
                // The table's heads title the analyses, on a hairline under
                // the summary.
                let top = ui.cursor().top();
                inventory_route = inventory_table(ui, manifest, run, dialog.rows_are_analyses);
                hairline(ui, top);
                section_header(ui, "Inputs", None);
                match facts(ui, &inputs.lines, label_width) {
                    Some(line) if Some(line) == inputs.plan => open_plan = true,
                    Some(line) if Some(line) == inputs.deck => open_deck = Some(run_id),
                    _ => {}
                }
                for (analysis, lines) in &convergence {
                    lines.show(ui, analysis, label_width, digits);
                }
                let fold = section_header_action(
                    ui,
                    "Identifiers",
                    if identifiers_shown { "Hide" } else { "Show" },
                    true,
                );
                if fold.clicked() {
                    identifiers_shown = !identifiers_shown;
                }
                if identifiers_shown {
                    let _ = facts(ui, &identifiers, label_width);
                    section_block_gap(ui);
                    if Button::new("Copy identifiers")
                        .show(ui)
                        .on_hover_text("Copies every identifier above, one per line.")
                        .clicked()
                    {
                        ui.ctx().copy_text(
                            identifiers
                                .iter()
                                .map(|line| {
                                    format!(
                                        "{}: {}",
                                        line.label,
                                        line.whole.as_deref().unwrap_or(&line.value)
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join("\n"),
                        );
                    }
                }
            });
        });

    let (show_analysis, show_signal) = match inventory_route {
        Some(InventoryRoute::Analysis(index)) => (Some((run_id, index)), None),
        Some(InventoryRoute::Signal(signal)) => (None, Some(signal)),
        None => (None, None),
    };
    let close = matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled);
    if close || open_plan || open_deck.is_some() || show_analysis.is_some() || show_signal.is_some()
    {
        *run_details = None;
    } else if let Some(dialog) = run_details.as_mut() {
        dialog.identifiers_shown = identifiers_shown;
    }
    RunDetailsRoutes {
        open_plan,
        open_deck,
        show_analysis,
        show_signal,
    }
}

/// What the run was: its name, how it ended, when it started, how long it
/// took and on which engine, and how its checks came out when it had any.
fn summary(
    ui: &Ui,
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
) -> Vec<Line> {
    let failed = run
        .analyses
        .iter()
        .filter(|analysis| !analysis.success && !analysis.is_live_partial())
        .count();
    // A run some of whose analyses failed completed: it is marked as a
    // warning, and as a failure only when nothing of it completed.
    let partly = failed > 0 && failed < run.analyses.len();
    let mark = match (run.lifecycle, failed) {
        // A run its results storage stopped is marked as the status bar marks it.
        (SimulationRunLifecycle::Completed | SimulationRunLifecycle::Failed, _)
            if run.results_storage_stop().is_some() =>
        {
            Some(Problem::Failure)
        }
        (SimulationRunLifecycle::Completed, 0) => None,
        (SimulationRunLifecycle::Completed, _) => Some(Problem::Warning),
        (SimulationRunLifecycle::Failed, _) if partly => Some(Problem::Warning),
        (SimulationRunLifecycle::Failed, _) => Some(Problem::Failure),
        (SimulationRunLifecycle::Interrupted, _) => Some(Problem::Warning),
        _ => None,
    };
    let state = Line::new("State", run_state_words(run)).marked(mark);
    let mut lines = vec![
        Line::new("Run", run.name()),
        state,
        Line::new(
            "Started",
            run.start_stamp()
                .unwrap_or_else(|| "Not recorded".to_owned()),
        ),
        Line::new(
            "Duration",
            super::result_authority::run_duration(ui, run, manifest),
        ),
        Line::new(
            "Engine",
            run.execution_target.map_or_else(
                || "Not recorded".to_owned(),
                |target| target.label().to_owned(),
            ),
        ),
    ];
    // Stated only for a run whose checks reported something: a run that
    // checked nothing has not passed anything.
    if let Some(verdict) = run.check_verdict() {
        use rspice_results::analysis_result::CheckVerdict;
        lines.push(match verdict {
            CheckVerdict::Passed => Line::new("Checks", "Passed"),
            CheckVerdict::Warned => {
                Line::new("Checks", "Passed with warnings").marked(Some(Problem::Warning))
            }
            CheckVerdict::Failed => Line::new("Checks", "Failed").marked(Some(Problem::Failure)),
        });
    }
    lines
}

/// The Inputs section's facts, and which of them are links.
struct Inputs {
    lines: Vec<Line>,
    /// The line that opens the producing plan.
    plan: Option<usize>,
    /// The line that opens the executed deck.
    deck: Option<usize>,
}

/// Where the run came from, in words: what it was prepared from, its plan,
/// configuration and revision, the check its source passed, and the deck it
/// executed.
fn inputs(
    model: &Model,
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
    plan_block: Option<Option<&'static str>>,
    executed: Option<&[String]>,
) -> Inputs {
    let mut lines = Vec::new();
    let mut plan = None;
    match &manifest.authority {
        Some(authority) => {
            lines.push(Line::new("Source", sentence(&authority.source_domain)));
            let plan_line = match (&authority.simulation_plan_id, plan_block) {
                (None, _) => Line::new("Plan", "None, a manual deck"),
                (Some(_), Some(None)) => {
                    plan = Some(lines.len());
                    Line::new("Plan", "Simulation plan").linked(
                        "Opens the Analyses page of the plan that produced this run, with the \
                         producing analysis selected",
                    )
                }
                (Some(_), Some(Some(reason))) => {
                    Line::new("Plan", "Simulation plan").whole(sentence(reason))
                }
                (Some(_), None) => Line::new("Plan", "Simulation plan"),
            };
            lines.push(plan_line);
            // The design the run elaborated, by the name it ran under; its
            // identity and revision are with the identifiers.
            if let Some(recorded) = run
                .prepared_receipt()
                .and_then(|receipt| receipt.configuration())
            {
                lines.push(Line::new(
                    "Configuration",
                    crate::state::workspace::recorded_configuration_words(
                        &model.workspace.content.configuration_sets,
                        recorded,
                    ),
                ));
            }
            lines.push(Line::new(
                "Project revision",
                authority.project_revision.as_str(),
            ));
            lines.push(Line::new("Source check", sentence(&authority.source_check)));
        }
        None => lines.push(Line::new("Inputs", "Not recorded for this run")),
    }
    let deck = executed_deck::lines(&mut lines, executed);
    Inputs { lines, plan, deck }
}

/// `words` with a capital first letter.
fn sentence(words: &str) -> String {
    let mut chars = words.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// How the run's results stand against the record it was prepared under,
/// behind a mark when they cannot be relied on.
fn integrity(run: &crate::state::SimulationRun) -> Line {
    let (words, problem) = match (run.prepared_receipt(), run.validate_provenance()) {
        (Some(_), Ok(())) => ("Sealed: the results match the run's receipt", None),
        (Some(_), Err(_)) => (
            "Not verified: the results differ from the run's receipt",
            Some(Problem::Failure),
        ),
        (None, Ok(())) => ("Recorded before runs were sealed", None),
        (None, Err(_)) if run.provenance().is_none() => {
            ("Not sealed: this run carries no signed provenance", None)
        }
        (None, Err(_)) => (
            "Not verified: the results differ from the run's record",
            Some(Problem::Failure),
        ),
    };
    Line::new("Integrity", words).marked(problem)
}

/// The identifiers and digests that prove where the run came from, and how
/// its results stand against them.
fn identifiers(
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
    dataset_digest: Option<&str>,
) -> Vec<Line> {
    let mut lines = vec![
        integrity(run),
        Line::code("Run ID", manifest.run_id.as_str()),
    ];
    if let Some(job) = run.job_id {
        lines.push(Line::code("Job ID", job.to_string()));
    }
    lines.push(Line::code("Dataset ID", manifest.dataset_id.as_str()));
    lines.push(match dataset_digest {
        Some(digest) => Line::code("Dataset digest", digest),
        None => Line::new("Dataset digest", "Computing\u{2026}"),
    });
    let Some(authority) = &manifest.authority else {
        return lines;
    };
    if let Some(plan) = &authority.simulation_plan_id {
        lines.push(Line::code("Simulation plan ID", plan.as_str()));
    }
    if let Some(recorded) = run
        .prepared_receipt()
        .and_then(|receipt| receipt.configuration())
        && let Some(id) = recorded.id
    {
        lines.push(Line::code(
            "Configuration ID",
            format!("{id} \u{b7} revision {}", recorded.revision),
        ));
    }
    lines.push(Line::code(
        "Prepared input digest",
        authority.prepared_snapshot_digest.as_str(),
    ));
    lines.push(Line::code(
        "Source content digest",
        authority.source_content_digest.as_str(),
    ));
    lines.push(Line::code(
        "Source-check digest",
        authority.source_check_digest.as_str(),
    ));
    for (identity, digest) in &authority.model_sources {
        // The model's name, then its digest: shortened from the middle, both
        // ends stay in view, and the hover names its source and revision.
        let name = identity.split(" \u{b7} ").next().unwrap_or(identity);
        lines.push(
            Line::code("Model", format!("{name} {digest}")).whole(format!("{identity}: {digest}")),
        );
    }
    lines
}

/// Where an inventory row leads.
enum InventoryRoute {
    /// The analysis at this place in the run, on Results.
    Analysis(usize),
    /// The signal, on the schematic.
    Signal(String),
}

/// One line of the inventory: an analysis, or a signal it kept, listed
/// under it while it is open.
#[derive(Clone, Copy)]
enum InventoryLine {
    Analysis(usize),
    Signal(usize, usize),
}

/// One analysis row in words: what it was, what it kept, and how it ended,
/// with the mark of a failure and why, when it has one.
struct AnalysisWords {
    name: String,
    /// What it kept; `None` when it kept nothing.
    kept: Option<String>,
    state: String,
    problem: Option<Problem>,
    why: Option<String>,
}

/// The inventory's row for the run's `index`th task, `task` naming it when
/// the run kept no result for it.
fn analysis_words(
    run: &crate::state::SimulationRun,
    task: &str,
    index: usize,
    rows_are_analyses: bool,
) -> AnalysisWords {
    use rspice_results::family_metadata::AnalysisResultFamilyMetadata as Family;

    if !rows_are_analyses {
        return AnalysisWords {
            name: task.to_owned(),
            kept: None,
            state: "Not verified".to_owned(),
            problem: Some(Problem::Failure),
            why: Some(
                "The run's results differ from the receipt it was prepared under, so this row \
                 cannot be traced to a result."
                    .to_owned(),
            ),
        };
    }
    let Some(analysis) = run.analyses.get(index) else {
        let state = match run.lifecycle {
            SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling => "Pending",
            SimulationRunLifecycle::Failed
            | SimulationRunLifecycle::Aborted
            | SimulationRunLifecycle::Interrupted => "Not produced",
            SimulationRunLifecycle::Completed | SimulationRunLifecycle::LegacyUnknown => "Not kept",
        };
        return AnalysisWords {
            name: task.to_owned(),
            kept: None,
            state: state.to_owned(),
            problem: None,
            why: None,
        };
    };
    let kind = analysis.kind_display_name();
    let (name, members) = match &analysis.family_metadata {
        Some(Family::Parametric {
            target,
            sweep_values,
            failed_points,
            ..
        }) => (
            format!("{kind} \u{b7} {target}, {} points", sweep_values.len()),
            Some((*failed_points, sweep_values.len(), "points")),
        ),
        Some(Family::Corner {
            corner_labels,
            failed_corners,
            ..
        }) => (
            format!("{kind} \u{b7} {} corners", corner_labels.len()),
            Some((*failed_corners, corner_labels.len(), "corners")),
        ),
        Some(Family::MonteCarlo {
            runs_requested,
            failures,
            ..
        }) => (
            format!("{kind} \u{b7} {runs_requested} samples"),
            Some((*failures, *runs_requested, "samples")),
        ),
        _ => (kind.to_owned(), None),
    };
    let kept = match (analysis.waveforms.len(), &analysis.dc_op) {
        (0, Some(op)) => Some(format!(
            "{} node voltages, {} currents",
            op.node_voltages.len(),
            op.branch_currents.len()
        )),
        (0, None) => None,
        (1, _) => Some("1 signal".to_owned()),
        (signals, _) => Some(format!("{signals} signals")),
    };
    let (state, problem, why) = if analysis.is_live_partial() {
        ("Running".to_owned(), None, None)
    } else if !analysis.success {
        (
            "Failed".to_owned(),
            Some(Problem::Failure),
            analysis.error_message.clone(),
        )
    } else {
        match members {
            Some((failed, of, members)) if failed > 0 => (
                format!("{failed} of {of} {members} failed"),
                Some(Problem::Warning),
                None,
            ),
            _ => ("Complete".to_owned(), None, None),
        }
    };
    AnalysisWords {
        name,
        kept,
        state,
        problem,
        why,
    }
}

/// The run's analyses, one row per task, each opening to the signals it
/// kept. Opening a row, by a double-click or Enter, shows the analysis on
/// Results or the signal on the schematic, as its menu does. The rows name
/// analyses only while the run's receipt matches it; a mismatched run lists
/// its tasks and leads nowhere.
///
/// The table rules its own edges, from the dialog's one side to the other:
/// its heads title it, on a hairline under the run's summary; the next head
/// stands on its last rule; and it is as tall as its rows, for the dialog's
/// body is what scrolls. Each column is as wide as its words, and the names
/// take what is left.
fn inventory_table(
    ui: &mut Ui,
    manifest: &rspice_results::manifest::ManifestViewModel,
    run: &crate::state::SimulationRun,
    rows_are_analyses: bool,
) -> Option<InventoryRoute> {
    let id = egui::Id::new(("rspice.results.run-details.inventory", run.id));
    let mut open: std::collections::BTreeSet<usize> = ui
        .data(|data| data.get_temp(id.with("open")))
        .unwrap_or_default();
    let signals = |index: usize| {
        run.analyses
            .get(index)
            .filter(|_| rows_are_analyses)
            .map_or(&[][..], |analysis| analysis.waveforms.as_slice())
    };
    let waveform = |index: usize, signal: usize| -> &rspice_results::waveform::RetainedWaveform {
        signals(index)[signal].as_ref()
    };
    let words: Vec<AnalysisWords> = manifest
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| analysis_words(run, &row.analysis, index, rows_are_analyses))
        .collect();
    let mut lines = Vec::new();
    for index in 0..words.len() {
        lines.push(InventoryLine::Analysis(index));
        if open.contains(&index) {
            lines.extend(
                (0..signals(index).len()).map(|signal| InventoryLine::Signal(index, signal)),
            );
        }
    }
    let keys: Vec<RowKey> = lines
        .iter()
        .map(|line| {
            RowKey::Text(match line {
                InventoryLine::Analysis(index) => format!("{index}"),
                InventoryLine::Signal(index, signal) => format!("{index}.{signal}"),
            })
        })
        .collect();
    let cells = |row: usize| match lines[row] {
        InventoryLine::Analysis(index) => {
            let words = &words[index];
            let name = Cell::text(&words.name);
            let state = match words.problem {
                Some(problem) => Cell::problem(&words.state, problem),
                None => Cell::text(&words.state),
            };
            vec![
                if signals(index).is_empty() {
                    name.leaf()
                } else {
                    name.twist(open.contains(&index))
                },
                words.kept.as_ref().map_or_else(Cell::absent, Cell::text),
                match &words.why {
                    Some(why) => state.hint(why),
                    None => state,
                },
            ]
        }
        InventoryLine::Signal(index, signal) => {
            let waveform = waveform(index, signal);
            vec![
                Cell::code(&waveform.name).indent(1).leaf(),
                Cell::text(format!(
                    "{} samples",
                    waveform.x.len().min(waveform.y.len())
                )),
                Cell::empty(),
            ]
        }
    };
    // Measured again only when the rows change, or the type they are set
    // in (the interface's fonts and scale, which the first head's width
    // stands for): a sweep's run can list a task per point, and an open
    // analysis every signal it kept.
    let heads = ["Analysis", "Kept", "State"];
    let shape = (
        lines.len(),
        open.iter().copied().collect::<Vec<_>>(),
        fitted_width(ui, heads[0], []).to_bits(),
    );
    let widths_id = id.with("widths");
    let measured = ui.data(|data| data.get_temp::<((usize, Vec<usize>, u32), [f32; 3])>(widths_id));
    let widths = match measured {
        Some((measured, widths)) if measured == shape => widths,
        _ => {
            let rows: Vec<Vec<Cell>> = (0..lines.len()).map(cells).collect();
            let widths = std::array::from_fn(|column| {
                fitted_width(
                    ui,
                    heads[column],
                    rows.iter().filter_map(|row| row.get(column)),
                )
            });
            ui.data_mut(|data| data.insert_temp(widths_id, (shape, widths)));
            widths
        }
    };
    // A fitted width pads its words by a cell's padding on each side, and
    // the outer columns' words stand on the table's edge inset instead.
    let edge = TABLE_EDGE_INSET - CELL_PAD;
    let columns = [
        Column::name(heads[0], widths[0] + edge),
        Column::words(heads[1], widths[1]),
        Column::words(heads[2], widths[2] + edge),
    ];
    let mut state = ui
        .data(|data| data.get_temp::<TableState>(id.with("state")))
        .unwrap_or_default();
    let response = DataTable::new(
        "rspice.results.run-details.inventory",
        &columns,
        lines.len(),
    )
    .state(&mut state)
    .keys(&keys)
    .show(ui, cells);
    ui.data_mut(|data| data.insert_temp(id.with("state"), state));

    if let Some((row, opened)) = response.expand
        && let Some(InventoryLine::Analysis(index)) = lines.get(row).copied()
    {
        if opened {
            open.insert(index);
        } else {
            open.remove(&index);
        }
    }
    ui.data_mut(|data| data.insert_temp(id.with("open"), open));

    let route_of = |line: InventoryLine| match line {
        InventoryLine::Analysis(index) => (rows_are_analyses && index < run.analyses.len())
            .then_some(InventoryRoute::Analysis(index)),
        InventoryLine::Signal(index, signal) => {
            Some(InventoryRoute::Signal(waveform(index, signal).name.clone()))
        }
    };
    let mut route = response
        .activate
        .and_then(|row| lines.get(row).copied())
        .and_then(route_of);

    let menu = id.with("menu");
    let ctx = ui.ctx().clone();
    if let Some(asked) = response.context_menu {
        ctx.data_mut(|data| data.insert_temp(menu, (asked.cell.row, asked.position)));
        egui::Popup::open_id(&ctx, menu);
    }
    if egui::Popup::is_id_open(&ctx, menu)
        && let Some((row, position)) = ctx.data(|data| data.get_temp::<(usize, egui::Pos2)>(menu))
        && let Some(line) = lines.get(row).copied()
    {
        egui::Popup::new(
            menu,
            ctx.clone(),
            egui::PopupAnchor::Position(position),
            egui::LayerId::new(egui::Order::Foreground, menu),
        )
        .kind(egui::PopupKind::Menu)
        .open_memory(None)
        .show(|ui| {
            let label = match line {
                InventoryLine::Analysis(_) => "Show on Results",
                InventoryLine::Signal(..) => "Show on schematic",
            };
            let target = route_of(line);
            let refusal = match &target {
                None if rows_are_analyses => {
                    Some("The run kept no result for this analysis.".to_owned())
                }
                None => Some(
                    "The run's results differ from its receipt, so this row names no result."
                        .to_owned(),
                ),
                Some(InventoryRoute::Signal(signal)) => {
                    crate::workbench::commands::schematic_cross_probe::schematic_refusal(signal)
                }
                Some(InventoryRoute::Analysis(_)) => None,
            };
            let show = ui.add_enabled(refusal.is_none(), egui::Button::new(label));
            match refusal {
                Some(refusal) => {
                    show.on_disabled_hover_text(refusal);
                }
                None if show.clicked() => {
                    route = target;
                    ui.close();
                }
                None => {}
            }
        });
    }
    route
}

/// One analysis's solver statistics: its transient's, and for a periodic
/// analysis its initialization's.
struct ConvergenceLines<'a> {
    transient: PhaseLines<'a>,
    initialization: Option<(&'static str, PhaseLines<'a>)>,
}

/// One phase's solver counters, the times it forced points at, and its
/// diagnostic when it failed.
struct PhaseLines<'a> {
    counters: Vec<Line>,
    /// Forced on the solver's own trajectory, before any cropping, envelope
    /// or Fourier step.
    forced: &'a [f64],
    diagnostic: Vec<(&'static str, String)>,
}

impl<'a> ConvergenceLines<'a> {
    fn phases(&self) -> impl Iterator<Item = &PhaseLines<'a>> {
        std::iter::once(&self.transient).chain(self.initialization.iter().map(|(_, phase)| phase))
    }

    /// Every counter line, for the label column.
    fn all(&self) -> impl Iterator<Item = &Line> {
        self.phases().flat_map(|phase| &phase.counters)
    }

    fn show(&self, ui: &mut Ui, analysis: &str, label_width: f32, digits: usize) {
        section_header(ui, &format!("Convergence \u{b7} {analysis}"), None);
        self.transient
            .show(ui, label_width, digits, &format!("{analysis}/transient"));
        if let Some((phase, lines)) = &self.initialization {
            subsection_header(ui, phase);
            lines.show(
                ui,
                label_width,
                digits,
                &format!("{analysis}/initialization"),
            );
        }
    }
}

impl PhaseLines<'_> {
    /// `salt` tells this phase's forced points from every other phase's in
    /// the dialog.
    fn show(&self, ui: &mut Ui, label_width: f32, digits: usize, salt: &str) {
        let _ = facts(ui, &self.counters, label_width);
        for (label, words) in &self.diagnostic {
            property_row_wrapped(ui, label, words);
        }
        if !self.forced.is_empty() {
            let top = ui.cursor().top();
            forced_points(ui, self.forced, digits, salt);
            hairline(ui, top);
        }
    }
}

/// Each point the phase forced, at the time it was forced, one row each.
/// The table is as tall as its rows and lays out only those in view: a long
/// run can force thousands, and the dialog's body is the one thing that
/// scrolls.
fn forced_points(ui: &mut Ui, times: &[f64], digits: usize, salt: &str) {
    let heads = ["Forced point", "Time"];
    let last = Cell::number(times.len().to_string());
    // The times take what the numbers leave, set from the left, so they
    // read on from the point they belong to.
    let columns = [
        Column::number(
            heads[0],
            fitted_width(ui, heads[0], [&last]) + TABLE_EDGE_INSET - CELL_PAD,
        ),
        Column::name(heads[1], fitted_width(ui, heads[1], [])),
    ];
    let id = format!("rspice.results.run-details.forced.{salt}");
    let _ = DataTable::new(&id, &columns, times.len()).show(ui, |row| {
        vec![
            Cell::number((row + 1).to_string()),
            Cell::number(fmt_si_significant(times[row], "s", digits.min(TIME_DIGITS))),
        ]
    });
}

fn convergence_lines(
    quality: &rspice_results::convergence_quality::TransientConvergenceEvidence,
) -> ConvergenceLines<'_> {
    use crate::state::PeriodicInitializationMethod;

    let forced = forced_lte_acceptances(quality);
    let mut transient = phase_lines(&quality.transient);
    transient.counters.insert(
        0,
        Line::new("LTE acceptances", format!("{forced} forced"))
            .whole(format!("{forced} forced; LTE is local truncation error")),
    );
    let initialization = quality.initialization.as_ref().map(|initialization| {
        let (phase, iterations) = match initialization.method {
            PeriodicInitializationMethod::Shooting => {
                ("Shooting initialization", "Shooting corrections")
            }
            PeriodicInitializationMethod::HarmonicBalance => {
                ("Harmonic-balance initialization", "HB Newton iterations")
            }
        };
        let mut lines = phase_lines(&initialization.report);
        lines.counters.splice(
            0..0,
            [
                Line::new(iterations, initialization.solver_iterations.to_string()),
                Line::new(
                    "Final periodic residual",
                    format!("{:.3e}", initialization.final_residual),
                ),
            ],
        );
        (phase, lines)
    });
    ConvergenceLines {
        transient,
        initialization,
    }
}

fn phase_lines(report: &crate::state::ConvergenceReport) -> PhaseLines<'_> {
    let mut counters: Vec<Line> = [
        ("Newton iterations", report.total_iterations),
        ("Gmin stepping", report.gmin_stepping_count),
        ("Source stepping", report.source_stepping_count),
        ("Step reductions", report.timestep_reductions),
        ("LTE rejections", report.lte_rejections),
        ("Device bypasses", report.bypassed_device_evaluations),
    ]
    .into_iter()
    .map(|(name, count)| Line::new(name, count.to_string()))
    .collect();
    counters.push(Line::new(
        "Maximum residual",
        format!("{:.3e}", report.max_residual),
    ));
    counters.push(Line::new(
        "Iterations per solve",
        format!("{:.2}", report.avg_iterations_per_solve),
    ));
    let forced = report
        .time_basis
        .as_ref()
        .map_or(&[][..], |basis| basis.force_accepted_times_s.as_slice());
    let mut diagnostic = Vec::new();
    if let Some(failure) = &report.failure_diagnostic {
        diagnostic.push(("Diagnostic", failure.summary()));
        if !failure.failure_message.is_empty() {
            diagnostic.push(("Solver message", failure.failure_message.clone()));
        }
    }
    PhaseLines {
        counters,
        forced,
        diagnostic,
    }
}
