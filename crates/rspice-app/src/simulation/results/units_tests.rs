//! Study measurement tests check waveform and transfer-unit normalization.

use rspice_simulation::results::SimulationResult;

fn converted(result: &SimulationResult, selector: &str, target: &str) -> f64 {
    result
        .study_measurement(selector)
        .unwrap()
        .value_in_unit(target)
        .unwrap()
        .unwrap()
}

#[test]
fn study_measurement_units_follow_quasi_periodic_channels() {
    use std::sync::Arc;
    let deck = rspice_core::Netlist::parse("Units\nV1 out 0 1\nR1 out 0 1k\n.end\n").unwrap();
    let point = rspice_core::Engine::default()
        .run_qpss(
            &deck,
            rspice_core::engine::QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]),
        )
        .unwrap();
    let result = SimulationResult::from_qpss_operating_point(Arc::new(point)).unwrap();
    assert!((converted(&result, "tuple:0,0:real:V(out)", "mV") - 1000.0).abs() < 1e-8);
    assert!((converted(&result, "tuple:0,0:real:I(V1)", "mA") + 1.0).abs() < 1e-8);
    assert_eq!(converted(&result, "tuple:0,0:phase:V(out)", "rad"), 0.0);
    assert!(
        result
            .study_measurement("scalar:qpss.normalized_residual")
            .unwrap()
            .value_in_unit("V")
            .is_err()
    );

    let noise = crate::simulation::results::qpnoise_test_fixture();
    for (selector, symbol) in [
        ("scalar:qpnoise.output_rms(1)", "V"),
        ("scalar:qpnoise.output_rms(2)", "A"),
        ("scalar:qpnoise.input_rms(2)", "V"),
    ] {
        assert!(converted(&noise, selector, symbol) > 0.0);
    }
    let SimulationResult::Qpnoise {
        response,
        waveforms,
        ..
    } = &noise
    else {
        unreachable!()
    };
    let source = &response.sources[0].name;
    let share = format!("scalar:qpnoise.contributor_share_percent(2,{source})");
    assert!(
        (converted(&noise, &share, "%") / 100.0 - converted(&noise, &share, "ratio")).abs() < 1e-14
    );
    let contribution = format!("scalar:qpnoise.contributor_rms(2,{source})");
    assert!(converted(&noise, &contribution, "A") >= 0.0);
    let trace = waveforms
        .values()
        .find(|trace| trace.y_unit == "A/√Hz")
        .unwrap();
    let value = converted(&noise, &format!("last:{}", trace.name), "nA/sqrt(Hz)");
    assert!((value / 1e9 - trace.y_values.last().unwrap()).abs() < 1e-20);
}
