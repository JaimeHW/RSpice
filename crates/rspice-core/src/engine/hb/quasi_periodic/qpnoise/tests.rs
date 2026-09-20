//! Analytic circuit, undefined-reference and retained-evidence regressions.
use super::*;
use crate::abort_signal::CountingAbort;

fn request() -> QpnoiseRequest {
    QpnoiseRequest {
        frequencies_hz: vec![100.0, 1000.0, 10_000.0],
        frequency_axis: QpnoiseFrequencyAxis::Output,
        outputs: vec![
            QpnoiseOutput {
                observation: QpnoiseObservation::Voltage {
                    positive: "out".into(),
                    negative: "0".into(),
                },
                lattice: vec![0, 0],
            },
            QpnoiseOutput {
                observation: QpnoiseObservation::BranchCurrent {
                    branch: "Rs".into(),
                },
                lattice: vec![0, 0],
            },
        ],
        input: Some(QpnoiseInput {
            source: "V1".into(),
            lattice: vec![0, 0],
        }),
        input_lattices: QpnoiseLattices::AllRetained,
        sources: QpnoiseSources::All,
        integration: Some(QpnoiseIntegration {
            band_hz: Some([200.0, 5000.0]),
            method: QpnoiseIntegrationMethod::Linear,
        }),
        contributor_ranking: true,
        noise_figure: Some(QpnoiseNoiseFigure {
            source_resistor: "Rs".into(),
            reference_temperature: 290.0,
            reference_lattices: None,
        }),
        linear: Default::default(),
    }
}
fn fixture() -> (Engine, Netlist, QpssOperatingPoint) {
    let netlist=Netlist::parse("QPNOISE RC\nV1 in 0 DC 0 AC 7\nRs in out 1k\nRl out 0 2k\nC1 out 0 1n\n.options device zeroresistancetol=1500\n.end\n").unwrap();
    let engine = Engine::default();
    // AC-only source deliberately drives a tone with amplitude seven. Noise
    // referral must nevertheless use a unit small-signal excitation.
    let mut config = QpssConfig::new(vec![1000.0, std::f64::consts::SQRT_2 * 1000.0], vec![1, 1]);
    config.source_tones = vec![QpssSourceTone {
        source: "V1".into(),
        tone: 0,
    }];
    let point = engine.run_qpss(&netlist, config).unwrap();
    (engine, netlist, point)
}
fn close(actual: Value, expected: Value) {
    assert!(
        (actual / expected - 1.0).abs() < 2e-8,
        "{actual:e} != {expected:e}"
    );
}
#[test]
fn qpnoise_engine_multioutput_referral_figure_and_integrated_contributors_match_rc() {
    let (engine, netlist, point) = fixture();
    let result = engine
        .run_qpnoise_from_qpss(&netlist, request(), &point)
        .unwrap();
    let kb = super::super::super::pnoise::pnoise_physical_constants(engine.config.spice_dialect)
        .boltzmann;
    let qs = 4.0 * kb * 300.15 / 1000.0;
    let ql = 4.0 * kb * 300.15 / 2000.0;
    for (i, &frequency) in result.metadata.request.frequencies_hz.iter().enumerate() {
        let z = Complex64::ONE / Complex64::new(0.0015, std::f64::consts::TAU * frequency * 1e-9);
        let gain = z / 1000.0;
        let total = &result.total_covariances[i];
        close(total.values[0].re, z.norm_sqr() * (qs + ql));
        close(
            total.values[3].re,
            (Complex64::ONE - gain).norm_sqr() * qs + gain.norm_sqr() * ql,
        );
        let cross = z * ((Complex64::ONE - gain.conj()) * qs - gain.conj() * ql);
        // Near DC the real cross-power cancels; use the physical noise scale.
        assert!((total.values[1] - cross).norm() < qs * 1e-5);
        assert_eq!(total.values[2], total.values[1].conj());
        assert!((result.outputs[0].input_transfer.as_ref().unwrap()[i] - gain).norm() < 1e-8);
        let QpnoiseValue::Finite(input) = result.outputs[0].input_noise.as_ref().unwrap()[i] else {
            panic!("undefined gain");
        };
        close(input, (qs + ql) * 1e6);
        let QpnoiseValue::Finite(db) = result.outputs[0].noise_figure_db.as_ref().unwrap()[i]
        else {
            panic!("undefined figure");
        };
        close(db, 10.0 * (1.0_f64 + 300.15 / 290.0 * 0.5).log10());
    }
    let integrated = result.outputs[0].integrated.as_ref().unwrap();
    let QpnoiseValue::Finite(rms) = integrated.input_rms.unwrap() else {
        panic!("undefined integral");
    };
    close(rms, ((qs + ql) * 1e6 * 4800.0).sqrt());
    let ranking = result.outputs[0].ranking.as_ref().unwrap();
    assert_eq!(result.sources[ranking[0].source_index].name, "RS thermal");
    close(ranking[0].percentage, 100.0 * 2.0 / 3.0);
    close(ranking[1].percentage, 100.0 / 3.0);
    assert_eq!(
        result.metadata.operating_point_identity,
        point.retained_identity()
    );
    let restored: QpnoiseAnalysisResult =
        serde_json::from_slice(&serde_json::to_vec(&result).unwrap()).unwrap();
    restored
        .validate_retained_payload_with_abort(&engine.config.resource_limits, &NoAbort)
        .unwrap();
    assert_eq!(restored, result);
    let mut bad = result.clone();
    bad.points[0].source_covariances[0].values[0].re *= 2.0;
    assert!(
        bad.validate_retained_payload_with_abort(&engine.config.resource_limits, &NoAbort)
            .is_err()
    );
    let mut bad = result.clone();
    bad.outputs[0].input_noise.as_mut().unwrap()[0] = QpnoiseValue::Finite(0.0);
    assert!(
        bad.validate_retained_payload_with_abort(&engine.config.resource_limits, &NoAbort)
            .is_err()
    );
    let mut bad = result;
    bad.metadata.observations[0][0].1 = -Complex64::ONE;
    assert!(
        bad.validate_retained_payload_with_abort(&engine.config.resource_limits, &NoAbort)
            .is_err()
    );
}
#[test]
fn qpnoise_engine_windows_filters_nulls_and_cancellation_are_explicit() {
    let (engine, netlist, point) = fixture();
    let mut req = request();
    req.input.as_mut().unwrap().lattice = vec![1, 0];
    req.input_lattices = QpnoiseLattices::Range {
        minimum: vec![0, -1],
        maximum: vec![1, 1],
    };
    let result = engine
        .run_qpnoise_from_qpss(&netlist, req.clone(), &point)
        .unwrap();
    assert_eq!(result.metadata.input_lattices.len(), 6);
    assert!(
        result.outputs[0]
            .input_noise
            .as_ref()
            .unwrap()
            .iter()
            .all(|v| *v == QpnoiseValue::Unavailable(QpnoiseUnavailable::ZeroInputTransfer))
    );
    assert_eq!(
        result.outputs[0].integrated.as_ref().unwrap().input_rms,
        Some(QpnoiseValue::Unavailable(
            QpnoiseUnavailable::UndefinedIntegrationSample
        ))
    );
    assert!(
        result.outputs[0]
            .noise_figure_db
            .as_ref()
            .unwrap()
            .iter()
            .all(|v| *v == QpnoiseValue::Unavailable(QpnoiseUnavailable::ZeroInputTransfer))
    );
    req.noise_figure = None;
    req.input = None;
    req.sources = QpnoiseSources::Except(vec!["rs thermal".into(), "rl thermal".into()]);
    let quiet = engine
        .run_qpnoise_from_qpss(&netlist, req.clone(), &point)
        .unwrap();
    assert!(quiet.sources.is_empty());
    assert!(
        quiet
            .total_covariances
            .iter()
            .all(|c| c.values.iter().all(|v| *v == Complex64::ZERO))
    );
    req.sources = QpnoiseSources::Only(vec!["rs thermal".into()]);
    req.frequencies_hz = vec![100.0];
    req.integration = None;
    let one = engine
        .run_qpnoise_from_qpss(&netlist, req.clone(), &point)
        .unwrap();
    assert_eq!(
        one.outputs[0].ranking.as_ref().unwrap()[0].percentage,
        100.0
    );
    req.input_lattices = QpnoiseLattices::Range {
        minimum: vec![-2, 0],
        maximum: vec![1, 1],
    };
    assert!(engine.run_qpnoise_from_qpss(&netlist, req, &point).is_err());
    let mut bad = request();
    bad.sources = QpnoiseSources::Only(vec!["rl thermal".into()]);
    assert!(engine.run_qpnoise_from_qpss(&netlist, bad, &point).is_err());
    assert!(matches!(
        engine.run_qpnoise_from_qpss_with_abort(
            &netlist,
            request(),
            &point,
            &CountingAbort::new(20)
        ),
        Err(SimulationError::Aborted)
    ));
    let mut limits = engine.config.resource_limits.clone();
    limits.max_result_values = 100;
    assert!(matches!(
        one.validate_retained_payload_with_abort(&limits, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    let changed = Netlist::parse("changed\nV1 in 0 0\nRs in out 2k\nRl out 0 2k\n.end\n").unwrap();
    assert!(
        engine
            .run_qpnoise_from_qpss(&changed, request(), &point)
            .is_err()
    );
}
#[test]
fn qpnoise_loglog_integration_clips_power_laws_and_preserves_rms_range() {
    let config = QpnoiseIntegration {
        band_hz: Some([2.0, 50.0]),
        method: QpnoiseIntegrationMethod::LogLog,
    };
    for exponent in [-2.0_f64, -1.0, 0.0, 2.0] {
        let frequencies = [1.0_f64, 10.0, 100.0];
        let values = frequencies
            .iter()
            .map(|f| QpnoiseValue::Finite(f.powf(exponent)))
            .collect::<Vec<_>>();
        let power = integration::integrate(&frequencies, &values, &config, &NoAbort)
            .unwrap()
            .unwrap();
        let expected = if exponent == -1.0 {
            25.0_f64.ln()
        } else {
            (50.0_f64.powf(exponent + 1.0) - 2.0_f64.powf(exponent + 1.0)) / (exponent + 1.0)
        };
        close(power.square_root(), expected.sqrt());
    }
    let config = QpnoiseIntegration {
        band_hz: None,
        method: QpnoiseIntegrationMethod::LogLog,
    };
    let flicker = integration::integrate(
        &[1.0, 1e16],
        &[QpnoiseValue::Finite(1.0), QpnoiseValue::Finite(1e-16)],
        &config,
        &NoAbort,
    )
    .unwrap()
    .unwrap();
    close(flicker.square_root(), 1e16_f64.ln().sqrt());
    let power = integration::integrate(
        &[1e-200, 2e-200],
        &[QpnoiseValue::Finite(1e-300); 2],
        &config,
        &NoAbort,
    )
    .unwrap()
    .unwrap();
    close(power.square_root(), 1e-250);
    let power = integration::integrate(
        &[1e200, 2e200],
        &[QpnoiseValue::Finite(1e300); 2],
        &config,
        &NoAbort,
    )
    .unwrap()
    .unwrap();
    close(power.square_root(), 1e250);
    assert_eq!(
        integration::integrate(
            &[-2.0, -1.0],
            &[QpnoiseValue::Finite(1.0); 2],
            &config,
            &NoAbort
        )
        .unwrap()
        .unwrap_err(),
        QpnoiseUnavailable::NegativeIntegrationFrequency
    );
}
