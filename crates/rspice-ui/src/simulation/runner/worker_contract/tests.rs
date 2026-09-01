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
fn pole_zero_worker_result_accepts_numeric_and_missing_gain_with_explicit_evidence() {
    let legacy: WorkerSimulationResult = serde_json::from_str(
        r#"{"PoleZero":{"poles":[[-1.0,2.0]],"zeros":[[-3.0,0.0]],"pole_evidence":{"status":"legacy_unknown"},"zero_evidence":{"status":"legacy_unknown"},"gain":4.25}}"#,
    )
    .expect("legacy numeric pole-zero gain deserializes");
    assert!(matches!(
        legacy,
        WorkerSimulationResult::PoleZero {
            gain: Some(4.25),
            pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            ..
        }
    ));

    let missing: WorkerSimulationResult =
        serde_json::from_str(r#"{"PoleZero":{"poles":[[-1.0,2.0]],"zeros":[[-3.0,0.0]],"pole_evidence":{"status":"legacy_unknown"},"zero_evidence":{"status":"legacy_unknown"}}}"#)
            .expect("missing pole-zero gain deserializes as unavailable");
    assert!(matches!(
        missing,
        WorkerSimulationResult::PoleZero {
            gain: None,
            pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            ..
        }
    ));

    assert!(
        serde_json::from_str::<WorkerSimulationResult>(
            r#"{"PoleZero":{"poles":[],"zeros":[],"gain":1.0}}"#
        )
        .is_err(),
        "current worker results must never invent missing root evidence"
    );
}

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
    let certificate = rspice_core::analysis::FloquetSpectrumCertificate::new(
        1,
        0.0,
        rspice_core::analysis::FloquetSpectrumCertificate::canonical_qualification_tolerance(1),
    )
    .unwrap();
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
        floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate },
        floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind::Driven,
        trivial_floquet_multiplier_index: None,
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

pub(super) fn authenticated_pstb_result() -> SimulationResult {
    let period = 2.0;
    let first = num_complex::Complex64::new(0.5, 0.0);
    let second = num_complex::Complex64::new(0.25, 0.0);
    let certificate = rspice_core::analysis::FloquetSpectrumCertificate::new(
        2,
        0.0,
        rspice_core::analysis::FloquetSpectrumCertificate::canonical_qualification_tolerance(2),
    )
    .unwrap();
    let modes = vec![
        crate::simulation::results::PstbFloquetMode {
            multiplier: (first.re, first.im),
            exponent: (first.ln().re / period, first.ln().im / period),
            probe_participation: 0.25,
            is_unstable: false,
            is_trivial: false,
            subharmonic_order: None,
        },
        crate::simulation::results::PstbFloquetMode {
            multiplier: (second.re, second.im),
            exponent: (second.ln().re / period, second.ln().im / period),
            probe_participation: 0.75,
            is_unstable: false,
            is_trivial: false,
            subharmonic_order: None,
        },
    ];
    let mode_indices = vec![1.0];
    let waveform = |name: &str, unit: &str, value: f64| {
        (
            name.to_owned(),
            WaveformData {
                name: name.to_owned(),
                x_values: mode_indices.clone(),
                y_values: vec![value],
                y_unit: unit.to_owned(),
                is_complex: false,
                y_imag: None,
            },
        )
    };
    let waveforms = HashMap::from([
        waveform("Floquet |lambda|", "", first.norm()),
        waveform(
            "Floquet Phase (deg)",
            "deg",
            first.arg() * 180.0 / std::f64::consts::PI,
        ),
        waveform("Stability Margin (dB)", "dB", -20.0 * first.norm().log10()),
        waveform("Mode Damping (1/s)", "1/s", -first.ln().re / period),
        waveform(
            "Mode Frequency (Hz)",
            "Hz",
            first.ln().im.abs() / period / (2.0 * std::f64::consts::PI),
        ),
        waveform("Probe Mode Participation", "", 0.25),
    ]);
    SimulationResult::Pstb {
        period,
        fundamental_frequency: 1.0 / period,
        stability_threshold: 1.0 + 1.0e-6,
        probe_instance: "LPROBE".to_owned(),
        detect_subharmonics: true,
        modes,
        floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate },
        orbit_kind: rspice_core::analysis::FloquetOrbitKind::Driven,
        trivial_multiplier_index: None,
        stability_verdict: rspice_core::analysis::FloquetStabilityVerdict::Stable,
        stability_classification: rspice_core::analysis::pstb::StabilityType::Stable,
        min_stability_margin_db: Some(-20.0 * first.norm().log10()),
        max_multiplier_magnitude: first.norm(),
        num_unstable: 0,
        subharmonics: Vec::new(),
        converged: true,
        iterations: 0,
        mode_indices,
        waveforms,
    }
}

pub(super) fn retained_hb_operating_point() -> rspice_core::engine::HbOperatingPoint {
    let config = rspice_core::analysis::HbConfig::new(1.0).with_harmonics(4);
    rspice_core::engine::HbOperatingPoint::try_from_parts_with_mna_branches(
        config,
        vec!["out".to_owned()],
        vec![vec![
            num_complex::Complex64::new(0.1, 0.0),
            num_complex::Complex64::new(0.2, -0.1),
            num_complex::Complex64::new(0.05, 0.02),
            num_complex::Complex64::new(0.01, 0.0),
            num_complex::Complex64::new(0.005, -0.001),
        ]],
        vec!["V1".to_owned()],
        vec![vec![
            num_complex::Complex64::new(-1.0e-3, 0.0),
            num_complex::Complex64::new(-2.0e-4, 1.0e-4),
            num_complex::Complex64::new(-5.0e-5, 2.0e-5),
            num_complex::Complex64::new(-1.0e-5, 0.0),
            num_complex::Complex64::new(-5.0e-6, -1.0e-6),
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
    assert_eq!(WORKER_RESPONSE_TRANSPORT_PROTOCOL, 13);
    assert_eq!(WORKER_REQUEST_TRANSPORT_PROTOCOL, 8);
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
    assert!(source.contains("rspice_ui_wasm_jit_eval_op_slice_v1"));
    assert!(source.contains("eval_op_slice_v1: wasmExports.rspice_ui_wasm_jit_eval_op_slice_v1"));
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
            raw_value: Some(1e-9),
            error: None,
            passed: true,
            expected: Some(1e-9),
            tolerance: Some(1e-12),
            failure_limit: Some(2e-9),
            failure_limit_exceeded: false,
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

fn projected_worker_measurement() -> WorkerMeasurement {
    WorkerMeasurement {
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

fn response_with_measurement(measurement: WorkerMeasurement) -> WorkerResponse {
    WorkerResponse {
        id: 901,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Transient {
            time: vec![0.0],
            waveforms: Vec::new(),
            measurements: vec![measurement],
            events: WorkerEventHistory::default(),
        })),
    }
}

fn transported_measurement_mut(transport: &mut WorkerResponseTransport) -> &mut WorkerMeasurement {
    let WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Transient {
        measurements,
        ..
    }) = &mut transport.response.outcome
    else {
        panic!("fixture must retain a transient measurement")
    };
    &mut measurements[0]
}

#[test]
fn protocol_v13_preserves_distinct_projected_and_raw_measurement_values() {
    let response = response_with_measurement(projected_worker_measurement());
    let transport = WorkerResponseTransport::from_response(response).expect("egress validates");
    let restored = transport.into_response().expect("ingress validates");
    let WorkerOutcome::Success(result) = restored.outcome else {
        panic!("fixture must succeed")
    };
    let WorkerSimulationResult::Transient { measurements, .. } = *result else {
        panic!("fixture must remain transient")
    };
    assert_eq!(measurements[0].value, Some(20.0));
    assert_eq!(measurements[0].raw_value, Some(3.0));

    let mut missing_raw = projected_worker_measurement();
    missing_raw.raw_value = None;
    let core = rspice_core::MeasureResult::from(missing_raw);
    assert_eq!(
        core.raw_value, None,
        "the current conversion must never synthesize raw evidence from a projected value"
    );
}

#[test]
fn protocol_v13_rejects_missing_or_inconsistent_failvalue_evidence_both_ways() {
    let mut unevaluated = projected_worker_measurement();
    unevaluated.value = None;
    unevaluated.raw_value = None;
    unevaluated.event_axis = None;
    unevaluated.passed = false;
    unevaluated.error = Some("signal was unavailable".to_owned());
    WorkerResponseTransport::from_response(response_with_measurement(unevaluated))
        .expect("an unevaluated failure retains its authored limit without inventing raw evidence");

    let mut missing_raw = projected_worker_measurement();
    missing_raw.raw_value = None;
    assert!(
        WorkerResponseTransport::from_response(response_with_measurement(missing_raw)).is_err(),
        "worker egress requires raw and published values together"
    );

    let mut false_positive = projected_worker_measurement();
    false_positive.failure_limit_exceeded = true;
    false_positive.passed = false;
    assert!(
        WorkerResponseTransport::from_response(response_with_measurement(false_positive)).is_err(),
        "the retained verdict must be recomputed from the raw value"
    );

    let mut passed_after_exceeded = projected_worker_measurement();
    passed_after_exceeded.raw_value = Some(-4.0);
    passed_after_exceeded.failure_limit_exceeded = true;
    assert!(
        WorkerResponseTransport::from_response(response_with_measurement(passed_after_exceeded))
            .is_err(),
        "a measurement cannot pass after reaching its inclusive FAILVALUE limit"
    );

    let mut transport = WorkerResponseTransport::from_response(response_with_measurement(
        projected_worker_measurement(),
    ))
    .expect("valid fixture transports");
    transported_measurement_mut(&mut transport).raw_value = None;
    assert!(
        transport.into_response().is_err(),
        "worker ingress applies the same evidence validation after reconstruction"
    );

    let mut transport = WorkerResponseTransport::from_response(response_with_measurement(
        projected_worker_measurement(),
    ))
    .expect("valid fixture transports");
    let measurement = transported_measurement_mut(&mut transport);
    measurement.raw_value = Some(5.0);
    assert!(
        transport.into_response().is_err(),
        "worker ingress rejects a false-negative FAILVALUE verdict"
    );
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
    assert_eq!(transport.buffers.len(), 8);
    assert_eq!(transport.clone().into_response().unwrap(), response);

    let mut tampered = transport;
    tampered.buffers[7][1] += 1.0;
    assert!(
        tampered
            .into_response()
            .unwrap_err()
            .contains("MNA branch spectral payload digest mismatch")
    );
}

#[test]
fn worker_transport_preserves_authenticated_hb_identity_and_rejects_identity_tamper() {
    let netlist = rspice_core::netlist::Netlist::parse(
        "authenticated retained HB worker fixture\n\
         V1 out 0 DC 1\n\
         R1 out 0 1k\n\
         .end\n",
    )
    .expect("worker identity fixture parses");
    let produced = rspice_core::engine::Engine::default()
        .run_hb(
            &netlist,
            rspice_core::analysis::HbConfig::new(1.0e6).with_harmonics(4),
        )
        .expect("worker identity fixture solves")
        .operating_point;
    assert!(produced.producer_identity().is_some());

    let mut buffers = Vec::new();
    let transport =
        WorkerHbOperatingPointTransport::from_operating_point(produced.clone(), &mut buffers);
    let restored = transport
        .clone()
        .into_operating_point(&buffers)
        .expect("authenticated worker state round-trips");
    assert_eq!(restored, produced);
    assert_eq!(restored.producer_identity(), produced.producer_identity());

    let mut tampered = serde_json::to_value(transport).expect("transport serializes");
    tampered["producer_identity"]["retained_state_identity"] =
        serde_json::Value::String("0".repeat(64));
    let tampered: WorkerHbOperatingPointTransport =
        serde_json::from_value(tampered).expect("structurally valid identity parses");
    let error = tampered
        .into_operating_point(&buffers)
        .expect_err("tampered authenticated state identity must fail closed");
    assert!(
        error.contains("numerical payload does not match"),
        "worker rejection must identify the retained payload authentication failure: {error}"
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

mod extended_contract;

use extended_contract::{assert_analysis_configs_match, round_trip_result};
