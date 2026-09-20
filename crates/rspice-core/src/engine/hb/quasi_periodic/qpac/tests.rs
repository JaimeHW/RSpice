use super::*;
use crate::abort_signal::CountingAbort;
use crate::config::SimulationConfig;

fn request() -> QpacRequest {
    QpacRequest {
        offsets_hz: vec![0.0, 37.0, 127.0],
        input_source: "Vprobe".into(),
        input_lattice: vec![1, 0, -1],
        output_node: "out".into(),
        output_ref: "ref".into(),
        output_lattice: vec![1, 0, -1],
        magnitude: 0.2,
        phase_degrees: 73.0,
        solver: QuasiPeriodicAcConfig::default(),
    }
}

#[test]
fn qpac_engine_binds_three_tone_sources_differential_outputs_and_retained_identity() {
    let deck = "three-tone RC\nVprobe in ref SIN(.4 .2 1k)\nVref ref 0 SIN(.1 .05 1414.213562373095)\nIprobe 0 out AC 0.001\nR1 in out 1k\nR2 out ref 2k\nC1 out ref 100n\n.end\n";
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let mut config = QpssConfig::new(
        vec![
            1e3,
            std::f64::consts::SQRT_2 * 1e3,
            std::f64::consts::PI * 1e3,
        ],
        vec![1; 3],
    );
    config.source_tones.push(QpssSourceTone {
        source: "Iprobe".into(),
        tone: 2,
    });
    let point = engine.run_qpss(&netlist, config).unwrap();
    let identity = point.retained_identity().to_owned();
    let voltage_request = request();
    for source in ["Vprobe", "Iprobe"] {
        let mut request = voltage_request.clone();
        request.input_source = source.into();
        let result = engine
            .run_qpac_from_qpss(&netlist, request.clone(), &point)
            .unwrap();
        let limits = crate::ResourceLimits::default();
        assert_eq!(
            result.metadata.input_quantity,
            if source == "Vprobe" {
                QpacInputQuantity::Voltage
            } else {
                QpacInputQuantity::Current
            }
        );
        let encoded = serde_json::to_string(&result).unwrap();
        let decoded: QpacAnalysisResult = serde_json::from_str(&encoded).unwrap();
        decoded
            .validate_retained_payload_with_abort(&limits, &NoAbort)
            .unwrap();
        assert_eq!(decoded, result);
        let (metadata, rows) = result.clone().into_transfer_parts();
        assert_eq!(
            QpacAnalysisResult::from_transfer_parts_with_abort(
                metadata.clone(),
                rows.clone(),
                &limits,
                &NoAbort
            )
            .unwrap(),
            result
        );
        let mut short = rows.clone();
        short[0].pop();
        assert!(
            QpacAnalysisResult::from_transfer_parts_with_abort(
                metadata.clone(),
                short,
                &limits,
                &NoAbort
            )
            .is_err()
        );
        let mut changed = result.clone();
        changed.unit_solutions[0].spectra[0][0].re += 0.1;
        assert!(
            changed
                .validate_retained_payload_with_abort(&limits, &NoAbort)
                .is_err()
        );
        let mut changed = result.clone();
        changed.metadata.output_frequencies_hz[0] += 1.0;
        assert!(
            changed
                .validate_retained_payload_with_abort(&limits, &NoAbort)
                .is_err()
        );
        let mut changed = result.clone();
        changed.metadata.request.phase_degrees += 1.0;
        assert!(
            changed
                .validate_retained_payload_with_abort(&limits, &NoAbort)
                .is_err()
        );
        let mut limited = limits.clone();
        limited.max_result_values = 1;
        assert!(
            metadata
                .validate_transfer_layout_with_abort(
                    &rows.iter().map(Vec::len).collect::<Vec<_>>(),
                    &limited,
                    &NoAbort
                )
                .is_err()
        );
        assert!(
            result
                .validate_retained_payload_with_abort(&limits, &CountingAbort::new(0))
                .is_err()
        );
        assert_eq!(result.metadata.operating_point_identity, identity);
        assert_eq!(result.metadata.tuples.len(), 27);
        assert!(result.metadata.tuples.iter().all(|tuple| tuple.len() == 3));
        let drive = Complex64::from_polar(request.magnitude, request.phase_degrees.to_radians());
        for k in 0..request.offsets_hz.len() {
            let frequency = request.offsets_hz[k] + 1e3 - std::f64::consts::PI * 1e3;
            let admittance = Complex64::new(0.0015, std::f64::consts::TAU * frequency * 100e-9);
            let expected =
                Complex64::new(if source == "Vprobe" { 0.001 } else { 1.0 }, 0.0) / admittance;
            assert!((result.metadata.input_frequencies_hz[k] - frequency).abs() < 1e-12);
            assert_eq!(
                result.metadata.input_frequencies_hz[k],
                result.metadata.output_frequencies_hz[k]
            );
            assert!((result.output_transfer[k] - expected).norm() < 1e-9);
            assert!((result.output_response[k] - expected * drive).norm() < 1e-9);
        }
    }
    // A linear circuit cannot translate into a different tuple. Changing the
    // observation must not accidentally keep publishing the input sideband.
    let mut converted = request();
    converted.output_lattice = vec![0, 1, 0];
    let result = engine
        .run_qpac_from_qpss(&netlist, converted, &point)
        .unwrap();
    assert!(result.output_transfer.iter().all(|v| v.norm() < 1e-12));
    assert_eq!(point.retained_identity(), identity);
    let altered = Netlist::parse(&deck.replace("R1 in out 1k", "R1 in out 2k")).unwrap();
    assert!(
        engine
            .run_qpac_from_qpss(&altered, request(), &point)
            .is_err()
    );
    for (input, output, source) in [
        (vec![0, 0], vec![0, 0], "Vprobe"),
        (vec![2, 0, 0], vec![0, 0, 0], "Vprobe"),
        (vec![0, 0, 0], vec![0, 0, 0], "missing"),
    ] {
        let mut bad = request();
        bad.input_lattice = input;
        bad.output_lattice = output;
        bad.input_source = source.into();
        assert!(engine.run_qpac_from_qpss(&netlist, bad, &point).is_err());
    }
    let abort = CountingAbort::new(10);
    assert!(matches!(
        engine.run_qpac_from_qpss_with_abort(&netlist, request(), &point, &abort),
        Err(SimulationError::Aborted)
    ));
}
