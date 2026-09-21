//! Round-trip and rejection tests for the worker protocol.
//!
//! Most cases here assert a refusal: a payload over the ingress limit, a
//! buffer whose length contradicts the metadata claiming it, or a response
//! whose id does not match the request. The round-trip cases pin the other
//! half — that a result which does survive transport is bit-identical to the
//! one that was sent.

use super::*;

mod dc_sweep;
mod hb_current;
mod result_round_trip;

#[test]
fn studio_measurement_reference_worker_validates_and_executes_captured_data() {
    use crate::simulation::measurement_references::PreparedMeasurementReferences;
    use crate::state::{SpecEntry, SpecificationDefinition};
    let source = "Captured worker reference\nV1 out 0 2.5\nR1 out 0 1k\n.MEAS TRAN fit ERROR V(out) FILE=worker-reference.csv COMP_FUNCTION=INFNORM INDEPVARCOL=0 DEPVARCOL=1\n.end\n";
    let mut definition = SpecificationDefinition::new_from_projection(&SpecEntry {
        measurement: "fit".into(),
        expression: String::new(),
        min: None,
        max: None,
        unit: String::new(),
        scope: crate::state::SpecPointScope::AllPoints,
    });
    definition.measurement_reference = Some(crate::state::workspace::MeasurementReferenceSource {
        logical_path: "worker-reference.csv".into(),
        contents: "TIME,V(out)\n0,2\n0.000001,2\n".into(),
    });
    let references = PreparedMeasurementReferences::capture(source, &[definition.clone()]).unwrap();
    let request = WorkerRequest {
        id: 97,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::from(
            &AnalysisConfig::Transient(TransientAnalysisConfig {
                stop_time: 1e-6,
                step_time: 1e-7,
                max_timestep: Some(1e-7),
                ..Default::default()
            }),
        ))),
        netlist: source.into(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        measurement_references: references,
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };
    let mut transport = WorkerRequestTransport::from_request(request.clone()).unwrap();
    transport.request =
        serde_json::from_str(&serde_json::to_string(&transport.request).unwrap()).unwrap();
    let mut tampered = transport.clone();
    let mut json = serde_json::to_value(&tampered.request).unwrap();
    json["request"]["measurement_references"]["entries"][0]["source"]["contents"] =
        serde_json::json!("TIME,V(out)\n0,99\n0.000001,99\n");
    tampered.request = serde_json::from_value(json).unwrap();
    assert!(
        tampered
            .into_request()
            .unwrap_err()
            .contains("digest mismatch")
    );
    let mut missing = transport.clone();
    missing.request.request.measurement_references = Default::default();
    assert!(missing.into_request().unwrap_err().contains("unsealed"));
    let mut wrong_column = transport.clone();
    wrong_column.request.request.netlist = source.replace("DEPVARCOL=1", "DEPVARCOL=7");
    assert!(wrong_column.into_request().is_err());
    let restored = transport.into_request().unwrap();
    assert_eq!(restored, request);
    let (request, input) = restored.into_runner_parts();
    let result = crate::simulation::runner::run_simulation_thread_with_progress_observer(
        request,
        input,
        Arc::new(Mutex::new(SimulationProgress::default())),
        Arc::new(AtomicBool::new(false)),
        Default::default(),
    )
    .unwrap();
    assert!((result.measurement("fit").unwrap() - 0.5).abs() < 1e-8);
    definition.measurement_reference.as_mut().unwrap().contents = "TIME,V(out)\n1,1\n0,2\n".into();
    assert!(PreparedMeasurementReferences::capture(source, &[definition]).is_err());
}

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

#[test]
fn output_failures_round_trip_through_worker_contract() {
    for expected in [
        SimulationError::RequestedSignalUnavailable {
            signal: "@Mdriver[Id]".to_string(),
            analysis: "DC".to_string(),
            coordinate: Some("v1 = 1".to_string()),
        },
        SimulationError::ResultSchemaMismatch(Box::new(ResultSchemaMismatch {
            analysis: "TRAN".to_string(),
            coordinate: None,
            signal_family: "node voltages".to_string(),
            expected_names: vec!["V(in)".to_string(), "V(out)".to_string()],
            actual_names: vec!["V(in)".to_string()],
            expected_value_count: 2,
            actual_value_count: 1,
        })),
    ] {
        let worker = WorkerSimulationError::from(expected.clone());
        let encoded = serde_json::to_string(&worker).expect("worker error serializes");
        let decoded: WorkerSimulationError =
            serde_json::from_str(&encoded).expect("worker error deserializes");

        assert_eq!(SimulationError::from(decoded), expected);
    }
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
        branch_names: Vec::new(),
        branch_waveforms: Vec::new(),
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
    assert_eq!(WORKER_RESPONSE_TRANSPORT_PROTOCOL, 29);
    assert_eq!(WORKER_REQUEST_TRANSPORT_PROTOCOL, 34);
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
use crate::simulation::dialog::IntegrationMethod;
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
        measurement_references: Default::default(),
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

/// A transient-noise request written before the noise floor existed still
/// decodes, and decodes as the engine derivation it was running under.
///
/// The transient-noise variant crosses the boundary as `CanonicalSpec`, so the
/// domain enum's own serde attributes are the wire format — there is no second
/// mirror to keep in step, and a field added without `serde(default)` would
/// refuse every request an older worker had already sent. `None` rather than a
/// number is the whole point: the run this request described asked the engine
/// for `1/tstop`, and decoding it as any particular frequency would change
/// what it asked for.
#[test]
fn a_wire_request_written_before_the_noise_floor_field_restores_without_one() {
    let fields = serde_json::json!({
        "stop_time": 1.0e-6,
        "step_time": 1.0e-9,
        "start_time": 0.0,
        "max_timestep": 1.0e-9,
        "seed": 97,
        "noise_fmax": 5.0e8,
        "scale": 1.0,
        "uic": false
    });
    let analysis: AnalysisSpec = serde_json::from_value(serde_json::json!({
        "TransientNoise": fields.clone()
    }))
    .expect("a transient-noise spec written before the floor deserializes");
    let worker: WorkerAnalysisSpec = serde_json::from_value(serde_json::json!({
        "CanonicalSpec": { "TransientNoise": fields }
    }))
    .expect("the same request deserializes as a worker payload");

    let transient_noise = |noise_fmin| AnalysisSpec::TransientNoise {
        stop_time: 1.0e-6,
        step_time: 1.0e-9,
        start_time: 0.0,
        max_timestep: 1.0e-9,
        seed: 97,
        noise_fmax: 5.0e8,
        noise_fmin,
        scale: 1.0,
        uic: false,
    };
    let expected = transient_noise(None);
    assert_eq!(analysis, expected);
    assert_eq!(AnalysisSpec::from(worker), expected);

    // And an authored floor survives the same crossing, so the default above
    // is a default rather than the only value this field can hold.
    let authored = transient_noise(Some(1.0e3));
    let carried = WorkerAnalysisSpec::try_from(&authored).expect("the request is transportable");
    let encoded = serde_json::to_string(&carried).expect("the request serializes");
    let decoded: WorkerAnalysisSpec =
        serde_json::from_str(&encoded).expect("the request deserializes");
    assert_eq!(AnalysisSpec::from(decoded), authored);
}

/// A DC mismatch request written before the share threshold existed still
/// decodes, and decodes as the untrimmed list it was running.
///
/// DC mismatch also crosses as `CanonicalSpec`, so the domain enum's serde
/// attributes *are* the wire format. `None` rather than a number is the point:
/// the run this request described listed every contributor its limit allowed,
/// and decoding it as any particular share would drop rows the request asked
/// to see.
#[test]
fn a_wire_request_written_before_the_share_threshold_field_restores_without_one() {
    let fields = serde_json::json!({
        "output_expression": "V(out)",
        "sigma_multiplier": 1.0,
        "contributor_limit": 10,
        "include_process": false,
        "include_mismatch": true,
        "normalized_contributions": true
    });
    let analysis: AnalysisSpec = serde_json::from_value(serde_json::json!({
        "DcMismatch": fields.clone()
    }))
    .expect("a DC mismatch spec written before the threshold deserializes");
    let worker: WorkerAnalysisSpec = serde_json::from_value(serde_json::json!({
        "CanonicalSpec": { "DcMismatch": fields }
    }))
    .expect("the same request deserializes as a worker payload");

    let dc_mismatch = |contribution_threshold| AnalysisSpec::DcMismatch {
        moment_options: Default::default(),
        output_expression: "V(out)".to_owned(),
        sigma_multiplier: 1.0,
        contributor_limit: 10,
        include_process: false,
        include_mismatch: true,
        normalized_contributions: true,
        contribution_threshold,
    };
    let expected = dc_mismatch(None);
    assert_eq!(analysis, expected);
    assert_eq!(AnalysisSpec::from(worker), expected);

    let authored = dc_mismatch(Some(0.05));
    let carried = WorkerAnalysisSpec::try_from(&authored).expect("the request is transportable");
    let encoded = serde_json::to_string(&carried).expect("the request serializes");
    let decoded: WorkerAnalysisSpec =
        serde_json::from_str(&encoded).expect("the request deserializes");
    assert_eq!(AnalysisSpec::from(decoded), authored);
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
        initialization: Default::default(),
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
        spectra: Vec::new(),
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
        false,
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
        measurement_references: Default::default(),
        id: 9,
        request: WorkerSimulationRequest::Spec {
            spec: Box::new(WorkerAnalysisSpec::Fourier {
                fundamental_freq: 2.0,
                num_harmonics: 4,
                num_periods: 1,
                output_node: "out".to_owned(),
                output_ref: "0".to_owned(),
                additional_outputs: Vec::new(),
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
fn standalone_connection_worker_transport_retains_selected_physics() {
    let (sources, deck) =
        crate::simulation::veriloga::test_support::standalone_connection_fixture();
    let request = WorkerRequest {
        measurement_references: Default::default(),
        id: 18,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::Transient {
            stop_time: 2e-9,
            step_time: 0.2e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        })),
        netlist: deck,
        source_path: None,
        project_veriloga_runtimes: sources,
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };
    let mut old = WorkerRequestTransport::from_request(request.clone()).unwrap();
    old.protocol = 9;
    assert!(
        old.into_request()
            .unwrap_err()
            .contains("unsupported worker request")
    );
    for (name, expected) in [("Low", 1.0), ("High", 5.0)] {
        let mut request = request.clone();
        request.netlist = request
            .netlist
            .replace("connectrules=Low", &format!("connectrules={name}"));
        let mut transport = WorkerRequestTransport::from_request(request.clone()).unwrap();
        let metadata = serde_json::to_vec(&transport.request).unwrap();
        transport.request = serde_json::from_slice(&metadata).unwrap();
        let restored = transport.into_request().unwrap();
        assert_eq!(restored, request);
        assert_eq!(restored.project_veriloga_runtimes.sources().count(), 2);
        assert_eq!(
            restored.project_veriloga_runtimes.device_runtimes().len(),
            1
        );
        let result = worker_response_from_request(restored)
            .into_result()
            .unwrap();
        let SimulationResult::Transient { waveforms, .. } = result else {
            panic!("transient result required")
        };
        let (_, q) = waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("q"))
            .unwrap_or_else(|| panic!("q voltage waveform missing from {:?}", waveforms.keys()));
        assert!(!q.y_values.is_empty());
        assert!(
            q.y_values
                .iter()
                .all(|value| (value - expected).abs() < 1e-9),
            "{name}: {:?}",
            q.y_values
        );
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
        measurement_references: Default::default(),
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
            .device_runtimes()
            .next()
            .unwrap()
            .validate()
            .is_ok()
    );
}

#[test]
fn worker_request_detaches_and_authenticates_op_previous_state() {
    use crate::simulation::dialog::OpInitialGuess;
    for initial_guess in [
        OpInitialGuess::PreviousConverged,
        OpInitialGuess::PreviousCompatible,
    ] {
        let mut config = nondefault_op_config();
        config.initial_guess = initial_guess;
        let request = WorkerRequest {
            measurement_references: Default::default(),
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
}

#[test]
fn unavailable_manifest_spec_round_trips_without_losing_typed_fields() {
    let spec = AnalysisSpec::DcMismatch {
        moment_options: Default::default(),
        output_expression: "V(out)".to_owned(),
        sigma_multiplier: 3.0,
        contributor_limit: 25,
        include_process: false,
        include_mismatch: true,
        normalized_contributions: true,
        contribution_threshold: Some(0.05),
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
        integration_method: None,
        tstab: 0.0,
        max_iterations: 100,
        abstol: 1.0e-12,
        damping: 1.0,
        max_period_change: 0.1,
        verbose: false,
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
    changed!(integration_method, Some(IntegrationMethod::Euler));
    changed!(tstab, 3.0e-9);
    changed!(max_iterations, 250);
    changed!(abstol, 1.0e-15);
    changed!(damping, 0.75);
    changed!(max_period_change, 0.25);
    changed!(verbose, true);

    for variant in variants {
        let encoded = encode(&variant);
        assert_ne!(baseline, encoded, "worker payload aliases {variant:?}");
        let worker: WorkerAnalysisSpec =
            serde_json::from_slice(&encoded).expect("PSS worker restores");
        assert_eq!(AnalysisSpec::from(worker), variant);
    }
}

/// A wire written before the solver controls existed restores as the run it
/// described.
///
/// The worker enum is the protocol, and an older worker's request carries none
/// of these keys. Each one is `serde(default)`-ed to the engine's own setting,
/// so the request that comes back is the one that was sent — a shooting solve
/// under the engine's integration method — rather than a decode failure.
#[test]
fn a_pss_wire_written_before_the_solver_controls_restores_with_the_engine_defaults() {
    let current = AnalysisSpec::Pss {
        method: PssMethod::Shooting,
        fundamental_freq: 1.0e6,
        tone_sources: vec!["VCLK".to_owned()],
        tstab_periods: 20,
        points_per_period: 512,
        tolerance: 1.0e-7,
        oscillator_mode: false,
        oscillator_node: None,
        num_harmonics: 20,
        integration_method: None,
        tstab: 0.0,
        max_iterations: 100,
        abstol: 1.0e-12,
        damping: 1.0,
        max_period_change: 0.1,
        verbose: false,
    };
    let worker = WorkerAnalysisSpec::try_from(&current).expect("PSS worker conversion");
    let mut wire = serde_json::to_value(&worker).expect("PSS worker serializes");
    let payload = wire["Pss"]
        .as_object_mut()
        .expect("the PSS wire payload is an object");
    for retired in SOLVER_CONTROL_WIRE_KEYS {
        assert!(
            payload.remove(*retired).is_some(),
            "the fixture must carry {retired} for its removal to mean anything"
        );
    }

    let restored: WorkerAnalysisSpec =
        serde_json::from_value(wire).expect("an older worker's PSS request decodes");
    assert_eq!(
        AnalysisSpec::from(restored),
        current,
        "an absent solver control means the engine's own setting"
    );
}

/// The wire keys the solver controls occupy, in the order they were added.
const SOLVER_CONTROL_WIRE_KEYS: &[&str] = &[
    "integration_method",
    "tstab",
    "max_iterations",
    "abstol",
    "damping",
    "max_period_change",
    "verbose",
];

#[test]
fn legacy_hb_specs_default_the_exact_collocation_grid() {
    let analysis = AnalysisSpec::HarmonicBalance {
        tones: vec![HbToneSpec::new(1.0e6, 3)],
        reltol: 1.0e-6,
        abstol: 1.0e-12,
        max_iterations: 40,
        damping: 0.7,
        min_damping: 0.01,
        oversample: 4,
        collocation_points: Some(7),
        max_mixing_order: 3,
        use_krylov: false,
        gmres_restart: 12,
        source_stepping: false,
        use_exact_jacobian: true,
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

/// A request an older worker encoded restores as the run it described.
///
/// The wire format is the protocol: a field added without a `serde(default)`
/// refuses the request outright, and one added with the wrong default accepts
/// it and solves something else. Both defaults are the values the engine used
/// while no layer carried the field — the line search's hardcoded `0.01`
/// floor, and the exact real-split Jacobian.
#[test]
fn an_hb_wire_written_before_the_solver_controls_restores_with_the_engine_defaults() {
    let analysis = AnalysisSpec::HarmonicBalance {
        tones: vec![HbToneSpec::new(1.0e6, 3)],
        reltol: 1.0e-6,
        abstol: 1.0e-12,
        max_iterations: 40,
        damping: 0.7,
        min_damping: 0.35,
        oversample: 4,
        collocation_points: Some(7),
        max_mixing_order: 3,
        use_krylov: false,
        gmres_restart: 12,
        source_stepping: false,
        use_exact_jacobian: false,
        verbose: false,
    };
    let worker = WorkerAnalysisSpec::try_from(&analysis).expect("worker spec converts");
    let mut worker_json = serde_json::to_value(&worker).expect("worker spec serializes");
    let payload = worker_json["HarmonicBalance"]
        .as_object_mut()
        .expect("worker HB payload is an object");
    for key in ["min_damping", "use_exact_jacobian"] {
        assert!(payload.remove(key).is_some(), "{key} crosses the wire");
    }

    let decoded: WorkerAnalysisSpec =
        serde_json::from_value(worker_json).expect("a wire without the solver controls restores");
    let WorkerAnalysisSpec::HarmonicBalance {
        min_damping,
        use_exact_jacobian,
        ..
    } = decoded
    else {
        panic!("an HB wire restores as an HB wire");
    };
    assert_eq!(min_damping, 0.01);
    assert!(use_exact_jacobian);
}

#[test]
fn transient_worker_result_round_trips_through_json() {
    let result = WorkerSimulationResult::Transient {
        spectra: Vec::new(),
        convergence: None,
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
            units: None,
        }],
        events: WorkerEventHistory {
            current_impulses: None,
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
            buses: Vec::new(),
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
        units: None,
    }
}

fn response_with_measurement(measurement: WorkerMeasurement) -> WorkerResponse {
    WorkerResponse {
        id: 901,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Transient {
            spectra: Vec::new(),
            convergence: None,
            time: vec![0.0],
            waveforms: Vec::new(),
            measurements: vec![measurement],
            events: WorkerEventHistory::default(),
        })),
    }
}

#[test]
fn current_impulse_history_survives_worker_transfer_and_rejects_invalid_charge() {
    let history = crate::state::CurrentImpulseHistoryEvidence::fixture();
    let mut response = response_with_measurement(projected_worker_measurement());
    let WorkerOutcome::Success(result) = &mut response.outcome else {
        unreachable!()
    };
    let WorkerSimulationResult::Transient { time, events, .. } = result.as_mut() else {
        unreachable!()
    };
    *time = vec![0.0, 1.0];
    events.current_impulses = Some(history.clone());
    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let metadata = serde_json::to_string(&transport.response).unwrap();
    let transported = WorkerResponseTransport {
        response: serde_json::from_str(&metadata).unwrap(),
        ..transport
    }
    .into_response()
    .unwrap()
    .into_result()
    .unwrap();
    let SimulationResult::Transient { events, .. } = transported else {
        panic!("transient")
    };
    assert_eq!(events.current_impulses, Some(history));
    let WorkerOutcome::Success(result) = &mut response.outcome else {
        unreachable!()
    };
    let WorkerSimulationResult::Transient { events, .. } = result.as_mut() else {
        unreachable!()
    };
    events.current_impulses.as_mut().unwrap().traces[0].points[0].charge_coulombs = f64::NAN;
    assert!(WorkerResponseTransport::from_response(response).is_err());
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
        spectra: Vec::new(),
        time: vec![0.0, 1e-9],
        waveforms: HashMap::new(),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Default::default(),
        events: crate::simulation::results::TransientEventHistory {
            current_impulses: None,
            digital: vec![crate::simulation::results::EventNodeHistory {
                node_name: "clk".to_owned(),
                points: vec![crate::simulation::results::DigitalEventPoint {
                    time_s: 5e-10,
                    value_code: 1,
                }],
            }],
            real: Vec::new(),
            digital_buses: Vec::new(),
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

/// The filter, the whole grid and all three columns survive the wire.
///
/// The frozen single-point payload carried two maps read at one frequency; a
/// study carries the band, so what has to survive is every point of it — a
/// transport that kept only the first would present a sweep as an operating
/// point and nothing downstream could tell.
#[test]
fn a_sensitivity_study_survives_the_worker_wire() {
    use crate::state::{
        ComplexResultValue, SensitivityBasisEvidence, SensitivityStudyEvidence, SensitivityStudyRow,
    };
    use rspice_core::analysis::sensitivity::{SensitivityUnavailability, SensitivityValue};
    let evidence = SensitivityStudyEvidence {
        output: "V(out)".to_owned(),
        filter: "R* PARAM:*".to_owned(),
        basis: SensitivityBasisEvidence::Ac {
            frequencies_hz: vec![10_000.0, 100_000.0],
            output: vec![
                ComplexResultValue {
                    real: 4.0,
                    imaginary: 0.0,
                },
                ComplexResultValue {
                    real: 0.0,
                    imaginary: -2.0,
                },
            ],
        },
        rows: vec![
            SensitivityStudyRow {
                parameter: "PARAM:LENGTH".to_owned(),
                nominal_value: 1.0e-6,
                raw: vec![(-1.0).into(), (-2.0).into()],
                normalized: vec![(-0.25).into(), (-0.5).into()],
                phase: vec![(0.125).into(), (0.25).into()],
            },
            SensitivityStudyRow {
                parameter: "R1".to_owned(),
                nominal_value: 1000.0,
                raw: vec![
                    SensitivityValue::unavailable(
                        SensitivityUnavailability::NondifferentiableMagnitude,
                    ),
                    (2.0).into(),
                ],
                normalized: vec![(0.5).into(), (0.75).into()],
                phase: vec![
                    SensitivityValue::unavailable(SensitivityUnavailability::OutOfRange),
                    (-0.25).into(),
                ],
            },
        ],
    };
    evidence.validate().expect("the fixture is valid evidence");
    let source = SimulationResult::SensitivityStudy {
        evidence: std::sync::Arc::new(evidence.clone()),
    };
    let worker = WorkerSimulationResult::try_from(source).expect("worker conversion");
    let encoded = serde_json::to_vec(&worker).expect("worker result serializes");
    let decoded: WorkerSimulationResult =
        serde_json::from_slice(&encoded).expect("worker result deserializes");
    let restored = SimulationResult::from(decoded);

    let SimulationResult::SensitivityStudy { evidence: restored } = restored else {
        panic!("sensitivity study result")
    };
    assert_eq!(*restored, evidence);
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
            mean_confidence: None,
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
        spectra: Vec::new(),
        convergence: None,
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
        convergence: None,
        noise_reference_temperature_kelvin: None,
        reference_impedances_ohm: None,
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
        spectra: Vec::new(),
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
        spectra: Vec::new(),
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
            spectra: Vec::new(),
            convergence: None,
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
                mean_confidence: None,
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
            convergence: None,
            noise_reference_temperature_kelvin: None,
            reference_impedances_ohm: None,
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
fn worker_transport_retains_and_validates_resolved_port_references() {
    for references in [vec![75.0], vec![75.0, 100.0]] {
        let response = WorkerResponse {
            id: 19,
            outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Ac {
                convergence: None,
                noise_reference_temperature_kelvin: None,
                frequencies: vec![1e6, 2e6],
                waveforms: Vec::new(),
                measurements: Vec::new(),
                reference_impedances_ohm: Some(references.clone()),
            })),
        };
        let mut transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
        assert_eq!(transport.buffers.last(), Some(&references));
        let metadata = serde_json::to_string(&transport.response).unwrap();
        transport.response = serde_json::from_str(&metadata).unwrap();
        assert_eq!(transport.clone().into_response().unwrap(), response);
        let restored = match transport.clone().into_response().unwrap().outcome {
            WorkerOutcome::Success(result) => SimulationResult::from(*result),
            other => panic!("expected success: {other:?}"),
        };
        assert!(
            matches!(restored, SimulationResult::Ac { reference_impedances_ohm: Some(actual), .. } if actual == references)
        );
        transport.buffers.last_mut().unwrap()[0] = -1.0;
        assert!(
            transport
                .into_response()
                .unwrap_err()
                .contains("references")
        );
    }
    for references in [vec![], vec![0.0], vec![f64::NAN], vec![f64::INFINITY]] {
        let response = WorkerResponse {
            id: 19,
            outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Ac {
                convergence: None,
                noise_reference_temperature_kelvin: None,
                frequencies: vec![1e6],
                waveforms: Vec::new(),
                measurements: Vec::new(),
                reference_impedances_ohm: Some(references),
            })),
        };
        assert!(
            WorkerResponseTransport::from_response(response)
                .unwrap_err()
                .contains("references")
        );
    }
}

#[test]
fn sp_noise_request_executes_and_retains_physical_results_through_worker_transport() {
    use crate::simulation::engine_bridge::EngineBridge;
    use crate::simulation::execution::ResolvedExecutionDependencies;
    for port_count in [1, 2] {
        for do_noise in [false, true] {
            let spec = AnalysisSpec::SParameter {
                start_freq: 1e6,
                stop_freq: 3e6,
                points_per_unit: 3,
                sweep: FrequencySweep::Linear,
                z0: 50.0,
                ports: if port_count == 1 {
                    vec![SpPort {
                        node_pos: "p1".into(),
                        node_neg: "0".into(),
                        z0: None,
                    }]
                } else {
                    Vec::new()
                },
                do_noise,
            };
            let worker_spec = WorkerAnalysisSpec::try_from(&spec).unwrap();
            let worker_spec: WorkerAnalysisSpec =
                serde_json::from_str(&serde_json::to_string(&worker_spec).unwrap()).unwrap();
            let restored_spec = AnalysisSpec::from(worker_spec);
            assert_eq!(restored_spec, spec);
            let deck = if port_count == 1 {
                "* configured one-port noise\nR1 p1 0 100\n.end\n"
            } else {
                "* hierarchical two-port noise\n.subckt generator p n params: ordinal=1\nP1 p n portnum={ordinal} z0=50\n.ends generator\nX1 p1 0 generator ordinal=1\nX2 p2 0 generator ordinal=2\nR1 p1 p2 100\n.end\n"
            };
            let result = crate::simulation::runner::spec::run_spec_request(
                &EngineBridge::new(),
                restored_spec,
                SpecExecutionOptions::default(),
                deck,
                None,
                &ResolvedExecutionDependencies::default(),
                &rspice_core::NoAbort,
            )
            .unwrap();
            let response = WorkerResponse {
                id: 71,
                outcome: WorkerOutcome::Success(Box::new(
                    WorkerSimulationResult::try_from(result).unwrap(),
                )),
            };
            if do_noise {
                for temperature in [0.0, -1.0, f64::NAN, f64::INFINITY] {
                    let mut invalid = response.clone();
                    let WorkerOutcome::Success(result) = &mut invalid.outcome else {
                        unreachable!()
                    };
                    let WorkerSimulationResult::Ac {
                        noise_reference_temperature_kelvin,
                        ..
                    } = result.as_mut()
                    else {
                        unreachable!()
                    };
                    *noise_reference_temperature_kelvin = Some(temperature);
                    assert!(
                        WorkerResponseTransport::from_response(invalid)
                            .unwrap_err()
                            .contains("temperature")
                    );
                }
            }
            let mut transport = WorkerResponseTransport::from_response(response).unwrap();
            transport.response =
                serde_json::from_str(&serde_json::to_string(&transport.response).unwrap()).unwrap();
            let WorkerOutcome::Success(result) = transport.into_response().unwrap().outcome else {
                panic!("noise solve failed")
            };
            let SimulationResult::Ac {
                frequencies,
                waveforms,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
                ..
            } = SimulationResult::from(*result)
            else {
                panic!("expected SP data")
            };
            assert_eq!(frequencies, [1e6, 2e6, 3e6]);
            assert_eq!(reference_impedances_ohm, Some(vec![50.0; port_count]));
            if !do_noise {
                assert_eq!(noise_reference_temperature_kelvin, None);
                assert_eq!(waveforms.len(), port_count * port_count);
                continue;
            }
            assert_eq!(noise_reference_temperature_kelvin, Some(300.15));
            let thermal = 4.0 * rspice_core::constants::K_BOLTZMANN * 300.15 / 100.0;
            for row in 1..=port_count {
                for column in 1..=port_count {
                    let waveform = &waveforms[&format!("CY({row},{column})")];
                    assert_eq!(waveform.x_values, frequencies);
                    assert_eq!(waveform.y_unit, "A²/Hz");
                    let sign = if row == column { 1.0 } else { -1.0 };
                    assert!(
                        waveform
                            .y_values
                            .iter()
                            .all(|value| (value / thermal - sign).abs() < 1e-10)
                    );
                    assert!(
                        waveform
                            .y_imag
                            .as_ref()
                            .unwrap()
                            .iter()
                            .all(|value| value.abs() < thermal * 1e-10)
                    );
                }
            }
            if port_count == 2 {
                for (name, expected, unit) in
                    [("Rn", 100.0, "Ω"), ("F", 3.0, "1"), ("Fmin", 1.0, "1")]
                {
                    let waveform = &waveforms[name];
                    assert_eq!(waveform.y_unit, unit);
                    assert!(
                        waveform
                            .y_values
                            .iter()
                            .all(|value| (value - expected).abs() < 1e-8),
                        "{name}: {:?}",
                        waveform.y_values
                    );
                }
                assert_eq!(waveforms["Sopt"].y_unit, "1");
                assert!(
                    waveforms["Sopt"]
                        .y_values
                        .iter()
                        .all(|value| (value - 1.0).abs() < 1e-8)
                );
            } else {
                assert!(!waveforms.contains_key("Rn"));
            }
        }
    }
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
            spectra: Vec::new(),
            convergence: None,
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
                spectra: Vec::new(),
                convergence: None,
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
                spectra: Vec::new(),
                convergence: None,
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

#[test]
fn configured_study_worker_transfers_and_authenticates_nested_op_seed() {
    use crate::simulation::runner::study::StudyRunConfig;
    let options = SpecExecutionOptions {
        study_base: Some(StudyRunConfig {
            postprocess: None,
            constraints: Vec::new(),
            objective_terms: Vec::new(),
            instance_id: crate::product::AnalysisInstanceId::new(),
            source_revision: crate::product::ObjectRevision::INITIAL,
            analysis: AnalysisConfig::DcOp(nondefault_op_config()).into(),
            analysis_line: ".OP".into(),
            numeric_options: String::new(),
            measurements: vec!["scalar:V(out)".into()],
            histogram_bins: 7,
        }),
        ..Default::default()
    };
    let request = WorkerRequest {
        measurement_references: Default::default(),
        id: 11,
        request: WorkerSimulationRequest::Spec {
            spec: Box::new(WorkerAnalysisSpec::MonteCarlo {
                variation_source: Default::default(),
                params: vec![],
            }),
            options: Box::new(WorkerSpecExecutionOptions::from(&options)),
        },
        netlist: "Trial\nV1 out 0 1\nR1 out 0 1k\n.mc 3 seed 7\n.end\n".into(),
        source_path: None,
        project_veriloga_runtimes: Default::default(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
    };
    let transport = WorkerRequestTransport::from_request(request.clone()).unwrap();
    assert_eq!(transport.buffers, vec![vec![1.25, -1.0e-3]]);
    assert_eq!(transport.clone().into_request().unwrap(), request);
    let mut corrupted = transport;
    corrupted.buffers[0][0] += 1.0;
    assert!(
        corrupted
            .into_request()
            .unwrap_err()
            .contains("solution digest")
    );
}

#[test]
fn weighted_optimization_components_survive_transfer_and_reject_corruption() {
    use crate::simulation::optimizer::{
        OptimizationObjectiveGoal, OptimizationObjectiveObservation, OptimizationObjectiveTerm,
    };
    let result = SimulationResult::Optimization {
        iterations: vec![0.0],
        waveforms: HashMap::from([(
            "OPT_COST".into(),
            WaveformData::new_time_domain("OPT_COST", vec![0.0], vec![0.25]),
        )]),
        best_cost: 0.25,
        best_variables: HashMap::from([("X".into(), 2.0)]),
        converged: true,
        best_constraints: vec![
            crate::simulation::optimizer::OptimizationConstraintObservation {
                constraint: crate::simulation::optimizer::OptimizationConstraint {
                    measurement: "gain".into(),
                    lower: Some(1.0),
                    upper: Some(2.0),
                    tolerance: 0.0,
                    scale: 1.0,
                },
                value: 2.0,
                violation: 0.0,
            },
        ],
        best_objectives: vec![OptimizationObjectiveObservation {
            objective: OptimizationObjectiveTerm {
                measurement: "gain".into(),
                goal: OptimizationObjectiveGoal::Target,
                target: Some(1.0),
                scale: 2.0,
                weight: 1.0,
            },
            value: 2.0,
            contribution: 0.25,
        }],
    };
    let response = WorkerResponse {
        id: 37,
        outcome: WorkerOutcome::Success(Box::new(
            WorkerSimulationResult::try_from(result).unwrap(),
        )),
    };
    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let json = serde_json::to_string(&transport.response).unwrap();
    let transport = WorkerResponseTransport {
        response: serde_json::from_str(&json).unwrap(),
        ..transport
    };
    assert_eq!(transport.clone().into_response().unwrap(), response);
    let corrupt_constraints = json.replace("\"violation\":0.0", "\"violation\":1.0");
    assert_ne!(json, corrupt_constraints);
    assert!(
        WorkerResponseTransport {
            response: serde_json::from_str(&corrupt_constraints).unwrap(),
            ..transport.clone()
        }
        .into_response()
        .is_err()
    );
    let corrupt = json.replace("\"contribution\":0.25", "\"contribution\":0.5");
    assert_ne!(json, corrupt);
    let transport = WorkerResponseTransport {
        response: serde_json::from_str(&corrupt).unwrap(),
        ..transport
    };
    assert!(transport.into_response().is_err());
}

#[test]
fn measurement_units_survive_worker_and_study_projection() {
    use rspice_core::analysis::{MeasurementUnit, MeasurementUnits};
    let units = MeasurementUnits {
        value: MeasurementUnit::Known("s".into()),
        raw_value: MeasurementUnit::Known("V".into()),
        axis: MeasurementUnit::Known("s".into()),
    };
    let mut measurement = projected_worker_measurement();
    measurement.units = Some(units.clone());
    let response = response_with_measurement(measurement);
    let mut transport = WorkerResponseTransport::from_response(response).unwrap();
    let metadata = serde_json::to_string(&transport.response).unwrap();
    transport.response = serde_json::from_str(&metadata).unwrap();
    let result = transport.into_response().unwrap().into_result().unwrap();
    let evidence = result.study_measurement("meas:peak_at").unwrap();
    assert_eq!(evidence.unit, Some(units.value.clone()));
    let SimulationResult::Transient { measurements, .. } = result else {
        panic!("transient");
    };
    assert_eq!(measurements[0].units, Some(units));
    let mut invalid = projected_worker_measurement();
    invalid.units = Some(MeasurementUnits {
        value: MeasurementUnit::Known("bogus".into()),
        raw_value: MeasurementUnit::Unknown,
        axis: MeasurementUnit::Unknown,
    });
    assert!(WorkerResponseTransport::from_response(response_with_measurement(invalid)).is_err());
}
