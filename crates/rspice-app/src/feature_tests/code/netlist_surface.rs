//! The Netlist surface drawn through the whole application: a problem
//! step that goes to its finding, and the run strip at every phase.

use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::surfaces::netlist::tests::seal_manual_run;
use crate::workbench::surfaces::netlist::{
    RUN_STRIP_HEIGHT, RunStripPhase, reveal_in_netlist, run_strip_projection, show_prepared,
    step_netlist_problem, toolbar,
};
use crate::workbench::{RSpiceApp, features};
use egui::vec2;

/// F8 in the netlist editor goes to the deck's next finding, on its
/// column, and Shift+F8 back; each says which finding it reached. A
/// finding of the deck's own file counts though it names that file.
#[test]
fn a_problem_step_in_the_deck_goes_to_the_finding_and_says_which() {
    use crate::state::netlist_diagnostics::{Diagnostic, NetlistDiagnosticCollection};
    use crate::workbench::documents::code_workspace::{ProblemStep, queue_problem_step};
    use crate::workbench::documents::netlist_document;
    use rspice_vocabulary::diagnostics::DiagnosticSeverity;

    let deck = "deck\nR1 1 0 1k\nC1 1 0 1pp\n.end\n";
    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.netlist_source = Some(deck.to_owned());
    app.state.model.simulation.source.netlist_content = deck.to_owned();
    app.state.code.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    app.state.code.netlist.active_document_initialized = true;
    let mut root = Diagnostic::current("lint", "UNITS", DiagnosticSeverity::Warning, "1pp");
    root.source_path = Some(std::path::PathBuf::from("deck.cir"));
    root.column = Some(7);
    let root = root.with_line(Some(2));
    let first =
        Diagnostic::current("lint", "NODE", DiagnosticSeverity::Error, "node 1").with_line(Some(1));
    app.state.code.netlist.diagnostics = std::sync::Arc::new(
        NetlistDiagnosticCollection::try_new(vec![root, first], deck)
            .expect("the fixture collection is well formed"),
    );
    // These findings stand for the deck's validation: it is not re-run.
    let revision = app.state.code.netlist.revision;
    app.state.code.netlist.mark_validated_for_test(revision);

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(900.0, 600.0),
        )),
        ..Default::default()
    };
    let editor = netlist_document::editor_id_in(&app.state.model, &app.state.code.netlist);
    // The page's order: the step, the reveal it queued, then the editor.
    let press = |app: &mut RSpiceApp, step: ProblemStep| {
        for frame in 0..3 {
            let _ = ctx.clone().run_ui(input.clone(), |ui| {
                if frame == 0 {
                    ui.ctx().memory_mut(|memory| memory.request_focus(editor));
                    queue_problem_step(ui.ctx(), editor, step);
                }
                step_netlist_problem(
                    ui.ctx(),
                    &app.state.model,
                    &app.state.code.netlist,
                    &mut app.state.workbench,
                    editor,
                );
                reveal_in_netlist(ui, &app.state.model, &mut app.state.code.netlist, editor);
                let state = &mut app.state;
                netlist_document::show_editor(
                    ui,
                    &mut state.model,
                    &mut state.code.netlist,
                    &state.code.code_workspace.include_resolutions,
                    &state.runs,
                    &mut state.workbench,
                );
            });
        }
        let caret = app.state.code.netlist.cursor_char_index;
        let before = deck.chars().take(caret).collect::<String>();
        let line = before.matches('\n').count() + 1;
        let column = before.chars().rev().take_while(|c| *c != '\n').count() + 1;
        (
            (line, column),
            app.state.prompt_text().unwrap_or_default().to_owned(),
        )
    };

    assert_eq!(
        press(&mut app, ProblemStep::Next),
        ((2, 1), "Error 1 of 2, line 2: node 1".to_owned())
    );
    assert_eq!(
        press(&mut app, ProblemStep::Next),
        ((3, 8), "Warning 2 of 2, line 3: 1pp".to_owned()),
        "on the finding's column"
    );
    assert_eq!(press(&mut app, ProblemStep::Previous).0, (2, 1));
    assert_eq!(press(&mut app, ProblemStep::Previous).0, (3, 8), "wraps");
}

/// Render the deck stage and read back the strip's own 24-point band.
///
/// The band is where the strip is or is not: a phase that painted nothing
/// there, or two phases that painted the same thing, would be a strip that
/// states its status in prose only.
fn run_strip_band(width: f32, phase: RunStripPhase) -> (crate::ui::raster::Canvas, Vec<u8>) {
    const DECK: &str = "run strip fixture\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    const EDITED: &str = "run strip fixture\nV1 out 0 1\nR1 out 0 2k\n.op\n.end\n";

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
    app.state.model.workspace.content.netlist_source = Some(DECK.to_owned());
    app.state.model.simulation.source.netlist_content = DECK.to_owned();
    app.state.code.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    app.state.code.netlist.active_document_initialized = true;
    app.state.runs.manual_run.last_run_buffer = Some(DECK.to_owned());
    app.state.runs.manual_run.last_run_id = Some(7);
    seal_manual_run(&mut app.state.model, 7, 0xAB, 4);
    match phase {
        RunStripPhase::Current => {}
        RunStripPhase::Snapshot => {
            assert!(
                crate::workbench::documents::netlist_document::open_run_deck_snapshot_in(
                    &mut app.state.model,
                    &mut app.state.code.netlist,
                    &app.state.runs,
                )
            );
        }
        RunStripPhase::Edited => {
            app.state.model.workspace.content.netlist_source = Some(EDITED.to_owned());
            app.state.model.simulation.source.netlist_content = EDITED.to_owned();
        }
        RunStripPhase::Running => {
            seal_manual_run(&mut app.state.model, 8, 0xCD, 5);
            app.state.runs.manual_run.pending_manual_run_id = Some(8);
        }
    }
    assert_eq!(
        run_strip_projection(&app.state.model, &app.state.code.netlist, &app.state.runs)
            .map(|projection| projection.phase),
        Some(phase),
        "the raster fixture must actually be in the phase it renders"
    );

    let canvas = crate::ui::raster::render(vec2(width, 520.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                show_prepared(ui, &mut features::cx_with_menu_answers(&mut app, |_| false));
            });
    });
    let band = egui::Rect::from_min_max(
        egui::pos2(0.0, toolbar::CODE_TOOLBAR_HEIGHT + 1.0),
        egui::pos2(width, toolbar::CODE_TOOLBAR_HEIGHT + RUN_STRIP_HEIGHT - 1.0),
    );
    let pixels = canvas
        .pixels_in(band)
        .flat_map(|pixel| pixel.to_array())
        .collect::<Vec<_>>();
    assert!(!pixels.is_empty(), "the strip band is off canvas");
    assert!(
        pixels
            .chunks_exact(4)
            .any(|pixel| pixel != canvas.background().to_array()),
        "the strip band painted nothing"
    );
    (canvas, pixels)
}

#[test]
fn run_strip_paints_a_distinct_row_for_every_phase_at_both_stage_widths() {
    for width in [1600.0, 1000.0] {
        let (_, current) = run_strip_band(width, RunStripPhase::Current);
        let (_, edited) = run_strip_band(width, RunStripPhase::Edited);
        let (_, running) = run_strip_band(width, RunStripPhase::Running);
        assert_ne!(current, edited, "ok and warn must not render alike");
        assert_ne!(current, running, "ok and running must not render alike");
        assert_ne!(edited, running, "warn and running must not render alike");
    }
}
