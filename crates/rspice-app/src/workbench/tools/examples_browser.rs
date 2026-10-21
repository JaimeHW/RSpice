//! Examples: the demonstration circuits, each opened as a project of its own.
//!
//! The circuits are the ones `workbench::examples` draws through the
//! schematic's own API, which until this dialog only the tests reached.
//! Opening one replaces the session with a new project named after it, draws
//! the circuit as its top cell, and saves it where the reader says: a file
//! they choose on the desktop, this browser's storage on the web. While the
//! open project has unsaved changes it refuses, and says so, rather than
//! replace them.

use egui::{Context, Sense, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize};
use crate::workbench::app_state::AppState;
use crate::workbench::examples::{EXAMPLES, Example, load_example};
use crate::workbench::workflows::project_workflow::{
    NewProjectParams, create_new_project_with, save_project_as,
};

/// The name column every row shares.
const NAME_COLUMN: f32 = 220.0;
/// How far a row's highlight reaches past its words on either side: the
/// words keep the dialog's left edge, and the highlight stands proud of it.
const HIGHLIGHT_BLEED: f32 = 6.0;

/// Replace the session with a new project holding `example`, named after it.
///
/// The project is not saved: [`show`] asks where once this succeeds.
pub(crate) fn create_example_project(state: &mut AppState, example: Example) -> Result<(), String> {
    let params = NewProjectParams {
        name: example.name.to_owned(),
        ..NewProjectParams::default()
    };
    create_new_project_with(state, &params)?;
    load_example(example.name, &mut state.schematic);
    state.sync_active_schematic_to_workspace();
    Ok(())
}

/// Why Open is refused, if it is.
fn open_refusal(state: &AppState) -> Option<&'static str> {
    (state.project_lifecycle.is_open()
        && crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state))
    .then_some("Save or close the open project first: an example replaces it")
}

/// Where a row of `rect` paints: its highlight, and the x its name starts
/// at, which is the dialog's left edge.
fn row_geometry(rect: egui::Rect) -> (egui::Rect, f32) {
    (rect.expand2(vec2(HIGHLIGHT_BLEED, 0.0)), rect.left())
}

/// One example in the list: its name, then what it is.
fn example_row(ui: &mut egui::Ui, example: Example, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), t.metrics.ctl_h + tokens::SP_4),
        Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            true,
            selected,
            format!("{}: {}", example.name, example.summary),
        )
    });
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let (highlight, name_left) = row_geometry(rect);
        if selected {
            painter.rect_filled(highlight, t.radius, t.color.bg_active);
        } else if response.hovered() {
            painter.rect_filled(highlight, t.radius, t.color.bg_hover);
        }
        painter.text(
            egui::pos2(name_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            example.name,
            theme::sans(tokens::FS_1, FontWeight::Medium),
            t.color.text,
        );
        painter.text(
            egui::pos2(name_left + NAME_COLUMN, rect.center().y),
            egui::Align2::LEFT_CENTER,
            example.summary,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            t.color.text_dim,
        );
    }
    response
}

/// The Examples dialog, while it is open.
pub(crate) fn show(ctx: &Context, state: &mut AppState) {
    let Some(mut selected) = state.workbench.examples_selection else {
        return;
    };
    selected = selected.min(EXAMPLES.len() - 1);
    let refusal = open_refusal(state);
    let (hint, tone) = match refusal {
        Some(refusal) => (refusal, DialogHintTone::Warn),
        None if cfg!(target_arch = "wasm32") => (
            "Opens as a new project saved in this browser",
            DialogHintTone::Neutral,
        ),
        None => (
            "Opens as a new project you save where you choose",
            DialogHintTone::Neutral,
        ),
    };
    let mut open = false;
    let choice = Dialog::new("Examples", "Open")
        .description("Open a demonstration circuit as a new project of its own.")
        .size(DialogSize::Transaction)
        // Six rows never scroll: no gutter cutting the rows short of the
        // right edge.
        .overlay_scrollbar()
        .ghost("Cancel")
        .primary_enabled(refusal.is_none())
        .hint(hint)
        .hint_tone(tone)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = tokens::SP_2;
            for (index, example) in EXAMPLES.into_iter().enumerate() {
                let response = example_row(ui, example, index == selected);
                if response.clicked() {
                    selected = index;
                }
                if response.double_clicked() && refusal.is_none() {
                    selected = index;
                    open = true;
                }
            }
            if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
                selected = (selected + 1).min(EXAMPLES.len() - 1);
            }
            if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
                selected = selected.saturating_sub(1);
            }
        });
    state.workbench.examples_selection = Some(selected);
    match choice {
        DialogChoice::Primary => open = true,
        DialogChoice::None => {}
        _ => {
            state.workbench.examples_selection = None;
            return;
        }
    }
    if open {
        state.workbench.examples_selection = None;
        if create_example_project(state, EXAMPLES[selected]).is_ok() {
            save_project_as(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::state::AnalysisType;
    use crate::workbench::app::RSpiceApp;
    use crate::workbench::commands::vocabulary::Command;

    /// Every example opens as a project whose default plan the engine
    /// prepares and authorizes, through the path the toolbar's Run takes:
    /// netlisted, its models bound, its decks built and checked, and the run
    /// queued. Running each to its end is left to the test below: in an
    /// unoptimized build that takes from seconds to minutes per example.
    #[test]
    fn every_example_opens_as_a_project_whose_default_analysis_is_authorized() {
        for example in EXAMPLES {
            let app = open_and_press_run(example, "10n");
            let report = app
                .state
                .workbench
                .preflight
                .report
                .as_ref()
                .expect("Run retains its report");
            assert!(
                report.prepared.is_some(),
                "{}: the engine prepared the run",
                example.name
            );
            assert!(
                app.state.simulation.execution.trigger_simulation,
                "{}: the authorized run is queued",
                example.name
            );
        }
    }

    /// An example opened as a project runs to its transient result.
    #[test]
    fn an_example_project_runs_to_its_result() {
        let example = EXAMPLES[0];
        let mut app = open_and_press_run(example, "10u");
        // What the frame loop does each frame while the run is dispatched
        // and collected.
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            app.simulation_controller
                .update(&mut app.state, &*app.export_workflow_io);
            let execution = &app.state.simulation.execution;
            if !execution.has_active_execution() && !execution.trigger_simulation {
                break;
            }
            assert!(Instant::now() < deadline, "{} timed out", example.name);
            std::thread::sleep(Duration::from_millis(5));
        }
        let run = app
            .state
            .simulation
            .retained
            .runs
            .last()
            .expect("the run is retained");
        assert!(
            run.analyses.iter().any(|analysis| {
                analysis.analysis_type == AnalysisType::Transient
                    && analysis.success
                    && !analysis.waveforms.is_empty()
            }),
            "{}: {:?}",
            example.name,
            run.analyses
                .iter()
                .map(|analysis| &analysis.error_message)
                .collect::<Vec<_>>()
        );
    }

    /// Open `example` as a project and press Run, as the toolbar does, with
    /// its transient cut to `stop`; preflight has settled on return.
    fn open_and_press_run(example: Example, stop: &str) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        create_example_project(&mut app.state, example).expect("the example opens as a project");
        assert_eq!(
            app.state.workspace.content.project.display_name(),
            example.name
        );
        assert!(
            !app.state.schematic.document().components.is_empty(),
            "{} draws its circuit",
            example.name
        );
        // The default plan runs a millisecond at 10 ns steps; the engine
        // checks the same plan over a shorter window.
        app.state.sim_setup.tran.step = "1n".to_owned();
        app.state.sim_setup.tran.stop = stop.to_owned();
        // The frame after the project opens, on which the controller lets go
        // of the design it replaced; a check started before it would be
        // dropped with that design.
        app.simulation_controller
            .update(&mut app.state, &*app.export_workflow_io);
        assert!(Command::RunSimulation.is_enabled(&app));
        Command::RunSimulation.execute(&mut app);
        assert!(app.state.workbench.preflight.take_run_and_queue_request());
        crate::workbench::preflight::start(&mut app.state, &mut app.simulation_controller, true);
        app.settle_preflight();
        let blockers = app
            .state
            .workbench
            .preflight
            .report
            .as_ref()
            .map(|report| report.blockers.clone())
            .unwrap_or_default();
        assert!(
            blockers.is_empty(),
            "{} is blocked: {blockers:#?}",
            example.name
        );
        app
    }

    /// An example's name starts on the dialog's left edge, the title's, and
    /// its highlight stands proud of the row on both sides rather than
    /// pushing the words in.
    #[test]
    fn an_example_row_keeps_the_dialog_left_edge() {
        let rect = egui::Rect::from_min_size(egui::pos2(353.0, 0.0), vec2(734.0, 36.0));
        let (highlight, name_left) = row_geometry(rect);
        assert_eq!(name_left, rect.left());
        assert!(highlight.left() < rect.left() && highlight.right() > rect.right());
        assert_eq!(highlight.y_range(), rect.y_range());
    }

    /// Opening an example refuses while the open project has unsaved
    /// changes, and says why, instead of replacing them.
    #[test]
    fn an_example_does_not_replace_unsaved_work() {
        let mut state = AppState::default();
        state.project_lifecycle.authority.open_session();
        load_example("Voltage Divider", &mut state.schematic);
        state.sync_active_schematic_to_workspace();
        assert!(open_refusal(&state).is_some());
    }
}
