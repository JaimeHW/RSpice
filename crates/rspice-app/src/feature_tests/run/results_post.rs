//! The loop-gain locus a stability run loads into the results viewer, and
//! the results that load none.

use std::collections::HashMap;

use rspice_simulation::results::STB_NYQUIST_CONTOUR_WAVEFORM;

use crate::simulation::SampledWaveform;
use crate::simulation::controller::SimulationController;
use crate::simulation::controller::results_post::LOOP_GAIN_LOCUS_LABEL;
use crate::simulation::multi_run::AnalysisSpec;
use crate::workbench::app_state::AppState;

fn stb_spec() -> AnalysisSpec {
    AnalysisSpec::Stb {
        probe_node: "VLOOP1".to_owned(),
        start_freq: 1.0,
        stop_freq: 1.0e6,
        sweep: Default::default(),
        points_per_decade: 10,
        compute_nyquist: true,
    }
}

/// A stability result and an AC result arrive at this seam spelled the
/// same way. The frequencies and both responses are identical here, so
/// only the analysis kind can separate them.
fn frequency_response() -> (Vec<f64>, HashMap<String, SampledWaveform>) {
    let frequencies = vec![1.0, 10.0, 100.0];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        STB_NYQUIST_CONTOUR_WAVEFORM.to_owned(),
        SampledWaveform::new_complex(
            STB_NYQUIST_CONTOUR_WAVEFORM,
            frequencies.clone(),
            vec![2.0, 0.5, -0.1],
            vec![-0.3, -0.8, -0.2],
        ),
    );
    waveforms.insert(
        "V(out)".to_owned(),
        SampledWaveform::new_complex(
            "V(out)",
            frequencies.clone(),
            vec![1.0, 0.7, 0.2],
            vec![0.0, -0.4, -0.6],
        ),
    );
    (frequencies, waveforms)
}

#[test]
fn a_stability_run_loads_its_loop_gain_contour_and_nothing_else() {
    let mut controller = SimulationController::new();
    controller.current_spec = Some(stb_spec());
    let mut state = AppState::default();
    let (frequencies, waveforms) = frequency_response();

    controller.populate_ac_post_views(&mut state, &frequencies, &waveforms);

    let curve = state
        .results
        .analysis
        .nyquist_state
        .curve()
        .expect("the stability run's contour reaches the sheet");
    assert_eq!(curve.name, LOOP_GAIN_LOCUS_LABEL);
    assert_eq!(curve.len(), frequencies.len());
    assert_eq!(curve.points[0].real, 2.0);
}

/// An AC node response is not a loop gain, so it must not reach a sheet
/// that puts a stability criterion on what it draws.
#[test]
fn an_ac_run_loads_no_loop_gain_locus() {
    let mut controller = SimulationController::new();
    controller.current_spec = Some(AnalysisSpec::Ac {
        start_freq: 1.0,
        stop_freq: 100.0,
        points_per_unit: 10,
        sweep: Default::default(),
    });
    let mut state = AppState::default();
    let (frequencies, waveforms) = frequency_response();

    controller.populate_ac_post_views(&mut state, &frequencies, &waveforms);

    assert!(state.results.analysis.nyquist_state.is_empty());
    assert!(state.results.analysis.cache_authority.nyquist.is_none());
    // The Bode sheet still receives every complex response.
    assert!(!state.results.analysis.bode_plot_state.is_empty());
}

/// And an analysis whose kind cannot be resolved gets no locus either:
/// the sheet is opened by evidence, never by the shape of the data.
#[test]
fn an_unattributed_result_loads_no_loop_gain_locus() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    let (frequencies, waveforms) = frequency_response();

    controller.populate_ac_post_views(&mut state, &frequencies, &waveforms);

    assert!(state.results.analysis.nyquist_state.is_empty());
}
