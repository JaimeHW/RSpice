//! Following a console row's anchor.
//!
//! Split from `console.rs` because it is a different concern from painting a
//! log line: everything here is about whether a row's target still exists and
//! what happens when the reader commits to it. The row rendering it wraps is
//! `super::row_with_sense`, which stays with the other row shapes.

use egui::{Sense, Ui};

use super::{AppState, SemanticTone, log_tone, row, row_with_sense, tone_color};

use crate::ui::tokens::Tokens;

/// One console line, plus the jump the reader asked it to follow.
///
/// The row cannot follow the anchor itself. The console paints its rows out
/// of the log buffer, so the session is borrowed immutably for the whole
/// pass, and navigating mutates it. The request rides back out to [`super::console`],
/// which holds the mutable borrow — the same shape [`super::netlist_problems`] uses
/// for the same reason. That also keeps the row off `RSpiceApp`: everything a
/// jump needs is on `AppState`.
pub(super) fn log_row(
    ui: &mut Ui,
    state: &AppState,
    entry: &crate::diagnostics::LogEntry,
) -> Option<crate::diagnostics::LogAnchor> {
    let t = Tokens::get(ui.ctx());
    let tone = log_tone(entry.severity);
    let source_color = tone_color(&t, tone);
    let message_color = match tone {
        SemanticTone::Error | SemanticTone::Warning | SemanticTone::Success => source_color,
        SemanticTone::Info | SemanticTone::Debug => t.color.text_dim,
        SemanticTone::Trace => t.color.text_faint,
    };
    let message = entry.context.as_ref().map_or_else(
        || entry.message.clone(),
        |context| format!("{} · {context}", entry.message),
    );
    let Some(anchor) = entry.anchor.as_ref() else {
        row(
            ui,
            &entry.format_timestamp(),
            entry.source.name(),
            &message,
            source_color,
            message_color,
        );
        return None;
    };
    // Asked before the row is sensed, not after it is clicked: an anchor
    // whose objects the drawing no longer carries must not look like a jump.
    let refusal = state.log_anchor_refusal(anchor);
    let response = row_with_sense(
        ui,
        &entry.format_timestamp(),
        entry.source.name(),
        &message,
        source_color,
        message_color,
        if refusal.is_some() {
            Sense::hover()
        } else {
            // accessibility-pointer-shim: this only picks the sense. The row it
            // configures is `row_with_sense`, which announces the whole line as
            // a button and paints the focus ring for the sense it is handed.
            Sense::click()
        },
    );
    if let Some(refusal) = refusal {
        response.on_hover_text(refusal);
        return None;
    }
    let response = response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(log_anchor_hint(anchor));
    response.clicked().then(|| anchor.clone())
}

/// What following this row's anchor will do, in the words of the surface it
/// lands on.
fn log_anchor_hint(anchor: &crate::diagnostics::LogAnchor) -> String {
    use crate::diagnostics::LogAnchor;
    match anchor {
        LogAnchor::Schematic { .. } => "Show on the schematic".to_owned(),
        LogAnchor::Symbol { pin_name, .. } => format!("Show pin {pin_name} in the symbol view"),
        LogAnchor::Simulation { nets, devices } => {
            let count = nets.len() + devices.len();
            let objects = if count == 1 { "object" } else { "objects" };
            format!("Mark the {count} {objects} this run named on the schematic")
        }
        LogAnchor::ResultRun { run_sequence } => {
            format!("Open run {run_sequence} in Results")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::console;
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
    fn console_row_position(state: &mut AppState, needle: &str) -> egui::Pos2 {
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

        let ctx = console_context_for_tests();
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

    /// Click the console row whose message contains `needle`.
    ///
    /// Two frames, because egui resolves a press against the widget rects the
    /// previous frame registered: the first lays the rows out, the second
    /// delivers the press to the row that is now known to be there.
    fn click_console_row(state: &mut AppState, needle: &str) {
        let position = console_row_position(state, needle);
        let ctx = console_context_for_tests();
        let _ = ctx.run_ui(console_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        });
        let events = vec![
            egui::Event::PointerMoved(position),
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ];
        let _ = ctx.run_ui(console_input(events), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
        });
    }

    /// A sheet drawing the conductor `OUT`, with a cross-probe map captured at
    /// the topology it is currently drawn at.
    fn state_with_probed_conductor() -> AppState {
        use crate::state::{Point, Wire};
        use std::collections::HashMap;

        let mut state = AppState::default();
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        state.schematic.wires.push(Wire::new(91, vec![a, b]));
        state.simulation.cross_probe.update(
            state.workspace.active_view.clone(),
            HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            HashMap::from([("OUT".to_owned(), vec![a, b])]),
            HashMap::new(),
            state.schematic.topology_version(),
        );
        state.log_buffer.clear();
        state
    }

    /// The whole point of an anchored row: the objects a failed run named are
    /// one click from the sentence that named them.
    #[test]
    fn clicking_an_anchored_console_row_marks_what_the_run_named() {
        let mut state = state_with_probed_conductor();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Error,
            LogSource::Simulation,
            "Analysis failed: no DC path at OUT",
            None,
            Some(crate::diagnostics::LogAnchor::Simulation {
                nets: vec!["OUT".to_owned()],
                devices: Vec::new(),
            }),
        );

        click_console_row(&mut state, "no DC path at OUT");

        assert!(
            state.schematic.selection.wires.contains(&91),
            "the row's anchor must mark the conductor the run named"
        );
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Design,
            "and take the reader to the drawing it marked it on"
        );
    }

    /// A name the drawing no longer carries must not look like a jump. The
    /// row stays put, says why on hover, and navigates nowhere when pressed.
    #[test]
    fn an_unresolvable_anchor_renders_inert_and_says_why() {
        let mut state = state_with_probed_conductor();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        let anchor = crate::diagnostics::LogAnchor::Simulation {
            nets: vec!["DELETED".to_owned()],
            devices: Vec::new(),
        };
        state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Error,
            LogSource::Simulation,
            "Analysis failed: no DC path at DELETED",
            None,
            Some(anchor.clone()),
        );

        let refusal = state
            .log_anchor_refusal(&anchor)
            .expect("a name this sheet does not draw cannot be jumped to");
        assert!(
            refusal.contains("DELETED"),
            "the tooltip must name what it could not find: {refusal}"
        );

        click_console_row(&mut state, "no DC path at DELETED");

        assert!(
            state.schematic.selection.wires.is_empty(),
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

    /// A run anchor opens the dataset it names, and refuses once that run is
    /// no longer retained rather than opening whatever is selected.
    #[test]
    fn a_run_anchor_opens_its_dataset_and_refuses_once_the_run_is_gone() {
        let mut app = RSpiceApp::test_instance();
        app.state.simulation.runs.push(SimulationRun::new(7));
        let anchor = crate::diagnostics::LogAnchor::ResultRun { run_sequence: 7 };
        assert!(
            app.state.log_anchor_refusal(&anchor).is_none(),
            "a retained run is reachable"
        );

        app.state.jump_to_log_anchor(anchor.clone());
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );

        app.state.simulation.runs.clear();
        let refusal = app
            .state
            .log_anchor_refusal(&anchor)
            .expect("a run this session dropped cannot be opened");
        assert!(refusal.contains("Run 7"), "{refusal}");
    }
}
