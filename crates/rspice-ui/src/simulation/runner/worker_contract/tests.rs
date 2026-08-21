//! Round-trip and rejection tests for the worker protocol.
//!
//! Most cases here assert a refusal: a payload over the ingress limit, a
//! buffer whose length contradicts the metadata claiming it, or a response
//! whose id does not match the request. The round-trip cases pin the other
//! half — that a result which does survive transport is bit-identical to the
//! one that was sent.

use super::*;

mod result_round_trip;

#[test]
fn behavioral_reference_error_round_trips_through_worker_contract() {
    let expected = SimulationError::BehavioralReference {
        owner_name: "b2".to_string(),
        canonical_owner_name: "B2".to_string(),
        dependency_name: "b1".to_string(),
        canonical_dependency_name: "B1".to_string(),
        reason: "lead_current_not_solution_variable".to_string(),
    };

    let worker = WorkerSimulationError::from(expected.clone());
    let encoded = serde_json::to_string(&worker).expect("worker error serializes");
    let decoded: WorkerSimulationError =
        serde_json::from_str(&encoded).expect("worker error deserializes");

    assert_eq!(SimulationError::from(decoded), expected);
}

pub(super) fn retained_pss_operating_point() -> rspice_core::engine::PssOperatingPoint {
    let config = rspice_core::analysis::PssConfig::new(1.0)
        .with_harmonics(4)
        .with_points_per_period(16);
    let time = (0..=16)
        .map(|index| index as f64 / 16.0)
        .collect::<Vec<_>>();
    let waveform = time
        .iter()
        .map(|time| (2.0 * std::f64::consts::PI * time).sin())
        .collect();
    let result = rspice_core::analysis::pss::PssResult {
        period: 1.0,
        frequency: 1.0,
        iterations: 2,
        residual_norm: 1.0e-10,
        time,
        waveforms: vec![rspice_core::analysis::pss::PeriodicWaveform::from_values(
            waveform,
        )],
        node_names: vec!["out".to_owned()],
        period_detected: false,
        floquet_multipliers: vec![num_complex::Complex64::new(0.9, 0.0)],
    };
    rspice_core::engine::PssOperatingPoint::try_from_parts(
        config,
        rspice_core::engine::PssAnalysisResult {
            result,
            iterations: 2,
            final_residual: 1.0e-10,
            period: 1.0,
            monodromy: vec![vec![0.9]],
            floquet_multipliers: vec![num_complex::Complex64::new(0.9, 0.0)],
            is_stable: true,
        },
        vec![0.25],
    )
    .unwrap()
}

fn retained_hb_operating_point() -> rspice_core::engine::HbOperatingPoint {
    let config = rspice_core::analysis::HbConfig::new(1.0).with_harmonics(4);
    rspice_core::engine::HbOperatingPoint::try_from_parts(
        config,
        vec!["out".to_owned()],
        vec![vec![
            num_complex::Complex64::new(0.1, 0.0),
            num_complex::Complex64::new(0.2, -0.1),
            num_complex::Complex64::new(0.05, 0.02),
            num_complex::Complex64::new(0.01, 0.0),
            num_complex::Complex64::new(0.005, -0.001),
        ]],
        3,
        1.0e-10,
    )
    .unwrap()
}

fn tf_spec() -> AnalysisSpec {
    AnalysisSpec::Tf {
        input_source: "Vstim".to_owned(),
        output_expression: "V(out)".to_owned(),
        transfer_gain: true,
        input_resistance: true,
        output_resistance: true,
        normalization: TfNormalization::None,
        accuracy: TfAccuracy::Balanced,
    }
}

pub(super) fn nondefault_op_config() -> crate::simulation::dialog::OpConfig {
    use crate::simulation::dialog::*;

    OpConfig {
        temperature_mode: OpTemperatureMode::Explicit,
        temperature_celsius: 85.0,
        initial_guess: OpInitialGuess::PreviousConverged,
        node_initialization: OpNodeInitialization::IgnoreIcAndNodeset,
        homotopy: OpHomotopy::PseudoTransient,
        annotation: OpAnnotation::VoltagesAndDeviceOp,
        device_detail: OpDeviceDetail::ViolationsOnly,
        save_device_op: OpSaveDevice::FinalPointOnly,
        accuracy: OpAccuracy::Robust,
        selected_devices: vec!["M1".to_owned()],
        previous_state: Some(OpPreviousState {
            source_content_digest: crate::product::ContentDigest::from_bytes([1; 32]),
            producer_snapshot_digest: crate::product::ContentDigest::from_bytes([2; 32]),
            producer_result_digest: crate::product::ContentDigest::from_bytes([3; 32]),
            node_names: vec!["out".to_owned()],
            branch_names: vec!["V1".to_owned()],
            solution: vec![1.25, -1.0e-3],
        }),
        violation_devices: vec!["M1".to_owned()],
        violation_source_content_digest: Some(crate::product::ContentDigest::from_bytes([1; 32])),
        run_point: OpRunPointContext {
            index: 2,
            count: 3,
            process: crate::product::ProcessCorner::SS,
            supply_voltage: Some(0.9),
            nominal_supply_voltage: Some(1.0),
            supply_source_names: vec!["VDD".to_owned()],
        },
    }
}

#[test]
fn browser_worker_transfer_protocol_matches_rust_transport() {
    let source = include_str!("../../../../web/simulation-worker.js");
    assert!(source.contains(&format!(
        "const WORKER_PROTOCOL_VERSION = {WORKER_RESPONSE_TRANSPORT_PROTOCOL};"
    )));
    assert!(source.contains(&format!(
        "const WORKER_REQUEST_PROTOCOL_VERSION = {WORKER_REQUEST_TRANSPORT_PROTOCOL};"
    )));
    assert!(source.contains("response.protocolVersion !== expectedProtocolVersion"));
    assert!(source.contains("protocolResponseTransferList(response, WORKER_PROTOCOL_VERSION)"));
    assert!(source.contains("request.protocolVersion !== WORKER_REQUEST_PROTOCOL_VERSION"));
}
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    PoleZeroConfig, PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    PssMethod, SpPort,
};
use crate::simulation::results::{
    DcOpResult, SimulationResult, TransferFunctionQuantity, TransferFunctionScalar, WaveformData,
};
use std::collections::HashMap;

#[test]
fn worker_request_round_trips_through_json() {
    let request = WorkerRequest {
        id: 7,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::Transient {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: Some(1e-9),
            uic: false,
        })),
        netlist: "V1 in 0 1\nR1 in 0 1k\n.tran 1n 1u\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let encoded = serde_json::to_string(&request).expect("request serializes");
    let decoded: WorkerRequest = serde_json::from_str(&encoded).expect("request deserializes");

    assert_eq!(decoded, request);
}

#[test]
fn legacy_dc_op_worker_requests_migrate_to_the_current_default_contract() {
    let config: WorkerAnalysisConfig =
        serde_json::from_str("\"DcOp\"").expect("legacy config wire value");
    assert_analysis_configs_match(&AnalysisConfig::from(config), &AnalysisConfig::dc_op());

    let spec: WorkerAnalysisSpec =
        serde_json::from_str("\"DcOp\"").expect("legacy spec wire value");
    assert_eq!(AnalysisSpec::from(spec), AnalysisSpec::LegacyDcOp);
}

#[test]
fn configured_dc_op_worker_contract_round_trips_every_context_field() {
    let config = nondefault_op_config();
    let worker = WorkerAnalysisConfig::from(&AnalysisConfig::DcOp(config.clone()));
    let encoded = serde_json::to_vec(&worker).expect("configured OP serializes");
    let restored: WorkerAnalysisConfig =
        serde_json::from_slice(&encoded).expect("configured OP restores");
    assert_analysis_configs_match(
        &AnalysisConfig::from(restored),
        &AnalysisConfig::DcOp(config),
    );
}

#[test]
fn legacy_fourier_specs_retain_dimensional_thd_behavior() {
    let fields = serde_json::json!({
        "fundamental_freq": 1.0,
        "num_harmonics": 4,
        "output_node": "out",
        "output_ref": "0",
        "start_time": 0.0,
        "stop_time": 1.0
    });
    let analysis: AnalysisSpec = serde_json::from_value(serde_json::json!({
        "Fourier": fields.clone()
    }))
    .expect("legacy analysis spec deserializes");
    let worker: WorkerAnalysisSpec = serde_json::from_value(serde_json::json!({
        "Fourier": fields
    }))
    .expect("legacy worker spec deserializes");

    assert!(matches!(
        analysis,
        AnalysisSpec::Fourier {
            compute_thd: true,
            normalize: false,
            ..
        }
    ));
    assert!(matches!(
        worker,
        WorkerAnalysisSpec::Fourier {
            compute_thd: true,
            normalize: false,
            ..
        }
    ));
}

#[test]
fn legacy_envelope_specs_migrate_identically_across_worker_transport() {
    let fields = serde_json::json!({
        "fundamental_freq": 1.0e6,
        "stop_time": 10.0e-3,
        "num_harmonics": 9,
        "max_step": 1.0e-6
    });
    let analysis: AnalysisSpec = serde_json::from_value(serde_json::json!({
        "Envelope": fields.clone()
    }))
    .expect("legacy analysis spec deserializes");
    let worker: WorkerAnalysisSpec = serde_json::from_value(serde_json::json!({
        "Envelope": fields
    }))
    .expect("legacy worker spec deserializes");

    let expected = AnalysisSpec::Envelope {
        fundamental_freq: 1.0e6,
        additional_carrier_tones: Vec::new(),
        stop_time: 10.0e-3,
        num_harmonics: 9,
        envelope_step: Some(1.0e-6),
        modulation_sources: Vec::new(),
        initial_periodic_solve: EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
        adaptive_mode: EnvelopeAdaptiveMode::FixedEnvelopeStep,
        extraction_path: EnvelopeExtractionPath::Projection,
    };
    assert_eq!(analysis, expected);
    assert_eq!(AnalysisSpec::from(worker), expected);
}

#[test]
fn fourier_worker_consumes_exact_transient_dependency_artifact() {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::execution::{
        ExecutionArtifactEnvelope, PreparedDependencyBinding, ResolvedExecutionDependencies,
    };

    let producer = AnalysisInstanceId::new();
    let source_revision = ObjectRevision::new(4).unwrap();
    let snapshot_digest = ContentDigest::from_bytes([0x41; 32]);
    let config_digest = ContentDigest::from_bytes([0x52; 32]);
    let time = (0..=200)
        .map(|index| f64::from(index) * 0.005)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|time| (2.0 * std::f64::consts::PI * 2.0 * time).sin())
        .collect::<Vec<_>>();
    let transient = SimulationResult::Transient {
        time: time.clone(),
        waveforms: HashMap::from([(
            "V(out)".to_owned(),
            WaveformData::new_time_domain("V(out)", time, values),
        )]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: Default::default(),
    };
    let artifact = ExecutionArtifactEnvelope::from_transient_result(
        snapshot_digest,
        producer,
        source_revision,
        config_digest,
        &transient,
        &["out".to_owned()],
    )
    .unwrap()
    .unwrap();
    let binding =
        PreparedDependencyBinding::transient_trajectory(producer, source_revision, config_digest);
    let dependencies = ResolvedExecutionDependencies::resolve(
        snapshot_digest,
        vec![binding],
        &HashMap::from([(producer, artifact)]),
    )
    .unwrap();
    let request = WorkerRequest {
        id: 9,
        request: WorkerSimulationRequest::Spec {
            spec: Box::new(WorkerAnalysisSpec::Fourier {
                fundamental_freq: 2.0,
                num_harmonics: 4,
                output_node: "out".to_owned(),
                output_ref: "0".to_owned(),
                start_time: 0.0,
                stop_time: 1.0,
                compute_thd: true,
                normalize: false,
            }),
            options: Box::new(WorkerSpecExecutionOptions::default()),
        },
        // Deliberately not a valid circuit: Fourier must consume the
        // bound trajectory rather than launch a replacement transient.
        netlist: "artifact-only Fourier request".to_owned(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies,
        environment: None,
        stream_transient_samples: false,
    };
    let transfer = WorkerRequestTransport::from_request(request.clone()).unwrap();
    assert_eq!(transfer.protocol, WORKER_REQUEST_TRANSPORT_PROTOCOL);
    assert_eq!(transfer.buffers.len(), 2);
    let metadata = serde_json::to_vec(&transfer.request).unwrap();
    assert!(
        metadata.len() < 4_096,
        "artifact samples must stay out of request metadata"
    );
    let restored = transfer.into_request().unwrap();
    assert_eq!(restored, request);

    let result = worker_response_from_request(restored)
        .into_result()
        .expect("worker Fourier consumes authenticated trajectory");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } => {
            assert_eq!(frequencies, vec![0.0, 2.0, 4.0, 6.0, 8.0]);
            assert!(waveforms.contains_key("V(out) Spectrum"));
        }
        other => panic!("expected Fourier AC result, got {other:?}"),
    }
}

#[test]
fn worker_request_round_trips_project_veriloga_runtime_artifacts() {
    let project_id = crate::product::ProjectId::new();
    let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::code_workspace(
                crate::state::ProjectSourceLanguage::VerilogA,
            ),
            crate::state::ProjectSourceLanguage::VerilogA,
            "different_file_name.va",
            "module worker_owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
            [],
            [],
        )
        .unwrap();
    let receipt = crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
        project_id,
        &bundle,
        Some("worker_owned"),
    )
    .unwrap();
    let runtime =
        crate::simulation::veriloga::PreparedVerilogARuntime::try_from_current_bundle_receipt(
            project_id, &bundle, &receipt,
        )
        .unwrap();
    let request = WorkerRequest {
        id: 8,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::Transient {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        })),
        netlist: format!(
            "{}\n.end\n",
            crate::simulation::veriloga::project_veriloga_directive(
                runtime.source_key(),
                runtime.module_name()
            )
        ),
        source_path: None,
        project_veriloga_runtimes:
            crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime]).unwrap(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let encoded = serde_json::to_vec(&request).unwrap();
    let restored: WorkerRequest = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(restored, request);
    assert!(
        restored
            .project_veriloga_runtimes
            .iter()
            .next()
            .unwrap()
            .validate()
            .is_ok()
    );
}

#[test]
fn worker_request_detaches_and_authenticates_op_previous_state() {
    let config = nondefault_op_config();
    let request = WorkerRequest {
        id: 10,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::DcOp(config))),
        netlist: "V1 out 0 1\n.op\n.end\n".to_owned(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let transport = WorkerRequestTransport::from_request(request.clone()).unwrap();
    let metadata = serde_json::to_string(&transport.request).unwrap();
    assert_eq!(transport.request.dependency_buffer_count, 0);
    assert_eq!(transport.buffers, vec![vec![1.25, -1.0e-3]]);
    assert!(metadata.contains("\"solution\":{\"Buffer\""));
    assert!(!metadata.contains("\"solution\":[1.25"));
    assert_eq!(transport.clone().into_request().unwrap(), request);

    let mut tampered = transport.clone();
    tampered.buffers[0][0] = 1.5;
    assert!(
        tampered
            .into_request()
            .unwrap_err()
            .contains("solution digest")
    );

    let mut oversized = transport.clone();
    oversized
        .request
        .op_previous_state
        .as_mut()
        .unwrap()
        .solution = WorkerF64Series::Buffer {
        buffer: 0,
        len: MAX_WORKER_F64_VALUES + 1,
    };
    assert!(oversized.into_request().unwrap_err().contains("exceeding"));

    let mut duplicate = transport;
    let WorkerSimulationRequest::Config(config) = &mut duplicate.request.request.request else {
        panic!("expected configured OP request")
    };
    let WorkerAnalysisConfig::DcOp(config) = config.as_mut() else {
        panic!("expected configured OP request")
    };
    config.previous_state = nondefault_op_config().previous_state;
    assert!(
        duplicate
            .into_request()
            .unwrap_err()
            .contains("duplicate inline")
    );
}

#[test]
fn unavailable_manifest_spec_round_trips_without_losing_typed_fields() {
    let spec = AnalysisSpec::DcMismatch {
        output_expression: "V(out)".to_owned(),
        sigma_multiplier: 3.0,
        contributor_limit: 25,
        include_process: false,
        include_mismatch: true,
        normalized_contributions: true,
    };
    let worker = WorkerAnalysisSpec::try_from(&spec).expect("worker spec converts");
    let encoded = serde_json::to_vec(&worker).expect("worker spec serializes");
    let restored: WorkerAnalysisSpec =
        serde_json::from_slice(&encoded).expect("worker spec restores");
    assert_eq!(AnalysisSpec::from(restored), spec);
}

#[test]
fn pss_worker_transport_preserves_every_exact_contract_field() {
    let base = AnalysisSpec::Pss {
        method: PssMethod::Shooting,
        fundamental_freq: 1.0e6,
        tone_sources: vec!["VCLK".to_owned()],
        tstab_periods: 20,
        points_per_period: 512,
        tolerance: 1.0e-7,
        oscillator_mode: false,
        oscillator_node: None,
        num_harmonics: 20,
    };
    let encode = |spec: &AnalysisSpec| {
        let worker = WorkerAnalysisSpec::try_from(spec).expect("PSS worker conversion");
        serde_json::to_vec(&worker).expect("PSS worker serialization")
    };
    let baseline = encode(&base);
    let mut variants = Vec::new();
    macro_rules! changed {
        ($field:ident, $value:expr) => {{
            let mut spec = base.clone();
            let AnalysisSpec::Pss { $field, .. } = &mut spec else {
                unreachable!()
            };
            *$field = $value;
            variants.push(spec);
        }};
    }
    changed!(method, PssMethod::HarmonicBalance);
    changed!(fundamental_freq, 2.0e6);
    changed!(tone_sources, vec!["VLO".to_owned(), "VRF".to_owned()]);
    changed!(tstab_periods, 31);
    changed!(points_per_period, 1024);
    changed!(tolerance, 2.0e-8);
    changed!(oscillator_mode, true);
    changed!(oscillator_node, Some("osc".to_owned()));
    changed!(num_harmonics, 0);

    for variant in variants {
        let encoded = encode(&variant);
        assert_ne!(baseline, encoded, "worker payload aliases {variant:?}");
        let worker: WorkerAnalysisSpec =
            serde_json::from_slice(&encoded).expect("PSS worker restores");
        assert_eq!(AnalysisSpec::from(worker), variant);
    }
}

#[test]
fn legacy_hb_specs_default_the_exact_collocation_grid() {
    let analysis = AnalysisSpec::HarmonicBalance {
        tones: vec![HbToneSpec::new(1.0e6, 3)],
        reltol: 1.0e-6,
        abstol: 1.0e-12,
        max_iterations: 40,
        damping: 0.7,
        oversample: 4,
        collocation_points: Some(7),
        max_mixing_order: 3,
        use_krylov: false,
        gmres_restart: 12,
        source_stepping: false,
        verbose: false,
    };
    let mut analysis_json = serde_json::to_value(&analysis).expect("analysis serializes");
    analysis_json["HarmonicBalance"]
        .as_object_mut()
        .expect("HB payload is an object")
        .remove("collocation_points");
    let decoded: AnalysisSpec =
        serde_json::from_value(analysis_json).expect("legacy analysis spec deserializes");
    assert!(matches!(
        decoded,
        AnalysisSpec::HarmonicBalance {
            collocation_points: None,
            ..
        }
    ));

    let worker = WorkerAnalysisSpec::try_from(&analysis).expect("worker spec converts");
    let mut worker_json = serde_json::to_value(&worker).expect("worker spec serializes");
    worker_json["HarmonicBalance"]
        .as_object_mut()
        .expect("worker HB payload is an object")
        .remove("collocation_points");
    let decoded: WorkerAnalysisSpec =
        serde_json::from_value(worker_json).expect("legacy worker spec deserializes");
    assert!(matches!(
        decoded,
        WorkerAnalysisSpec::HarmonicBalance {
            collocation_points: None,
            ..
        }
    ));
}

#[test]
fn transient_worker_result_round_trips_through_json() {
    let result = WorkerSimulationResult::Transient {
        time: vec![0.0, 1e-9],
        waveforms: vec![WorkerWaveform {
            name: "V(out)".to_string(),
            x_values: vec![0.0, 1e-9],
            y_values: vec![0.0, 1.0],
            y_unit: "V".to_string(),
            is_complex: false,
            y_imag: None,
        }],
        measurements: vec![WorkerMeasurement {
            name: "rise".to_string(),
            value: Some(1e-9),
            error: None,
            passed: true,
            expected: Some(1e-9),
            tolerance: Some(1e-12),
            event_axis: Some(1e-9),
        }],
        events: WorkerEventHistory {
            digital: vec![WorkerDigitalEventTrace {
                node_name: "clk".to_string(),
                points: vec![
                    WorkerDigitalEventPoint {
                        time_s: 0.0,
                        value_code: 0,
                    },
                    WorkerDigitalEventPoint {
                        time_s: 5e-10,
                        value_code: 1,
                    },
                ],
            }],
            real: vec![WorkerRealEventTrace {
                node_name: "level".to_string(),
                points: vec![WorkerRealEventPoint {
                    time_s: 2.5e-10,
                    value: 0.75,
                }],
            }],
        },
    };

    let encoded = serde_json::to_string(&result).expect("result serializes");
    let decoded: WorkerSimulationResult =
        serde_json::from_str(&encoded).expect("result deserializes");

    assert_eq!(decoded, result);
}

/// A worker built before event transport omits the field entirely. It must
/// still decode — reporting no events, which is the truth for that worker —
/// rather than failing the whole result.
#[test]
fn a_transient_result_without_an_events_field_still_decodes() {
    let encoded = r#"{"Transient":{"time":[0.0],"waveforms":[],"measurements":[]}}"#;
    let decoded: WorkerSimulationResult =
        serde_json::from_str(encoded).expect("legacy result deserializes");
    let WorkerSimulationResult::Transient { events, .. } = decoded else {
        panic!("expected a transient result");
    };
    assert_eq!(events, WorkerEventHistory::default());
}

/// The event schedule is the datum. A round trip that quietly dropped it
/// would leave the browser build with no event history at all.
#[test]
fn event_histories_survive_the_worker_edge_in_both_directions() {
    let source = SimulationResult::Transient {
        time: vec![0.0, 1e-9],
        waveforms: HashMap::new(),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: crate::simulation::results::TransientEventHistory {
            digital: vec![crate::simulation::results::EventNodeHistory {
                node_name: "clk".to_owned(),
                points: vec![crate::simulation::results::DigitalEventPoint {
                    time_s: 5e-10,
                    value_code: 1,
                }],
            }],
            real: Vec::new(),
        },
    };
    let wire = WorkerSimulationResult::try_from(source.clone()).expect("transient converts");
    let SimulationResult::Transient { events, .. } = SimulationResult::from(wire) else {
        panic!("expected a transient result");
    };
    let SimulationResult::Transient {
        events: expected, ..
    } = source
    else {
        unreachable!("constructed as a transient result");
    };
    assert_eq!(events, expected);
}

#[test]
fn sensitivity_worker_result_round_trips_output_basis_and_exact_values() {
    let source = SimulationResult::Sensitivity {
        output: "V(out)".to_owned(),
        ac_mode: true,
        frequency_hz: Some(10_000.0),
        sensitivities: HashMap::from([("length".to_owned(), -1.0), ("width".to_owned(), 2.0)]),
        normalized: HashMap::from([("length".to_owned(), -0.25), ("width".to_owned(), 0.5)]),
    };
    let worker = WorkerSimulationResult::try_from(source).expect("worker conversion");
    assert_eq!(worker.estimated_numeric_payload_bytes(), 40);
    let encoded = serde_json::to_vec(&worker).expect("worker result serializes");
    let decoded: WorkerSimulationResult =
        serde_json::from_slice(&encoded).expect("worker result deserializes");
    let restored = SimulationResult::from(decoded);

    let SimulationResult::Sensitivity {
        output,
        ac_mode,
        frequency_hz,
        sensitivities,
        normalized,
    } = restored
    else {
        panic!("sensitivity result")
    };
    assert_eq!(output, "V(out)");
    assert!(ac_mode);
    assert_eq!(frequency_hz, Some(10_000.0));
    assert_eq!(sensitivities["length"], -1.0);
    assert_eq!(sensitivities["width"], 2.0);
    assert_eq!(normalized["length"], -0.25);
    assert_eq!(normalized["width"], 0.5);
}

#[test]
fn monte_carlo_worker_result_round_trips_seed_and_exact_samples_through_json() {
    let result = WorkerSimulationResult::MonteCarlo {
        member_measurements: Vec::new(),
        seed: 0xfedc_ba98_7654_3210,
        runs_requested: 4,
        runs_completed: 4,
        num_failures: 0,
        all_converged: true,
        variables: vec![WorkerMonteCarloVariable {
            name: "V(out)".to_owned(),
            samples: vec![0.91, 0.97, 1.02, 1.08],
            mean: 0.995,
            std_dev: 0.073_711_6,
            min: 0.91,
            max: 1.08,
            histogram: vec![1, 1, 2],
            bin_edges: vec![0.91, 0.95, 1.0, 1.08],
        }],
    };

    let encoded = serde_json::to_string(&result).expect("result serializes");
    let decoded: WorkerSimulationResult =
        serde_json::from_str(&encoded).expect("result deserializes");

    assert_eq!(decoded, result);
}

#[test]
fn worker_result_payload_estimate_counts_high_volume_arrays() {
    let transient = WorkerSimulationResult::Transient {
        time: vec![0.0, 1.0],
        waveforms: vec![WorkerWaveform {
            name: "V(out)".to_string(),
            x_values: vec![0.0, 1.0],
            y_values: vec![0.2, 0.4],
            y_unit: "V".to_string(),
            is_complex: false,
            y_imag: None,
        }],
        measurements: Vec::new(),
        events: WorkerEventHistory::default(),
    };
    assert_eq!(transient.estimated_numeric_payload_bytes(), 48);

    let ac = WorkerSimulationResult::Ac {
        frequencies: vec![1.0, 10.0, 100.0],
        waveforms: vec![WorkerWaveform {
            name: "V(out)".to_string(),
            x_values: vec![1.0, 10.0, 100.0],
            y_values: vec![1.0, 0.5, 0.25],
            y_unit: String::new(),
            is_complex: true,
            y_imag: Some(vec![0.0, -0.1, -0.2]),
        }],
        measurements: Vec::new(),
    };
    assert_eq!(ac.estimated_numeric_payload_bytes(), 96);

    let noise = WorkerSimulationResult::Noise {
        frequencies: vec![1.0, 10.0],
        output_noise: vec![1.0e-18, 2.0e-18],
        input_noise: Some(vec![3.0e-18, 4.0e-18]),
        contributors: HashMap::from([
            ("R1".to_string(), vec![0.5e-18, 1.0e-18]),
            ("M1".to_string(), vec![0.25e-18, 0.5e-18, 1.0e-18]),
        ]),
        summary: None,
        measurements: Vec::new(),
    };
    assert_eq!(noise.estimated_numeric_payload_bytes(), 88);
}

#[test]
fn worker_response_rejects_payloads_that_exceed_transport_limit() {
    let result = SimulationResult::Transient {
        time: vec![0.0, 1.0],
        waveforms: HashMap::from([(
            "V(out)".to_string(),
            WaveformData::new_time_domain("V(out)", vec![0.0, 1.0], vec![0.2, 0.4]),
        )]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: Default::default(),
    };

    let accepted = worker_outcome_from_result(Ok(result.clone()), 48);
    assert!(matches!(accepted, WorkerOutcome::Success(_)));

    let rejected = worker_outcome_from_result(Ok(result), 47);
    match rejected {
        WorkerOutcome::Failure(WorkerSimulationError::InvalidConfig(message)) => {
            assert!(message.contains("browser worker result"));
            assert!(message.contains("48 B"));
            assert!(message.contains("47 B"));
        }
        other => panic!("expected InvalidConfig failure, got {other:?}"),
    }
}

#[test]
fn worker_transfer_response_does_not_apply_legacy_clone_budget() {
    let result = SimulationResult::Transient {
        time: vec![0.0, 1.0],
        waveforms: HashMap::from([(
            "V(out)".to_string(),
            WaveformData::new_time_domain("V(out)", vec![0.0, 1.0], vec![0.2, 0.4]),
        )]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: Default::default(),
    };

    let legacy = worker_outcome_from_result(Ok(result.clone()), 47);
    assert!(matches!(
        legacy,
        WorkerOutcome::Failure(WorkerSimulationError::InvalidConfig(_))
    ));

    let transfer_response = WorkerResponse::from_result_for_transfer(87, Ok(result));
    let WorkerOutcome::Success(_) = &transfer_response.outcome else {
        panic!(
            "transfer path must accept payloads above legacy clone budget, got {:?}",
            transfer_response.outcome
        );
    };

    let transport = WorkerResponseTransport::from_response(transfer_response.clone()).unwrap();
    assert!(!transport.buffers.is_empty());
    assert_eq!(
        transport.into_response().expect("transport reconstructs"),
        transfer_response
    );
}

#[test]
fn worker_transport_extracts_transient_waveform_buffers() {
    let response = WorkerResponse {
        id: 77,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Transient {
            time: vec![0.0, 1.0],
            waveforms: vec![WorkerWaveform {
                name: "V(out)".to_string(),
                x_values: vec![0.0, 1.0],
                y_values: vec![0.2, 0.4],
                y_unit: "V".to_string(),
                is_complex: false,
                y_imag: None,
            }],
            measurements: Vec::new(),
            events: WorkerEventHistory::default(),
        })),
    };

    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();

    assert_eq!(transport.protocol, WORKER_RESPONSE_TRANSPORT_PROTOCOL);
    assert_eq!(
        transport.buffers,
        vec![vec![0.0, 1.0], vec![0.0, 1.0], vec![0.2, 0.4]]
    );
    match &transport.response.outcome {
        WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Transient {
            time,
            waveforms,
            ..
        }) => {
            assert_eq!(time, &WorkerF64Series::Buffer { buffer: 0, len: 2 });
            assert_eq!(
                waveforms[0].x_values,
                WorkerF64Series::Buffer { buffer: 1, len: 2 }
            );
            assert_eq!(
                waveforms[0].y_values,
                WorkerF64Series::Buffer { buffer: 2, len: 2 }
            );
            assert_eq!(waveforms[0].y_imag, None);
        }
        other => panic!("expected transient transport, got {other:?}"),
    }

    assert_eq!(
        transport.into_response().expect("transport reconstructs"),
        response
    );
}

#[test]
fn worker_transport_retains_monte_carlo_seed_and_samples() {
    let response = WorkerResponse {
        id: 91,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 77,
            runs_requested: 3,
            runs_completed: 3,
            num_failures: 0,
            all_converged: true,
            variables: vec![WorkerMonteCarloVariable {
                name: "V(out)".to_owned(),
                samples: vec![0.9, 1.0, 1.1],
                mean: 1.0,
                std_dev: 0.1,
                min: 0.9,
                max: 1.1,
                histogram: vec![1, 1, 1],
                bin_edges: vec![0.9, 0.95, 1.05, 1.1],
            }],
        })),
    };

    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();

    assert_eq!(
        transport.into_response().expect("transport reconstructs"),
        response
    );
}

#[test]
fn worker_transport_round_trips_ac_and_noise_buffers() {
    let ac = WorkerResponse {
        id: 10,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Ac {
            frequencies: vec![1.0, 10.0, 100.0],
            waveforms: vec![WorkerWaveform {
                name: "V(out)".to_string(),
                x_values: vec![1.0, 10.0, 100.0],
                y_values: vec![0.5, 0.25, 0.125],
                y_unit: String::new(),
                is_complex: true,
                y_imag: Some(vec![-0.1, -0.2, -0.3]),
            }],
            measurements: Vec::new(),
        })),
    };
    let ac_transport = WorkerResponseTransport::from_response(ac.clone()).unwrap();
    assert_eq!(ac_transport.buffers.len(), 4);
    assert_eq!(ac_transport.into_response().expect("ac reconstructs"), ac);

    let noise = WorkerResponse {
        id: 11,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Noise {
            frequencies: vec![1.0, 10.0],
            output_noise: vec![1.0e-18, 2.0e-18],
            input_noise: Some(vec![3.0e-18, 4.0e-18]),
            contributors: HashMap::from([
                ("R1".to_string(), vec![0.5e-18, 1.0e-18]),
                ("M1".to_string(), vec![0.25e-18, 0.5e-18]),
            ]),
            summary: None,
            measurements: Vec::new(),
        })),
    };
    let noise_transport = WorkerResponseTransport::from_response(noise.clone()).unwrap();
    assert_eq!(noise_transport.buffers.len(), 5);
    assert_eq!(
        noise_transport.into_response().expect("noise reconstructs"),
        noise
    );
}

#[test]
fn worker_transport_round_trips_hb_display_and_retained_state() {
    let response = WorkerResponse {
        id: 12,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Hb {
            frequencies: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            waveforms: vec![WorkerWaveform {
                name: "V(out)".to_owned(),
                x_values: vec![0.0, 1.0, 2.0, 3.0, 4.0],
                y_values: vec![0.1, 0.4, 0.1, 0.02, 0.01],
                y_unit: String::new(),
                is_complex: true,
                y_imag: Some(vec![0.0, -0.2, 0.04, 0.0, -0.002]),
            }],
            measurements: Vec::new(),
            operating_point: retained_hb_operating_point(),
        })),
    };
    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    assert_eq!(transport.buffers.len(), 6);
    assert_eq!(transport.clone().into_response().unwrap(), response);

    let mut tampered = transport;
    tampered.buffers[5][1] += 1.0;
    assert!(
        tampered
            .into_response()
            .unwrap_err()
            .contains("spectral payload digest mismatch")
    );
}

#[test]
fn worker_transport_rejects_missing_or_mismatched_buffers() {
    let response = WorkerResponse {
        id: 12,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Transient {
            time: vec![0.0, 1.0],
            waveforms: vec![WorkerWaveform {
                name: "V(out)".to_string(),
                x_values: vec![0.0, 1.0],
                y_values: vec![0.2, 0.4],
                y_unit: "V".to_string(),
                is_complex: false,
                y_imag: None,
            }],
            measurements: Vec::new(),
            events: WorkerEventHistory::default(),
        })),
    };

    let mut missing = WorkerResponseTransport::from_response(response.clone()).unwrap();
    missing.buffers.pop();
    let error = missing
        .into_response()
        .expect_err("missing buffer must fail");
    assert!(error.contains("missing transferable buffer 2"));

    let mut mismatched = WorkerResponseTransport::from_response(response).unwrap();
    mismatched.buffers[0].push(2.0);
    let error = mismatched
        .into_response()
        .expect_err("length mismatch must fail");
    assert!(error.contains("length 3"));
    assert!(error.contains("expected 2"));
}

#[test]
fn worker_transport_validates_complex_waveform_shape() {
    let invalid_complex = WorkerResponseTransport {
        protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
        response: WorkerResponseTransportMetadata {
            id: 44,
            outcome: WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Transient {
                time: WorkerF64Series::Buffer { buffer: 0, len: 2 },
                waveforms: vec![WorkerWaveformTransport {
                    name: "V(out)".to_string(),
                    x_values: WorkerF64Series::Buffer { buffer: 1, len: 2 },
                    y_values: WorkerF64Series::Buffer { buffer: 2, len: 2 },
                    y_unit: "V".to_string(),
                    is_complex: true,
                    y_imag: Some(WorkerF64Series::Buffer { buffer: 3, len: 1 }),
                }],
                measurements: Vec::new(),
                events: WorkerEventHistory::default(),
            }),
        },
        buffers: vec![vec![0.0, 1.0], vec![0.0, 1.0], vec![0.2, 0.4], vec![0.1]],
    };
    let error = invalid_complex
        .into_response()
        .expect_err("complex waveform imaginary length mismatch must fail");
    assert!(error.contains("complex waveform"));

    let invalid_real = WorkerResponseTransport {
        protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
        response: WorkerResponseTransportMetadata {
            id: 45,
            outcome: WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Transient {
                time: WorkerF64Series::Buffer { buffer: 0, len: 2 },
                waveforms: vec![WorkerWaveformTransport {
                    name: "V(out)".to_string(),
                    x_values: WorkerF64Series::Buffer { buffer: 1, len: 2 },
                    y_values: WorkerF64Series::Buffer { buffer: 2, len: 2 },
                    y_unit: "V".to_string(),
                    is_complex: false,
                    y_imag: Some(WorkerF64Series::Buffer { buffer: 3, len: 2 }),
                }],
                measurements: Vec::new(),
                events: WorkerEventHistory::default(),
            }),
        },
        buffers: vec![
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            vec![0.2, 0.4],
            vec![0.1, 0.2],
        ],
    };
    let error = invalid_real
        .into_response()
        .expect_err("real waveform imaginary buffer must fail deterministically");
    assert!(error.contains("non-complex waveform"));
}

#[test]
fn worker_response_id_validation_rejects_outer_inner_mismatch() {
    let response = WorkerResponse {
        id: 31,
        outcome: WorkerOutcome::Failure(WorkerSimulationError::Aborted),
    };

    validate_worker_response_id(31, &response).expect("matching ids are valid");
    match validate_worker_response_id(32, &response) {
        Err(SimulationError::InvalidConfig(message)) => {
            assert!(message.contains("outer id 32"));
            assert!(message.contains("response id 31"));
        }
        other => panic!("expected InvalidConfig mismatch, got {other:?}"),
    }
}

#[test]
fn worker_progress_snapshot_applies_to_progress_state() {
    let mut progress = SimulationProgress::new();
    progress.update_status(SimulationStatus::Transient {
        time: 5.0e-7,
        stop_time: 1.0e-6,
    });

    let snapshot = WorkerProgressSnapshot::from_progress(22, &progress);
    let encoded = serde_json::to_string(&snapshot).expect("progress snapshot serializes");
    let decoded: WorkerProgressSnapshot =
        serde_json::from_str(&encoded).expect("progress snapshot deserializes");
    assert_eq!(decoded.id, 22);
    assert_eq!(decoded.progress, Some(0.5));

    let mut applied = SimulationProgress::new();
    decoded.apply_to(&mut applied);

    assert!(matches!(
        applied.status,
        SimulationStatus::Transient {
            time,
            stop_time
        } if (time - 5.0e-7).abs() < 1e-15 && (stop_time - 1.0e-6).abs() < 1e-15
    ));
}

#[test]
fn worker_device_op_entry_unknown_static_labels_are_bounded() {
    let entry: rspice_core::circuit::DeviceOpEntry = WorkerDeviceOpEntry {
        name: "XU1".to_string(),
        device_kind: "third-party-kind".to_string(),
        region: Some("vendor-region".to_string()),
        params: vec![WorkerNamedValue {
            name: "vendor-param".to_string(),
            value: 1.25,
        }],
    }
    .into();

    assert_eq!(entry.device_kind, "unknown");
    assert_eq!(entry.region, Some("unknown"));
    assert_eq!(entry.params, vec![("unknown", 1.25)]);
}

#[test]
fn worker_device_op_report_preserves_core_static_labels() {
    let report = rspice_core::circuit::DeviceOpReport {
        entries: vec![
            rspice_core::circuit::DeviceOpEntry {
                name: "M1".to_string(),
                device_kind: "MOSFET",
                region: Some("saturation"),
                params: vec![
                    ("id", 1.0),
                    ("vgs", 2.0),
                    ("vds", 3.0),
                    ("vbs", 4.0),
                    ("vth", 5.0),
                    ("gm", 6.0),
                    ("gds", 7.0),
                    ("gmb", 8.0),
                ],
            },
            rspice_core::circuit::DeviceOpEntry {
                name: "B3".to_string(),
                device_kind: "BSIM3",
                region: Some("linear"),
                params: vec![("vdsat", 9.0), ("gmbs", 10.0)],
            },
            rspice_core::circuit::DeviceOpEntry {
                name: "B4".to_string(),
                device_kind: "BSIM4",
                region: Some("subthreshold"),
                params: vec![("id", 11.0)],
            },
            rspice_core::circuit::DeviceOpEntry {
                name: "Q1".to_string(),
                device_kind: "BJT",
                region: None,
                params: vec![
                    ("ic", 12.0),
                    ("ib", 13.0),
                    ("vbe", 14.0),
                    ("vce", 15.0),
                    ("beta", 16.0),
                ],
            },
            rspice_core::circuit::DeviceOpEntry {
                name: "D1".to_string(),
                device_kind: "DIODE",
                region: None,
                params: vec![("vd", 17.0), ("gd", 18.0)],
            },
            rspice_core::circuit::DeviceOpEntry {
                name: "J1".to_string(),
                device_kind: "JFET",
                region: None,
                params: vec![("igs", 19.0), ("igd", 20.0)],
            },
            rspice_core::circuit::DeviceOpEntry {
                name: "Z1".to_string(),
                device_kind: "MESFET",
                region: None,
                params: vec![("id", 21.0)],
            },
        ],
    };

    let round_tripped =
        rspice_core::circuit::DeviceOpReport::from(WorkerDeviceOpReport::from(report.clone()));

    assert_eq!(round_tripped.entries.len(), report.entries.len());
    for (actual, expected) in round_tripped.entries.iter().zip(report.entries.iter()) {
        assert_eq!(actual.name, expected.name);
        assert_eq!(actual.device_kind, expected.device_kind);
        assert_eq!(actual.region, expected.region);
        assert_eq!(actual.params, expected.params);
        assert_ne!(actual.device_kind, "unknown");
        assert!(!actual.params.iter().any(|(name, _)| *name == "unknown"));
    }
}

/// The worker boundary interns against the engine's own vocabulary, so a label
/// the engine can emit crosses it unchanged rather than degrading to `unknown`.
#[test]
fn the_worker_boundary_interns_every_label_the_engine_can_emit() {
    for label in rspice_core::circuit::OP_LABELS {
        assert_eq!(
            super::conversions::known_static_label(label.as_str()),
            Some(label.as_str()),
            "{label} would degrade crossing the worker boundary"
        );
    }
}

#[test]
fn analysis_config_round_trips_supported_variants() {
    let configs = vec![
        AnalysisConfig::dc_op(),
        AnalysisConfig::DcSweep(DcSweepConfig {
            source: "V1".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.5,
            source2: Some("V2".to_string()),
            start2: Some(0.0),
            stop2: Some(1.0),
            step2: Some(0.25),
        }),
        AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 1e-10,
            max_timestep: Some(2e-9),
            uic: true,
        }),
        AnalysisConfig::Ac(AcAnalysisConfig {
            sweep_type: AcSweepType::Octave,
            num_points: 12,
            start_freq: 10.0,
            stop_freq: 1e6,
        }),
        AnalysisConfig::Noise(NoiseAnalysisConfig {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "VIN".to_string(),
            sweep_type: AcSweepType::Linear,
            num_points: 3,
            start_freq: 1.0,
            stop_freq: 100.0,
            explicit_frequencies: Some(vec![1.0, 7.0, 100.0]),
            data_table_name: Some("noise_points".to_owned()),
            contribution_detail: NoiseContributionDetail::AllContributors,
            integration_mode: NoiseIntegrationMode::OutputNoiseOnly,
            temperature_kelvin: 398.15,
        }),
        AnalysisConfig::PoleZero(PoleZeroConfig {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "CUR".to_string(),
            analysis_type: PzAnalysisType::PolesOnly,
        }),
        AnalysisConfig::Sensitivity(SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(1e3),
        }),
    ];

    for config in configs {
        let worker = WorkerAnalysisConfig::from(&config);
        let encoded = serde_json::to_string(&worker).expect("worker config serializes");
        let decoded: WorkerAnalysisConfig =
            serde_json::from_str(&encoded).expect("worker config deserializes");
        let reconstructed = AnalysisConfig::from(decoded);

        assert_analysis_configs_match(&reconstructed, &config);
    }
}

#[test]
fn analysis_spec_round_trips_supported_variants() {
    let specs = vec![
        AnalysisSpec::dc_op(),
        AnalysisSpec::DcSweep {
            source_name: "V1".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.1,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        },
        AnalysisSpec::Transient {
            stop_time: 2e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: Some(5e-9),
            uic: false,
        },
        AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
        },
        AnalysisSpec::AcData {
            table_name: "pts".to_string(),
            frequencies: vec![1.0, 2.5, 10.0],
        },
        AnalysisSpec::Noise {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "V1".to_string(),
            start_freq: 10.0,
            stop_freq: 1e6,
            points_per_decade: 8,
            sweep: NoiseSweepType::Decade,
            explicit_frequencies: None,
            data_table_name: None,
            contribution_detail: NoiseContributionDetail::Top20,
            integration_mode: NoiseIntegrationMode::OutputNoiseOnly,
            temperature: 300.0,
        },
        AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: "ZER".to_string(),
        },
        AnalysisSpec::Sensitivity {
            output_var: "I(R1)".to_string(),
            ac_mode: false,
            frequency: None,
        },
        tf_spec(),
        AnalysisSpec::Pac,
        AnalysisSpec::Pxf,
        AnalysisSpec::Pnoise,
        AnalysisSpec::Pstb,
        AnalysisSpec::Parametric,
        AnalysisSpec::Corner,
        AnalysisSpec::MonteCarlo {
            variation_source: Default::default(),
        },
        AnalysisSpec::Stb {
            probe_node: "Vprobe".to_string(),
            start_freq: 1.0,
            stop_freq: 1e6,
            sweep: FrequencySweep::Decade,
            points_per_decade: 12,
            compute_nyquist: false,
        },
        AnalysisSpec::SParameter {
            start_freq: 1.0,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
            z0: 50.0,
            ports: vec![SpPort {
                node_pos: "in".to_string(),
                node_neg: "0".to_string(),
                z0: Some(75.0),
            }],
        },
        AnalysisSpec::Disto {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: FrequencySweep::Octave,
            f2_over_f1: Some(0.8),
        },
        AnalysisSpec::Pss {
            method: PssMethod::HarmonicBalance,
            fundamental_freq: 1e6,
            tone_sources: vec!["VIN".to_owned()],
            tstab_periods: 27,
            points_per_period: 1024,
            tolerance: 1e-6,
            oscillator_mode: false,
            oscillator_node: None,
            num_harmonics: 5,
        },
        AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1e6, 3).with_source("VIN")],
            reltol: 1e-6,
            abstol: 1e-12,
            max_iterations: 40,
            damping: 0.7,
            oversample: 4,
            collocation_points: Some(7),
            max_mixing_order: 3,
            use_krylov: true,
            gmres_restart: 12,
            source_stepping: true,
            verbose: false,
        },
        AnalysisSpec::Envelope {
            fundamental_freq: 1e6,
            additional_carrier_tones: vec![2e6],
            stop_time: 10e-6,
            num_harmonics: 7,
            envelope_step: Some(10e-9),
            modulation_sources: vec!["VIN_AM".to_owned(), "VCTRL".to_owned()],
            initial_periodic_solve: EnvelopeInitialPeriodicSolve::PeriodicSteadyState,
            adaptive_mode: EnvelopeAdaptiveMode::EventAlignedOnly,
            extraction_path: EnvelopeExtractionPath::Projection,
        },
        AnalysisSpec::Fourier {
            fundamental_freq: 1e6,
            num_harmonics: 9,
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            start_time: 1e-6,
            stop_time: 10e-6,
            compute_thd: false,
            normalize: true,
        },
        AnalysisSpec::Reliability {
            target_years: vec![1.0, 10.0],
            enable_hci: true,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 1.2,
        },
        AnalysisSpec::Optimization {
            variables: vec![OptimizationVariable {
                name: "RLOAD".to_string(),
                min: 500.0,
                max: 5_000.0,
                initial: 1_000.0,
            }],
            objective_node: "out".to_string(),
            objective_ref: "0".to_string(),
            goal: OptimizationGoal::Target,
            target: Some(1.2),
            algorithm: OptimizationAlgorithm::PatternSearch,
            max_iterations: 80,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.2,
            min_step: 1e-7,
        },
        AnalysisSpec::Soa {
            stop_time: 10e-6,
            step_time: 10e-9,
            check_vgs_max: true,
            max_vgs: 1.2,
            check_vds_max: true,
            max_vds: 1.8,
            check_vbe_max: true,
            max_vbe: 0.8,
            check_vce_max: true,
            max_vce: 2.0,
        },
    ];

    for spec in specs {
        let worker = WorkerAnalysisSpec::try_from(&spec).expect("spec is supported");
        let encoded = serde_json::to_string(&worker).expect("worker spec serializes");
        let decoded: WorkerAnalysisSpec =
            serde_json::from_str(&encoded).expect("worker spec deserializes");
        let reconstructed = AnalysisSpec::from(decoded);

        assert_eq!(reconstructed, spec);
    }
}

#[test]
fn worker_spec_request_preserves_monte_carlo() {
    let request = SimulationRequest::Spec {
        spec: Box::new(AnalysisSpec::MonteCarlo {
            variation_source: Default::default(),
        }),
        options: Box::new(SpecExecutionOptions::default()),
    };
    let input = NetlistInput {
        netlist: "V1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.mc 10 R1 0.05 gaussian\n.end\n"
            .to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let worker =
        WorkerRequest::from_runner_parts(101, &request, &input).expect("Monte Carlo converts");
    let (round_tripped, _) = worker.into_runner_parts();

    match round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert!(matches!(*spec, AnalysisSpec::MonteCarlo { .. }));
            assert!(options.temp.is_none());
            assert!(options.corner.is_none());
            assert!(options.pac.is_none());
            assert!(options.pxf.is_none());
            assert!(options.pnoise.is_none());
            assert!(options.pstb.is_none());
        }
        other => panic!("expected Monte Carlo spec request, got {other:?}"),
    }
}

#[test]
fn worker_spec_request_preserves_structured_tf_contract() {
    let tf = tf_spec();
    let request = SimulationRequest::Spec {
        spec: Box::new(tf.clone()),
        options: Box::new(SpecExecutionOptions::default()),
    };
    let input = NetlistInput {
        netlist: "Vstim in 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) Vstim\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let worker = WorkerRequest::from_runner_parts(51, &request, &input).expect("request converts");
    let (round_tripped, _) = worker.into_runner_parts();

    match round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert_eq!(*spec, tf);
            assert!(options.temp.is_none());
            assert!(options.corner.is_none());
            assert!(options.pac.is_none());
            assert!(options.pxf.is_none());
            assert!(options.pnoise.is_none());
            assert!(options.pstb.is_none());
        }
        other => panic!("expected spec request, got {other:?}"),
    }
}

#[test]
fn worker_spec_request_preserves_pac_pxf_execution_options() {
    let pac = crate::services::simulation_runner::PacRunConfig {
        pss_fundamental_freq: 2.0e6,
        pss_num_harmonics: 7,
        pss_tolerance: 2.5e-6,
        start_freq: 100.0,
        stop_freq: 10.0e6,
        points_per_unit: 13,
        sweep: crate::services::simulation_runner::PacFrequencySweep::Octave,
        max_sideband: 4,
        input_source: "VRF".to_string(),
        output_node: "mix_out".to_string(),
        output_ref: Some("vref".to_string()),
        pac_magnitude: 0.25,
        include_dc: false,
        reltol: 4.0e-5,
        abstol: 9.0e-13,
    };
    let pxf = crate::services::simulation_runner::PxfRunConfig {
        pss_fundamental_freq: 1.5e6,
        pss_num_harmonics: 9,
        pss_tolerance: 7.0e-7,
        start_freq: 50.0,
        stop_freq: 20.0e6,
        points_per_unit: 11,
        sweep: crate::services::simulation_runner::PxfFrequencySweep::Linear,
        input_source: "VIN".to_string(),
        input_sideband: -1,
        output_node: "if_out".to_string(),
        output_ref: Some("0".to_string()),
        output_sideband: 2,
        max_sideband: 5,
        reltol: 8.0e-5,
        abstol: 2.0e-13,
    };

    let pac_request = SimulationRequest::Spec {
        spec: Box::new(AnalysisSpec::Pac),
        options: Box::new(SpecExecutionOptions {
            pac: Some(pac.clone()),
            ..SpecExecutionOptions::default()
        }),
    };
    let pxf_request = SimulationRequest::Spec {
        spec: Box::new(AnalysisSpec::Pxf),
        options: Box::new(SpecExecutionOptions {
            pxf: Some(pxf.clone()),
            ..SpecExecutionOptions::default()
        }),
    };
    let input = NetlistInput {
        netlist: "V1 in 0 0\nR1 in out 1k\nR2 out 0 1k\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let pac_worker =
        WorkerRequest::from_runner_parts(61, &pac_request, &input).expect("PAC converts");
    let (pac_round_tripped, _) = pac_worker.into_runner_parts();
    match pac_round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert!(matches!(*spec, AnalysisSpec::Pac));
            assert_pac_config_matches(
                &options.pac.expect("PAC options survive worker contract"),
                &pac,
            );
        }
        other => panic!("expected PAC spec request, got {other:?}"),
    }

    let pxf_worker =
        WorkerRequest::from_runner_parts(62, &pxf_request, &input).expect("PXF converts");
    let (pxf_round_tripped, _) = pxf_worker.into_runner_parts();
    match pxf_round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert!(matches!(*spec, AnalysisSpec::Pxf));
            assert_pxf_config_matches(
                &options.pxf.expect("PXF options survive worker contract"),
                &pxf,
            );
        }
        other => panic!("expected PXF spec request, got {other:?}"),
    }
}

#[test]
fn worker_spec_request_preserves_pnoise_pstb_execution_options() {
    let pnoise = crate::services::simulation_runner::PnoiseRunConfig {
        pss_fundamental_freq: 3.0e6,
        pss_num_harmonics: 8,
        pss_tolerance: 6.0e-7,
        start_freq: 10.0,
        stop_freq: 5.0e6,
        points_per_unit: 19,
        sweep: crate::services::simulation_runner::PnoiseFrequencySweep::Linear,
        max_sideband: 6,
        output_node: "vout".to_string(),
        output_ref: Some("vref".to_string()),
        input_source: "VIN".to_string(),
        noise_ref: crate::services::simulation_runner::PnoiseReference::Phase,
        integrated_noise: true,
        noise_summary: false,
        reltol: 3.0e-5,
        abstol: 4.0e-18,
    };
    let pstb = crate::services::simulation_runner::PstbRunConfig {
        pss_fundamental_freq: 4.0e6,
        pss_num_harmonics: 11,
        pss_tolerance: 9.0e-7,
        probe_instance: "LLOOP".to_string(),
        max_harmonics: 12,
        num_multipliers: 6,
        stability_threshold: 1.0002,
        detect_subharmonics: false,
        eigenvalue_tolerance: 5.0e-11,
    };

    let pnoise_request = SimulationRequest::Spec {
        spec: Box::new(AnalysisSpec::Pnoise),
        options: Box::new(SpecExecutionOptions {
            pnoise: Some(pnoise.clone()),
            ..SpecExecutionOptions::default()
        }),
    };
    let pstb_request = SimulationRequest::Spec {
        spec: Box::new(AnalysisSpec::Pstb),
        options: Box::new(SpecExecutionOptions {
            pstb: Some(pstb.clone()),
            ..SpecExecutionOptions::default()
        }),
    };
    let input = NetlistInput {
        netlist: "V1 in 0 0\nR1 in out 1k\nR2 out 0 1k\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let pnoise_worker =
        WorkerRequest::from_runner_parts(71, &pnoise_request, &input).expect("PNOISE converts");
    let (pnoise_round_tripped, _) = pnoise_worker.into_runner_parts();
    match pnoise_round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert!(matches!(*spec, AnalysisSpec::Pnoise));
            assert_pnoise_config_matches(
                &options
                    .pnoise
                    .expect("PNOISE options survive worker contract"),
                &pnoise,
            );
        }
        other => panic!("expected PNOISE spec request, got {other:?}"),
    }

    let pstb_worker =
        WorkerRequest::from_runner_parts(72, &pstb_request, &input).expect("PSTB converts");
    let (pstb_round_tripped, _) = pstb_worker.into_runner_parts();
    match pstb_round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert!(matches!(*spec, AnalysisSpec::Pstb));
            assert_pstb_config_matches(
                &options.pstb.expect("PSTB options survive worker contract"),
                &pstb,
            );
        }
        other => panic!("expected PSTB spec request, got {other:?}"),
    }
}

#[test]
fn worker_spec_request_preserves_parametric_temp_execution_options() {
    let temp = crate::services::simulation_runner::TempRunConfig {
        temperatures_c: vec![-40.0, 25.0, 125.0],
        base_mode: crate::services::simulation_runner::CornerBaseMode::Ac {
            start_freq: 10.0,
            stop_freq: 1.0e6,
            points_per_unit: 21,
            sweep: crate::services::simulation_runner::CornerFrequencySweep::Octave,
        },
    };
    let request = SimulationRequest::Spec {
        spec: Box::new(AnalysisSpec::Parametric),
        options: Box::new(SpecExecutionOptions {
            temp: Some(temp.clone()),
            ..SpecExecutionOptions::default()
        }),
    };
    let input = NetlistInput {
        netlist: "V1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.step temp -40 125 55\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let worker =
        WorkerRequest::from_runner_parts(81, &request, &input).expect("Parametric converts");
    let (round_tripped, _) = worker.into_runner_parts();

    match round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert!(matches!(*spec, AnalysisSpec::Parametric));
            assert_temp_config_matches(
                &options
                    .temp
                    .expect("temperature sweep options survive worker contract"),
                &temp,
            );
        }
        other => panic!("expected Parametric spec request, got {other:?}"),
    }
}

#[test]
fn worker_spec_request_preserves_corner_execution_options() {
    let corner = crate::services::simulation_runner::CornerRunConfig {
        process_corners: vec![
            crate::services::simulation_runner::CornerProcess::SS,
            crate::services::simulation_runner::CornerProcess::FF,
        ],
        voltages: vec![0.9, 1.0, 1.1],
        supply_source_names: vec!["VDD".to_owned()],
        temperatures_c: vec![-40.0, 25.0, 125.0],
        full_matrix: false,
        nominal_voltage: Some(1.0),
        base_mode: crate::services::simulation_runner::CornerBaseMode::Transient {
            stop_time: 5.0e-6,
            step_time: 10.0e-9,
        },
        model_bindings: vec![
            crate::services::simulation_runner::CornerModelBinding {
                process: crate::services::simulation_runner::CornerProcess::SS,
                source_label: "C:/pdk/models.lib [ss]".to_owned(),
                section: Some("ss".to_owned()),
                materialized_model_cards: ".model slow D (IS=1e-13)".to_owned(),
            },
            crate::services::simulation_runner::CornerModelBinding {
                process: crate::services::simulation_runner::CornerProcess::FF,
                source_label: "C:/pdk/models.lib [ff]".to_owned(),
                section: Some("ff".to_owned()),
                materialized_model_cards: ".model fast D (IS=1e-11)".to_owned(),
            },
        ],
        // A filtered run space crosses the boundary as its points. A worker
        // handed only the axes would rebuild the cross product and solve the
        // combinations the declaration removed.
        points: vec![
            crate::services::simulation_runner::CornerPoint {
                process: crate::services::simulation_runner::CornerProcess::SS,
                voltage: 0.9,
                temperature_c: 125.0,
            },
            crate::services::simulation_runner::CornerPoint {
                process: crate::services::simulation_runner::CornerProcess::FF,
                voltage: 1.1,
                temperature_c: -40.0,
            },
        ],
    };
    let request = SimulationRequest::Spec {
        spec: Box::new(AnalysisSpec::Corner),
        options: Box::new(SpecExecutionOptions {
            corner: Some(corner.clone()),
            ..SpecExecutionOptions::default()
        }),
    };
    let input = NetlistInput {
        netlist: "VDD vdd 0 1\nR1 vdd out 1k\nR2 out 0 1k\n.temp 25\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let worker = WorkerRequest::from_runner_parts(91, &request, &input).expect("Corner converts");
    let (round_tripped, _) = worker.into_runner_parts();

    match round_tripped {
        SimulationRequest::Spec { spec, options } => {
            assert!(matches!(*spec, AnalysisSpec::Corner));
            assert_corner_config_matches(
                &options
                    .corner
                    .expect("corner options survive worker contract"),
                &corner,
            );
        }
        other => panic!("expected Corner spec request, got {other:?}"),
    }
}

#[test]
fn worker_request_from_runner_parts_preserves_payload() {
    let request = SimulationRequest::Config(Box::new(AnalysisConfig::dc_op()));
    let input = NetlistInput {
        netlist: "V1 in 0 1\nR1 in 0 1k\n.op\n.end\n".to_string(),
        source_path: Some(std::path::PathBuf::from("deck.cir")),
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: true,
    };

    let worker = WorkerRequest::from_runner_parts(41, &request, &input).expect("request converts");

    assert_eq!(worker.id, 41);
    assert!(matches!(worker.request, WorkerSimulationRequest::Config(_)));
    assert_eq!(worker.netlist, input.netlist);
    assert_eq!(worker.source_path.as_deref(), Some("deck.cir"));
    assert!(worker.stream_transient_samples);
}

#[test]
fn dc_op_worker_result_round_trip_preserves_exact_mna_state_and_contract() {
    let expected_config = nondefault_op_config();
    let result = SimulationResult::DcOp(Box::new(DcOpResult {
        configuration: expected_config.clone(),
        validated_startup_directives: 2,
        mna_node_names: vec!["out".to_owned()],
        mna_branch_names: vec!["V1".to_owned()],
        mna_solution: vec![1.25, -1.0e-3],
        node_voltages: HashMap::from([("out".to_owned(), 1.25)]),
        branch_currents: HashMap::from([("V1".to_owned(), -1.0e-3)]),
        device_report: None,
    }));

    let restored = round_trip_result(result);
    let SimulationResult::DcOp(restored) = restored else {
        panic!("expected OP result");
    };
    assert_eq!(restored.configuration, expected_config);
    assert_eq!(restored.validated_startup_directives, 2);
    assert_eq!(restored.mna_node_names, ["out"]);
    assert_eq!(restored.mna_branch_names, ["V1"]);
    assert_eq!(restored.mna_solution, [1.25, -1.0e-3]);
}

#[test]
fn worker_request_runs_dc_op() {
    let request = WorkerRequest {
        id: 12,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::DcOp(
            crate::simulation::dialog::OpConfig::default(),
        ))),
        netlist: "* worker op\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };

    let response = worker_response_from_request(request);
    assert_eq!(response.id, 12);

    let result = response.into_result().expect("worker response succeeds");
    match result {
        SimulationResult::DcOp(result) => {
            let in_voltage = result
                .node_voltages
                .iter()
                .find_map(|(name, value)| name.eq_ignore_ascii_case("in").then_some(*value))
                .expect("dc op preserves node voltage");
            assert!((in_voltage - 1.0).abs() < 1e-9);
        }
        other => panic!("expected dc op result, got {other:?}"),
    }
}

#[test]
fn worker_request_runs_structured_tf_spec() {
    let request = SimulationRequest::Spec {
        spec: Box::new(tf_spec()),
        options: Box::new(SpecExecutionOptions::default()),
    };
    let input = NetlistInput {
        netlist: "* worker tf\nVstim in 0 DC 0\nR1 in out 1k\nR2 out 0 1k\n.end\n".to_string(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };
    let worker = WorkerRequest::from_runner_parts(13, &request, &input).expect("request converts");
    let encoded_request = serde_json::to_vec(&worker).expect("TF request serializes");
    let worker: WorkerRequest =
        serde_json::from_slice(&encoded_request).expect("TF request deserializes");

    let WorkerSimulationRequest::Spec { spec, options } = &worker.request else {
        panic!("TF request must remain structured")
    };
    assert_eq!(AnalysisSpec::from(spec.as_ref().clone()), tf_spec());
    assert_eq!(options.as_ref(), &WorkerSpecExecutionOptions::default());

    let response = worker_response_from_request(worker);
    assert_eq!(response.id, 13);

    let result = response.into_result().expect("worker TF succeeds");
    match result {
        SimulationResult::TransferFunction {
            input_source,
            output_expression,
            gain,
            ..
        } => {
            assert_eq!(input_source, "Vstim");
            assert!(output_expression.eq_ignore_ascii_case("V(out)"));
            assert!(gain.is_some());
        }
        other => panic!("expected scalar TF result, got {other:?}"),
    }
}

#[test]
fn tf_worker_result_round_trip_keeps_infinite_resistance_json_safe() {
    let result = SimulationResult::TransferFunction {
        input_source: "VIN".to_owned(),
        output_expression: "I(VMEAS)".to_owned(),
        input_quantity: TransferFunctionQuantity::Voltage,
        output_quantity: TransferFunctionQuantity::Current,
        input_unit: "V".to_owned(),
        output_unit: "A".to_owned(),
        normalization: TfNormalization::PerSourceUnit,
        accuracy: TfAccuracy::Accurate,
        gain: Some(TransferFunctionScalar::Finite(-2.5e-3)),
        input_resistance: Some(TransferFunctionScalar::NegativeInfinity),
        output_resistance: Some(TransferFunctionScalar::PositiveInfinity),
        nominal_input: None,
        nominal_output: None,
    };
    let expected = WorkerSimulationResult::try_from(result.clone())
        .expect("TF result converts to worker contract");
    let response = WorkerResponse::from_result_for_transfer(313, Ok(result));
    let transport = WorkerResponseTransport::from_response(response).unwrap();
    assert!(transport.buffers.is_empty(), "scalar TF must stay inline");

    let encoded = serde_json::to_string(&transport.response).expect("TF response serializes");
    assert!(encoded.contains("PositiveInfinity"));
    assert!(encoded.contains("NegativeInfinity"));
    assert!(!encoded.contains(":Infinity"));
    assert!(!encoded.contains(":-Infinity"));

    let response: WorkerResponseTransportMetadata =
        serde_json::from_str(&encoded).expect("TF response deserializes");
    let restored = WorkerResponseTransport {
        protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
        response,
        buffers: transport.buffers,
    }
    .into_response()
    .expect("TF transport reconstructs")
    .into_result()
    .expect("TF result reconstructs");
    let restored = WorkerSimulationResult::try_from(restored)
        .expect("restored TF result converts to worker contract");
    assert_eq!(restored, expected);
}

fn round_trip_result(result: SimulationResult) -> SimulationResult {
    let worker = WorkerSimulationResult::try_from(result).expect("result is supported");
    SimulationResult::from(worker)
}

fn assert_analysis_configs_match(actual: &AnalysisConfig, expected: &AnalysisConfig) {
    match (actual, expected) {
        (AnalysisConfig::DcOp(actual), AnalysisConfig::DcOp(expected)) => {
            assert_eq!(actual, expected);
        }
        (AnalysisConfig::DcSweep(actual), AnalysisConfig::DcSweep(expected)) => {
            assert_eq!(actual.source, expected.source);
            assert_eq!(actual.start, expected.start);
            assert_eq!(actual.stop, expected.stop);
            assert_eq!(actual.step, expected.step);
            assert_eq!(actual.source2, expected.source2);
            assert_eq!(actual.start2, expected.start2);
            assert_eq!(actual.stop2, expected.stop2);
            assert_eq!(actual.step2, expected.step2);
        }
        (AnalysisConfig::Transient(actual), AnalysisConfig::Transient(expected)) => {
            assert_eq!(actual.stop_time, expected.stop_time);
            assert_eq!(actual.step_time, expected.step_time);
            assert_eq!(actual.start_time, expected.start_time);
            assert_eq!(actual.max_timestep, expected.max_timestep);
            assert_eq!(actual.uic, expected.uic);
        }
        (AnalysisConfig::Ac(actual), AnalysisConfig::Ac(expected)) => {
            assert_eq!(actual.sweep_type, expected.sweep_type);
            assert_eq!(actual.num_points, expected.num_points);
            assert_eq!(actual.start_freq, expected.start_freq);
            assert_eq!(actual.stop_freq, expected.stop_freq);
        }
        (AnalysisConfig::Noise(actual), AnalysisConfig::Noise(expected)) => {
            assert_eq!(actual.output_node, expected.output_node);
            assert_eq!(actual.reference_node, expected.reference_node);
            assert_eq!(actual.input_source, expected.input_source);
            assert_eq!(actual.sweep_type, expected.sweep_type);
            assert_eq!(actual.num_points, expected.num_points);
            assert_eq!(actual.start_freq, expected.start_freq);
            assert_eq!(actual.stop_freq, expected.stop_freq);
            assert_eq!(actual.explicit_frequencies, expected.explicit_frequencies);
            assert_eq!(actual.data_table_name, expected.data_table_name);
            assert_eq!(actual.contribution_detail, expected.contribution_detail);
            assert_eq!(actual.integration_mode, expected.integration_mode);
            assert_eq!(actual.temperature_kelvin, expected.temperature_kelvin);
        }
        (AnalysisConfig::PoleZero(actual), AnalysisConfig::PoleZero(expected)) => {
            assert_eq!(actual.input_node, expected.input_node);
            assert_eq!(actual.input_ref, expected.input_ref);
            assert_eq!(actual.output_node, expected.output_node);
            assert_eq!(actual.output_ref, expected.output_ref);
            assert_eq!(actual.transfer_type, expected.transfer_type);
            assert_eq!(actual.analysis_type, expected.analysis_type);
        }
        (AnalysisConfig::Sensitivity(actual), AnalysisConfig::Sensitivity(expected)) => {
            assert_eq!(actual.output_var, expected.output_var);
            assert_eq!(actual.ac_mode, expected.ac_mode);
            assert_eq!(actual.frequency, expected.frequency);
        }
        (actual, expected) => {
            panic!("config mismatch: actual={actual:?}, expected={expected:?}")
        }
    }
}

fn assert_pac_config_matches(
    actual: &crate::services::simulation_runner::PacRunConfig,
    expected: &crate::services::simulation_runner::PacRunConfig,
) {
    assert_eq!(actual.pss_fundamental_freq, expected.pss_fundamental_freq);
    assert_eq!(actual.pss_num_harmonics, expected.pss_num_harmonics);
    assert_eq!(actual.pss_tolerance, expected.pss_tolerance);
    assert_eq!(actual.start_freq, expected.start_freq);
    assert_eq!(actual.stop_freq, expected.stop_freq);
    assert_eq!(actual.points_per_unit, expected.points_per_unit);
    assert_eq!(actual.sweep, expected.sweep);
    assert_eq!(actual.max_sideband, expected.max_sideband);
    assert_eq!(actual.input_source, expected.input_source);
    assert_eq!(actual.output_node, expected.output_node);
    assert_eq!(actual.output_ref, expected.output_ref);
    assert_eq!(actual.pac_magnitude, expected.pac_magnitude);
    assert_eq!(actual.include_dc, expected.include_dc);
    assert_eq!(actual.reltol, expected.reltol);
    assert_eq!(actual.abstol, expected.abstol);
}

fn assert_pxf_config_matches(
    actual: &crate::services::simulation_runner::PxfRunConfig,
    expected: &crate::services::simulation_runner::PxfRunConfig,
) {
    assert_eq!(actual.pss_fundamental_freq, expected.pss_fundamental_freq);
    assert_eq!(actual.pss_num_harmonics, expected.pss_num_harmonics);
    assert_eq!(actual.pss_tolerance, expected.pss_tolerance);
    assert_eq!(actual.start_freq, expected.start_freq);
    assert_eq!(actual.stop_freq, expected.stop_freq);
    assert_eq!(actual.points_per_unit, expected.points_per_unit);
    assert_eq!(actual.sweep, expected.sweep);
    assert_eq!(actual.input_source, expected.input_source);
    assert_eq!(actual.input_sideband, expected.input_sideband);
    assert_eq!(actual.output_node, expected.output_node);
    assert_eq!(actual.output_ref, expected.output_ref);
    assert_eq!(actual.output_sideband, expected.output_sideband);
    assert_eq!(actual.max_sideband, expected.max_sideband);
    assert_eq!(actual.reltol, expected.reltol);
    assert_eq!(actual.abstol, expected.abstol);
}

fn assert_pnoise_config_matches(
    actual: &crate::services::simulation_runner::PnoiseRunConfig,
    expected: &crate::services::simulation_runner::PnoiseRunConfig,
) {
    assert_eq!(actual.pss_fundamental_freq, expected.pss_fundamental_freq);
    assert_eq!(actual.pss_num_harmonics, expected.pss_num_harmonics);
    assert_eq!(actual.pss_tolerance, expected.pss_tolerance);
    assert_eq!(actual.start_freq, expected.start_freq);
    assert_eq!(actual.stop_freq, expected.stop_freq);
    assert_eq!(actual.points_per_unit, expected.points_per_unit);
    assert_eq!(actual.sweep, expected.sweep);
    assert_eq!(actual.max_sideband, expected.max_sideband);
    assert_eq!(actual.output_node, expected.output_node);
    assert_eq!(actual.output_ref, expected.output_ref);
    assert_eq!(actual.input_source, expected.input_source);
    assert_eq!(actual.noise_ref, expected.noise_ref);
    assert_eq!(actual.integrated_noise, expected.integrated_noise);
    assert_eq!(actual.noise_summary, expected.noise_summary);
    assert_eq!(actual.reltol, expected.reltol);
    assert_eq!(actual.abstol, expected.abstol);
}

fn assert_pstb_config_matches(
    actual: &crate::services::simulation_runner::PstbRunConfig,
    expected: &crate::services::simulation_runner::PstbRunConfig,
) {
    assert_eq!(actual.pss_fundamental_freq, expected.pss_fundamental_freq);
    assert_eq!(actual.pss_num_harmonics, expected.pss_num_harmonics);
    assert_eq!(actual.pss_tolerance, expected.pss_tolerance);
    assert_eq!(actual.probe_instance, expected.probe_instance);
    assert_eq!(actual.max_harmonics, expected.max_harmonics);
    assert_eq!(actual.num_multipliers, expected.num_multipliers);
    assert_eq!(actual.stability_threshold, expected.stability_threshold);
    assert_eq!(actual.detect_subharmonics, expected.detect_subharmonics);
    assert_eq!(actual.eigenvalue_tolerance, expected.eigenvalue_tolerance);
}

fn assert_temp_config_matches(
    actual: &crate::services::simulation_runner::TempRunConfig,
    expected: &crate::services::simulation_runner::TempRunConfig,
) {
    assert_eq!(actual.temperatures_c, expected.temperatures_c);
    assert_corner_base_mode_matches(&actual.base_mode, &expected.base_mode);
}

fn assert_corner_config_matches(
    actual: &crate::services::simulation_runner::CornerRunConfig,
    expected: &crate::services::simulation_runner::CornerRunConfig,
) {
    assert_eq!(actual.process_corners, expected.process_corners);
    assert_eq!(actual.voltages, expected.voltages);
    assert_eq!(actual.temperatures_c, expected.temperatures_c);
    assert_eq!(actual.full_matrix, expected.full_matrix);
    assert_eq!(actual.nominal_voltage, expected.nominal_voltage);
    assert_eq!(actual.model_bindings, expected.model_bindings);
    assert_eq!(actual.points, expected.points);
    assert_corner_base_mode_matches(&actual.base_mode, &expected.base_mode);
}

fn assert_corner_base_mode_matches(
    actual: &crate::services::simulation_runner::CornerBaseMode,
    expected: &crate::services::simulation_runner::CornerBaseMode,
) {
    use crate::services::simulation_runner::CornerBaseMode;

    match (actual, expected) {
        (CornerBaseMode::Op, CornerBaseMode::Op) => {}
        (
            CornerBaseMode::DcSweep {
                source_name: actual_source,
                start: actual_start,
                stop: actual_stop,
                step: actual_step,
            },
            CornerBaseMode::DcSweep {
                source_name: expected_source,
                start: expected_start,
                stop: expected_stop,
                step: expected_step,
            },
        ) => {
            assert_eq!(actual_source, expected_source);
            assert_eq!(actual_start, expected_start);
            assert_eq!(actual_stop, expected_stop);
            assert_eq!(actual_step, expected_step);
        }
        (
            CornerBaseMode::Transient {
                stop_time: actual_stop,
                step_time: actual_step,
            },
            CornerBaseMode::Transient {
                stop_time: expected_stop,
                step_time: expected_step,
            },
        ) => {
            assert_eq!(actual_stop, expected_stop);
            assert_eq!(actual_step, expected_step);
        }
        (
            CornerBaseMode::Ac {
                start_freq: actual_start,
                stop_freq: actual_stop,
                points_per_unit: actual_points,
                sweep: actual_sweep,
            },
            CornerBaseMode::Ac {
                start_freq: expected_start,
                stop_freq: expected_stop,
                points_per_unit: expected_points,
                sweep: expected_sweep,
            },
        ) => {
            assert_eq!(actual_start, expected_start);
            assert_eq!(actual_stop, expected_stop);
            assert_eq!(actual_points, expected_points);
            assert_eq!(actual_sweep, expected_sweep);
        }
        (actual, expected) => {
            panic!("base mode mismatch: actual={actual:?}, expected={expected:?}")
        }
    }
}
