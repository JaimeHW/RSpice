//! The status bar's cells: what each one reports, and the order the bar lays
//! them out in.
//!
//! The bar measures and paints; every cell here only says what it shows. A
//! zone's place is taken by the first cell listed for it that applies to the
//! workspace showing and has something to say, so a cell for one workspace is
//! listed before the cell that answers for all the others.

use std::borrow::Cow;

use crate::simulation::status::{EngineAvailability, preflight_status, simulation_engine_status};
use crate::ui::tokens::Tokens;
use crate::workbench::app_state::diagnostic_store::listing::listing_in_pass;
use crate::workbench::chrome::status_bar::design::{
    design_engineering_context, selection_summary, zoom_factor,
};
use crate::workbench::chrome::status_bar::results::{format_plot_zoom, results_view_summary};
use crate::workbench::chrome::status_bar::sheets::sheet_position_summary;
use crate::workbench::chrome::status_bar::shell::platform_label;
use crate::workbench::chrome::status_bar::{
    check_problem, check_summary, current_prompt, engine_status_mark,
};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::documents::result_document::{SharedXStatus, active_shared_x_status};
use crate::workbench::framework::host::{
    StatusCell, StatusControl, StatusKey, StatusMark, StatusText, StatusZone,
};
use crate::workbench::state::Workspace;
use crate::workbench::{AppState, RSpiceApp};

/// What the cells read in common on a frame, worked out before any of them.
pub(crate) struct StatusReadings {
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
        .then(|| active_shared_x_status(&t, state))
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
        key: StatusKey::Checks,
        zone: StatusZone::Leading,
        order: 1,
        workspace: None,
        content: checks_cell,
        activate: Some(|app, _| Command::OpenProblems.execute(app)),
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
        key: StatusKey::Context,
        zone: StatusZone::Leading,
        order: 2,
        workspace: None,
        content: revision_cell,
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
        activate: Some(|app, _| Command::ZoomFit.execute(app)),
    },
    StatusCell {
        key: StatusKey::Zoom,
        zone: StatusZone::Trailing,
        order: 0,
        workspace: None,
        content: canvas_zoom_cell,
        activate: Some(|app, _| Command::ZoomOneToOne.execute(app)),
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
                        crate::diagnostics::LogSeverity::Error,
                        crate::diagnostics::LogSource::Studio,
                        format!("Engine startup could not be retried: {error}"),
                    );
                }
            } else if can_open {
                crate::workbench::commands::result_navigation::open_newest_retained_run(app);
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

fn design_context_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    Some(StatusText::label(design_engineering_context(&app.state)))
}

fn results_context_cell<'a>(
    app: &RSpiceApp,
    readings: &'a StatusReadings,
) -> Option<StatusText<'a>> {
    Some(StatusText::label(results_view_summary(
        app,
        readings.results_view.as_ref(),
    )))
}

fn revision_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    Some(StatusText::label(format!(
        "revision {}",
        app.state.model.workspace.content.project.revision().get()
    )))
}

fn selection_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    Some(StatusText::label(selection_summary(app)))
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
        control: if Command::ZoomFit.is_enabled(app) {
            StatusControl::Button
        } else {
            StatusControl::Label
        },
        hover: Some(Cow::Borrowed("Fit the active result plot")),
    })
}

fn canvas_zoom_cell<'a>(app: &RSpiceApp, _: &'a StatusReadings) -> Option<StatusText<'a>> {
    Some(StatusText {
        text: Cow::Owned(format!("{}%", (zoom_factor(app) * 100.0).round())),
        mark: StatusMark::None,
        control: if Command::ZoomOneToOne.is_enabled(app) {
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
    let checking = app
        .state
        .runs
        .preflight
        .checking
        .as_ref()
        .and(app.simulation_controller.preflight_stage());
    let text = match checking {
        Some(stage) => Some(preflight_status(stage)),
        None => simulation_engine_status(&app.state.model.simulation, availability),
    }?;
    let mark = engine_status_mark(
        availability,
        app.state.model.simulation.execution.has_active_execution() || checking.is_some(),
    );
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
            enabled: can_open || can_retry,
        },
        hover: Some(hover),
    })
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
}
