//! Adversarial tests for authentication and validation of retained result payloads.

use super::*;

fn projected_failvalue_measurement() -> rspice_core::MeasureResult {
    rspice_core::MeasureResult {
        name: "peak_at".to_owned(),
        value: Some(20.0),
        raw_value: Some(3.0),
        error: None,
        passed: true,
        expected: None,
        tolerance: None,
        failure_limit: Some(4.0),
        failure_limit_exceeded: false,
        event_axis: Some(20.0),
    }
}

#[test]
fn retained_measurements_require_exact_failvalue_evidence() {
    let valid = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_measurements(vec![projected_failvalue_measurement()]);
    valid
        .validate_retained_evidence()
        .expect("a projected value may differ from its exact raw FAILVALUE evidence");

    let mut unevaluated = projected_failvalue_measurement();
    unevaluated.value = None;
    unevaluated.raw_value = None;
    unevaluated.event_axis = None;
    unevaluated.passed = false;
    unevaluated.error = Some("signal was unavailable".to_owned());
    AnalysisResult::new(2, AnalysisType::Transient, "TRAN")
        .with_measurements(vec![unevaluated])
        .validate_retained_evidence()
        .expect("an early failure retains the authored limit without raw evidence");

    let mut missing_raw = valid.clone();
    missing_raw.measurements[0].raw_value = None;
    assert!(missing_raw.validate_retained_evidence().is_err());

    let mut missing_published = valid.clone();
    missing_published.measurements[0].value = None;
    assert!(missing_published.validate_retained_evidence().is_err());

    let mut nonfinite_raw = valid.clone();
    nonfinite_raw.measurements[0].raw_value = Some(f64::NAN);
    assert!(nonfinite_raw.validate_retained_evidence().is_err());

    let mut nonfinite_limit = valid.clone();
    nonfinite_limit.measurements[0].failure_limit = Some(f64::INFINITY);
    assert!(nonfinite_limit.validate_retained_evidence().is_err());

    let mut false_positive = valid.clone();
    false_positive.measurements[0].failure_limit_exceeded = true;
    false_positive.measurements[0].passed = false;
    assert!(false_positive.validate_retained_evidence().is_err());

    let mut false_negative = valid.clone();
    false_negative.measurements[0].raw_value = Some(-4.0);
    false_negative.measurements[0].passed = false;
    assert!(false_negative.validate_retained_evidence().is_err());

    let mut passed_after_exceeded = valid;
    passed_after_exceeded.measurements[0].raw_value = Some(4.0);
    passed_after_exceeded.measurements[0].failure_limit_exceeded = true;
    assert!(passed_after_exceeded.validate_retained_evidence().is_err());
}

fn floquet_certificate(problem_order: u64) -> FloquetSpectrumCertificateEvidence {
    FloquetSpectrumCertificateEvidence {
        problem_order,
        max_backward_error: 0.0,
        qualification_tolerance:
            FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(problem_order)
                .unwrap(),
    }
}

fn stable_pstb_payload() -> AnalysisResultPayload {
    let multiplier = ComplexResultValue {
        real: 0.5,
        imaginary: 0.0,
    };
    let exponent = complex_value(multiplier).ln();
    AnalysisResultPayload::Pstb {
        period_s: Some(1.0),
        fundamental_frequency_hz: Some(1.0),
        stability_threshold: Some(1.0),
        probe_instance: Some("LPROBE".to_owned()),
        detect_subharmonics: Some(false),
        modes: vec![PstbFloquetModeEvidence {
            multiplier,
            exponent: ComplexResultValue {
                real: exponent.re,
                imaginary: exponent.im,
            },
            probe_participation: 0.25,
            is_unstable: false,
            is_trivial: false,
            subharmonic_order: None,
        }],
        floquet_evidence: FloquetSpectrumEvidence::Qualified {
            certificate: floquet_certificate(1),
        },
        orbit_kind: FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        stability_classification: PstbStabilityClassificationEvidence::Stable,
        min_stability_margin_db: Some(-20.0 * 0.5_f64.log10()),
        max_multiplier_magnitude: Some(0.5),
        num_unstable: Some(0),
        subharmonics: Vec::new(),
        converged: Some(true),
        iterations: Some(0),
    }
}

#[test]
fn pss_floquet_payload_requires_core_authentic_complete_evidence() {
    let valid = AnalysisResultPayload::PssFloquet {
        period_s: Some(2.0),
        fundamental_frequency_hz: Some(0.5),
        iterations: Some(4),
        residual_norm: Some(1.0e-12),
        multipliers: vec![PssFloquetMultiplierEvidence {
            multiplier: ComplexResultValue {
                real: 0.5,
                imaginary: 0.0,
            },
        }],
        floquet_evidence: FloquetSpectrumEvidence::Qualified {
            certificate: floquet_certificate(1),
        },
        orbit_kind: FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
    };
    assert!(valid.validate_for(AnalysisType::Pss).is_ok());
    assert!(valid.validate_for(AnalysisType::Pstb).is_err());

    let mut inflated = valid.clone();
    let AnalysisResultPayload::PssFloquet {
        floquet_evidence: FloquetSpectrumEvidence::Qualified { certificate },
        ..
    } = &mut inflated
    else {
        unreachable!()
    };
    certificate.qualification_tolerance *= 2.0;
    assert!(inflated.validate_for(AnalysisType::Pss).is_err());

    let legacy = AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pss).unwrap();
    assert!(legacy.validate_for(AnalysisType::Pss).is_ok());
    let mut forged_legacy = legacy;
    let AnalysisResultPayload::PssFloquet { period_s, .. } = &mut forged_legacy else {
        unreachable!()
    };
    *period_s = Some(1.0);
    assert!(forged_legacy.validate_for(AnalysisType::Pss).is_err());
}

#[test]
fn pstb_payload_recomputes_mode_flags_counts_metrics_and_classification() {
    let valid = stable_pstb_payload();
    assert!(valid.validate_for(AnalysisType::Pstb).is_ok());

    let mutations: [fn(&mut AnalysisResultPayload); 3] = [
        |payload: &mut AnalysisResultPayload| {
            let AnalysisResultPayload::Pstb { num_unstable, .. } = payload else {
                unreachable!()
            };
            *num_unstable = Some(1);
        },
        |payload: &mut AnalysisResultPayload| {
            let AnalysisResultPayload::Pstb {
                stability_classification,
                ..
            } = payload
            else {
                unreachable!()
            };
            *stability_classification = PstbStabilityClassificationEvidence::UnstableReal;
        },
        |payload: &mut AnalysisResultPayload| {
            let AnalysisResultPayload::Pstb { probe_instance, .. } = payload else {
                unreachable!()
            };
            *probe_instance = Some(" LPROBE".to_owned());
        },
    ];
    for mutate in mutations {
        let mut tampered = valid.clone();
        mutate(&mut tampered);
        assert!(tampered.validate_for(AnalysisType::Pstb).is_err());
    }
}

#[test]
fn pstb_zero_dynamic_modes_and_single_autonomous_phase_are_json_safe() {
    let zero_order = AnalysisResultPayload::Pstb {
        period_s: Some(1.0),
        fundamental_frequency_hz: Some(1.0),
        stability_threshold: Some(1.0),
        probe_instance: Some("LPROBE".to_owned()),
        detect_subharmonics: Some(false),
        modes: Vec::new(),
        floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
        orbit_kind: FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        stability_classification: PstbStabilityClassificationEvidence::Stable,
        min_stability_margin_db: None,
        max_multiplier_magnitude: Some(0.0),
        num_unstable: Some(0),
        subharmonics: Vec::new(),
        converged: Some(true),
        iterations: Some(0),
    };
    assert!(zero_order.validate_for(AnalysisType::Pstb).is_ok());
    serde_json::to_string(&zero_order).expect("zero-order PSTB payload is strict-JSON safe");

    let multiplier = ComplexResultValue {
        real: 1.0 + 0.5 * rspice_core::analysis::FLOQUET_UNIT_CIRCLE_BAND,
        imaginary: 0.0,
    };
    let exponent = complex_value(multiplier).ln();
    let autonomous = AnalysisResultPayload::Pstb {
        period_s: Some(1.0),
        fundamental_frequency_hz: Some(1.0),
        stability_threshold: Some(1.0),
        probe_instance: Some("LPROBE".to_owned()),
        detect_subharmonics: Some(false),
        modes: vec![PstbFloquetModeEvidence {
            multiplier,
            exponent: ComplexResultValue {
                real: exponent.re,
                imaginary: exponent.im,
            },
            probe_participation: 1.0,
            is_unstable: false,
            is_trivial: true,
            subharmonic_order: None,
        }],
        floquet_evidence: FloquetSpectrumEvidence::Qualified {
            certificate: floquet_certificate(1),
        },
        orbit_kind: FloquetOrbitKindEvidence::Autonomous,
        trivial_multiplier_index: Some(0),
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        stability_classification: PstbStabilityClassificationEvidence::Stable,
        min_stability_margin_db: None,
        max_multiplier_magnitude: Some(complex_value(multiplier).norm()),
        num_unstable: Some(0),
        subharmonics: Vec::new(),
        converged: Some(true),
        iterations: Some(0),
    };
    assert!(autonomous.validate_for(AnalysisType::Pstb).is_ok());
}

#[test]
fn durable_floquet_scalars_are_central_and_never_fabricate_a_stability_bool() {
    let result = AnalysisResult::new(1, AnalysisType::Pstb, "PSTB")
        .with_result_payload(stable_pstb_payload());
    let scalar = |name: &str| {
        let candidates = result.scalar_evidence(name);
        assert_eq!(candidates.len(), 1, "missing scalar evidence {name}");
        assert!(candidates[0].passed);
        candidates[0].value.unwrap()
    };

    assert_eq!(scalar("pstb_period"), 1.0);
    assert_eq!(scalar("PSTB_FUNDAMENTAL_FREQUENCY"), 1.0);
    assert_eq!(scalar("pstb_mode_count"), 1.0);
    assert_eq!(scalar("pstb_unstable_mode_count"), 0.0);
    assert_eq!(scalar("pstb_max_multiplier_magnitude"), 0.5);
    assert_eq!(
        scalar("pstb_min_stability_margin_db"),
        -20.0 * 0.5_f64.log10()
    );
    assert_eq!(
        scalar("pstb.min_stability_margin_db"),
        -20.0 * 0.5_f64.log10(),
        "the established dotted runtime spelling remains compatible"
    );
    assert_eq!(
        result.scalar_evidence_names(),
        [
            "pstb_period",
            "pstb_fundamental_frequency",
            "pstb_mode_count",
            "pstb_unstable_mode_count",
            "pstb_max_multiplier_magnitude",
            "pstb_min_stability_margin_db",
        ]
    );
    assert!(result.scalar_evidence("pstb.is_stable").is_empty());
    assert!(
        !result
            .scalar_evidence_names()
            .iter()
            .any(|name| name == "pstb.is_stable" || name == "pstb.stability")
    );

    let legacy = AnalysisResult::new(2, AnalysisType::Pstb, "legacy").with_result_payload(
        AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pstb).unwrap(),
    );
    assert!(legacy.scalar_evidence("pstb.mode_count").is_empty());
    assert!(legacy.scalar_evidence_names().is_empty());
}

#[test]
fn zero_order_pss_exposes_an_authenticated_zero_mode_count() {
    let payload = AnalysisResultPayload::PssFloquet {
        period_s: Some(2.0),
        fundamental_frequency_hz: Some(0.5),
        iterations: Some(2),
        residual_norm: Some(1.0e-12),
        multipliers: Vec::new(),
        floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
        orbit_kind: FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
    };
    let result = AnalysisResult::new(1, AnalysisType::Pss, "PSS").with_result_payload(payload);

    assert_eq!(result.scalar_evidence("pss_period")[0].value, Some(2.0));
    assert_eq!(
        result.scalar_evidence("pss_fundamental_frequency")[0].value,
        Some(0.5)
    );
    assert_eq!(result.scalar_evidence("pss_mode_count")[0].value, Some(0.0));
    assert_eq!(
        result.scalar_evidence("pss.mode_count")[0].value,
        Some(0.0),
        "the established dotted runtime spelling remains compatible"
    );
    assert_eq!(
        result.scalar_evidence_names(),
        ["pss_period", "pss_fundamental_frequency", "pss_mode_count"]
    );
    assert!(result.scalar_evidence("pss.is_stable").is_empty());
}

#[test]
fn retained_waveforms_require_exact_finite_aligned_unique_evidence() {
    let valid = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.25, 0.5], "#00aaff")
            .with_unit("V")
            .with_complex_components("V(out)", vec![0.25, 0.5], vec![0.0, -0.125]),
    ]);
    assert!(valid.validate_retained_evidence().is_ok());

    let misaligned = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.25], "#00aaff"),
    ]);
    assert!(
        misaligned
            .validate_retained_evidence()
            .expect_err("misaligned retained samples must fail closed")
            .contains("coordinates")
    );

    let non_finite = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.25, f64::NAN], "#00aaff"),
    ]);
    assert!(
        non_finite
            .validate_retained_evidence()
            .expect_err("non-finite retained samples must fail closed")
            .contains("non-finite")
    );

    let duplicated = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new("V(out)", vec![0.0], vec![0.25], "#00aaff"),
        WaveformData::new("V(out)", vec![0.0], vec![0.5], "#ffaa00"),
    ]);
    assert!(
        duplicated
            .validate_retained_evidence()
            .expect_err("duplicate retained identities must fail closed")
            .contains("duplicated")
    );

    let bad_complex = AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
        WaveformData::new("V(out)", vec![1.0, 10.0], vec![0.25, 0.5], "#00aaff")
            .with_complex_components("V(out)", vec![0.25], vec![0.0, -0.125]),
    ]);
    assert!(
        bad_complex
            .validate_retained_evidence()
            .expect_err("misaligned complex evidence must fail closed")
            .contains("complex components")
    );
}

#[test]
fn retained_scalar_and_operating_point_evidence_is_finite_and_unambiguous() {
    let invalid_op = AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
        node_voltages: vec![OperatingPointValue {
            name: "V(out)".to_owned(),
            value: f64::INFINITY,
            unit: "V".to_owned(),
        }],
        ..DcOpResult::default()
    });
    assert!(
        invalid_op
            .validate_retained_evidence()
            .expect_err("non-finite OP evidence must fail closed")
            .contains("non-finite")
    );

    let invalid_measurement = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_measurements(vec![rspice_core::MeasureResult::success("gain", f64::NAN)]);
    assert!(
        invalid_measurement
            .validate_retained_evidence()
            .expect_err("non-finite measurement evidence must fail closed")
            .contains("measurement")
    );
}

#[test]
fn summary_only_noise_retains_integrated_totals_without_contributor_rows() {
    let summary = NoiseSummary {
        rows: Vec::new(),
        total_rms: Some(2.5e-6),
        input_rms: Some(1.25e-6),
        band: (10.0, 1.0e6),
    };
    let result =
        AnalysisResult::new(1, AnalysisType::Noise, "NOISE").with_noise_summary(summary.clone());

    assert_eq!(result.noise_summary, Some(summary));
}

fn transfer_function_payload() -> AnalysisResultPayload {
    AnalysisResultPayload::TransferFunction {
        input_source: "VIN".to_owned(),
        output_expression: "V(OUT,REF)".to_owned(),
        input_quantity: TransferFunctionQuantityEvidence::Voltage,
        output_quantity: TransferFunctionQuantityEvidence::Voltage,
        input_unit: "V".to_owned(),
        output_unit: "V".to_owned(),
        normalization: TransferFunctionNormalizationEvidence::None,
        accuracy: TransferFunctionAccuracyEvidence::Balanced,
        gain: Some(TransferFunctionScalarEvidence::Finite(0.5)),
        input_resistance: Some(TransferFunctionScalarEvidence::PositiveInfinity),
        output_resistance: Some(TransferFunctionScalarEvidence::Finite(-25.0)),
        nominal_input: None,
        nominal_output: None,
    }
}

#[test]
fn pole_zero_payload_requires_matching_type_and_finite_values() {
    let payload = AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: -1.0,
            imaginary: 2.0,
        }],
        zeros: Vec::new(),
        pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        gain: Some(1.0),
    };
    assert!(payload.validate_for(AnalysisType::PoleZero).is_ok());
    assert!(payload.validate_for(AnalysisType::Ac).is_err());

    let unavailable_gain = AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: -1.0,
            imaginary: 2.0,
        }],
        zeros: Vec::new(),
        pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        gain: None,
    };
    assert!(
        unavailable_gain
            .validate_for(AnalysisType::PoleZero)
            .is_ok()
    );

    let invalid = AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: f64::INFINITY,
            imaginary: 0.0,
        }],
        zeros: Vec::new(),
        pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        gain: Some(1.0),
    };
    assert!(invalid.validate_for(AnalysisType::PoleZero).is_err());

    let invalid_gain = AnalysisResultPayload::PoleZero {
        poles: Vec::new(),
        zeros: Vec::new(),
        pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
        gain: Some(f64::INFINITY),
    };
    assert!(invalid_gain.validate_for(AnalysisType::PoleZero).is_err());

    let invalid_tolerance = AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: -1.0,
            imaginary: 0.0,
        }],
        zeros: Vec::new(),
        pole_evidence: PoleZeroRootSetEvidence::Qualified {
            certificate: PoleZeroSpectrumCertificate {
                problem_order: 1,
                infinite_count: 0,
                max_backward_error: 0.0,
                qualification_tolerance: 2.0
                    * PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
            },
        },
        zero_evidence: PoleZeroRootSetEvidence::NotRequested,
        gain: Some(1.0),
    };
    assert!(
        invalid_tolerance
            .validate_for(AnalysisType::PoleZero)
            .expect_err("inflated qualification tolerance is not core-authentic")
            .contains("pole evidence")
    );
}

#[test]
fn pole_zero_payload_deserializes_legacy_numeric_and_missing_gain() {
    let legacy: AnalysisResultPayload =
        serde_json::from_str(r#"{"kind":"pole_zero","poles":[],"zeros":[],"gain":4.25}"#)
            .expect("legacy numeric pole-zero gain deserializes");
    assert!(matches!(
        legacy,
        AnalysisResultPayload::PoleZero {
            gain: Some(4.25),
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            ..
        }
    ));

    let missing: AnalysisResultPayload =
        serde_json::from_str(r#"{"kind":"pole_zero","poles":[],"zeros":[]}"#)
            .expect("missing pole-zero gain deserializes as unavailable");
    assert!(matches!(
        missing,
        AnalysisResultPayload::PoleZero {
            gain: None,
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            ..
        }
    ));
}

#[test]
fn sensitivity_payload_requires_canonical_unique_rows_and_valid_basis() {
    let valid = AnalysisResultPayload::Sensitivity {
        output: "V(out)".to_owned(),
        result_mode: SensitivityResultMode::Ac {
            frequency_hz: 1_000.0,
        },
        rows: vec![
            SensitivityResultRow {
                parameter: "length".to_owned(),
                raw: -1.0,
                normalized: -0.25,
            },
            SensitivityResultRow {
                parameter: "width".to_owned(),
                raw: 2.0,
                normalized: 0.5,
            },
        ],
    };
    assert!(valid.validate_for(AnalysisType::Sensitivity).is_ok());

    let duplicate = AnalysisResultPayload::Sensitivity {
        output: "V(out)".to_owned(),
        result_mode: SensitivityResultMode::Dc,
        rows: vec![
            SensitivityResultRow {
                parameter: "width".to_owned(),
                raw: 1.0,
                normalized: 1.0,
            },
            SensitivityResultRow {
                parameter: "width".to_owned(),
                raw: 2.0,
                normalized: 2.0,
            },
        ],
    };
    assert!(duplicate.validate_for(AnalysisType::Sensitivity).is_err());

    let invalid_frequency = AnalysisResultPayload::Sensitivity {
        output: "V(out)".to_owned(),
        result_mode: SensitivityResultMode::Ac { frequency_hz: 0.0 },
        rows: Vec::new(),
    };
    assert!(
        invalid_frequency
            .validate_for(AnalysisType::Sensitivity)
            .is_err()
    );
}

#[test]
fn transfer_function_payload_is_typed_non_finite_safe_and_analysis_specific() {
    let payload = transfer_function_payload();
    assert!(payload.validate_for(AnalysisType::Tf).is_ok());
    assert!(payload.validate_for(AnalysisType::Ac).is_err());
    assert!(payload.has_data());

    let encoded = serde_json::to_string(&payload).expect("TF payload serializes");
    assert!(encoded.contains("positive_infinity"));
    assert!(!encoded.contains("Infinity"));
    let decoded: AnalysisResultPayload =
        serde_json::from_str(&encoded).expect("TF payload deserializes");
    assert_eq!(decoded, payload);

    assert_eq!(
        TransferFunctionScalarEvidence::from_f64(f64::INFINITY),
        Some(TransferFunctionScalarEvidence::PositiveInfinity)
    );
    assert_eq!(
        TransferFunctionScalarEvidence::from_f64(f64::NEG_INFINITY),
        Some(TransferFunctionScalarEvidence::NegativeInfinity)
    );
    assert_eq!(TransferFunctionScalarEvidence::from_f64(f64::NAN), None);

    let scalar = AnalysisResultPayload::ScalarMeasurements {
        values: BTreeMap::from([("gain".to_owned(), 0.5)]),
    };
    assert!(scalar.validate_for(AnalysisType::Tf).is_err());
}

#[test]
fn transfer_function_payload_rejects_contradictory_or_malformed_evidence() {
    let mut invalid_finite = transfer_function_payload();
    let AnalysisResultPayload::TransferFunction { gain, .. } = &mut invalid_finite else {
        unreachable!()
    };
    *gain = Some(TransferFunctionScalarEvidence::Finite(f64::INFINITY));
    assert!(
        invalid_finite
            .validate_for(AnalysisType::Tf)
            .expect_err("infinity cannot use the finite classification")
            .contains("finite classification")
    );

    let mut wrong_unit = transfer_function_payload();
    let AnalysisResultPayload::TransferFunction { input_unit, .. } = &mut wrong_unit else {
        unreachable!()
    };
    *input_unit = "A".to_owned();
    assert!(wrong_unit.validate_for(AnalysisType::Tf).is_err());

    let mut wrong_quantity = transfer_function_payload();
    let AnalysisResultPayload::TransferFunction {
        output_quantity, ..
    } = &mut wrong_quantity
    else {
        unreachable!()
    };
    *output_quantity = TransferFunctionQuantityEvidence::Current;
    assert!(wrong_quantity.validate_for(AnalysisType::Tf).is_err());

    for expression in [" V(OUT,REF)", "V (OUT,REF)", "V( OUT,REF)"] {
        let mut malformed = transfer_function_payload();
        let AnalysisResultPayload::TransferFunction {
            output_expression, ..
        } = &mut malformed
        else {
            unreachable!()
        };
        *output_expression = expression.to_owned();
        assert!(malformed.validate_for(AnalysisType::Tf).is_err());
    }

    let mut empty = transfer_function_payload();
    let AnalysisResultPayload::TransferFunction {
        gain,
        input_resistance,
        output_resistance,
        ..
    } = &mut empty
    else {
        unreachable!()
    };
    *gain = None;
    *input_resistance = None;
    *output_resistance = None;
    assert!(empty.validate_for(AnalysisType::Tf).is_err());
}

#[test]
fn relative_transfer_function_gain_requires_exact_nonzero_nominals() {
    let mut relative = transfer_function_payload();
    let AnalysisResultPayload::TransferFunction {
        normalization,
        nominal_input,
        nominal_output,
        ..
    } = &mut relative
    else {
        unreachable!()
    };
    *normalization = TransferFunctionNormalizationEvidence::RelativeToNominal;
    *nominal_input = Some(1.0);
    *nominal_output = Some(0.5);
    assert!(relative.validate_for(AnalysisType::Tf).is_ok());

    let mut missing = relative.clone();
    let AnalysisResultPayload::TransferFunction { nominal_output, .. } = &mut missing else {
        unreachable!()
    };
    *nominal_output = None;
    assert!(missing.validate_for(AnalysisType::Tf).is_err());

    let mut zero = relative;
    let AnalysisResultPayload::TransferFunction { nominal_input, .. } = &mut zero else {
        unreachable!()
    };
    *nominal_input = Some(0.0);
    assert!(zero.validate_for(AnalysisType::Tf).is_err());
}

#[test]
fn reliability_payload_requires_canonical_devices_and_exact_lifetime_coverage() {
    let device = ReliabilityDeviceEvidence {
        device_id: "M1".to_owned(),
        stress: ReliabilityStressEvidence {
            average_gate_stress_v: 1.2,
            average_drain_stress_v: 1.8,
            average_temperature_k: 358.15,
            duration_s: 3_600.0,
        },
        checkpoints: vec![
            ReliabilityCheckpointEvidence {
                years: 1.0,
                shift: ReliabilityShiftEvidence {
                    threshold_voltage_shift_v: 0.01,
                    mobility_shift: -0.001,
                    drain_source_resistance_shift: 0.0005,
                },
            },
            ReliabilityCheckpointEvidence {
                years: 10.0,
                shift: ReliabilityShiftEvidence {
                    threshold_voltage_shift_v: 0.03,
                    mobility_shift: -0.004,
                    drain_source_resistance_shift: 0.0015,
                },
            },
        ],
    };
    let valid = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
            years: vec![1.0, 10.0],
        })
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![device.clone()],
        });
    assert!(valid.validate_retained_evidence().is_ok());

    let payload_without_axis = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![device.clone()],
        });
    assert!(
        payload_without_axis
            .validate_retained_evidence()
            .expect_err("reliability payload requires its lifetime axis")
            .contains("missing its retained lifetime axis")
    );

    let incomplete = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
            years: vec![1.0, 5.0, 10.0],
        })
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![device],
        });
    assert!(
        incomplete
            .validate_retained_evidence()
            .expect_err("missing lifetime evidence is rejected")
            .contains("do not match")
    );
}

#[test]
fn soa_payload_requires_complete_rule_coverage_consistent_events_and_axis() {
    let evaluation = SoaEvaluationEvidence {
        device_id: "M1".to_owned(),
        parameter: SoaParameterEvidence::DrainSourceVoltage,
        limit_value: 3.3,
        worst_actual_value: 3.2,
        worst_time_s: 1.0,
        sample_count: 2,
        unit: "V".to_owned(),
        description: "Maximum drain-source voltage".to_owned(),
        verdict: SoaRuleVerdictEvidence::Warning,
    };
    let event = SoaViolationEvidence {
        device_id: "M1".to_owned(),
        parameter: SoaParameterEvidence::DrainSourceVoltage,
        limit_value: 3.3,
        actual_value: 3.2,
        time_s: 1.0,
        severity: SoaViolationSeverityEvidence::Warning,
    };
    let valid = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 1.0],
        })
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![evaluation.clone()],
            violations: vec![event.clone()],
        });
    assert!(valid.validate_retained_evidence().is_ok());

    let payload_without_axis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![evaluation.clone()],
            violations: vec![event.clone()],
        });
    assert!(
        payload_without_axis
            .validate_retained_evidence()
            .expect_err("SOA payload requires its time axis")
            .contains("missing its retained time axis")
    );

    let mut invalid_event = event.clone();
    invalid_event.severity = SoaViolationSeverityEvidence::Critical;
    assert!(
        AnalysisResultPayload::Soa {
            evaluations: vec![evaluation.clone()],
            violations: vec![invalid_event],
        }
        .validate_for(AnalysisType::Soa)
        .expect_err("contradictory event severity is rejected")
        .contains("severity")
    );

    let contradictory_limit = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 1.0],
        })
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![evaluation.clone()],
            violations: vec![SoaViolationEvidence {
                limit_value: 3.4,
                ..event.clone()
            }],
        });
    assert!(
        contradictory_limit
            .validate_retained_evidence()
            .expect_err("event rule limit must be exact")
            .contains("contradicts its evaluated rule limit")
    );

    let missing_worst_event = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 1.0],
        })
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![evaluation.clone()],
            violations: Vec::new(),
        });
    assert!(
        missing_worst_event
            .validate_retained_evidence()
            .expect_err("non-pass verdict requires exact worst event")
            .contains("no exact event at its worst point")
    );

    let incomplete = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 0.5, 1.0],
        })
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations: vec![evaluation],
            violations: Vec::new(),
        });
    assert!(
        incomplete
            .validate_retained_evidence()
            .expect_err("incomplete sample coverage is rejected")
            .contains("covers 2 samples")
    );
}

#[test]
fn reliability_and_soa_axes_are_canonical_engineering_coordinates() {
    for years in [Vec::new(), vec![0.0], vec![10.0, 1.0], vec![1.0, 1.0]] {
        assert!(
            AnalysisResultFamilyMetadata::Reliability { years }
                .validate_for(AnalysisType::Reliability)
                .is_err()
        );
    }
    for time in [Vec::new(), vec![-1.0, 0.0], vec![0.0, 0.0], vec![1.0, 0.0]] {
        assert!(
            AnalysisResultFamilyMetadata::Soa { time }
                .validate_for(AnalysisType::Soa)
                .is_err()
        );
    }
    assert!(
        AnalysisResultFamilyMetadata::Reliability {
            years: vec![1.0, 5.0, 10.0],
        }
        .validate_for(AnalysisType::Reliability)
        .is_ok()
    );
    assert!(
        AnalysisResultFamilyMetadata::Soa {
            time: vec![-0.0, 1.0e-9, 2.0e-9],
        }
        .validate_for(AnalysisType::Soa)
        .is_ok()
    );
}
