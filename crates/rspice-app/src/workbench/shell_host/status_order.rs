//! The status bar's cells: what each one reports, and the order the bar lays
//! them out in.
//!
//! The bar measures and paints; every cell here only says what it shows. A
//! zone's place is taken by the first cell listed for it that applies to the
//! workspace showing and has something to say, so a cell for one workspace is
//! listed before the cell that answers for all the others.

use rspice_app_workbench::workbench::framework::command_registry::ids;
use std::borrow::Cow;

use crate::workbench::chrome::status_bar::shell::platform_label;
use crate::workbench::commands::vocabulary::ShellCommand;
use crate::workbench::features;
use crate::workbench::{AppState, RSpiceApp};
use rspice_app_design::workbench::app::actions::workspace::symbol_in_place::in_place_session;
use rspice_app_design::workbench::chrome::status_bar::design::{
    design_engineering_context, selection_summary, symbol_tool_hint, zoom_factor,
};
use rspice_app_results::workbench::chrome::status_bar::results::{
    format_plot_zoom, results_view_summary,
};
use rspice_app_results::workbench::documents::result_document::{
    SharedXStatus, active_shared_x_status,
};
use rspice_app_results::workbench::results::ResultsFeature;
use rspice_app_runs::simulation::controller::PreflightStage;
use rspice_app_runs::simulation::status::{
    EngineAvailability, preflight_status, simulation_engine_status,
};
use rspice_app_sheets::workbench::chrome::status_bar::sheets::sheet_position_summary;
use rspice_app_state::workbench::app_state::diagnostic_store::listing::listing_in_pass;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::ui::tokens::Tokens;
use rspice_app_workbench::ui::widgets::Problem;
use rspice_app_workbench::workbench::chrome::status_bar::{
    check_problem, check_summary, current_prompt, engine_status_mark,
};
use rspice_app_workbench::workbench::framework::host::{
    StatusCell, StatusControl, StatusKey, StatusMark, StatusText, StatusZone,
};
use rspice_results::verification::{RowStatus, Sheet};

use crate::workbench::diagnostic_routes::open_specification;

/// What the cells read in common on a frame, worked out before any of them.
pub struct StatusReadings {
    /// The pass the frame is painting, which the checks list is read in.
    pass: u64,
    /// The prompt still showing. One that expired was dropped as this was
    /// read.
    prompt: Option<String>,
    /// The view the Results instrument shows, while Results is showing.
    results_view: Option<SharedXStatus>,
    /// The engine's state, read once so the engine cell's text and what
    /// pressing it does describe the same engine.
    availability: EngineAvailability,
}

pub(super) fn status_readings(
    state: &mut AppState,
    availability: EngineAvailability,
    ctx: &egui::Context,
) -> StatusReadings {
    let t = Tokens::get(ctx);
    // The Results workspace reports the view its instrument shows: the
    // visible X interval in the coordinate segment, and that interval's
    // magnification in the zoom chip beside it.
    let results_view = (state.workbench.workspace == Workspace::Results)
        .then(|| active_shared_x_status(&t, &mut state.results_parts()))
        .flatten();
    StatusReadings {
        pass: ctx.cumulative_pass_nr(),
        prompt: current_prompt(ctx, &mut state.workbench.prompt),
        results_view,
        availability,
    }
}

/// Every cell, each zone counted from its own edge: the leading zone from the
/// left, the trailing zone from the right.
pub(super) static STATUS_ORDER: &[StatusCell<RSpiceApp>] = &[
    StatusCell {
        key: StatusKey::Prompt,
        zone: StatusZone::Leading,
        order: 0,
        workspace: None,
        content: prompt_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Prompt,
        zone: StatusZone::Leading,
        order: 0,
        workspace: Some(Workspace::Design),
        content: symbol_tool_hint_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Checks,
        zone: StatusZone::Leading,
        order: 1,
        workspace: None,
        content: checks_cell,
        activate: Some(|app, _| app.execute_command_key(ShellCommand::OpenProblems.key())),
    },
    StatusCell {
        key: StatusKey::Context,
        zone: StatusZone::Leading,
        order: 2,
        workspace: None,
        content: no_project_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Context,
        zone: StatusZone::Leading,
        order: 2,
        workspace: Some(Workspace::Design),
        content: design_context_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Context,
        zone: StatusZone::Leading,
        order: 2,
        workspace: Some(Workspace::Results),
        content: results_context_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Selection,
        zone: StatusZone::Leading,
        order: 3,
        workspace: None,
        content: selection_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Page,
        zone: StatusZone::Leading,
        order: 4,
        workspace: Some(Workspace::Design),
        content: in_place_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Page,
        zone: StatusZone::Leading,
        order: 4,
        workspace: None,
        content: page_cell,
        activate: None,
    },
    StatusCell {
        key: StatusKey::Zoom,
        zone: StatusZone::Trailing,
        order: 0,
        workspace: Some(Workspace::Results),
        content: plot_zoom_cell,
        activate: Some(|app, _| app.execute_command_key(ids::ZOOM_FIT)),
    },
    StatusCell {
        key: StatusKey::Zoom,
        zone: StatusZone::Trailing,
        order: 0,
        workspace: None,
        content: canvas_zoom_cell,
        activate: Some(|app, _| app.execute_command_key(ids::ZOOM_ONE_TO_ONE)),
    },
    StatusCell {
        key: StatusKey::Engine,
        zone: StatusZone::Trailing,
        order: 1,
        workspace: None,
        content: engine_cell,
        activate: Some(|app, readings| {
            let EnginePress {
                can_retry,
                can_open,
            } = engine_press(app, &readings.availability);
            if can_retry {
                if let Err(error) = app.simulation_controller.retry_engine_startup() {
                    app.state.record(
                        rspice_app_state::diagnostics::LogSeverity::Error,
                        rspice_app_state::diagnostics::LogSource::Studio,
                        format!("Engine startup could not be retried: {error}"),
                    );
                }
            } else if let Some(rollup) = shown_rollup(app, &readings.availability) {
                open_specification(app, rollup.first_failing);
            } else if let Some(outcome) = run_outcome(app, &readings.availability) {
                rspice_app_results::workbench::docks::inspector::run_details::open(
                    &mut app.state.results.run_details,
                    &app.state.model.simulation,
                    outcome.run,
                );
            } else if can_open {
                app.open_newest_retained_run();
            }
        }),
    },
    StatusCell {
        key: StatusKey::Platform,
        zone: StatusZone::Trailing,
        order: 2,
        workspace: None,
        content: platform_cell,
        activate: None,
    },
];

fn prompt_cell<'a>(_: &RSpiceApp, readings: &'a StatusReadings) -> Option<StatusText<'a>> {
    readings.prompt.as_deref().map(StatusText::label)
}

/// With no prompt showing, the armed symbol drawing tool's next step.
fn symbol_tool_hint_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    symbol_tool_hint(
        &app.state.design.symbol,
        &app.state.model,
        &app.state.workbench,
    )
    .map(StatusText::label)
}

/// The bar reads the checks list once per pass, for both the text and the
/// mark.
fn checks_cell<'a>(app: &RSpiceApp, readings: &'a StatusReadings) -> Option<StatusText<'a>> {
    let counts = listing_in_pass(&app.state, readings.pass).counts();
    Some(StatusText {
        text: Cow::Owned(check_summary(counts)?),
        mark: check_problem(counts).map_or(StatusMark::None, StatusMark::Problem),
        control: StatusControl::Button,
        hover: None,
    })
}

/// With nothing open, the context cell says so whatever the workspace.
fn no_project_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    (!app.state.platform.project_lifecycle.is_open())
        .then(|| StatusText::label("No project loaded"))
}

/// Only a drawn view has coordinates to read out; a source or document view
/// leaves the segment empty rather than filling it.
fn design_context_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    let context = design_engineering_context(
        app.state.design.canvas_hover,
        &app.state.model,
        &app.state.design.symbol,
    );
    (!context.is_empty()).then(|| StatusText::label(context))
}

fn results_context_cell<'a>(_: &RSpiceApp, readings: &'a StatusReadings) -> Option<StatusText<'a>> {
    Some(StatusText::label(results_view_summary(
        readings.results_view.as_ref(),
    )))
}

fn selection_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    let summary = selection_summary(
        &app.state.model,
        &app.state.workbench,
        &app.state.platform,
        &app.state.design.symbol,
    );
    (!summary.is_empty()).then(|| StatusText::label(summary))
}

/// A symbol edited in place says where it stands, in the page's slot: the
/// instance it is edited at and the sheet that places it. Its prompt says so
/// once on the way in and expires; this stays while the session does.
fn in_place_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    let session = in_place_session(&app.state.design.symbol.in_place, &app.state.model)?;
    Some(StatusText {
        text: Cow::Owned(format!(
            "In place on {} \u{00b7} {}",
            session.instance_name, session.parent.cell
        )),
        mark: StatusMark::None,
        control: StatusControl::Label,
        hover: Some(Cow::Owned(format!(
            "Editing the {} symbol where {} stands on {}. Esc or Return goes back to {}.",
            session.symbol.cell, session.instance_name, session.parent.cell, session.parent.cell
        ))),
    })
}

/// The page a multi-sheet drawing is open at reads beside where the cursor
/// is, because both answer where the reader is rather than what is selected.
fn page_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    let (text, detail) = sheet_position_summary(&app.state.model)?;
    Some(StatusText {
        text: Cow::Owned(text),
        mark: StatusMark::None,
        control: StatusControl::Label,
        hover: Some(Cow::Owned(detail)),
    })
}

fn plot_zoom_cell<'a>(app: &RSpiceApp, readings: &'a StatusReadings) -> Option<StatusText<'a>> {
    let text = if let Some(view) = readings.results_view.as_ref() {
        format_plot_zoom(view.zoom)
    } else {
        // A result sheet with nothing plotted still reports in the
        // instrument's unit rather than flipping the chip to a canvas
        // percentage it never uses.
        format_plot_zoom(1.0)
    };
    Some(StatusText {
        text: Cow::Owned(text),
        mark: StatusMark::None,
        control: if app.command_key_enabled(ids::ZOOM_FIT) {
            StatusControl::Button
        } else {
            StatusControl::Label
        },
        hover: Some(Cow::Borrowed("Fit the active result plot")),
    })
}

fn canvas_zoom_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    let zoom = zoom_factor(
        &app.state.design.symbol,
        &app.state.model,
        &app.state.workbench,
    );
    Some(StatusText {
        text: Cow::Owned(format!("{}%", (zoom * 100.0).round())),
        mark: StatusMark::None,
        control: if app.command_key_enabled(ids::ZOOM_ONE_TO_ONE) {
            StatusControl::Button
        } else {
            StatusControl::Label
        },
        hover: Some(Cow::Borrowed("Reset active canvas zoom to 100%")),
    })
}

/// The engine chip reports what the solver is doing; the dataset it produces
/// is the one thing a reader wants next, so the chip is the route to it rather
/// than a label about it. An idle, ready engine is nothing to report, so the
/// bar shows no chip then.
fn engine_cell<'a>(app: &RSpiceApp, readings: &'a StatusReadings) -> Option<StatusText<'a>> {
    let availability = &readings.availability;
    if let Some(rollup) = shown_rollup(app, availability) {
        return Some(StatusText {
            text: Cow::Owned(rollup.text),
            mark: if rollup.failing {
                StatusMark::Problem(Problem::Failure)
            } else {
                StatusMark::None
            },
            control: StatusControl::Button,
            hover: Some(Cow::Owned(format!(
                "Open Run {}'s specifications in Verify",
                rollup.run
            ))),
        });
    }
    let checking = preflight_checking(app);
    let text = engine_words(app, availability);
    let outcome = run_outcome(app, availability);
    let (text, mark) = match (text, &outcome) {
        (Some(text), _) => (
            text,
            engine_status_mark(
                availability,
                app.state.model.simulation.execution.has_active_execution()
                    || checking.is_some()
                    || !app.state.runs.waiting.is_empty(),
            ),
        ),
        (None, Some(outcome)) => (outcome.text.clone(), outcome.mark),
        (None, None) => return None,
    };
    let EnginePress {
        can_retry,
        can_open,
    } = engine_press(app, availability);
    let hover: Cow<'a, str> = if can_retry {
        availability.failure_reason().map_or(
            Cow::Borrowed("Restart the simulation engine. This does not queue a simulation."),
            |reason| {
                Cow::Owned(format!(
                    "{reason}\nRetry engine startup. This does not queue a simulation."
                ))
            },
        )
    } else if !app.state.runs.waiting.is_empty() {
        Cow::Borrowed("Waits for the Verilog-A compile running in the background. Stop drops it.")
    } else if let Some(outcome) = &outcome {
        Cow::Owned(format!("Show Run {}'s details", outcome.run))
    } else if can_open {
        Cow::Borrowed("Open the newest retained result dataset in Results")
    } else {
        Cow::Borrowed(availability.failure_reason().unwrap_or(match availability {
            EngineAvailability::Starting => {
                "The simulation engine is starting. Validated Run requests wait for initialization."
            }
            EngineAvailability::Restartable => {
                "The current execution is finishing its cancellation before the engine can restart."
            }
            _ => "No run has retained a dataset yet, so there is nothing to open in Results.",
        }))
    };
    Some(StatusText {
        text: Cow::Owned(text),
        mark,
        control: StatusControl::Guarded {
            enabled: can_open || can_retry || outcome.is_some(),
        },
        hover: Some(hover),
    })
}

/// How the last run that ended went, as the run segment states it once the
/// engine has nothing of its own to say.
struct RunOutcome {
    /// The run's sequence number.
    run: u64,
    /// `Run 1 complete · 2 warnings · 1.76 s`, or `Run 2 complete ·
    /// 3 analyses · 0.52 s` for a plan of several analyses.
    text: String,
    mark: StatusMark,
}

/// The outcome of the run seen running that ended last, while the engine is
/// idle and ready and no check is preparing another; `None` before any run
/// has ended in this session, or once that run has left the history.
fn run_outcome(app: &RSpiceApp, availability: &EngineAvailability) -> Option<RunOutcome> {
    use rspice_app_state::diagnostics::LogSeverity;
    use rspice_app_state::state::SimulationRunLifecycle as Lifecycle;

    let simulation = &app.state.model.simulation;
    if app.state.runs.preflight.checking.is_some()
        || simulation_engine_status(simulation, availability).is_some()
    {
        return None;
    }
    let run = simulation
        .retained
        .run_by_stable_id(app.state.runs.run_watch.last_ended?)?;
    let count = |severity, one: &str, many: &str| match run.log.count(severity) {
        0 => None,
        1 => Some(format!("1 {one}")),
        count => Some(format!("{count} {many}")),
    };
    let warnings = count(LogSeverity::Warning, "warning", "warnings");
    let errors = count(LogSeverity::Error, "error", "errors");
    let (verdict, raised, mark) = match run.lifecycle {
        // A transient its results storage stopped kept what it wrote; the
        // run says where it stopped and why, before any count.
        Lifecycle::Completed | Lifecycle::Failed if run.results_storage_stop().is_some() => {
            let at = run
                .results_storage_stop()
                .and_then(|stop| stop.at)
                .map(|at| {
                    format!(
                        " at {}s",
                        rspice_app_state::quantity::engineering::format_engineering_display_with(
                            at,
                            rspice_app_state::quantity::engineering::EngineeringPrecision::Adaptive,
                        )
                    )
                })
                .unwrap_or_default();
            (
                Cow::Owned(format!("stopped{at}: results storage full")),
                errors,
                StatusMark::Problem(Problem::Failure),
            )
        }
        Lifecycle::Completed => (
            Cow::Borrowed("complete"),
            warnings.clone(),
            if warnings.is_some() {
                StatusMark::Problem(Problem::Warning)
            } else {
                StatusMark::None
            },
        ),
        Lifecycle::Failed if run.analyses.iter().any(|analysis| analysis.success) => (
            Cow::Borrowed("completed with errors"),
            errors,
            StatusMark::Problem(Problem::Failure),
        ),
        Lifecycle::Failed => (
            Cow::Borrowed("failed"),
            errors,
            StatusMark::Problem(Problem::Failure),
        ),
        Lifecycle::Interrupted => (
            Cow::Borrowed("interrupted"),
            None,
            StatusMark::Problem(Problem::Warning),
        ),
        Lifecycle::Aborted => (Cow::Borrowed("stopped"), None, StatusMark::None),
        _ => return None,
    };
    // A plan of several analyses says how many the run held; one analysis
    // goes without saying.
    let analyses = (run.analyses.len() > 1).then(|| format!("{} analyses", run.analyses.len()));
    let text = std::iter::once(format!("Run {} {verdict}", run.id))
        .chain(analyses)
        .chain(raised)
        .chain(std::iter::once(rspice_app_state::quantity::format_elapsed(
            run.elapsed_time,
        )))
        .collect::<Vec<_>>()
        .join(" \u{b7} ");
    Some(RunOutcome {
        run: run.id,
        text,
        mark,
    })
}

/// Whether a preflight check is building its decks, and how far it is.
fn preflight_checking(app: &RSpiceApp) -> Option<PreflightStage> {
    app.state
        .runs
        .preflight
        .checking
        .as_ref()
        .and(app.simulation_controller.preflight_stage())
}

/// What the engine is doing, when that is worth a word: a check, a run, a
/// start or a failure. An idle, ready engine says nothing.
fn engine_words(app: &RSpiceApp, availability: &EngineAvailability) -> Option<String> {
    match preflight_checking(app) {
        Some(stage) => Some(preflight_status(stage)),
        None => waiting_words(app)
            .or_else(|| simulation_engine_status(&app.state.model.simulation, availability)),
    }
}

/// What waits for a Verilog-A compile before it is prepared: a manual deck's
/// validation or a campaign's start.
fn waiting_words(app: &RSpiceApp) -> Option<String> {
    app.state.runs.waiting.status()
}

/// The active run's specifications as the verification sheet judged them,
/// which the run segment states while the plan has any.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SpecificationRollup {
    /// The judged run's sequence number.
    run: u64,
    text: String,
    /// Some specification fails or errs.
    failing: bool,
    /// The first specification that fails or errs, which pressing the
    /// segment selects.
    first_failing: Option<String>,
}

/// The run segment's roll-up, when the engine has nothing to say and the
/// sheet judges the active run against specifications. A run that ended
/// since, while another is the one judged, keeps its own words.
fn shown_rollup(app: &RSpiceApp, availability: &EngineAvailability) -> Option<SpecificationRollup> {
    if engine_words(app, availability).is_some() {
        return None;
    }
    let simulation = &app.state.model.simulation;
    let run = simulation.active_run()?;
    if app
        .state
        .runs
        .run_watch
        .last_ended
        .is_some_and(|last| last != run.run_id)
    {
        return None;
    }
    let sheet = simulation.verification_sheet()?;
    let words = specification_rollup(sheet)?;
    let counts = sheet.summary.counts;
    Some(SpecificationRollup {
        run: run.id,
        text: format!("Run {} \u{b7} {words}", run.id),
        failing: counts.fail + counts.error > 0,
        first_failing: sheet
            .rows
            .iter()
            .find(|row| matches!(row.status, Some(RowStatus::Fail | RowStatus::Error)))
            .map(|row| row.measurement.clone()),
    })
}

/// "3 of 4 specifications pass · 1 fails": the sheet's own counts, with the
/// near ones passing; `None` when it judges no specification.
fn specification_rollup(sheet: &Sheet) -> Option<String> {
    if !sheet.is_judged() || sheet.rows.is_empty() {
        return None;
    }
    let counts = sheet.summary.counts;
    let passing = counts.pass + counts.near;
    let mut parts = vec![match sheet.rows.len() {
        1 => format!("{passing} of 1 specification passes"),
        total => format!("{passing} of {total} specifications pass"),
    }];
    if counts.fail > 0 {
        parts.push(format!(
            "{} {}",
            counts.fail,
            if counts.fail == 1 { "fails" } else { "fail" }
        ));
    }
    if counts.error > 0 {
        parts.push(format!(
            "{} error{}",
            counts.error,
            if counts.error == 1 { "" } else { "s" }
        ));
    }
    Some(parts.join(" \u{b7} "))
}

impl RSpiceApp {
    /// Open Results on a run a press of Run finished, in the results feature's
    /// context, then answer what it raised.
    pub(crate) fn open_finished_run(&mut self, run_sequence: u64) {
        ResultsFeature::open_finished_run(&mut features::cx(self), run_sequence);
        self.after_contribution();
    }

    /// Open Results on the newest run that holds a dataset, in the results
    /// feature's context, then answer what it raised; false when there is
    /// none.
    pub(crate) fn open_newest_retained_run(&mut self) -> bool {
        let opened = ResultsFeature::open_newest_retained_run(&mut features::cx(self));
        self.after_contribution();
        opened
    }
}

/// What pressing the engine chip can do: restart an engine that stopped or
/// failed, or open the dataset a ready engine last retained.
struct EnginePress {
    can_retry: bool,
    can_open: bool,
}

fn engine_press(app: &RSpiceApp, availability: &EngineAvailability) -> EnginePress {
    let can_retry = (*availability == EngineAvailability::Restartable
        || availability.failure_reason().is_some())
        && !app.state.model.simulation.execution.has_active_execution();
    let can_open = *availability == EngineAvailability::Ready
        && app.state.platform.project_lifecycle.is_open()
        && app
            .state
            .model
            .simulation
            .retained
            .newest_retained_result_run_index()
            .is_some();
    EnginePress {
        can_retry,
        can_open,
    }
}

fn platform_cell<'a>(_: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    Some(StatusText {
        text: Cow::Owned(platform_label()),
        mark: StatusMark::Processor,
        control: StatusControl::Label,
        hover: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cells read, left to right, as they did when the bar named them
    /// itself: the prompt, the checks, where the reader is, what is selected
    /// and the page; and from the right edge in, the zoom, the engine and the
    /// platform.
    #[test]
    fn the_status_cells_keep_todays_order() {
        let laid_out = |zone: StatusZone| {
            let mut keys = STATUS_ORDER
                .iter()
                .filter(|cell| cell.zone == zone)
                .map(|cell| (cell.order, cell.key))
                .collect::<Vec<_>>();
            keys.dedup();
            keys
        };
        assert_eq!(
            laid_out(StatusZone::Leading),
            [
                (0, StatusKey::Prompt),
                (1, StatusKey::Checks),
                (2, StatusKey::Context),
                (3, StatusKey::Selection),
                (4, StatusKey::Page),
            ]
        );
        assert_eq!(
            laid_out(StatusZone::Trailing),
            [
                (0, StatusKey::Zoom),
                (1, StatusKey::Engine),
                (2, StatusKey::Platform),
            ]
        );
    }

    /// Results states the run once, in the run segment at the bar's right,
    /// which counts a plan's analyses when the run held several.
    #[test]
    fn results_state_the_run_once() {
        use rspice_app_state::state::{
            AnalysisResult, AnalysisType, SimulationRun, SimulationRunLifecycle,
        };
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        run.add_analysis(AnalysisResult::new(2, AnalysisType::Ac, "AC"));
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.elapsed_time = 0.52;
        let (run_id, elapsed) = (run.run_id, run.elapsed_time);
        app.state.model.simulation.retained.runs = vec![run].into();
        assert!(app.state.model.simulation.select_run(0));
        app.state.runs.run_watch.last_ended = Some(run_id);
        app.state.workbench.activate(Workspace::Results);
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);

        let readings = status_readings(&mut app.state, EngineAvailability::Ready, &ctx);

        let mut placed = std::collections::BTreeSet::new();
        let said = STATUS_ORDER
            .iter()
            .filter(|cell| {
                cell.workspace
                    .is_none_or(|workspace| workspace == Workspace::Results)
            })
            .filter_map(|cell| {
                let text = (cell.content)(&app, &readings)?;
                placed
                    .insert((cell.zone == StatusZone::Leading, cell.order))
                    .then(|| text.text.into_owned())
            })
            .filter(|text| text.contains("Run 1"))
            .collect::<Vec<_>>();
        let elapsed = rspice_app_state::quantity::format_elapsed(elapsed);
        assert_eq!(
            said,
            [format!("Run 1 complete \u{b7} 2 analyses \u{b7} {elapsed}")]
        );
    }

    /// A transient its results storage stopped kept what it wrote, so its
    /// run completed; the run segment says where it stopped and why, in
    /// plain text with the failure mark, before the run's count of errors.
    #[test]
    fn the_run_segment_says_a_full_results_storage_stopped_the_run() {
        use rspice_app_state::state::{
            AnalysisResult, AnalysisType, ResultsStorageStop, SimulationRun, SimulationRunLifecycle,
        };
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        run.stop_on_full_results_storage(ResultsStorageStop {
            analysis: "TRAN".to_owned(),
            at: Some(2.31e-3),
            rows: 2_310_004,
            bytes: 1 << 30,
            free: Some(84 << 20),
            folder: "/Volumes/X/rspice-results/7-1-0".to_owned(),
        });
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.elapsed_time = 0.52;
        let (run_id, elapsed) = (run.run_id, run.elapsed_time);
        app.state.model.simulation.retained.runs = vec![run].into();
        app.state.runs.run_watch.last_ended = Some(run_id);

        let outcome = run_outcome(&app, &EngineAvailability::Ready).expect("the run ended");

        let elapsed = rspice_app_state::quantity::format_elapsed(elapsed);
        assert_eq!(
            outcome.text,
            format!("Run 1 stopped at 2.31ms: results storage full \u{b7} {elapsed}")
        );
        assert_eq!(outcome.mark, StatusMark::Problem(Problem::Failure));
    }

    /// A source view has no coordinates to read out, and the project's
    /// revision counter is nothing a reader acts on: no cell fills the slot
    /// with it.
    #[test]
    fn a_source_view_reports_no_revision() {
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        let mut cell = rspice_app_state::state::Cell::new("amp");
        cell.add_view(rspice_app_state::state::View::new(
            "veriloga",
            rspice_app_state::state::ViewType::VerilogA,
        ));
        let mut library = rspice_app_state::state::Library::new("analog");
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state
            .open_workspace_view(rspice_app_state::state::CellViewRef::new(
                "analog", "amp", "veriloga",
            ));
        app.state.workbench.activate(Workspace::Design);
        assert_eq!(
            app.state.model.workspace.content.active_view_type(),
            Some(rspice_app_state::state::ViewType::VerilogA)
        );
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);

        let readings = status_readings(&mut app.state, EngineAvailability::Ready, &ctx);

        let said = STATUS_ORDER
            .iter()
            .filter(|cell| {
                cell.workspace
                    .is_none_or(|workspace| workspace == Workspace::Design)
            })
            .filter_map(|cell| (cell.content)(&app, &readings))
            .map(|status| status.text.into_owned())
            .collect::<Vec<_>>();
        assert!(
            !said.iter().any(|text| text.contains("revision")),
            "{said:?}"
        );
    }

    /// The engine segment as the bar would show it, with a ready, idle
    /// engine.
    fn run_segment(app: &mut RSpiceApp) -> Option<(String, StatusMark)> {
        let ctx = egui::Context::default();
        let readings = status_readings(&mut app.state, EngineAvailability::Ready, &ctx);
        engine_cell(app, &readings).map(|status| (status.text.into_owned(), status.mark))
    }

    /// After a run of a plan with specifications, the run segment states the
    /// sheet's roll-up: neutral while every specification passes, marked
    /// failing when one fails. With no specifications it says nothing.
    #[test]
    fn the_run_segment_states_the_specifications_roll_up() {
        use rspice_app_simulate::workbench::surfaces::verify::raster::{seed_corner, seed_run_set};

        let mut app = RSpiceApp::test_instance();
        assert_eq!(run_segment(&mut app), None);

        seed_corner(&mut app.state.model);
        let run = app.state.model.simulation.active_run().map(|run| run.id);
        let run = run.expect("the corner run is active");
        assert_eq!(
            run_segment(&mut app),
            Some((
                format!("Run {run} \u{b7} 3 of 4 specifications pass \u{b7} 1 fails"),
                StatusMark::Problem(Problem::Failure)
            ))
        );

        let mut app = RSpiceApp::test_instance();
        seed_run_set(&mut app.state.model, &["TT"], &[1.8], &[27.0]);
        let run = app.state.model.simulation.active_run().map(|run| run.id);
        assert_eq!(
            run_segment(&mut app),
            Some((
                format!(
                    "Run {} \u{b7} 2 of 2 specifications pass",
                    run.expect("the sweep is active")
                ),
                StatusMark::None
            ))
        );
    }

    /// Pressing the roll-up opens Verify on Specifications with the failing
    /// specification selected.
    #[test]
    fn pressing_the_roll_up_opens_the_failing_specification() {
        use rspice_app_simulate::workbench::surfaces::verify::raster::seed_corner;
        use rspice_app_state::state::VerificationPage;

        let mut app = RSpiceApp::test_instance();
        seed_corner(&mut app.state.model);
        app.state.workbench.activate(Workspace::Results);
        let ctx = egui::Context::default();
        let readings = status_readings(&mut app.state, EngineAvailability::Ready, &ctx);
        let engine = STATUS_ORDER
            .iter()
            .find(|cell| cell.key == StatusKey::Engine)
            .and_then(|cell| cell.activate)
            .expect("the engine segment answers a press");
        engine(&mut app, &readings);

        assert_eq!(app.state.workbench.workspace, Workspace::Verify);
        assert_eq!(
            app.state.simulate.verification_page,
            VerificationPage::Specifications
        );
        assert_eq!(
            app.state.simulate.selected_specification.as_deref(),
            Some("gain")
        );
    }
}
