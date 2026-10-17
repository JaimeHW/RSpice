//! Following a console line's anchor.
//!
//! Split from the page because it is a different concern from painting a
//! line: everything here is about whether a line's target still exists and
//! what happens when the reader commits to it. A run's line opens that run's
//! log on the page itself; any other anchor leaves the page for the drawing or
//! symbol it names, which is the session's to do once the page has drawn.

use egui::Ui;

use super::AppState;
use crate::diagnostics::LogAnchor;
use crate::workbench::state::{ConsoleChannel, ConsoleView};

/// Follow `anchor`, from a double-click, Enter or the context menu.
///
/// A jump off the page is handed back through `jump`: the page draws from the
/// session immutably, and navigating mutates it. Returns whether following
/// leaves the page. An anchor whose target is gone does nothing; the line said
/// so under the pointer before the reader committed.
pub(super) fn follow(
    state: &AppState,
    view: &mut ConsoleView,
    anchor: &LogAnchor,
    jump: &mut Option<LogAnchor>,
) -> bool {
    if state.log_anchor_refusal(anchor).is_some() {
        return false;
    }
    match anchor {
        LogAnchor::ResultRun { run_sequence } => {
            view.open(ConsoleChannel::Run(*run_sequence));
            false
        }
        other => {
            *jump = Some(other.clone());
            true
        }
    }
}

/// The context menu's item for a line's anchor. Returns whether choosing it
/// leaves the page.
pub(super) fn menu(
    ui: &mut Ui,
    state: &AppState,
    view: &mut ConsoleView,
    anchor: &LogAnchor,
    jump: &mut Option<LogAnchor>,
) -> bool {
    let label = match anchor {
        LogAnchor::ResultRun { .. } => "Open run log",
        LogAnchor::Symbol { .. } => "Show in symbol view",
        LogAnchor::Schematic { .. } | LogAnchor::Simulation { .. } => "Show on schematic",
    };
    let refusal = state.log_anchor_refusal(anchor);
    let item = ui.add_enabled(refusal.is_none(), egui::Button::new(label));
    if let Some(refusal) = refusal {
        item.on_disabled_hover_text(refusal);
        return false;
    }
    if !item.clicked() {
        return false;
    }
    ui.close();
    follow(state, view, anchor, jump)
}

/// What following this line's anchor will do, in the words of the surface it
/// lands on.
pub(super) fn hint(anchor: &LogAnchor) -> String {
    match anchor {
        LogAnchor::Schematic { .. } => "Double-click to show on the schematic".to_owned(),
        LogAnchor::Symbol { pin_name, .. } => {
            format!("Double-click to show pin {pin_name} in the symbol view")
        }
        LogAnchor::Simulation { nets, devices } => {
            let count = nets.len() + devices.len();
            let objects = if count == 1 { "object" } else { "objects" };
            format!("Double-click to mark the {count} {objects} this run named on the schematic")
        }
        LogAnchor::ResultRun { run_sequence } => {
            format!("Double-click to open the log of Run {run_sequence}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::transcript::console;
    use super::*;
    use crate::diagnostics::LogSource;
    use crate::state::SimulationRun;
    use crate::workbench::RSpiceApp;
    use egui::Vec2;

    /// Where the console painted the row whose message contains `needle`.
    ///
    /// Rows are painted galleys rather than widgets, so there is no widget id
    /// to look a rect up by. The text the row drew is the only handle a test
    /// has on it, which is the same handle a reader has.
    fn console_row_position(ctx: &egui::Context, state: &mut AppState, needle: &str) -> egui::Pos2 {
        fn scan(shape: &egui::epaint::Shape, needle: &str, found: &mut Option<egui::Pos2>) {
            match shape {
                egui::epaint::Shape::Text(text) if found.is_none() => {
                    if text.galley.job.text.contains(needle) {
                        *found = Some(text.pos + text.galley.size() * 0.5);
                    }
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        scan(shape, needle, found);
                    }
                }
                _ => {}
            }
        }

        let output = ctx.run_ui(console_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        });
        let mut found = None;
        for clipped in &output.shapes {
            scan(&clipped.shape, needle, &mut found);
        }
        found.unwrap_or_else(|| panic!("the console painted no row containing {needle:?}"))
    }

    fn console_context_for_tests() -> egui::Context {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx
    }

    fn console_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(900.0, 600.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn press(position: egui::Pos2) -> Vec<egui::Event> {
        let button = |pressed| egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        vec![
            egui::Event::PointerMoved(position),
            button(true),
            button(false),
        ]
    }

    /// Double-click the console row whose message contains `needle`.
    ///
    /// egui resolves a press against the widget rects the previous frame
    /// registered, so the rows are laid out first; the two presses then land
    /// in successive frames, inside the double-click interval.
    fn double_click_console_row(state: &mut AppState, needle: &str) {
        let ctx = console_context_for_tests();
        let position = console_row_position(&ctx, state, needle);
        for _ in 0..2 {
            let _ = ctx.run_ui(console_input(press(position)), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
            });
        }
    }

    /// A sheet drawing the conductor `OUT`, with a cross-probe map captured at
    /// the topology it is currently drawn at.
    fn state_with_probed_conductor() -> AppState {
        use crate::state::{Point, Wire};
        use std::collections::HashMap;

        let mut state = AppState::default();
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(91, vec![a, b]));
        state.simulation.source.cross_probe.update(
            state.workspace.content.active_view.clone(),
            HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            HashMap::from([("OUT".to_owned(), vec![a, b])]),
            HashMap::new(),
            state.schematic.topology_version(),
        );
        state.log_buffer.clear();
        state
    }

    /// The whole point of an anchored line: the objects a failed run named
    /// are a double-click from the sentence that named them.
    #[test]
    fn double_clicking_an_anchored_console_row_marks_what_the_run_named() {
        let mut state = state_with_probed_conductor();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Error,
            LogSource::Run,
            "Analysis failed: no DC path at OUT",
            Some(LogAnchor::Simulation {
                nets: vec!["OUT".to_owned()],
                devices: Vec::new(),
            }),
        );

        double_click_console_row(&mut state, "no DC path at OUT");

        assert!(
            state.schematic.session.editor.selection.wires.contains(&91),
            "the row's anchor must mark the conductor the run named"
        );
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Design,
            "and take the reader to the drawing it marked it on"
        );
    }

    /// A name the drawing no longer carries must not look like a jump. The
    /// row stays put, says why on hover, and navigates nowhere when followed.
    #[test]
    fn an_unresolvable_anchor_renders_inert_and_says_why() {
        let mut state = state_with_probed_conductor();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        let anchor = LogAnchor::Simulation {
            nets: vec!["DELETED".to_owned()],
            devices: Vec::new(),
        };
        state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Error,
            LogSource::Run,
            "Analysis failed: no DC path at DELETED",
            Some(anchor.clone()),
        );

        let refusal = state
            .log_anchor_refusal(&anchor)
            .expect("a name this sheet does not draw cannot be jumped to");
        assert!(
            refusal.contains("DELETED"),
            "the tooltip must name what it could not find: {refusal}"
        );

        double_click_console_row(&mut state, "no DC path at DELETED");

        assert!(
            state.schematic.session.editor.selection.wires.is_empty(),
            "an inert row must mark nothing"
        );
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Results,
            "and must not navigate away from what the reader was looking at"
        );
        assert!(
            state
                .log_buffer
                .entries()
                .all(|entry| !entry.message.contains("Marked")),
            "nor report a marking it did not perform"
        );
    }

    /// A run's line opens that run's log on the page, and is inert once the
    /// run is no longer held rather than opening an empty channel.
    #[test]
    fn a_run_line_opens_its_log_and_refuses_once_the_run_is_gone() {
        let mut app = RSpiceApp::test_instance();
        app.state.log_buffer.clear();
        app.state
            .simulation
            .retained
            .runs
            .push(SimulationRun::new(7));
        app.state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Info,
            LogSource::Run,
            "Run 7 started: Transient",
            Some(LogAnchor::ResultRun { run_sequence: 7 }),
        );

        let workspace = app.state.workbench.workspace;
        double_click_console_row(&mut app.state, "Run 7 started");
        assert_eq!(
            app.state.workbench.console_view.channel,
            ConsoleChannel::Run(7),
            "the run's line opens the run's own log"
        );
        assert_eq!(
            app.state.workbench.workspace, workspace,
            "on the page, without leaving the workspace"
        );

        app.state
            .workbench
            .console_view
            .open(ConsoleChannel::Session);
        app.state.simulation.retained.runs.clear();
        let anchor = LogAnchor::ResultRun { run_sequence: 7 };
        let refusal = app
            .state
            .log_anchor_refusal(&anchor)
            .expect("a run this session dropped cannot be opened");
        assert!(refusal.contains("Run 7"), "{refusal}");
        let mut view = ConsoleView::default();
        let mut jump = None;
        assert!(!follow(&app.state, &mut view, &anchor, &mut jump));
        assert_eq!(view.channel, ConsoleChannel::Session);
        assert!(jump.is_none());
    }
}
