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
fn autonomous_qpnoise_matches_rotating_oscillator_noise_and_cross_spectrum() {
    use crate::analysis::quasi_periodic::QuasiPeriodicSampling;
    use std::f64::consts::{SQRT_2, TAU};
    let netlist = Netlist::parse("Autonomous noise\nCx x 0 1\nCy y 0 1\nRx x 0 1\nRy y 0 1\nIprobe 0 x DC 0\nBx 0 x I={(2-v(x)^2-v(y)^2)*v(x)-v(y)}\nBy 0 y I={(2-v(x)^2-v(y)^2)*v(y)+v(x)}\n.end\n").unwrap();
    let engine = Engine::default();
    let mut config = QpssConfig::new(vec![1.2 / TAU, SQRT_2 / TAU], vec![3, 1]);
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![17, 9]);
    config.solver.relative_tolerance = 1e-9;
    let mut oscillator = QpssOscillator::new(0, "x".into());
    oscillator.initial_amplitude = 0.8;
    oscillator.additional_seeds.push(QpssOscillatorSeed {
        node: "y".into(),
        amplitude: 0.8,
        phase_degrees: -90.0,
    });
    config.oscillator = Some(oscillator);
    let point = engine.run_qpss(&netlist, config).unwrap();
    let request = QpnoiseRequest {
        frequencies_hz: vec![1e-7, 0.005, 0.02],
        frequency_axis: QpnoiseFrequencyAxis::Offset,
        outputs: ["x", "y"]
            .into_iter()
            .map(|node| QpnoiseOutput {
                observation: QpnoiseObservation::Voltage {
                    positive: node.into(),
                    negative: "0".into(),
                },
                lattice: vec![1, 0],
            })
            .collect(),
        input: Some(QpnoiseInput {
            source: "Iprobe".into(),
            lattice: vec![1, 0],
        }),
        input_lattices: QpnoiseLattices::AllRetained,
        sources: QpnoiseSources::Only(vec!["RX thermal".into(), "RY thermal".into()]),
        integration: Some(QpnoiseIntegration {
            band_hz: None,
            method: QpnoiseIntegrationMethod::Linear,
        }),
        contributor_ranking: true,
        noise_figure: None,
        linear: Default::default(),
    };
    let result = engine
        .run_qpnoise_from_qpss(&netlist, request.clone(), &point)
        .unwrap();
    let s = 4.0
        * super::super::super::pnoise::pnoise_physical_constants(engine.config.spice_dialect)
            .boltzmann
        * 300.15;
    let mut expected_densities = vec![];
    for (i, offset) in request.frequencies_hz.iter().enumerate() {
        // In rotating coordinates, independent equal x/y noise remains
        // isotropic. Radial relaxation is 2; phase is an integrator. Each
        // physical output mixes their spectra at omega +/- omega_carrier.
        let d = TAU * offset;
        let power = |w: Value| 1.0 / (4.0 + w * w) + 1.0 / (w * w);
        let psd = s / 4.0 * (power(d) + power(2.0 + d));
        expected_densities.push(psd);
        let cross = Complex64::new(0.0, s / 4.0 * (power(d) - power(2.0 + d)));
        let covariance = &result.total_covariances[i].values;
        close(covariance[0].re, psd);
        close(covariance[3].re, psd);
        assert!((covariance[1] - cross).norm() < psd * 2e-8);
        assert_eq!(covariance[2], covariance[1].conj());
        let transfer =
            |w| Complex64::ONE / Complex64::new(2.0, w) + Complex64::ONE / Complex64::new(0.0, w);
        let gain = (transfer(d) + transfer(2.0 + d)) * 0.25;
        assert!(
            (result.outputs[0].input_transfer.as_ref().unwrap()[i] - gain).norm()
                < gain.norm() * 2e-8
        );
        let QpnoiseValue::Finite(referred) = result.outputs[0].input_noise.as_ref().unwrap()[i]
        else {
            panic!("finite input referral expected")
        };
        close(referred, psd / gain.norm_sqr());
    }
    let expected_power: Value = request
        .frequencies_hz
        .windows(2)
        .zip(expected_densities.windows(2))
        .map(|(f, q)| (f[1] - f[0]) * (q[0] + q[1]) * 0.5)
        .sum();
    let QpnoiseValue::Finite(rms) = result.outputs[0].integrated.as_ref().unwrap().output_rms
    else {
        panic!("finite integral expected")
    };
    close(rms, expected_power.sqrt());
    let ranking = result.outputs[0].ranking.as_ref().unwrap();
    close(ranking.iter().map(|r| r.percentage).sum(), 100.0);
    assert_eq!(ranking.len(), 2);
    let (metadata, buffers) = result
        .clone()
        .into_transfer_parts_with_abort(&engine.config.resource_limits, &NoAbort)
        .unwrap();
    let restored = QpnoiseAnalysisResult::from_transfer_parts_with_abort(
        metadata,
        buffers,
        &engine.config.resource_limits,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(restored, result);
    let mut zero = request.clone();
    zero.frequencies_hz = vec![0.0];
    assert!(
        engine
            .run_qpnoise_from_qpss(&netlist, zero, &point)
            .unwrap_err()
            .to_string()
            .contains("nonzero")
    );
    let mut crossing = request.clone();
    crossing.frequencies_hz = vec![-0.01, 0.01];
    assert!(
        engine
            .run_qpnoise_from_qpss(&netlist, crossing.clone(), &point)
            .unwrap_err()
            .to_string()
            .contains("spectral line")
    );
    let grid = engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    let carrier = point.oscillator_frequency_hz().unwrap();
    let mut clipped = request.clone();
    clipped.frequencies_hz = vec![-0.01, 0.01, 0.02];
    clipped.integration.as_mut().unwrap().band_hz = Some([carrier + 0.005, carrier + 0.015]);
    assert!(
        autonomous::validate_integration(
            &clipped,
            &point,
            &grid,
            &result.metadata.observations,
            &NoAbort
        )
        .is_err()
    );
    // The unused crossing interval must not invalidate an otherwise usable
    // clipped band. The absolute-frequency axis obeys the same rule.
    clipped.frequencies_hz = vec![-0.01, 0.003, 0.01, 0.02];
    autonomous::validate_integration(
        &clipped,
        &point,
        &grid,
        &result.metadata.observations,
        &NoAbort,
    )
    .unwrap();
    clipped.frequency_axis = QpnoiseFrequencyAxis::Output;
    clipped
        .frequencies_hz
        .iter_mut()
        .for_each(|f| *f += carrier);
    autonomous::validate_integration(
        &clipped,
        &point,
        &grid,
        &result.metadata.observations,
        &NoAbort,
    )
    .unwrap();
    clipped.frequencies_hz.remove(1);
    assert!(
        autonomous::validate_integration(
            &clipped,
            &point,
            &grid,
            &result.metadata.observations,
            &NoAbort
        )
        .is_err()
    );
    crossing.integration = None;
    crossing.contributor_ranking = false;
    engine
        .run_qpnoise_from_qpss(&netlist, crossing, &point)
        .unwrap();
    let mut narrow = request;
    narrow.input_lattices = QpnoiseLattices::Explicit {
        tuples: vec![vec![1, 0]],
    };
    narrow.sources = QpnoiseSources::Only(vec!["RX thermal".into()]);
    let narrow = engine
        .run_qpnoise_from_qpss(&netlist, narrow, &point)
        .unwrap();
    assert_eq!(narrow.sources.len(), 1);
    assert_eq!(narrow.metadata.input_lattices, [vec![1, 0]]);
    for (i, offset) in narrow.metadata.request.frequencies_hz.iter().enumerate() {
        let d = TAU * offset;
        let transfer =
            |w| Complex64::ONE / Complex64::new(2.0, w) + Complex64::ONE / Complex64::new(0.0, w);
        close(
            narrow.total_covariances[i].values[0].re,
            s * ((transfer(d) + transfer(2.0 + d)) * 0.25).norm_sqr(),
        );
    }
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
    let mut limits = engine.config.resource_limits;
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

#[test]
fn qpnoise_packed_transport_preserves_colored_lattices_and_rejects_corruption() {
    let engine = Engine::default();
    let netlist=Netlist::parse("Packed QPNOISE\nV1 in 0 SIN(1 .1 1k)\nRs in out RM 1k\nRl out 0 2k\nC1 out 0 1n\n.model RM R(KF=1e-12 AF=2 EF=1)\n.options device zeroresistancetol=1500\n.end\n").unwrap();
    let point = engine
        .run_qpss(
            &netlist,
            QpssConfig::new(vec![1000.0, std::f64::consts::SQRT_2 * 1000.0], vec![1, 1]),
        )
        .unwrap();
    let mut req = request();
    req.noise_figure = None;
    req.integration = None;
    req.frequencies_hz = vec![100.0, 300.0, 700.0];
    let mut result = engine.run_qpnoise_from_qpss(&netlist, req, &point).unwrap();
    let grid = result
        .validate_retained_payload_with_abort(&engine.config.resource_limits, &NoAbort)
        .unwrap();
    // The equivalent explicit lattice exercises numeric tuple packing as used
    // by full compact-device modulation spectra, beyond their circuit basis.
    let mut colored = false;
    for source in &mut result.sources {
        if let QuasiPeriodicNoiseSpectrum::PowerLaw {
            modulation_lattices,
            ..
        } = &mut source.spectrum
        {
            assert!(modulation_lattices.is_none());
            *modulation_lattices = Some(grid.indices().to_vec());
            colored = true;
        }
    }
    assert!(colored);
    result.metadata.retained_identity = result.payload_identity(&NoAbort).unwrap();
    let limits = &engine.config.resource_limits;
    let (metadata, values) = result
        .clone()
        .into_transfer_parts_with_abort(limits, &NoAbort)
        .unwrap();
    let restored = QpnoiseAnalysisResult::from_transfer_parts_with_abort(
        metadata.clone(),
        values.clone(),
        limits,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(restored, result);
    assert!(
        metadata
            .validate_transfer_layout_with_abort(values.len() - 1, limits, &NoAbort)
            .is_err()
    );
    let mut altered = values.clone();
    altered[0] *= 2.0;
    assert!(
        QpnoiseAnalysisResult::from_transfer_parts_with_abort(
            metadata.clone(),
            altered,
            limits,
            &NoAbort
        )
        .is_err()
    );
    let mut altered = values.clone();
    altered[0] = Value::NAN;
    assert!(
        QpnoiseAnalysisResult::from_transfer_parts_with_abort(
            metadata.clone(),
            altered,
            limits,
            &NoAbort
        )
        .is_err()
    );
    let mut tuple_offset = 0;
    for source in &metadata.sources {
        match source.spectrum {
            QpnoiseSpectrumLayout::Bsim4Correlated { sample_count, .. } => {
                tuple_offset += 7 * sample_count
            }
            QpnoiseSpectrumLayout::White { density_count, .. } => tuple_offset += density_count,
            QpnoiseSpectrumLayout::PowerLaw {
                mode_count,
                explicit_lattices,
                ..
            } => {
                tuple_offset += 2 * mode_count;
                if explicit_lattices {
                    break;
                }
            }
        }
    }
    let mut altered = values.clone();
    altered[tuple_offset] = 0.5;
    assert!(
        QpnoiseAnalysisResult::from_transfer_parts_with_abort(
            metadata.clone(),
            altered,
            limits,
            &NoAbort
        )
        .is_err()
    );
    let mut bounded = *limits;
    bounded.max_result_values = values.len();
    assert!(matches!(
        metadata.validate_transfer_layout_with_abort(values.len(), &bounded, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        QpnoiseAnalysisResult::from_transfer_parts_with_abort(
            metadata,
            values,
            limits,
            &CountingAbort::new(20)
        ),
        Err(SimulationError::Aborted)
    ));
}
