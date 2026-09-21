//! What the canonical result encoding must keep stable.
//!
//! The assertions here are about identity, not arithmetic: a digest has to
//! change when the authoritative data changes and stay put when only
//! presentation does, and a seal written under an earlier schema has to keep
//! authenticating against the bytes it was written from.

use super::*;
use crate::state::simulation::analysis_result::{
    PssFloquetMultiplierEvidence, PstbFloquetModeEvidence, PstbStabilityClassificationEvidence,
};

fn stable_pstb_result() -> AnalysisResult {
    let certificate = FloquetSpectrumCertificateEvidence {
        problem_order: 1,
        max_backward_error: 0.0,
        qualification_tolerance:
            FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(1).unwrap(),
    };
    let multiplier = ComplexResultValue {
        real: 0.5,
        imaginary: 0.0,
    };
    AnalysisResult::new(1, AnalysisType::Pstb, "PSTB").with_result_payload(
        AnalysisResultPayload::Pstb {
            period_s: Some(1.0),
            fundamental_frequency_hz: Some(1.0),
            stability_threshold: Some(1.0),
            probe_instance: Some("LPROBE".to_owned()),
            detect_subharmonics: Some(false),
            modes: vec![PstbFloquetModeEvidence {
                multiplier,
                exponent: ComplexResultValue {
                    real: 0.5_f64.ln(),
                    imaginary: 0.0,
                },
                probe_participation: 0.25,
                is_unstable: false,
                is_trivial: false,
                subharmonic_order: None,
            }],
            floquet_evidence: FloquetSpectrumEvidence::Qualified { certificate },
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
        },
    )
}

#[test]
fn periodic_payload_fields_are_v8_identity_while_v7_stays_legacy() {
    let pss = AnalysisResult::new(1, AnalysisType::Pss, "PSS").with_result_payload(
        AnalysisResultPayload::PssFloquet {
            period_s: Some(1.0),
            fundamental_frequency_hz: Some(1.0),
            iterations: Some(2),
            residual_norm: Some(1.0e-12),
            multipliers: vec![PssFloquetMultiplierEvidence {
                multiplier: ComplexResultValue {
                    real: 0.5,
                    imaginary: 0.0,
                },
            }],
            floquet_evidence: FloquetSpectrumEvidence::Qualified {
                certificate: FloquetSpectrumCertificateEvidence {
                    problem_order: 1,
                    max_backward_error: 0.0,
                    qualification_tolerance:
                        FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(1)
                            .unwrap(),
                },
            },
            orbit_kind: FloquetOrbitKindEvidence::Driven,
            trivial_multiplier_index: None,
            stability_verdict: FloquetStabilityVerdictEvidence::Stable,
        },
    );
    let mut changed_pss = pss.clone();
    let Some(AnalysisResultPayload::PssFloquet { multipliers, .. }) =
        changed_pss.result_payload.as_mut()
    else {
        unreachable!()
    };
    multipliers[0].multiplier.real = 0.25;
    assert_ne!(pss.result_data_digest(), changed_pss.result_data_digest());
    assert_eq!(
        pss.legacy_v7_result_data_digest(),
        changed_pss.legacy_v7_result_data_digest()
    );

    let source = stable_pstb_result();
    assert!(source.validate_retained_evidence().is_ok());
    let baseline = source.result_data_digest();
    let legacy = source.legacy_v7_result_data_digest();

    macro_rules! assert_mutation_is_identity {
        ($pattern:pat, $mutation:expr) => {{
            let mut changed = source.clone();
            let Some($pattern) = changed.result_payload.as_mut() else {
                unreachable!()
            };
            $mutation;
            assert_ne!(baseline, changed.result_data_digest());
            assert_eq!(
                legacy,
                changed.legacy_v7_result_data_digest(),
                "schema-v16 encoded no periodic payload semantics"
            );
        }};
    }

    assert_mutation_is_identity!(
        AnalysisResultPayload::Pstb { period_s, .. },
        *period_s = Some(2.0)
    );
    assert_mutation_is_identity!(
        AnalysisResultPayload::Pstb {
            stability_threshold,
            ..
        },
        *stability_threshold = Some(1.1)
    );
    assert_mutation_is_identity!(
        AnalysisResultPayload::Pstb { probe_instance, .. },
        *probe_instance = Some("LALT".to_owned())
    );
    assert_mutation_is_identity!(
        AnalysisResultPayload::Pstb { modes, .. },
        modes[0].probe_participation = 0.5
    );
    assert_mutation_is_identity!(
        AnalysisResultPayload::Pstb {
            floquet_evidence,
            ..
        },
        *floquet_evidence = FloquetSpectrumEvidence::NotComputed
    );
    assert_mutation_is_identity!(
        AnalysisResultPayload::Pstb {
            stability_classification,
            ..
        },
        *stability_classification = PstbStabilityClassificationEvidence::Marginal
    );
    assert_mutation_is_identity!(
        AnalysisResultPayload::Pstb { iterations, .. },
        *iterations = Some(1)
    );

    let mut first_run = SimulationRun::new(1);
    first_run.analyses = vec![source.clone()];
    let mut second_run = first_run.clone();
    let Some(AnalysisResultPayload::Pstb { modes, .. }) =
        second_run.analyses[0].result_payload.as_mut()
    else {
        unreachable!()
    };
    modes[0].probe_participation = 0.75;
    assert_ne!(
        first_run.dataset_content_digest(),
        second_run.dataset_content_digest(),
        "a full mode hidden from presentation remains dataset identity"
    );
    assert_eq!(
        first_run.legacy_v7_dataset_content_digest(),
        second_run.legacy_v7_dataset_content_digest()
    );
}

#[test]
fn measurement_verification_fields_are_v9_identity_while_v8_stays_legacy() {
    let mut measurement = rspice_core::MeasureResult::success("peak", 12.0);
    measurement.failure_limit = Some(10.0);
    measurement.failure_limit_exceeded = true;
    measurement.passed = false;
    measurement.error = Some("FAILVALUE limit exceeded".to_owned());
    let source = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
        .with_measurements(vec![measurement]);
    let current = source.result_data_digest();
    let legacy = source.legacy_v8_result_data_digest();

    let mut raw_changed = source.clone();
    raw_changed.measurements[0].raw_value = Some(13.0);
    assert_ne!(current, raw_changed.result_data_digest());
    assert_eq!(legacy, raw_changed.legacy_v8_result_data_digest());

    let mut limit_changed = source.clone();
    limit_changed.measurements[0].failure_limit = Some(11.0);
    assert_ne!(current, limit_changed.result_data_digest());
    assert_eq!(legacy, limit_changed.legacy_v8_result_data_digest());

    let mut verdict_changed = source.clone();
    verdict_changed.measurements[0].failure_limit_exceeded = false;
    assert_ne!(current, verdict_changed.result_data_digest());
    assert_eq!(legacy, verdict_changed.legacy_v8_result_data_digest());

    let mut first_run = SimulationRun::new(1);
    first_run.analyses = vec![source];
    let mut second_run = first_run.clone();
    second_run.analyses[0].measurements[0].raw_value = Some(13.0);
    assert_ne!(
        first_run.dataset_content_digest(),
        second_run.dataset_content_digest()
    );
    assert_eq!(
        first_run.legacy_v8_dataset_content_digest(),
        second_run.legacy_v8_dataset_content_digest()
    );
}

fn operating_point_result() -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_result_payload(
        AnalysisResultPayload::OperatingPoint {
            temperature_mode: OperatingPointTemperatureEvidence::PvtRunSet,
            temperature_celsius: 27.0,
            initial_guess: OperatingPointInitialGuessEvidence::Automatic,
            node_initialization: OperatingPointNodeInitializationEvidence::UseIcAndNodeset,
            homotopy: OperatingPointHomotopyEvidence::Adaptive,
            annotation: OperatingPointAnnotationEvidence::VoltagesAndCurrents,
            device_detail: OperatingPointDeviceDetailEvidence::SelectedAndViolations,
            save_device_op: OperatingPointSaveDeviceEvidence::Enabled,
            accuracy: OperatingPointAccuracyEvidence::Balanced,
            selected_devices: vec!["M1".to_owned()],
            violation_devices: vec!["M2".to_owned()],
            violation_source_content_digest: Some(ContentDigest::from_bytes([7; 32])),
            validated_startup_directives: 2,
            mna_node_names: vec!["in".to_owned(), "out".to_owned()],
            mna_branch_names: vec!["V1".to_owned()],
            mna_solution: vec![1.0, 0.5, -0.5e-3],
            effective_source_content_digest: Some(ContentDigest::from_bytes([8; 32])),
            previous_state: None,
            run_point_index: 1,
            run_point_count: 2,
            run_point_process: OperatingPointProcessEvidence::TT,
            run_point_supply_voltage: None,
            run_point_nominal_supply_voltage: None,
        },
    )
}

#[test]
fn previous_op_lineage_is_optional_but_each_identity_is_authenticated() {
    let legacy = operating_point_result();
    let json = serde_json::to_value(legacy.result_payload.as_ref().unwrap()).unwrap();
    assert!(json.get("previous_state").is_none());
    let mut restored = legacy.clone();
    restored.result_payload = Some(serde_json::from_value(json).unwrap());
    assert_eq!(legacy.result_data_digest(), restored.result_data_digest());
    for field in 0..3 {
        let mut source = legacy.clone();
        let Some(AnalysisResultPayload::OperatingPoint {
            initial_guess,
            previous_state,
            ..
        }) = &mut source.result_payload
        else {
            unreachable!()
        };
        *initial_guess = OperatingPointInitialGuessEvidence::PreviousCompatible;
        *previous_state = Some(OperatingPointPreviousStateEvidence {
            source_content_digest: ContentDigest::from_bytes([1; 32]),
            producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
            producer_result_digest: ContentDigest::from_bytes([3; 32]),
        });
        let baseline = source.result_data_digest();
        assert_ne!(baseline, legacy.result_data_digest());
        let Some(AnalysisResultPayload::OperatingPoint {
            previous_state: Some(previous),
            ..
        }) = &mut source.result_payload
        else {
            unreachable!()
        };
        match field {
            0 => previous.source_content_digest = ContentDigest::from_bytes([9; 32]),
            1 => previous.producer_snapshot_digest = ContentDigest::from_bytes([9; 32]),
            _ => previous.producer_result_digest = ContentDigest::from_bytes([9; 32]),
        }
        assert_ne!(baseline, source.result_data_digest());
    }
}

#[test]
fn operating_point_mna_and_context_are_field_sensitive_content_identity() {
    let source = operating_point_result();
    let baseline = source.result_data_digest();
    let mut changed = source.clone();
    let Some(AnalysisResultPayload::OperatingPoint { mna_solution, .. }) =
        changed.result_payload.as_mut()
    else {
        panic!("OP payload")
    };
    mna_solution[1] = 0.500_000_000_000_000_1;
    assert_ne!(baseline, changed.result_data_digest());

    let mut changed = source.clone();
    let Some(AnalysisResultPayload::OperatingPoint {
        violation_devices, ..
    }) = changed.result_payload.as_mut()
    else {
        panic!("OP payload")
    };
    violation_devices.push("M3".to_owned());
    assert_ne!(baseline, changed.result_data_digest());

    let mut changed = source.clone();
    let Some(AnalysisResultPayload::OperatingPoint {
        run_point_index, ..
    }) = changed.result_payload.as_mut()
    else {
        panic!("OP payload")
    };
    *run_point_index = 0;
    assert_ne!(baseline, changed.result_data_digest());

    let mut changed = source.clone();
    let Some(AnalysisResultPayload::OperatingPoint {
        run_point_process, ..
    }) = changed.result_payload.as_mut()
    else {
        panic!("OP payload")
    };
    *run_point_process = OperatingPointProcessEvidence::SS;
    assert_ne!(baseline, changed.result_data_digest());

    let mut changed = source.clone();
    let Some(AnalysisResultPayload::OperatingPoint {
        run_point_supply_voltage,
        run_point_nominal_supply_voltage,
        ..
    }) = changed.result_payload.as_mut()
    else {
        panic!("OP payload")
    };
    *run_point_supply_voltage = Some(0.9);
    *run_point_nominal_supply_voltage = Some(1.0);
    assert_ne!(baseline, changed.result_data_digest());
}
use std::collections::BTreeMap;
use std::sync::Arc;

fn analysis(kind: AnalysisType) -> AnalysisResult {
    AnalysisResult::new(1, kind, "presentation label").with_waveforms(vec![
        WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.25, 0.5, 0.75],
            "#00aaff",
        )
        .with_complex_components("V(out)", vec![0.25, 0.5, 0.75], vec![-0.5, 0.0, 0.5]),
    ])
}

#[test]
fn analysis_digest_is_deterministic_and_ignores_presentation_cache() {
    let source = analysis(AnalysisType::Ac);
    let mut presentation_changed = source.clone();
    presentation_changed.label = "renamed".to_owned();
    presentation_changed.timestamp += 100.0;
    presentation_changed.waveforms[0].color = "#ff0000".to_owned();
    presentation_changed.waveforms[0].visible = false;
    presentation_changed.waveforms[0].rebuild_display_cache(2);

    assert_eq!(
        source.result_data_digest(),
        source.clone().result_data_digest()
    );
    assert_eq!(
        source.result_data_digest(),
        presentation_changed.result_data_digest()
    );
}

#[test]
fn analysis_digest_detects_real_imaginary_and_kind_changes() {
    let source = analysis(AnalysisType::Ac);
    let mut real_changed = source.clone();
    Arc::make_mut(&mut real_changed.waveforms[0].y)[1] = 0.500_000_000_000_000_1;
    let mut imaginary_changed = source.clone();
    Arc::make_mut(
        &mut imaginary_changed.waveforms[0]
            .complex
            .as_mut()
            .expect("complex evidence")
            .imag,
    )[1] = 1.0e-30;
    let kind_changed = analysis(AnalysisType::Transient);

    assert_ne!(
        source.result_data_digest(),
        real_changed.result_data_digest()
    );
    assert_ne!(
        source.result_data_digest(),
        imaginary_changed.result_data_digest()
    );
    assert_ne!(
        source.result_data_digest(),
        kind_changed.result_data_digest()
    );
}

#[test]
fn retained_waveform_unit_is_content_identity_without_rewriting_v12_history() {
    let unstated = analysis(AnalysisType::Transient);
    let volts = {
        let mut result = unstated.clone();
        result.waveforms[0].unit = Some("V".to_owned());
        result
    };
    let amps = {
        let mut result = unstated.clone();
        result.waveforms[0].unit = Some("A".to_owned());
        result
    };

    assert_ne!(unstated.result_data_digest(), volts.result_data_digest());
    assert_ne!(volts.result_data_digest(), amps.result_data_digest());
    assert_eq!(
        volts.legacy_v5_result_data_digest(),
        amps.legacy_v5_result_data_digest(),
        "schema-v12 never contained waveform unit bytes"
    );

    let mut stated_run = SimulationRun::new(1);
    stated_run.analyses = vec![volts];
    let mut restated_run = stated_run.clone();
    restated_run.analyses = vec![amps];
    assert_ne!(
        stated_run.dataset_content_digest(),
        restated_run.dataset_content_digest()
    );
    assert_eq!(
        stated_run.legacy_v5_dataset_content_digest(),
        restated_run.legacy_v5_dataset_content_digest()
    );
}

#[test]
fn canonical_float_encoding_normalizes_signed_zero_and_nan_payloads() {
    let mut positive_zero = analysis(AnalysisType::Ac);
    let mut negative_zero = positive_zero.clone();
    Arc::make_mut(&mut positive_zero.waveforms[0].y)[0] = 0.0;
    Arc::make_mut(&mut negative_zero.waveforms[0].y)[0] = -0.0;
    assert_eq!(
        positive_zero.result_data_digest(),
        negative_zero.result_data_digest()
    );

    let mut first_nan = positive_zero.clone();
    let mut second_nan = positive_zero;
    Arc::make_mut(&mut first_nan.waveforms[0].y)[0] = f64::from_bits(0x7ff8_0000_0000_0001);
    Arc::make_mut(&mut second_nan.waveforms[0].y)[0] = f64::from_bits(0x7fff_ffff_ffff_ffff);
    assert_eq!(
        first_nan.result_data_digest(),
        second_nan.result_data_digest()
    );
}

#[test]
fn retained_storage_count_covers_authoritative_samples_and_display_cache() {
    let without_cache = analysis(AnalysisType::Transient);
    let mut with_cache = without_cache.clone();
    with_cache.waveforms[0].rebuild_display_cache(2);

    let base = without_cache.retained_storage_bytes();
    let cached = with_cache.retained_storage_bytes();

    assert!(base >= 4 * std::mem::size_of::<f64>() as u64);
    assert_eq!(
        cached - base,
        4 * std::mem::size_of::<f32>() as u64 + std::mem::size_of::<usize>() as u64
    );
    assert_eq!(
        without_cache.result_data_digest(),
        with_cache.result_data_digest(),
        "presentation caches remain outside immutable content identity"
    );
}

#[test]
fn family_samples_and_dataset_order_are_content_identity() {
    let mut first = analysis(AnalysisType::MonteCarlo).with_family_metadata(
        AnalysisResultFamilyMetadata::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 9,
            runs_requested: 2,
            runs_completed: 2,
            failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableMetadata {
                mean_confidence: None,
                name: "V(out)".to_owned(),
                samples: vec![0.99, 1.01],
                mean: 1.0,
                std_dev: 0.01,
                min: 0.99,
                max: 1.01,
            }],
        },
    );
    first.id = 1;
    let mut changed_family = first.clone();
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) =
        changed_family.family_metadata.as_mut()
    else {
        panic!("Monte Carlo family metadata")
    };
    variables[0].samples[1] = 1.02;
    assert_ne!(
        first.result_data_digest(),
        changed_family.result_data_digest()
    );

    let mut second = analysis(AnalysisType::Ac);
    second.id = 2;
    let mut run = SimulationRun::new(1);
    run.analyses = vec![first.clone(), second.clone()];
    let digest = run.dataset_content_digest();
    run.analyses.swap(0, 1);
    assert_ne!(digest, run.dataset_content_digest());
    run.analyses = vec![first, changed_family];
    assert_ne!(digest, run.dataset_content_digest());
}

#[test]
fn periodic_noise_quantity_and_carrier_are_content_identity() {
    let output_psd = analysis(AnalysisType::Pnoise).with_family_metadata(
        AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
            carrier_frequency_hz: Some(2.4e9),
        },
    );
    let phase_noise = analysis(AnalysisType::Pnoise).with_family_metadata(
        AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(2.4e9),
        },
    );
    let other_carrier = analysis(AnalysisType::Pnoise).with_family_metadata(
        AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            carrier_frequency_hz: Some(5.0e9),
        },
    );

    assert_ne!(
        output_psd.result_data_digest(),
        phase_noise.result_data_digest()
    );
    assert_ne!(
        phase_noise.result_data_digest(),
        other_carrier.result_data_digest()
    );
}

#[test]
fn typed_payload_is_current_content_identity_without_rewriting_v1_history() {
    let pole_zero = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
        AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: -1.0,
                imaginary: 2.0,
            }],
            zeros: vec![ComplexResultValue {
                real: -3.0,
                imaginary: 0.0,
            }],
            pole_evidence: PoleZeroRootSetEvidence::Qualified {
                certificate: PoleZeroSpectrumCertificate {
                    problem_order: 1,
                    infinite_count: 0,
                    max_backward_error: 1.0e-14,
                    qualification_tolerance:
                        PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
                },
            },
            zero_evidence: PoleZeroRootSetEvidence::Qualified {
                certificate: PoleZeroSpectrumCertificate {
                    problem_order: 1,
                    infinite_count: 0,
                    max_backward_error: 2.0e-14,
                    qualification_tolerance:
                        PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
                },
            },
            gain: Some(4.0),
        },
    );
    let mut changed_root = pole_zero.clone();
    let Some(AnalysisResultPayload::PoleZero { poles, .. }) = changed_root.result_payload.as_mut()
    else {
        panic!("pole-zero payload")
    };
    poles[0].imaginary = 2.5;

    let mut unavailable_gain = pole_zero.clone();
    let Some(AnalysisResultPayload::PoleZero { gain, .. }) =
        unavailable_gain.result_payload.as_mut()
    else {
        panic!("pole-zero payload")
    };
    *gain = None;

    let mut approximate_evidence = pole_zero.clone();
    let Some(AnalysisResultPayload::PoleZero { pole_evidence, .. }) =
        approximate_evidence.result_payload.as_mut()
    else {
        panic!("pole-zero payload")
    };
    *pole_evidence = PoleZeroRootSetEvidence::Approximate {
        certificate: PoleZeroSpectrumCertificate {
            problem_order: 1,
            infinite_count: 0,
            max_backward_error: 1.0e-9,
            qualification_tolerance:
                PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
        },
    };

    assert_ne!(
        pole_zero.result_data_digest(),
        changed_root.result_data_digest()
    );
    assert_ne!(
        pole_zero.result_data_digest(),
        unavailable_gain.result_data_digest(),
        "gain availability is authenticated result evidence"
    );
    assert_ne!(
        pole_zero.result_data_digest(),
        approximate_evidence.result_data_digest(),
        "root-set qualification is authenticated result evidence"
    );
    assert_eq!(
        pole_zero.legacy_v1_result_data_digest(),
        changed_root.legacy_v1_result_data_digest(),
        "schema-v8 never contained typed payload bytes"
    );

    let mut first_run = SimulationRun::new(1);
    first_run.analyses = vec![pole_zero];
    let mut second_run = first_run.clone();
    second_run.analyses = vec![changed_root];
    assert_ne!(
        first_run.dataset_content_digest(),
        second_run.dataset_content_digest()
    );
    assert_eq!(
        first_run.legacy_v1_dataset_content_digest(),
        second_run.legacy_v1_dataset_content_digest()
    );
}

#[test]
fn sensitivity_and_scalar_payload_digests_are_deterministic_and_field_sensitive() {
    let sensitivity = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
        .with_result_payload(AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac {
                frequency_hz: 1_000.0,
            },
            rows: vec![SensitivityResultRow {
                parameter: "gain".to_owned(),
                raw: (2.0).into(),
                normalized: (0.5).into(),
            }],
        });
    let mut changed = sensitivity.clone();
    let Some(AnalysisResultPayload::Sensitivity { rows, .. }) = changed.result_payload.as_mut()
    else {
        panic!("sensitivity payload")
    };
    rows[0].normalized = 0.500_000_000_000_000_1.into();
    assert_ne!(
        sensitivity.result_data_digest(),
        changed.result_data_digest()
    );

    let scalar = AnalysisResult::new(1, AnalysisType::Disto, "DISTO").with_result_payload(
        AnalysisResultPayload::ScalarMeasurements {
            values: BTreeMap::from([("gain".to_owned(), 10.0), ("resistance".to_owned(), 50.0)]),
        },
    );
    assert_eq!(
        scalar.result_data_digest(),
        scalar.clone().result_data_digest()
    );

    let positive_zero = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
        AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: 0.0,
                imaginary: 0.0,
            }],
            zeros: Vec::new(),
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            gain: Some(1.0),
        },
    );
    let negative_zero = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
        AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: -0.0,
                imaginary: -0.0,
            }],
            zeros: Vec::new(),
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            gain: Some(1.0),
        },
    );
    assert_eq!(
        positive_zero.result_data_digest(),
        negative_zero.result_data_digest()
    );
}

#[test]
fn transfer_function_evidence_is_field_sensitive_v4_content_identity() {
    let source = AnalysisResult::new(1, AnalysisType::Tf, "TF").with_result_payload(
        AnalysisResultPayload::TransferFunction {
            input_source: "VIN".to_owned(),
            output_expression: "V(OUT)".to_owned(),
            input_quantity: TransferFunctionQuantityEvidence::Voltage,
            output_quantity: TransferFunctionQuantityEvidence::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: TransferFunctionNormalizationEvidence::RelativeToNominal,
            accuracy: TransferFunctionAccuracyEvidence::Accurate,
            gain: Some(TransferFunctionScalarEvidence::Finite(0.5)),
            input_resistance: Some(TransferFunctionScalarEvidence::PositiveInfinity),
            output_resistance: Some(TransferFunctionScalarEvidence::Finite(50.0)),
            nominal_input: Some(1.0),
            nominal_output: Some(0.5),
        },
    );
    assert_eq!(
        source.result_data_digest(),
        source.clone().result_data_digest()
    );

    let mutate = |mutation: fn(&mut AnalysisResultPayload)| {
        let mut changed = source.clone();
        mutation(changed.result_payload.as_mut().expect("TF payload"));
        assert_ne!(source.result_data_digest(), changed.result_data_digest());
    };
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { input_source, .. } = payload else {
            unreachable!()
        };
        *input_source = "IIN".to_owned();
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction {
            output_expression, ..
        } = payload
        else {
            unreachable!()
        };
        *output_expression = "V(ALT)".to_owned();
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { input_quantity, .. } = payload else {
            unreachable!()
        };
        *input_quantity = TransferFunctionQuantityEvidence::Current;
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction {
            output_quantity, ..
        } = payload
        else {
            unreachable!()
        };
        *output_quantity = TransferFunctionQuantityEvidence::Current;
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { input_unit, .. } = payload else {
            unreachable!()
        };
        *input_unit = "mV".to_owned();
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { output_unit, .. } = payload else {
            unreachable!()
        };
        *output_unit = "mV".to_owned();
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { normalization, .. } = payload else {
            unreachable!()
        };
        *normalization = TransferFunctionNormalizationEvidence::None;
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { accuracy, .. } = payload else {
            unreachable!()
        };
        *accuracy = TransferFunctionAccuracyEvidence::Robust;
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { gain, .. } = payload else {
            unreachable!()
        };
        *gain = Some(TransferFunctionScalarEvidence::Finite(
            0.500_000_000_000_000_1,
        ));
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction {
            input_resistance, ..
        } = payload
        else {
            unreachable!()
        };
        *input_resistance = Some(TransferFunctionScalarEvidence::NegativeInfinity);
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction {
            output_resistance, ..
        } = payload
        else {
            unreachable!()
        };
        *output_resistance = None;
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { nominal_input, .. } = payload else {
            unreachable!()
        };
        *nominal_input = Some(2.0);
    });
    mutate(|payload| {
        let AnalysisResultPayload::TransferFunction { nominal_output, .. } = payload else {
            unreachable!()
        };
        *nominal_output = Some(0.25);
    });

    assert_ne!(
        source.result_data_digest(),
        source.legacy_v3_result_data_digest(),
        "current results must be sealed in the v6 domain"
    );
}

#[test]
fn reliability_and_soa_evidence_are_field_sensitive_v4_content_identity() {
    let reliability = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
        .with_result_payload(AnalysisResultPayload::Reliability {
            devices: vec![ReliabilityDeviceEvidence {
                device_id: "M1".to_owned(),
                stress: ReliabilityStressEvidence {
                    average_gate_stress_v: 1.2,
                    average_drain_stress_v: 1.8,
                    average_temperature_k: 358.15,
                    duration_s: 3_600.0,
                },
                checkpoints: vec![ReliabilityCheckpointEvidence {
                    years: 10.0,
                    shift: ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.03,
                        mobility_shift: -0.004,
                        drain_source_resistance_shift: 0.0015,
                    },
                }],
            }],
        });
    let mut changed_reliability = reliability.clone();
    let Some(AnalysisResultPayload::Reliability { devices }) =
        changed_reliability.result_payload.as_mut()
    else {
        panic!("reliability payload")
    };
    devices[0].stress.duration_s = 3_601.0;
    assert_ne!(
        reliability.result_data_digest(),
        changed_reliability.result_data_digest()
    );

    let soa = AnalysisResult::new(1, AnalysisType::Soa, "SOA").with_result_payload(
        AnalysisResultPayload::Soa {
            evaluations: vec![SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                derating: None,
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.2,
                worst_time_s: 1.0e-6,
                sample_count: 1_001,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.2,
                time_s: 1.0e-6,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        },
    );
    for (warning_fraction, critical_fraction) in [
        (Some(0.8), Some(1.2)),
        (None, Some(1.2)),
        (Some(0.9), Some(1.5)),
        (Some(0.9), None),
    ] {
        let mut changed = soa.clone();
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = &mut changed.result_payload
        else {
            unreachable!()
        };
        evaluations[0].thresholds = crate::services::safety::SoaThresholds {
            warning_fraction,
            critical_fraction,
        };
        assert_ne!(soa.result_data_digest(), changed.result_data_digest());
    }
    let mut timed = soa.clone();
    let Some(AnalysisResultPayload::Soa { evaluations, .. }) = &mut timed.result_payload else {
        unreachable!()
    };
    evaluations[0].duration = Some(crate::services::safety::SoaDurationEvidence {
        minimum_duration_s: 1e-9,
        total_exceedance_s: 3e-9,
        longest_excursion_s: 2e-9,
        qualified_excursions: 1,
        rejected_excursions: 1,
        clipped_excursions: 0,
    });
    assert_ne!(soa.result_data_digest(), timed.result_data_digest());
    for field in 0..6 {
        let mut changed = timed.clone();
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = &mut changed.result_payload
        else {
            unreachable!()
        };
        let duration = evaluations[0].duration.as_mut().unwrap();
        match field {
            0 => duration.minimum_duration_s *= 2.,
            1 => duration.total_exceedance_s *= 2.,
            2 => duration.longest_excursion_s *= 2.,
            3 => duration.qualified_excursions += 1,
            4 => duration.rejected_excursions += 1,
            _ => duration.clipped_excursions += 1,
        }
        assert_ne!(timed.result_data_digest(), changed.result_data_digest());
    }
    let mut changed_soa = soa.clone();
    let Some(AnalysisResultPayload::Soa { evaluations, .. }) = changed_soa.result_payload.as_mut()
    else {
        panic!("SOA payload")
    };
    evaluations[0].sample_count = 1_002;
    assert_ne!(soa.result_data_digest(), changed_soa.result_data_digest());
    assert_eq!(soa.result_data_digest(), soa.clone().result_data_digest());
    let mut derated = soa.clone();
    let Some(AnalysisResultPayload::Soa { evaluations, .. }) = derated.result_payload.as_mut()
    else {
        unreachable!()
    };
    evaluations[0].parameter = SoaParameterEvidence::PowerDissipation;
    evaluations[0].unit = "W".into();
    evaluations[0].derating = Some(crate::services::safety::SoaPowerDeratingEvidence {
        rated_power_w: 1.0,
        curve: crate::services::safety::SoaPowerDerating {
            reference_temperature_kelvin: 300.0,
            watts_per_kelvin: 0.01,
        },
    });
    for field in 0..3 {
        let mut changed = derated.clone();
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = changed.result_payload.as_mut()
        else {
            unreachable!()
        };
        let derating = evaluations[0].derating.as_mut().unwrap();
        match field {
            0 => derating.rated_power_w = 2.0,
            1 => derating.curve.reference_temperature_kelvin = 310.0,
            _ => derating.curve.watts_per_kelvin = 0.02,
        }
        assert_ne!(derated.result_data_digest(), changed.result_data_digest());
    }
}
/// Two results whose event histories are identical, one of which says
/// eight of the conductors are one word.
fn events_with_and_without_a_bus() -> (AnalysisResult, AnalysisResult) {
    use crate::state::simulation::analysis_result::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence,
    };

    let trace = |name: &str| DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: vec![
            DigitalEventPointEvidence {
                time_s: 0.0,
                value_code: 0,
            },
            DigitalEventPointEvidence {
                time_s: 5.0e-9,
                value_code: 1,
            },
        ],
    };
    let payload = |digital_buses| AnalysisResultPayload::TransientEvents {
        current_impulses: None,
        digital_traces: vec![trace("count#1"), trace("count#0")],
        real_traces: Vec::new(),
        digital_buses,
    };
    let analysis = |digital_buses| {
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(payload(digital_buses))
    };
    (
        analysis(Vec::new()),
        analysis(vec![DigitalBusEvidence {
            name: "count".to_owned(),
            msb: 1,
            lsb: 0,
            members: vec!["count#1".to_owned(), "count#0".to_owned()],
            source: DigitalBusSourceEvidence::Engine,
        }]),
    )
}

#[test]
fn declaring_a_bus_over_the_same_traces_is_a_different_result() {
    let (plain, bussed) = events_with_and_without_a_bus();
    assert_ne!(plain.result_data_digest(), bussed.result_data_digest());
    assert_eq!(bussed.result_data_digest(), bussed.result_data_digest());
}

/// The V9 encoding is what schema-v18 files were sealed with. It has to
/// keep answering the same way for a table it never knew about, or every
/// such file fails authentication the moment one is added.
#[test]
fn the_schema_v18_encoding_cannot_see_a_bus_table() {
    let (plain, bussed) = events_with_and_without_a_bus();
    assert_eq!(
        plain.legacy_v9_result_data_digest(),
        bussed.legacy_v9_result_data_digest()
    );
    assert_ne!(
        plain.legacy_v9_result_data_digest(),
        plain.result_data_digest(),
        "V10 states the table, even an empty one, so it is a different domain"
    );
}

/// A two-contributor divider spread, shaped as the engine produces one.
fn dc_mismatch_evidence() -> crate::state::DcMismatchEvidence {
    use crate::state::{DcMismatchContributorEvidence, DcMismatchScopeEvidence};

    crate::state::DcMismatchEvidence {
        output: "V(OUT)".to_owned(),
        output_unit: "V".to_owned(),
        nominal_value: 2.0 / 3.0,
        sigma_multiplier: 3.0,
        sigma_total: (5.0_f64).sqrt() * 1.0e-3,
        sigma_mismatch: (5.0_f64).sqrt() * 1.0e-3,
        sigma_process: 0.0,
        include_mismatch: true,
        include_process: false,
        contributor_limit: 10,
        threshold: 0.0,
        normalized_contributions: true,
        applied_correlations_mismatch: 0,
        applied_correlations_process: 0,
        evaluated_contributors: 2,
        contributors: vec![
            DcMismatchContributorEvidence {
                instance: "R1".to_owned(),
                parameter: "R1V".to_owned(),
                scope: DcMismatchScopeEvidence::Mismatch,
                sigma_parameter: 10.0,
                sensitivity: 2.0e-4,
                contribution: 2.0e-3,
                share: 0.8,
            },
            DcMismatchContributorEvidence {
                instance: "R2".to_owned(),
                parameter: "R2V".to_owned(),
                scope: DcMismatchScopeEvidence::Mismatch,
                sigma_parameter: 10.0,
                sensitivity: -1.0e-4,
                contribution: -1.0e-3,
                share: 0.2,
            },
        ],
    }
}

/// Every field of a DC mismatch payload is content identity.
///
/// Walked field by field rather than spot-checked: a field the encoder forgot
/// is a field an edited result could change while keeping the seal that
/// authenticated it. The list is the struct's own, so a field added without
/// an encoder line fails here.
#[test]
fn every_dc_mismatch_evidence_field_moves_the_result_digest() {
    // Assigned rather than attached through `with_result_payload`: that
    // constructor debug-asserts the payload is valid, and a field-coverage
    // walk has to perturb one field at a time, which is exactly what an
    // invariant like the variance identity refuses. What is under test here
    // is the encoder's reach, not the validator's.
    let result = |evidence: crate::state::DcMismatchEvidence| {
        let mut analysis = AnalysisResult::new(1, AnalysisType::DcMismatch, "DCMATCH");
        analysis.result_payload = Some(AnalysisResultPayload::DcMismatch {
            evidence: std::sync::Arc::new(evidence),
        });
        analysis
    };
    let source = result(dc_mismatch_evidence());
    assert_eq!(
        source.result_data_digest(),
        result(dc_mismatch_evidence()).result_data_digest(),
        "the same evidence digests to the same bytes"
    );

    type EvidenceEdit = (&'static str, fn(&mut crate::state::DcMismatchEvidence));
    let mutations: [EvidenceEdit; 16] = [
        ("output", |e| e.output = "V(MID)".to_owned()),
        ("output_unit", |e| e.output_unit = "A".to_owned()),
        ("nominal_value", |e| e.nominal_value += 1.0e-15),
        ("sigma_multiplier", |e| e.sigma_multiplier = 6.0),
        ("sigma_total", |e| e.sigma_total += 1.0e-15),
        ("sigma_mismatch", |e| e.sigma_mismatch += 1.0e-15),
        ("sigma_process", |e| e.sigma_process = 1.0e-9),
        ("include_mismatch", |e| e.include_mismatch = false),
        ("include_process", |e| e.include_process = true),
        ("contributor_limit", |e| e.contributor_limit = 0),
        ("threshold", |e| e.threshold = 0.05),
        ("normalized_contributions", |e| {
            e.normalized_contributions = false
        }),
        ("applied_correlations_mismatch", |e| {
            e.applied_correlations_mismatch = 1
        }),
        ("applied_correlations_process", |e| {
            e.applied_correlations_process = 1
        }),
        ("evaluated_contributors", |e| e.evaluated_contributors = 6),
        ("contributors", |e| e.contributors.truncate(1)),
    ];
    for (field, mutate) in mutations {
        let mut evidence = dc_mismatch_evidence();
        mutate(&mut evidence);
        assert_ne!(
            source.result_data_digest(),
            result(evidence).result_data_digest(),
            "{field} does not reach the result digest"
        );
    }

    // And each contributor field in turn.
    type ContributorEdit = (
        &'static str,
        fn(&mut crate::state::DcMismatchContributorEvidence),
    );
    let rows: [ContributorEdit; 7] = [
        ("instance", |row| row.instance = "R3".to_owned()),
        ("parameter", |row| row.parameter = "R3V".to_owned()),
        ("scope", |row| {
            row.scope = crate::state::DcMismatchScopeEvidence::Process
        }),
        ("sigma_parameter", |row| row.sigma_parameter = 11.0),
        ("sensitivity", |row| row.sensitivity += 1.0e-18),
        ("contribution", |row| row.contribution += 1.0e-18),
        ("share", |row| row.share = 0.7),
    ];
    for (field, mutate) in rows {
        let mut evidence = dc_mismatch_evidence();
        mutate(&mut evidence.contributors[0]);
        assert_ne!(
            source.result_data_digest(),
            result(evidence).result_data_digest(),
            "contributor {field} does not reach the result digest"
        );
    }
}

/// Every field of a sensitivity study is content identity.
///
/// Walked field by field rather than spot-checked: a field the encoder forgot
/// is a field an edited result could change while keeping the seal that
/// authenticated it. The filter and the grid are in the walk for the same
/// reason the numbers are — two studies of one deck that selected different
/// variables, or solved different bands, are different results.
#[test]
fn every_sensitivity_study_field_moves_the_result_digest() {
    use crate::state::{SensitivityBasisEvidence, SensitivityStudyEvidence, SensitivityStudyRow};
    use rspice_core::analysis::sensitivity::{SensitivityUnavailability, SensitivityValue};

    fn study() -> SensitivityStudyEvidence {
        SensitivityStudyEvidence {
            output: "V(OUT)".to_owned(),
            filter: "R* PARAM:*".to_owned(),
            basis: SensitivityBasisEvidence::Ac {
                frequencies_hz: vec![10.0, 100.0],
                output: vec![
                    ComplexResultValue {
                        real: 1.0,
                        imaginary: 0.0,
                    },
                    ComplexResultValue {
                        real: 0.5,
                        imaginary: -0.25,
                    },
                ],
            },
            rows: vec![SensitivityStudyRow {
                parameter: "PARAM:GAIN".to_owned(),
                nominal_value: 2.0,
                raw: vec![
                    SensitivityValue::Available(1.0),
                    SensitivityValue::Available(2.0),
                ],
                normalized: vec![
                    SensitivityValue::Available(0.25),
                    SensitivityValue::Available(0.5),
                ],
                phase: vec![
                    SensitivityValue::Available(0.0),
                    SensitivityValue::Available(-1.5),
                ],
            }],
        }
    }

    // Assigned rather than attached through `with_result_payload`: that
    // constructor debug-asserts the payload is valid, and a field-coverage
    // walk perturbs one field at a time, which an invariant like "every
    // column spans the grid" refuses. The encoder's reach is what is under
    // test, not the validator's.
    let result = |evidence: SensitivityStudyEvidence| {
        let mut analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS");
        analysis.result_payload = Some(AnalysisResultPayload::SensitivityStudy {
            evidence: std::sync::Arc::new(evidence),
        });
        analysis
    };
    let source = result(study());
    assert_eq!(
        source.result_data_digest(),
        result(study()).result_data_digest(),
        "the same evidence digests to the same bytes"
    );

    type StudyEdit = (&'static str, fn(&mut SensitivityStudyEvidence));
    let mutations: [StudyEdit; 8] = [
        ("output", |e| e.output = "V(MID)".to_owned()),
        ("filter", |e| e.filter = "R*".to_owned()),
        ("basis kind", |e| {
            e.basis = SensitivityBasisEvidence::Dc { output: 1.0 }
        }),
        ("basis frequencies", |e| {
            if let SensitivityBasisEvidence::Ac { frequencies_hz, .. } = &mut e.basis {
                frequencies_hz[1] = 200.0;
            }
        }),
        ("basis output", |e| {
            if let SensitivityBasisEvidence::Ac { output, .. } = &mut e.basis {
                output[0].imaginary = 1.0e-18;
            }
        }),
        ("rows", |e| e.rows.clear()),
        ("row parameter", |e| {
            e.rows[0].parameter = "PARAM:SCALE".to_owned()
        }),
        ("row nominal_value", |e| e.rows[0].nominal_value += 1.0e-15),
    ];
    for (field, mutate) in mutations {
        let mut evidence = study();
        mutate(&mut evidence);
        assert_ne!(
            source.result_data_digest(),
            result(evidence).result_data_digest(),
            "{field} does not reach the result digest"
        );
    }

    // Every column, at every point, including the reason an unavailable
    // value gives: "zero output" and "out of range" are different answers.
    type ColumnEdit = (&'static str, fn(&mut SensitivityStudyRow));
    let columns: [ColumnEdit; 6] = [
        ("raw[0]", |row| {
            row.raw[0] = SensitivityValue::Available(9.0)
        }),
        ("raw[1]", |row| {
            row.raw[1] = SensitivityValue::Available(9.0)
        }),
        ("normalized[0]", |row| {
            row.normalized[0] = SensitivityValue::Available(9.0)
        }),
        ("normalized[1]", |row| {
            row.normalized[1] = SensitivityValue::unavailable(SensitivityUnavailability::OutOfRange)
        }),
        ("phase[0]", |row| {
            row.phase[0] = SensitivityValue::Available(9.0)
        }),
        ("phase[1]", |row| {
            row.phase[1] =
                SensitivityValue::unavailable(SensitivityUnavailability::NondifferentiableMagnitude)
        }),
    ];
    for (field, mutate) in columns {
        let mut evidence = study();
        mutate(&mut evidence.rows[0]);
        assert_ne!(
            source.result_data_digest(),
            result(evidence).result_data_digest(),
            "{field} does not reach the result digest"
        );
    }
}

/// Every typed payload arm opens with a tag no other arm uses.
///
/// Two lanes added a payload on the same day and each took "the next unused
/// tag" from its own base: both wrote 11, nothing conflicted textually, and
/// every gate passed. The tags are read out of the encoder's own source so the
/// list cannot fall behind it.
#[test]
fn every_typed_payload_digests_under_its_own_tag() {
    let source = crate::source_guard::production_source(include_str!("../result_digest.rs"));
    let mut tags: Vec<(u32, String)> = Vec::new();
    let mut arm: Option<String> = None;
    for line in source.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("AnalysisResultPayload::") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            arm = Some(name);
        } else if let (Some(name), Some(rest)) = (arm.as_ref(), line.strip_prefix("writer.u8("))
            && let Some(tag) = rest
                .strip_suffix(");")
                .and_then(|tag| tag.parse::<u32>().ok())
        {
            tags.push((tag, name.clone()));
            arm = None;
        }
    }
    assert!(
        tags.len() >= 14,
        "the scan stopped finding the payload arms it is here to compare: {tags:?}"
    );
    let mut seen = std::collections::BTreeMap::new();
    for (tag, name) in &tags {
        if let Some(first) = seen.insert(*tag, name.clone()) {
            assert_eq!(
                &first, name,
                "payload tag {tag} is written by both {first} and {name}"
            );
        }
    }
}

#[test]
fn monte_carlo_mean_confidence_is_authenticated_without_changing_absent_evidence() {
    use crate::state::{MonteCarloMeanConfidence, MonteCarloMeanInterval, MonteCarloMeanMethod};
    let mut result = analysis(AnalysisType::MonteCarlo).with_family_metadata(
        AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 9,
            runs_requested: 2,
            runs_completed: 2,
            failures: 0,
            all_converged: true,
            member_measurements: Vec::new(),
            variables: vec![MonteCarloVariableMetadata {
                name: "V(out)".into(),
                samples: vec![0.9, 1.1],
                mean: 1.0,
                std_dev: 0.1,
                min: 0.9,
                max: 1.1,
                mean_confidence: None,
            }],
        },
    );
    let absent = result.result_data_digest();
    let evidence = MonteCarloMeanConfidence {
        level_pct: 95.0,
        method: MonteCarloMeanMethod::StudentT,
        successful_samples: 2,
        conditional_on_successful_trials: false,
        interval: MonteCarloMeanInterval::Available {
            lower: 0.5,
            upper: 1.5,
        },
    };
    let set = |result: &mut AnalysisResult, evidence| {
        let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) =
            &mut result.family_metadata
        else {
            panic!("Monte Carlo")
        };
        variables[0].mean_confidence = evidence;
    };
    set(&mut result, Some(evidence));
    let retained = result.result_data_digest();
    assert_ne!(absent, retained);
    for changed in [
        MonteCarloMeanConfidence {
            level_pct: 90.0,
            ..evidence
        },
        MonteCarloMeanConfidence {
            method: MonteCarloMeanMethod::PercentileBootstrap {
                resamples: 1000,
                seed: 7,
            },
            ..evidence
        },
        MonteCarloMeanConfidence {
            successful_samples: 3,
            ..evidence
        },
        MonteCarloMeanConfidence {
            conditional_on_successful_trials: true,
            ..evidence
        },
        MonteCarloMeanConfidence {
            interval: MonteCarloMeanInterval::Available {
                lower: 0.4,
                upper: 1.5,
            },
            ..evidence
        },
    ] {
        set(&mut result, Some(changed));
        assert_ne!(retained, result.result_data_digest());
    }
    set(&mut result, None);
    assert_eq!(absent, result.result_data_digest());
}

#[test]
fn monte_carlo_sequence_trial_digest_seals_stream_policy_seed_and_original_index() {
    use crate::state::{FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements};
    let metadata = AnalysisResultFamilyMetadata::MonteCarlo {
        seed: u64::MAX,
        runs_requested: 1,
        runs_completed: 1,
        failures: 0,
        all_converged: true,
        variables: vec![MonteCarloVariableMetadata {
            name: "V(out)".into(),
            samples: vec![1.0],
            mean: 1.0,
            std_dev: 0.0,
            min: 1.0,
            max: 1.0,
            mean_confidence: None,
        }],
        member_measurements: vec![FamilyMemberMeasurements::new(
            FamilyMemberId::MonteCarloSequenceTrial {
                index: 0,
                seed: u64::MAX,
                policy: "parameter-xoroshiro128plus-2018-v1".into(),
            },
            vec![FamilyMeasurementEvidence {
                name: "V(out)".into(),
                value: Some(1.0),
                passed: true,
                error: None,
            }],
        )],
    };
    let original = analysis(AnalysisType::MonteCarlo).with_family_metadata(metadata);
    for mutation in 0..4 {
        let mut changed = original.clone();
        let Some(AnalysisResultFamilyMetadata::MonteCarlo {
            member_measurements,
            ..
        }) = &mut changed.family_metadata
        else {
            panic!("MC")
        };
        let FamilyMemberId::MonteCarloSequenceTrial {
            index,
            seed,
            policy,
        } = &mut member_measurements[0].member
        else {
            panic!("sequence")
        };
        match mutation {
            0 => *index = 1,
            1 => *seed = 7,
            2 => *policy = "spectre-coordinate-splitmix64-v1".into(),
            _ => {
                member_measurements[0].member = FamilyMemberId::MonteCarloTrial {
                    index: 0,
                    seed: u64::MAX,
                }
            }
        }
        assert_ne!(original.result_data_digest(), changed.result_data_digest());
    }
}
