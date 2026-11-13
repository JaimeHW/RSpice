//! Examples: the demonstration circuits, each opened as a project of its own.
//!
//! The circuits are the ones `workbench::examples` draws through the
//! schematic's own API, which until this dialog only the tests reached.
//! Opening one replaces the session with a new project named after it, draws
//! the circuit as its top cell, and saves it where the reader says: a file
//! they choose on the desktop, this browser's storage on the web. While the
//! open project has unsaved changes, Open asks to save them first, as New
//! project does.

use egui::{Context, Sense, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize};
use crate::workbench::app_state::{AppState, Model, PlatformState};
use crate::workbench::framework::feature::{Cx, ModelsPort};
use crate::workbench::project::ProjectFeature;
use crate::workbench::slots::ProjectState;
use crate::workbench::workflows::project_workflow::NewProjectParams;
use rspice_app_state::workbench::examples::{EXAMPLES, Example, load_example};

/// The name column every row shares.
const NAME_COLUMN: f32 = 220.0;
/// How far a row's highlight reaches past its words on either side: the
/// words keep the dialog's left edge, and the highlight runs out to the
/// frame, as a table row's fill does.
const HIGHLIGHT_BLEED: f32 = crate::ui::widgets::DIALOG_BODY_INSET;

/// Replace the session with a new project holding `example`, named after it.
///
/// The project is not saved: [`show`] asks where once this succeeds.
pub(crate) fn create_example_project(state: &mut AppState, example: Example) -> Result<(), String> {
    let params = NewProjectParams {
        name: example.name.to_owned(),
        ..NewProjectParams::default()
    };
    state.create_new_project_with(&params)?;
    load_example(example.name, &mut state.model.schematic);
    state.sync_active_schematic_to_workspace();
    Ok(())
}

/// Open `name` as a new project, then ask where to save it.
fn open_example(state: &mut AppState, name: &str) {
    if let Some(example) = EXAMPLES.into_iter().find(|example| example.name == name)
        && create_example_project(state, example).is_ok()
    {
        state.save_project_as();
    }
}

/// Open the example New project's prompt let through, once the project it
/// replaces was saved or let go.
pub(crate) fn open_confirmed(state: &mut AppState) {
    if let Some(name) = state.project.examples_confirmed.take() {
        open_example(state, &name);
    }
}

/// Open `example` in place of the open project, asking first about any
/// changes it has, as New project does.
pub(crate) fn request_example(state: &mut AppState, example: Example) {
    let unsaved = state.platform.project_lifecycle.is_open()
        && (crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state)
            || ModelsPort::model_editor_has_unsaved_changes(&*state.models));
    if unsaved {
        state
            .workbench
            .confirmation_dialog
            .show_example(example.name);
    } else {
        open_example(state, example.name);
    }
}

/// Why Open is refused, if it is: configuration edits not applied, which
/// no save prompt resolves.
fn open_refusal(
    project: &ProjectState,
    model: &Model,
    platform: &PlatformState,
) -> Option<&'static str> {
    if !platform.project_lifecycle.is_open() {
        return None;
    }
    project
        .configuration
        .is_dirty(&model.workspace.content.configuration_sets)
        .then_some("Apply or revert the configuration edits first: an example replaces the project")
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
            painter.rect_filled(highlight, 0.0, t.color.bg_active);
        } else if response.hovered() {
            painter.rect_filled(highlight, 0.0, t.color.bg_hover);
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
    // The selection fill already shows which row a click chose; the ring is
    // for a reader who tabbed here.
    if theme::focus_is_visible(ui.ctx(), &response) {
        theme::paint_focus_ring(ui, &response, row_geometry(rect).0);
    }
    response
}

/// The Examples dialog, while it is open. It returns the example the reader
/// asked to open, for the shell to request in place of the open project.
pub(crate) fn show(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) -> Option<Example> {
    let mut selected = cx.state.examples_selection?;
    selected = selected.min(EXAMPLES.len() - 1);
    let refusal = open_refusal(cx.state, cx.model, cx.platform);
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
    cx.state.examples_selection = Some(selected);
    match choice {
        DialogChoice::Primary => open = true,
        DialogChoice::None => {}
        _ => {
            cx.state.examples_selection = None;
            return None;
        }
    }
    if open {
        cx.state.examples_selection = None;
        return Some(EXAMPLES[selected]);
    }
    None
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use crate::state::AnalysisType;
    use crate::workbench::app::RSpiceApp;
    use crate::workbench::simulate::commands::SimulateCommand;

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
                .runs
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
                app.state.model.simulation.execution.trigger_simulation,
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
            let execution = &app.state.model.simulation.execution;
            if !execution.has_active_execution() && !execution.trigger_simulation {
                break;
            }
            assert!(Instant::now() < deadline, "{} timed out", example.name);
            std::thread::sleep(Duration::from_millis(5));
        }
        let run = app
            .state
            .model
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
            app.state.model.workspace.content.project.display_name(),
            example.name
        );
        assert!(
            !app.state.model.schematic.document().components.is_empty(),
            "{} draws its circuit",
            example.name
        );
        // The default plan runs a millisecond at 10 ns steps; the engine
        // checks the same plan over a shorter window.
        app.state.model.sim_setup.tran.step = "1n".to_owned();
        app.state.model.sim_setup.tran.stop = stop.to_owned();
        // The frame after the project opens, on which the controller lets go
        // of the design it replaced; a check started before it would be
        // dropped with that design.
        app.simulation_controller
            .update(&mut app.state, &*app.export_workflow_io);
        assert!(app.command_key_enabled(SimulateCommand::Run.key()));
        app.execute_command_key(SimulateCommand::Run.key());
        assert!(app.state.runs.preflight.take_run_and_queue_request());
        crate::workbench::preflight::start(&mut app.state, &mut app.simulation_controller, true);
        app.settle_preflight();
        let blockers = app
            .state
            .runs
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

    /// Open stays offered while the open project has unsaved changes, and
    /// asks about them, as New project does, instead of replacing them.
    #[test]
    fn an_example_asks_to_save_unsaved_work_before_replacing_it() {
        let mut state = AppState::default();
        state.open_project_session();
        load_example("Voltage Divider", &mut state.model.schematic);
        state.sync_active_schematic_to_workspace();
        assert!(open_refusal(&state.project, &state.model, &state.platform).is_none());
        let components = state.model.schematic.document().components.len();
        request_example(&mut state, EXAMPLES[0]);
        let confirmation = &state.workbench.confirmation_dialog;
        assert!(confirmation.visible);
        assert_eq!(
            confirmation.pending_action,
            Some(crate::workbench::app::ConfirmationAction::ProjectNew)
        );
        assert_eq!(
            confirmation.pending_example.as_deref(),
            Some(EXAMPLES[0].name)
        );
        assert_eq!(
            state.model.schematic.document().components.len(),
            components
        );
    }

    /// Open hands the shell the example the reader selected, in the frame it
    /// was pressed in, and the dialog closes; the dialog itself replaces
    /// nothing.
    #[test]
    fn open_hands_back_the_selected_example_and_closes_the_dialog() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.project.examples_selection = Some(1);
        let components = app.state.model.schematic.document().components.len();
        let frame = |app: &mut RSpiceApp, events: Vec<egui::Event>| {
            let mut asked = None;
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        vec2(1440.0, 900.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| asked = show(ui.ctx(), &mut crate::workbench::features::cx(app)),
            );
            (output, asked)
        };
        let mut settled = frame(&mut app, Vec::new());
        for _ in 0..2 {
            settled = frame(&mut app, Vec::new());
        }
        assert_eq!(settled.1, None);
        let bounds = settled
            .0
            .platform_output
            .accesskit_update
            .expect("the settled frame publishes its tree")
            .nodes
            .iter()
            .find_map(|(_, node)| (node.label()? == "Open").then_some(node.bounds()?))
            .expect("the dialog announces Open");
        let at = egui::pos2(
            ((bounds.x0 + bounds.x1) / 2.0) as f32,
            ((bounds.y0 + bounds.y1) / 2.0) as f32,
        );
        let press = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let (_, asked) = frame(
            &mut app,
            vec![egui::Event::PointerMoved(at), press(true), press(false)],
        );

        assert_eq!(asked, Some(EXAMPLES[1]));
        assert_eq!(app.state.project.examples_selection, None);
        assert_eq!(
            app.state.model.schematic.document().components.len(),
            components
        );
    }
}
