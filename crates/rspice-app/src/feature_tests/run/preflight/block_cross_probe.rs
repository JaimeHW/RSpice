//! A run read inside a block: the tab descended into an instance badges and
//! cross-probes that instance's own nets, never reporting the drawing stale.

use super::engine::add_operating_point;
use super::startup::rc_in_a_block;
use crate::workbench::RSpiceApp;
use rspice_app_design::schematic::view::scene::operating_point_annotations;
use rspice_app_design::schematic::view::scene::tests::text;
use rspice_app_state::state::{AnalysisType, SchematicAnnotationVisibility};
use rspice_schematic_editor::view::scene::BadgeSubject;

/// Press Run on the tab in front and let the run end.
fn run(app: &mut RSpiceApp) {
    app.invalidate_simulation_preflight();
    let _ = app.queued_by_its_check();
    let _ = app.dispatch_frame();
    app.finish_the_run();
    let run = app
        .state
        .model
        .simulation
        .retained
        .runs
        .first()
        .expect("the run is retained");
    assert!(run.success, "{:?}", run.analyses);
}

/// The RC's R1 and C1 made a block X1, run with an operating point, and
/// read on the tab descended into X1: the block's `out` carries its
/// operating-point badge, and the result V(x1.out) shows on the block's own
/// conductor instead of reporting that the schematic changed.
#[test]
fn a_block_descended_into_badges_and_probes_its_own_nets() {
    let mut app = rc_in_a_block();
    add_operating_point(&mut app.state);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .occurrence_path()
            .to_string(),
        "/X1",
        "the block is open at its instance"
    );
    run(&mut app);

    assert!(
        app.state.model.result_mapping_is_current(),
        "the run read the block in front, so its geometry answers for it"
    );

    let operating_point = app.state.model.simulation.retained.runs[0]
        .analyses
        .iter()
        .position(|analysis| analysis.analysis_type == AnalysisType::DcOp)
        .expect("the run solved the operating point");
    app.state.model.simulation.view.active_run_idx = Some(0);
    app.state.model.simulation.view.active_analysis_idx = Some(operating_point);
    app.state.design.schematic_visibility.annotations =
        SchematicAnnotationVisibility::OperatingPoint;
    let out_label = app
        .state
        .model
        .schematic
        .document()
        .net_labels
        .iter()
        .find(|label| label.name.eq_ignore_ascii_case("out"))
        .map(|label| label.id)
        .expect("the block labels its out net");
    let badges = operating_point_annotations(&app.state.canvas_facts());
    let out = badges
        .iter()
        .find(|badge| {
            matches!(badge.subject, BadgeSubject::Net { label: Some(label), .. } if label == out_label)
        })
        .unwrap_or_else(|| {
            panic!(
                "the block's out net carries a badge after its label: {:?}",
                badges.iter().map(text).collect::<Vec<_>>()
            )
        });
    assert_eq!(
        text(out),
        "1 V",
        "the 1 V the drive holds out at, read at /X1"
    );

    let located = app.state.model.select_signal_conductor("V(x1.out)");
    assert_eq!(located.as_deref(), Ok("out"));
    let editor = &app.state.model.schematic.session.editor;
    let out_wires = app
        .state
        .model
        .schematic
        .document()
        .wires
        .iter()
        .filter(|wire| editor.selection.wires.contains(&wire.id))
        .count();
    assert!(out_wires > 0, "the block's out conductor is selected");
}
