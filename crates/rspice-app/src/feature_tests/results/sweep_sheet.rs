//! The Sweep sheet over a run the engine solved: a stepped transient's
//! `.MEAS` read against the stepped value, checked against the divider the
//! deck states.

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::{
    ResultViewer, right_panel, show_persistent_pane_viewer, viewer_availability,
};
use rspice_results::sweep_table::{OutputKind, signal_values, sweep_table};

/// A divider whose upper leg steps through 1k, 2k and 4k over a 1 kΩ lower
/// leg, its gain measured at 5 µs: 1k / (r + 1k) at each point.
const DECK: &str = "* stepped divider\n.param r=1k\nV1 in 0 DC 1\nR1 in out {r}\nR2 out 0 1k\n\
.tran 1u 10u\n.step param r list 1k 2k 4k\n.meas tran gain find V(out) at=5u\n.end\n";

fn divider_gain(r: f64) -> f64 {
    1e3 / (r + 1e3)
}

#[test]
fn a_stepped_transient_reads_its_gain_against_the_stepped_value() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch(DECK);
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(1));
    assert!(
        viewer_availability(app.state.results_reads(), ResultViewer::Sweep).available,
        "a stepped transient's point is a point of a sweep"
    );

    let run = app.state.model.simulation.active_run().expect("the run");
    let table = sweep_table(&run.analyses, 1).expect("the stepped family as a table");
    assert_eq!(table.rows, ["r=1k", "r=2k", "r=4k"]);
    let gain = table.output("gain").unwrap_or_else(|| {
        let names: Vec<&str> = table.outputs.iter().map(|o| o.name.as_str()).collect();
        panic!("the deck's .MEAS among {names:?}")
    });
    assert_eq!(gain.kind, OutputKind::Measurement);
    // A node's voltage is retained under the node's name.
    let ends = signal_values(&run.analyses, &table, "out", None);
    for ((measured, end), r) in gain.values.iter().zip(&ends).zip([1e3, 2e3, 4e3]) {
        let measured = measured.expect("each point measured its gain");
        let end = end.expect("each point kept its output node");
        assert!(
            (measured - divider_gain(r)).abs() < 1e-6,
            "r={r}: gain {measured} against {}",
            divider_gain(r)
        );
        assert!((end - divider_gain(r)).abs() < 1e-6, "r={r}: OUT {end}");
    }

    // The sheet paints the family through the workbench's own dispatch.
    app.state.results.results.session.viewer = ResultViewer::Sweep;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    for _ in 0..2 {
        let _ = ctx.run_ui(input.clone(), |ctx| {
            egui::Panel::right("inspector")
                .show(ctx, |ui| right_panel(ui, &mut app.state.results_parts()));
            egui::CentralPanel::default().show(ctx, |ui| {
                super::with_surface_parts(&mut app, |state, run_controller| {
                    show_persistent_pane_viewer(
                        ui,
                        state,
                        run_controller,
                        ResultViewer::Sweep,
                        None,
                    )
                })
            });
        });
    }
    let drawn = app
        .state
        .results
        .results
        .session
        .drawn_axes
        .get(&ResultViewer::Sweep)
        .copied()
        .expect("the sheet drew its plot");
    // The plot spans the stepped values, which a decade apart read on a
    // linear axis.
    assert!(drawn.0.0 <= 1e3 && drawn.0.1 >= 4e3, "{drawn:?}");
}
