//! What the measurement table admits as a measurement.

use super::active_measurement_rows;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};
use crate::workbench::RSpiceApp;

/// A frequency response whose margins the Bode extraction can read.
///
/// `|V(out)|` carries linear magnitude, which the extraction converts, so the
/// unity-gain crossing is the 1.0 at 100 Hz and the phase there sets the
/// margin at 60°.
fn ac_run() -> SimulationRun {
    let frequency = vec![1.0, 10.0, 100.0, 1000.0];
    let mut magnitude = WaveformData::new(
        "|V(out)|",
        frequency.clone(),
        vec![100.0, 10.0, 1.0, 0.1],
        "#fff",
    );
    magnitude.visible = true;
    let mut phase = WaveformData::new(
        "phase(V(out))",
        frequency,
        vec![-45.0, -80.0, -120.0, -160.0],
        "#fff",
    );
    phase.visible = true;
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![magnitude, phase]),
    );
    run
}

fn row_names(app: &RSpiceApp) -> Vec<String> {
    active_measurement_rows(app, &[])
        .into_iter()
        .map(|row| row.name)
        .collect()
}

/// A failed solve has no margins to report.
///
/// The derived AC stability margins join the table as first-class rows, and a
/// `phase_margin` specification binds to them exactly as it binds to a
/// `.measure` result. Nothing asked whether the solve behind them succeeded.
/// The retained vectors of a failed AC run are whatever the engine emitted
/// before giving up, and a margin read off them is not a measurement — which
/// is precisely why the Bode sheet refuses them.
#[test]
fn stability_margins_from_a_failed_solve_never_reach_the_measurement_table() {
    let mut app = RSpiceApp::test_instance();
    app.state.simulation.runs = vec![ac_run()];
    assert!(app.state.simulation.select_run(0));
    app.state.simulation.active_analysis_idx = Some(0);

    assert!(
        row_names(&app).iter().any(|name| name == "phase_margin"),
        "the fixture has to produce margins before the gate can withhold them: {:?}",
        row_names(&app)
    );

    app.state.simulation.runs[0].analyses[0].success = false;
    app.state.simulation.runs[0].analyses[0].error_message =
        Some("the solve did not converge".to_owned());

    assert!(
        row_names(&app).is_empty(),
        "a failed AC solve still contributed stability margins: {:?}",
        row_names(&app)
    );
}
