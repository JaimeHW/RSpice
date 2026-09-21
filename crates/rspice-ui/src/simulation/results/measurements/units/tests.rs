use super::*;

fn converted(result: &SimulationResult, selector: &str, target: &str) -> f64 {
    result
        .study_measurement(selector)
        .unwrap()
        .value_in_unit(target)
        .unwrap()
        .unwrap()
}

#[test]
fn study_measurement_units_follow_waveforms_and_transfer_normalization() {
    let mut voltage = WaveformData::new_complex("V(out)", vec![1.0], vec![0.003], vec![0.004]);
    voltage.y_unit = "V".into();
    let current = WaveformData::new_time_domain_in_unit("I(V1)", vec![1.0], vec![0.002], "A");
    let unknown = WaveformData::new_time_domain_in_unit("undocumented", vec![1.0], vec![1.0], "");
    let result = SimulationResult::Ac {
        convergence: None,
        frequencies: vec![1.0],
        waveforms: HashMap::from([
            ("V(out)".into(), voltage),
            ("I(V1)".into(), current),
            ("undocumented".into(), unknown),
        ]),
        measurements: vec![],
        reference_impedances_ohm: None,
        noise_reference_temperature_kelvin: None,
    };
    assert_eq!(converted(&result, "bin:0:magnitude:v(OUT)", "mV"), 5.0);
    assert_eq!(converted(&result, "last:V(out)", "mV"), 3.0);
    assert_eq!(converted(&result, "last:V1", "mA"), 2.0);
    assert_eq!(converted(&result, "scalar:I(V1)", "mA"), 2.0);
    let angle = converted(&result, "bin:0:phase:V(out)", "rad");
    assert!((angle - 4_f64.atan2(3.0)).abs() < 1e-14);
    assert!(
        result
            .study_measurement("last:undocumented")
            .unwrap()
            .value_in_unit("V")
            .is_err()
    );
    assert!(result.study_measurement("bin:0:imag:I(V1)").is_none());

    let op = SimulationResult::DcOp(Box::new(DcOpResult {
        node_voltages: HashMap::from([("out".into(), 0.25)]),
        branch_currents: HashMap::from([("V1".into(), 0.002)]),
        ..Default::default()
    }));
    assert_eq!(converted(&op, "scalar:v(OUT)", "mV"), 250.0);
    assert_eq!(converted(&op, "scalar:i(v1)", "mA"), 2.0);

    use crate::simulation::multi_run::{TfAccuracy, TfNormalization};
    for (input, output, symbol) in [
        (
            TransferFunctionQuantity::Current,
            TransferFunctionQuantity::Voltage,
            "ohm",
        ),
        (
            TransferFunctionQuantity::Voltage,
            TransferFunctionQuantity::Current,
            "S",
        ),
        (
            TransferFunctionQuantity::Voltage,
            TransferFunctionQuantity::Voltage,
            "1",
        ),
        (
            TransferFunctionQuantity::Current,
            TransferFunctionQuantity::Current,
            "1",
        ),
    ] {
        for normalization in [
            TfNormalization::None,
            TfNormalization::PerSourceUnit,
            TfNormalization::RelativeToNominal,
        ] {
            let result = SimulationResult::TransferFunction {
                input_source: "source".into(),
                output_expression: "output".into(),
                input_quantity: input,
                output_quantity: output,
                input_unit: String::new(),
                output_unit: String::new(),
                normalization,
                accuracy: TfAccuracy::default(),
                gain: Some(TransferFunctionScalar::Finite(2.0)),
                input_resistance: Some(TransferFunctionScalar::Finite(1000.0)),
                output_resistance: None,
                nominal_input: None,
                nominal_output: None,
            };
            let expected = if normalization == TfNormalization::RelativeToNominal {
                "1"
            } else {
                symbol
            };
            assert_eq!(converted(&result, "scalar:tf.gain", expected), 2.0);
            assert_eq!(converted(&result, "scalar:rin", "kohm"), 1.0);
        }
    }
}

#[test]
fn study_measurement_units_follow_quasi_periodic_channels() {
    use std::sync::Arc;
    let deck = rspice_core::Netlist::parse("Units\nV1 out 0 1\nR1 out 0 1k\n.end\n").unwrap();
    let point = rspice_core::Engine::default()
        .run_qpss(
            &deck,
            rspice_core::engine::QpssConfig::new(vec![1000.0, 1414.2135623730951], vec![1, 1]),
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

    let noise = SimulationResult::qpnoise_test_fixture();
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
