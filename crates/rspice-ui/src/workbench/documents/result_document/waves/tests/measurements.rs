//! Retained result measurements use the calculator's interval mathematics and
//! report sweep branches and missing coverage in the visible readout.

use super::*;

fn transient(x: Vec<f64>, y: Vec<f64>) -> AppState {
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![WaveformData::new("V(out)", x, y, "#fff")]),
    );
    state.simulation.complete_run();
    state
}

#[test]
fn measurement_readout_uses_interpolated_endpoints_and_matches_calculator() {
    let mut state = transient(vec![0.0, 1.0], vec![0.0, 1.0]);
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let model = &models[0];
    let trace = &model.traces[0];
    let stats = trace_interval_statistics(
        &mut state.ui.results.derived,
        model,
        trace,
        Some((0.25, 0.75)),
        None,
    )
    .unwrap();
    assert_eq!((stats.min, stats.max, stats.mean), (0.25, 0.75, 0.5));
    assert!((stats.rms - (13.0_f64 / 48.0).sqrt()).abs() < 1e-14);
    let rows = readout::measurement_values(
        &mut state.ui.results.derived,
        model,
        Some((0.25, 0.75)),
        8,
        state.ui.preferences.quantity_presentation_policy(),
    );
    for ((label, actual), (suffix, expected)) in
        rows.iter()
            .zip([("min", stats.min), ("max", stats.max), ("rms", stats.rms)])
    {
        assert_eq!(label, &format!("V(out) {suffix}"));
        assert_eq!(
            actual,
            &model.format_trace_value(
                trace,
                expected,
                8,
                state.ui.preferences.quantity_presentation_policy()
            )
        );
    }
    let full =
        trace_interval_statistics(&mut state.ui.results.derived, model, trace, None, None).unwrap();
    let expression = calculator::parser::try_parse("rms(V(out))").unwrap();
    let calculator = calculator::evaluator::evaluate(
        &expression,
        &calculator::SimulationContext::new(&state.simulation),
    )
    .unwrap();
    assert!(
        matches!(calculator, calculator::CalcValue::Scalar(value) if (value - full.rms).abs() < 1e-14)
    );
    let reversed = trace_interval_statistics(
        &mut state.ui.results.derived,
        model,
        trace,
        Some((0.75, 0.25)),
        None,
    )
    .unwrap();
    assert_eq!(stats, reversed);
    assert!(
        trace_interval_statistics(
            &mut state.ui.results.derived,
            model,
            trace,
            Some((f64::NAN, 0.5)),
            None,
        )
        .is_err()
    );
    assert_eq!(
        trace_interval_statistics(
            &mut state.ui.results.derived,
            model,
            trace,
            Some((0.5, 0.5)),
            None,
        )
        .unwrap()
        .mean,
        0.5
    );
}

#[test]
fn measurement_readout_identifies_each_sweep_branch() {
    let mut state = branches::hysteresis_run();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let rows = readout::measurement_values(
        &mut state.ui.results.derived,
        &models[0],
        Some((0.25, 0.75)),
        8,
        state.ui.preferences.quantity_presentation_policy(),
    );
    assert_eq!(rows.len(), 6);
    assert!(rows[0].0.ends_with("fwd min"));
    assert!(rows[3].0.ends_with("rev min"));
    let model = &models[0];
    for (branch, expected) in [(0, (0.5, 1.5, 1.0)), (1, (2.5, 3.5, 3.0))] {
        let stats = trace_interval_statistics(
            &mut state.ui.results.derived,
            model,
            &model.traces[0],
            Some((0.25, 0.75)),
            Some(branch),
        )
        .unwrap();
        assert_eq!((stats.min, stats.max, stats.mean), expected);
    }
}

#[test]
fn measurement_readout_explains_missing_coverage_instead_of_hiding_it() {
    let mut state = transient(
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![1.0, 1.0, f64::NAN, 1.0, 1.0],
    );
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    let rows = readout::measurement_values(
        &mut state.ui.results.derived,
        &models[0],
        None,
        8,
        state.ui.preferences.quantity_presentation_policy(),
    );
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, "V(out)");
    assert!(rows[0].1.contains("Unavailable"));
    assert!(rows[0].1.contains("50.0% interval coverage"));
}
