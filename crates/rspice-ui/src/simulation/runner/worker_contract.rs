//! The simulation worker contract.
//!
//! Everything crossing the boundary between the application and the engine
//! worker: the request and result messages, their transport encoding, and
//! the state each side must hold for a request to be replayable. The two
//! sides run in different threads natively and different contexts in the
//! browser, so this is a serialized contract rather than a shared type.

mod analysis;
mod transport;

pub(crate) use transport::*;

pub(crate) use analysis::*;

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = rspice_core::analysis::advanced::pss::PssResult {
            period: 1.0,
            frequency: 1.0,
            iterations: 2,
            residual_norm: 1.0e-10,
            time,
            waveforms: vec![
                rspice_core::analysis::advanced::pss::PeriodicWaveform::from_values(waveform),
            ],
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
            violation_source_content_digest: Some(crate::product::ContentDigest::from_bytes(
                [1; 32],
            )),
            run_point: OpRunPointContext {
                index: 2,
                count: 3,
                process: crate::simulation::dialog::corner::ProcessCorner::SS,
                supply_voltage: Some(0.9),
                nominal_supply_voltage: Some(1.0),
            },
        }
    }

    #[test]
    fn browser_worker_transfer_protocol_matches_rust_transport() {
        let source = include_str!("../../../web/simulation-worker.js");
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
        DcOpResult, SimulationResult, TransferFunctionQuantity, TransferFunctionScalar,
        WaveformData,
    };
    use std::collections::HashMap;

    #[test]
    fn worker_request_round_trips_through_json() {
        let request = WorkerRequest {
            id: 7,
            request: WorkerSimulationRequest::Config(WorkerAnalysisConfig::Transient {
                stop_time: 1e-6,
                step_time: 1e-9,
                start_time: 0.0,
                max_timestep: Some(1e-9),
                uic: false,
            }),
            netlist: "V1 in 0 1\nR1 in 0 1k\n.tran 1n 1u\n.end\n".to_string(),
            source_path: None,
            project_veriloga_runtimes: Default::default(),
            dependencies: Default::default(),
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
            extraction_path: EnvelopeExtractionPath::Preview,
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
        let binding = PreparedDependencyBinding::transient_trajectory(
            producer,
            source_revision,
            config_digest,
        );
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
        let runtime = crate::simulation::veriloga::PreparedVerilogARuntime::try_from_current_bundle_receipt(
            project_id, &bundle, &receipt,
        )
        .unwrap();
        let request = WorkerRequest {
            id: 8,
            request: WorkerSimulationRequest::Config(WorkerAnalysisConfig::Transient {
                stop_time: 1e-6,
                step_time: 1e-9,
                start_time: 0.0,
                max_timestep: None,
                uic: false,
            }),
            netlist: format!(
                "{}\n.end\n",
                crate::simulation::veriloga::project_veriloga_directive(
                    runtime.source_key(),
                    runtime.module_name()
                )
            ),
            source_path: None,
            project_veriloga_runtimes:
                crate::simulation::veriloga::PreparedVerilogARuntimeSet::try_new(vec![runtime])
                    .unwrap(),
            dependencies: Default::default(),
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
            request: WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp(config)),
            netlist: "V1 out 0 1\n.op\n.end\n".to_owned(),
            source_path: None,
            project_veriloga_runtimes: Default::default(),
            dependencies: Default::default(),
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
        let WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp(config)) =
            &mut duplicate.request.request.request
        else {
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
                x_unit: "s".to_string(),
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
        };

        let encoded = serde_json::to_string(&result).expect("result serializes");
        let decoded: WorkerSimulationResult =
            serde_json::from_str(&encoded).expect("result deserializes");

        assert_eq!(decoded, result);
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
                x_unit: "s".to_string(),
                is_complex: false,
                y_imag: None,
            }],
            measurements: Vec::new(),
        };
        assert_eq!(transient.estimated_numeric_payload_bytes(), 48);

        let ac = WorkerSimulationResult::Ac {
            frequencies: vec![1.0, 10.0, 100.0],
            waveforms: vec![WorkerWaveform {
                name: "V(out)".to_string(),
                x_values: vec![1.0, 10.0, 100.0],
                y_values: vec![1.0, 0.5, 0.25],
                y_unit: String::new(),
                x_unit: "Hz".to_string(),
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
            outcome: WorkerOutcome::Success(WorkerSimulationResult::Transient {
                time: vec![0.0, 1.0],
                waveforms: vec![WorkerWaveform {
                    name: "V(out)".to_string(),
                    x_values: vec![0.0, 1.0],
                    y_values: vec![0.2, 0.4],
                    y_unit: "V".to_string(),
                    x_unit: "s".to_string(),
                    is_complex: false,
                    y_imag: None,
                }],
                measurements: Vec::new(),
            }),
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
            outcome: WorkerOutcome::Success(WorkerSimulationResult::MonteCarlo {
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
            }),
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
            outcome: WorkerOutcome::Success(WorkerSimulationResult::Ac {
                frequencies: vec![1.0, 10.0, 100.0],
                waveforms: vec![WorkerWaveform {
                    name: "V(out)".to_string(),
                    x_values: vec![1.0, 10.0, 100.0],
                    y_values: vec![0.5, 0.25, 0.125],
                    y_unit: String::new(),
                    x_unit: "Hz".to_string(),
                    is_complex: true,
                    y_imag: Some(vec![-0.1, -0.2, -0.3]),
                }],
                measurements: Vec::new(),
            }),
        };
        let ac_transport = WorkerResponseTransport::from_response(ac.clone()).unwrap();
        assert_eq!(ac_transport.buffers.len(), 4);
        assert_eq!(ac_transport.into_response().expect("ac reconstructs"), ac);

        let noise = WorkerResponse {
            id: 11,
            outcome: WorkerOutcome::Success(WorkerSimulationResult::Noise {
                frequencies: vec![1.0, 10.0],
                output_noise: vec![1.0e-18, 2.0e-18],
                input_noise: Some(vec![3.0e-18, 4.0e-18]),
                contributors: HashMap::from([
                    ("R1".to_string(), vec![0.5e-18, 1.0e-18]),
                    ("M1".to_string(), vec![0.25e-18, 0.5e-18]),
                ]),
                summary: None,
            }),
        };
        let noise_transport = WorkerResponseTransport::from_response(noise.clone()).unwrap();
        assert_eq!(noise_transport.buffers.len(), 5);
        assert_eq!(
            noise_transport.into_response().expect("noise reconstructs"),
            noise
        );
    }

    #[test]
    fn worker_transport_rejects_missing_or_mismatched_buffers() {
        let response = WorkerResponse {
            id: 12,
            outcome: WorkerOutcome::Success(WorkerSimulationResult::Transient {
                time: vec![0.0, 1.0],
                waveforms: vec![WorkerWaveform {
                    name: "V(out)".to_string(),
                    x_values: vec![0.0, 1.0],
                    y_values: vec![0.2, 0.4],
                    y_unit: "V".to_string(),
                    x_unit: "s".to_string(),
                    is_complex: false,
                    y_imag: None,
                }],
                measurements: Vec::new(),
            }),
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
                outcome: WorkerOutcomeTransport::Success(
                    WorkerSimulationResultTransport::Transient {
                        time: WorkerF64Series::Buffer { buffer: 0, len: 2 },
                        waveforms: vec![WorkerWaveformTransport {
                            name: "V(out)".to_string(),
                            x_values: WorkerF64Series::Buffer { buffer: 1, len: 2 },
                            y_values: WorkerF64Series::Buffer { buffer: 2, len: 2 },
                            y_unit: "V".to_string(),
                            x_unit: "s".to_string(),
                            is_complex: true,
                            y_imag: Some(WorkerF64Series::Buffer { buffer: 3, len: 1 }),
                        }],
                        measurements: Vec::new(),
                    },
                ),
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
                outcome: WorkerOutcomeTransport::Success(
                    WorkerSimulationResultTransport::Transient {
                        time: WorkerF64Series::Buffer { buffer: 0, len: 2 },
                        waveforms: vec![WorkerWaveformTransport {
                            name: "V(out)".to_string(),
                            x_values: WorkerF64Series::Buffer { buffer: 1, len: 2 },
                            y_values: WorkerF64Series::Buffer { buffer: 2, len: 2 },
                            y_unit: "V".to_string(),
                            x_unit: "s".to_string(),
                            is_complex: false,
                            y_imag: Some(WorkerF64Series::Buffer { buffer: 3, len: 2 }),
                        }],
                        measurements: Vec::new(),
                    },
                ),
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
        progress.message = Some("Halfway through transient".to_string());

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
        assert_eq!(
            applied.message.as_deref(),
            Some("Halfway through transient")
        );
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
            AnalysisSpec::MonteCarlo,
            AnalysisSpec::Stb {
                probe_node: "Vprobe".to_string(),
                start_freq: 1.0,
                stop_freq: 1e6,
                sweep: FrequencySweep::Decade,
                points_per_decade: 12,
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
                f2_over_f1: Some(1.2),
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
                extraction_path: EnvelopeExtractionPath::Preview,
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
            spec: Box::new(AnalysisSpec::MonteCarlo),
            options: Box::new(SpecExecutionOptions::default()),
        };
        let input = NetlistInput {
            netlist: "V1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.mc 10 R1 0.05 gaussian\n.end\n"
                .to_string(),
            source_path: None,
            project_veriloga_runtimes: Default::default(),
            dependencies: Default::default(),
        };

        let worker =
            WorkerRequest::from_runner_parts(101, &request, &input).expect("Monte Carlo converts");
        let (round_tripped, _) = worker.into_runner_parts();

        match round_tripped {
            SimulationRequest::Spec { spec, options } => {
                assert!(matches!(*spec, AnalysisSpec::MonteCarlo));
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
            netlist: "Vstim in 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) Vstim\n.end\n"
                .to_string(),
            source_path: None,
            project_veriloga_runtimes: Default::default(),
            dependencies: Default::default(),
        };

        let worker =
            WorkerRequest::from_runner_parts(51, &request, &input).expect("request converts");
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
            netlist: "V1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.step temp -40 125 55\n.end\n"
                .to_string(),
            source_path: None,
            project_veriloga_runtimes: Default::default(),
            dependencies: Default::default(),
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
        };

        let worker =
            WorkerRequest::from_runner_parts(91, &request, &input).expect("Corner converts");
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
    fn worker_result_round_trip() {
        let dc_op = SimulationResult::DcOp(DcOpResult {
            configuration: crate::simulation::dialog::OpConfig::default(),
            validated_startup_directives: 0,
            mna_node_names: vec!["out".to_owned()],
            mna_branch_names: vec!["V1".to_owned()],
            mna_solution: vec![1.2, -0.01],
            node_voltages: HashMap::from([("out".to_string(), 1.2)]),
            branch_currents: HashMap::from([("V1".to_string(), -0.01)]),
            device_ops: HashMap::new(),
            device_report: None,
        });
        let dc_op = round_trip_result(dc_op);
        match dc_op {
            SimulationResult::DcOp(result) => {
                assert_eq!(result.node_voltages.get("out"), Some(&1.2));
                assert_eq!(result.branch_currents.get("V1"), Some(&-0.01));
                assert!(result.device_report.is_none());
            }
            other => panic!("expected dc op result, got {other:?}"),
        }

        let transient = SimulationResult::Transient {
            time: vec![0.0, 1e-9],
            waveforms: HashMap::from([(
                "V(out)".to_string(),
                WaveformData::new_time_domain("V(out)", vec![0.0, 1e-9], vec![0.0, 0.8]),
            )]),
            measurements: vec![rspice_core::MeasureResult::failed(
                "delay",
                "target not found",
            )],
            periodic_state: None,
        };
        let transient = round_trip_result(transient);
        match transient {
            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
                ..
            } => {
                assert_eq!(time, vec![0.0, 1e-9]);
                let waveform = waveforms.get("V(out)").expect("waveform is preserved");
                assert_eq!(waveform.name, "V(out)");
                assert_eq!(waveform.x_values, vec![0.0, 1e-9]);
                assert_eq!(waveform.y_values, vec![0.0, 0.8]);
                assert_eq!(measurements[0].name, "delay");
                assert!(!measurements[0].passed);
                assert_eq!(measurements[0].value, None);
                assert_eq!(measurements[0].error.as_deref(), Some("target not found"));
            }
            other => panic!("expected transient result, got {other:?}"),
        }

        let parametric = SimulationResult::Parametric {
            target: "TEMP".to_string(),
            sweep_values: vec![-40.0, 25.0, 125.0],
            waveforms: HashMap::from([(
                "V(out)".to_string(),
                WaveformData::new_time_domain(
                    "V(out)",
                    vec![-40.0, 25.0, 125.0],
                    vec![0.8, 0.9, 1.0],
                ),
            )]),
            num_failures: 1,
        };
        let parametric = round_trip_result(parametric);
        match parametric {
            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => {
                assert_eq!(target, "TEMP");
                assert_eq!(sweep_values, vec![-40.0, 25.0, 125.0]);
                assert_eq!(waveforms["V(out)"].y_values, vec![0.8, 0.9, 1.0]);
                assert_eq!(num_failures, 1);
            }
            other => panic!("expected parametric result, got {other:?}"),
        }

        let corner = SimulationResult::Corner {
            x_values: vec![0.0, 1.0],
            x_label: "Corner Index".to_string(),
            x_unit: String::new(),
            temperatures_c: vec![25.0, 125.0],
            corner_labels: vec!["TT_1.0V_25C".to_string(), "FF_1.1V_125C".to_string()],
            waveforms: HashMap::from([(
                "V(out)".to_string(),
                WaveformData::new_time_domain("V(out)", vec![0.0, 1.0], vec![1.0, 1.1]),
            )]),
            num_failures: 2,
        };
        let corner = round_trip_result(corner);
        match corner {
            SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
            } => {
                assert_eq!(x_values, vec![0.0, 1.0]);
                assert_eq!(x_label, "Corner Index");
                assert_eq!(x_unit, "");
                assert_eq!(temperatures_c, vec![25.0, 125.0]);
                assert_eq!(
                    corner_labels,
                    vec!["TT_1.0V_25C".to_string(), "FF_1.1V_125C".to_string()]
                );
                assert_eq!(waveforms["V(out)"].y_values, vec![1.0, 1.1]);
                assert_eq!(num_failures, 2);
            }
            other => panic!("expected corner result, got {other:?}"),
        }

        let monte_carlo = SimulationResult::MonteCarlo {
            seed: 0x1234_5678_9abc_def0,
            runs_requested: 20,
            runs_completed: 18,
            num_failures: 2,
            all_converged: false,
            variables: vec![crate::simulation::results::MonteCarloVariableResult {
                name: "V(out)".to_string(),
                samples: vec![0.9, 1.0, 1.1],
                mean: 1.0,
                std_dev: 0.05,
                min: 0.9,
                max: 1.1,
                histogram: vec![2, 10, 6],
                bin_edges: vec![0.9, 0.95, 1.05, 1.1],
            }],
        };
        let monte_carlo = round_trip_result(monte_carlo);
        match monte_carlo {
            SimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
            } => {
                assert_eq!(seed, 0x1234_5678_9abc_def0);
                assert_eq!(runs_requested, 20);
                assert_eq!(runs_completed, 18);
                assert_eq!(num_failures, 2);
                assert!(!all_converged);
                assert_eq!(variables[0].name, "V(out)");
                assert_eq!(variables[0].samples, vec![0.9, 1.0, 1.1]);
                assert_eq!(variables[0].histogram, vec![2, 10, 6]);
                assert_eq!(variables[0].bin_edges, vec![0.9, 0.95, 1.05, 1.1]);
            }
            other => panic!("expected Monte Carlo result, got {other:?}"),
        }

        let reliability = SimulationResult::Reliability {
            years: vec![1.0, 10.0],
            waveforms: HashMap::from([(
                "DVTH(M1)".to_string(),
                WaveformData::new_time_domain("DVTH(M1)", vec![1.0, 10.0], vec![0.01, 0.03]),
            )]),
            device_results: vec![crate::simulation::reliability_engine::ReliabilityResult {
                device_id: "M1".to_string(),
                stress: crate::simulation::reliability_engine::StressMetrics {
                    avg_vgs_stress: 1.1,
                    avg_vds_stress: 1.7,
                    avg_temp: 398.0,
                    duration: 1.0e6,
                },
                shifts: HashMap::from([(
                    "10y".to_string(),
                    crate::simulation::reliability_engine::ParamShift {
                        vth_shift: 0.03,
                        mobility_shift: -0.02,
                        rds_shift: 0.004,
                    },
                )]),
            }],
        };
        let reliability = round_trip_result(reliability);
        match reliability {
            SimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => {
                assert_eq!(years, vec![1.0, 10.0]);
                assert_eq!(waveforms["DVTH(M1)"].y_values, vec![0.01, 0.03]);
                assert_eq!(device_results[0].device_id, "M1");
                assert_eq!(device_results[0].stress.avg_temp, 398.0);
                assert_eq!(device_results[0].shifts["10y"].vth_shift, 0.03);
            }
            other => panic!("expected reliability result, got {other:?}"),
        }

        let optimization = SimulationResult::Optimization {
            iterations: vec![0.0, 1.0, 2.0],
            waveforms: HashMap::from([(
                "OPT_COST".to_string(),
                WaveformData::new_time_domain("OPT_COST", vec![0.0, 1.0, 2.0], vec![2.0, 1.0, 0.1]),
            )]),
            best_cost: 0.1,
            best_variables: HashMap::from([("RLOAD".to_string(), 1234.0)]),
            converged: true,
        };
        let optimization = round_trip_result(optimization);
        match optimization {
            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => {
                assert_eq!(iterations, vec![0.0, 1.0, 2.0]);
                assert_eq!(waveforms["OPT_COST"].y_values, vec![2.0, 1.0, 0.1]);
                assert_eq!(best_cost, 0.1);
                assert_eq!(best_variables["RLOAD"], 1234.0);
                assert!(converged);
            }
            other => panic!("expected optimization result, got {other:?}"),
        }

        let soa = SimulationResult::Soa {
            time: vec![0.0, 1e-6],
            waveforms: HashMap::from([(
                "SOA_VIOLATION_COUNT".to_string(),
                WaveformData::new_time_domain(
                    "SOA_VIOLATION_COUNT",
                    vec![0.0, 1e-6],
                    vec![0.0, 1.0],
                ),
            )]),
            violations: vec![crate::services::safety::SoAViolation {
                device_id: "M1".to_string(),
                parameter: crate::services::safety::SoAParameter::Vgs,
                limit_value: 1.2,
                actual_value: 1.35,
                time: 1e-6,
                severity: crate::services::safety::ViolationSeverity::Critical,
            }],
            evaluations: vec![crate::services::safety::SoAEvaluation {
                device_id: "M1".to_string(),
                parameter: crate::services::safety::SoAParameter::Vgs,
                limit_value: 1.2,
                worst_actual_value: 1.35,
                worst_time: 1e-6,
                sample_count: 2,
                unit: "V".to_string(),
                description: "Maximum gate-source voltage".to_string(),
                verdict: crate::services::safety::SoARuleVerdict::Violation,
            }],
        };
        let soa = round_trip_result(soa);
        match soa {
            SimulationResult::Soa {
                time,
                waveforms,
                violations,
                evaluations,
            } => {
                assert_eq!(time, vec![0.0, 1e-6]);
                assert_eq!(waveforms["SOA_VIOLATION_COUNT"].y_values, vec![0.0, 1.0]);
                assert_eq!(violations[0].device_id, "M1");
                assert_eq!(
                    violations[0].severity,
                    crate::services::safety::ViolationSeverity::Critical
                );
                assert_eq!(evaluations[0].sample_count, 2);
                assert_eq!(
                    evaluations[0].verdict,
                    crate::services::safety::SoARuleVerdict::Violation
                );
            }
            other => panic!("expected SOA result, got {other:?}"),
        }

        let noise_summary = crate::state::NoiseSummary {
            rows: vec![
                crate::state::NoiseContributorRow {
                    device: "R1".to_string(),
                    mechanism: "thermal".to_owned(),
                    power: 2.5e-18,
                    share_pct: 75.0,
                },
                crate::state::NoiseContributorRow {
                    device: "BNOISE1".to_string(),
                    mechanism: "white".to_owned(),
                    power: 0.5e-18,
                    share_pct: 15.0,
                },
                crate::state::NoiseContributorRow {
                    device: "ATABLE1".to_string(),
                    mechanism: "table".to_owned(),
                    power: 0.25e-18,
                    share_pct: 10.0,
                },
            ],
            total_rms: Some(1.2e-6),
            input_rms: Some(8.0e-7),
            band: (1.0, 1.0e6),
        };
        let noise = SimulationResult::Noise {
            frequencies: vec![1.0, 10.0],
            output_noise: vec![1.0e-18, 2.0e-18],
            input_noise: Some(vec![3.0e-18, 4.0e-18]),
            contributors: HashMap::from([("R1".to_string(), vec![0.7e-18, 1.4e-18])]),
            summary: Some(noise_summary.clone()),
        };
        let noise = round_trip_result(noise);
        match noise {
            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => {
                assert_eq!(frequencies, vec![1.0, 10.0]);
                assert_eq!(output_noise, vec![1.0e-18, 2.0e-18]);
                assert_eq!(input_noise, Some(vec![3.0e-18, 4.0e-18]));
                assert_eq!(contributors["R1"], vec![0.7e-18, 1.4e-18]);
                assert_eq!(summary, Some(noise_summary));
            }
            other => panic!("expected noise result, got {other:?}"),
        }

        let ac = SimulationResult::Ac {
            frequencies: vec![1.0, 10.0],
            waveforms: HashMap::from([(
                "V(out)".to_string(),
                WaveformData::new_complex(
                    "V(out)",
                    vec![1.0, 10.0],
                    vec![0.5, 0.25],
                    vec![-0.1, -0.2],
                ),
            )]),
            measurements: vec![rspice_core::MeasureResult::success("gain", 0.5)],
        };
        let ac = round_trip_result(ac);
        match ac {
            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => {
                assert_eq!(frequencies, vec![1.0, 10.0]);
                let waveform = waveforms.get("V(out)").expect("waveform is preserved");
                assert!(waveform.is_complex);
                assert_eq!(waveform.y_values, vec![0.5, 0.25]);
                assert_eq!(waveform.y_imag.as_deref(), Some(&[-0.1, -0.2][..]));
                assert_eq!(measurements[0].name, "gain");
                assert_eq!(measurements[0].value, Some(0.5));
                assert!(measurements[0].passed);
            }
            other => panic!("expected ac result, got {other:?}"),
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
        };

        let worker =
            WorkerRequest::from_runner_parts(41, &request, &input).expect("request converts");

        assert_eq!(worker.id, 41);
        assert!(matches!(
            worker.request,
            WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp(_))
        ));
        assert_eq!(worker.netlist, input.netlist);
        assert_eq!(worker.source_path.as_deref(), Some("deck.cir"));
    }

    #[test]
    fn dc_op_worker_result_round_trip_preserves_exact_mna_state_and_contract() {
        let expected_config = nondefault_op_config();
        let result = SimulationResult::DcOp(DcOpResult {
            configuration: expected_config.clone(),
            validated_startup_directives: 2,
            mna_node_names: vec!["out".to_owned()],
            mna_branch_names: vec!["V1".to_owned()],
            mna_solution: vec![1.25, -1.0e-3],
            node_voltages: HashMap::from([("out".to_owned(), 1.25)]),
            branch_currents: HashMap::from([("V1".to_owned(), -1.0e-3)]),
            device_ops: HashMap::new(),
            device_report: None,
        });

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
            request: WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp(
                crate::simulation::dialog::OpConfig::default(),
            )),
            netlist: "* worker op\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n".to_string(),
            source_path: None,
            project_veriloga_runtimes: Default::default(),
            dependencies: Default::default(),
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
        };
        let worker =
            WorkerRequest::from_runner_parts(13, &request, &input).expect("request converts");
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
            gain_unit: "A/V".to_owned(),
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
}
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

use serde::{Deserialize, Serialize};

use super::{NetlistInput, SimulationError, SimulationRequest, SpecExecutionOptions};
use crate::services::safety::{
    SoAEvaluation, SoAParameter, SoARuleVerdict, SoAViolation, ViolationSeverity,
};
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType, PoleZeroConfig, PzAnalysisType,
    SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    PssMethod, SpPort, TfAccuracy, TfNormalization,
};
use crate::simulation::reliability_engine::{ParamShift, ReliabilityResult, StressMetrics};
use crate::simulation::results::{
    DcOpResult, DeviceOpPoint, MonteCarloVariableResult, SimulationResult,
    TransferFunctionQuantity, TransferFunctionScalar, WaveformData,
};
use crate::simulation::status::{SimulationProgress, SimulationStatus};
use crate::state::{NoiseContributorRow, NoiseSummary};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRequest {
    pub id: u64,
    pub request: WorkerSimulationRequest,
    pub netlist: String,
    pub source_path: Option<String>,
    #[serde(default)]
    pub project_veriloga_runtimes: crate::simulation::veriloga::PreparedVerilogARuntimeSet,
    #[serde(default)]
    pub(in crate::simulation) dependencies:
        crate::simulation::execution::ResolvedExecutionDependencies,
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const WORKER_REQUEST_TRANSPORT_PROTOCOL: u8 = 5;

/// Browser-worker request split into compact metadata and transferable
/// floating-point buffers. The embedded request deliberately carries empty
/// dependencies; authenticated dependency metadata is encoded separately so
/// its numerical payload never expands into per-sample JavaScript objects.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WorkerRequestTransport {
    pub protocol: u8,
    pub request: WorkerRequestTransportMetadata,
    pub buffers: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRequestTransportMetadata {
    pub request: WorkerRequest,
    pub dependency_metadata: String,
    /// Number of leading buffers owned by `dependency_metadata`. Any
    /// remaining buffer is reserved for the detached OP previous-state MNA
    /// vector below.
    pub dependency_buffer_count: usize,
    #[serde(default)]
    pub op_previous_state: Option<WorkerOpPreviousStateTransport>,
}

/// Authenticated scalar half of a retained OP initial guess. The numerical
/// MNA state is always a single transferable Float64 buffer; accepting an
/// inline representation here would silently reintroduce the browser JSON
/// expansion this transport exists to prevent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerOpPreviousStateTransport {
    source_content_digest: crate::product::ContentDigest,
    producer_snapshot_digest: crate::product::ContentDigest,
    producer_result_digest: crate::product::ContentDigest,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    solution: WorkerF64Series,
    solution_digest: crate::product::ContentDigest,
}

#[cfg(any(target_arch = "wasm32", test))]
impl WorkerRequestTransport {
    #[cfg(test)]
    pub(crate) fn from_request(mut request: WorkerRequest) -> Result<Self, String> {
        let dependencies = std::mem::take(&mut request.dependencies);
        let (dependency_metadata, mut buffers) = dependencies
            .encode_transfer()
            .map_err(|error| error.to_string())?;
        let dependency_buffer_count = buffers.len();
        let (op_previous_state, op_buffers) = take_worker_request_op_previous_state(&mut request)?;
        buffers.extend(op_buffers);
        validate_worker_request_transfer_buffers(&buffers)?;
        Ok(Self {
            protocol: WORKER_REQUEST_TRANSPORT_PROTOCOL,
            request: WorkerRequestTransportMetadata {
                request,
                dependency_metadata,
                dependency_buffer_count,
                op_previous_state,
            },
            buffers,
        })
    }

    pub(crate) fn into_request(self) -> Result<WorkerRequest, String> {
        if self.protocol != WORKER_REQUEST_TRANSPORT_PROTOCOL {
            return Err(format!(
                "unsupported worker request transport protocol {}",
                self.protocol
            ));
        }
        let WorkerRequestTransportMetadata {
            mut request,
            dependency_metadata,
            dependency_buffer_count,
            op_previous_state,
        } = self.request;
        if request.dependencies != Default::default() {
            return Err("worker request metadata carries duplicate inline dependencies".to_owned());
        }
        reject_inline_worker_request_op_previous_state(&request)?;
        validate_worker_request_transfer_buffers(&self.buffers)?;
        if dependency_buffer_count > self.buffers.len() {
            return Err(format!(
                "worker request declares {dependency_buffer_count} dependency buffers but carries only {} total buffers",
                self.buffers.len()
            ));
        }
        let mut dependency_buffers = self.buffers;
        let op_buffers = dependency_buffers.split_off(dependency_buffer_count);
        request.dependencies =
            crate::simulation::execution::ResolvedExecutionDependencies::decode_transfer(
                &dependency_metadata,
                dependency_buffers,
            )
            .map_err(|error| error.to_string())?;
        restore_worker_request_op_previous_state(&mut request, op_previous_state, &op_buffers)?;
        Ok(request)
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn worker_request_op_config_mut(
    request: &mut WorkerRequest,
) -> Option<&mut crate::simulation::dialog::OpConfig> {
    match &mut request.request {
        WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp(config)) => Some(config),
        WorkerSimulationRequest::Spec { spec, .. } => match spec.as_mut() {
            WorkerAnalysisSpec::DcOp(config) => Some(config),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn worker_request_op_config(
    request: &WorkerRequest,
) -> Option<&crate::simulation::dialog::OpConfig> {
    match &request.request {
        WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp(config)) => Some(config),
        WorkerSimulationRequest::Spec { spec, .. } => match spec.as_ref() {
            WorkerAnalysisSpec::DcOp(config) => Some(config),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn take_worker_request_op_previous_state(
    request: &mut WorkerRequest,
) -> Result<(Option<WorkerOpPreviousStateTransport>, Vec<Vec<f64>>), String> {
    let Some(config) = worker_request_op_config_mut(request) else {
        return Ok((None, Vec::new()));
    };
    config.validate_for_execution()?;
    let Some(previous_state) = config.previous_state.take() else {
        return Ok((None, Vec::new()));
    };
    let mut buffers = Vec::with_capacity(1);
    let transport =
        WorkerOpPreviousStateTransport::from_previous_state(previous_state, &mut buffers)?;
    Ok((Some(transport), buffers))
}

#[cfg(any(target_arch = "wasm32", test))]
fn reject_inline_worker_request_op_previous_state(request: &WorkerRequest) -> Result<(), String> {
    if worker_request_op_config(request).is_some_and(|config| config.previous_state.is_some()) {
        return Err(
            "worker request metadata carries a duplicate inline OP previous-state solution"
                .to_owned(),
        );
    }
    Ok(())
}

#[cfg(any(target_arch = "wasm32", test))]
fn restore_worker_request_op_previous_state(
    request: &mut WorkerRequest,
    previous_state: Option<WorkerOpPreviousStateTransport>,
    buffers: &[Vec<f64>],
) -> Result<(), String> {
    let expected_buffers = usize::from(previous_state.is_some());
    if buffers.len() != expected_buffers {
        return Err(format!(
            "worker OP previous-state transfer carries {} buffers, expected {expected_buffers}",
            buffers.len()
        ));
    }
    let config = worker_request_op_config_mut(request);
    match (config, previous_state) {
        (Some(config), Some(previous_state)) => {
            config.previous_state = Some(previous_state.into_previous_state(buffers)?);
            config.validate_for_execution()?;
        }
        (Some(config), None) => config.validate_for_execution()?,
        (None, Some(_)) => {
            return Err(
                "worker request carries OP previous-state metadata for a non-OP analysis"
                    .to_owned(),
            );
        }
        (None, None) => {}
    }
    Ok(())
}

impl WorkerRequest {
    pub(crate) fn from_runner_parts(
        id: u64,
        request: &SimulationRequest,
        input: &NetlistInput,
    ) -> Result<Self, SimulationError> {
        Ok(Self {
            id,
            request: WorkerSimulationRequest::try_from(request)?,
            netlist: input.netlist.clone(),
            source_path: input
                .source_path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            project_veriloga_runtimes: input.project_veriloga_runtimes.clone(),
            dependencies: input.dependencies.clone(),
        })
    }

    pub(crate) fn into_runner_parts(self) -> (SimulationRequest, NetlistInput) {
        (
            SimulationRequest::from(self.request),
            NetlistInput {
                netlist: self.netlist,
                source_path: self.source_path.map(PathBuf::from),
                project_veriloga_runtimes: self.project_veriloga_runtimes,
                dependencies: self.dependencies,
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationRequest {
    Config(WorkerAnalysisConfig),
    Spec {
        spec: Box<WorkerAnalysisSpec>,
        options: Box<WorkerSpecExecutionOptions>,
    },
}


fn worker_default_pss_tone_sources() -> Vec<String> {
    vec!["VIN_DIFF".to_owned()]
}

const fn worker_default_pss_stabilization_cycles() -> usize {
    20
}

const fn worker_default_pss_shooting_points() -> usize {
    512
}

const fn worker_default_true() -> bool {
    true
}

const fn worker_default_noise_temperature() -> f64 {
    rspice_core::constants::TEMP_REFERENCE
}

fn worker_default_noise_reference_node() -> String {
    "0".to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerAnalysisSpec {
    #[serde(rename = "DcOp")]
    LegacyDcOp,
    #[serde(rename = "DcOpConfigured")]
    DcOp(crate::simulation::dialog::OpConfig),
    DcSweep {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
        source2: Option<String>,
        start2: Option<f64>,
        stop2: Option<f64>,
        step2: Option<f64>,
    },
    Transient {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: Option<f64>,
        uic: bool,
    },
    Ac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
    },
    AcData {
        table_name: String,
        frequencies: Vec<f64>,
    },
    Noise {
        output_node: String,
        #[serde(default = "worker_default_noise_reference_node")]
        reference_node: String,
        #[serde(default)]
        input_source: String,
        start_freq: f64,
        stop_freq: f64,
        points_per_decade: usize,
        #[serde(default)]
        sweep: NoiseSweepType,
        #[serde(default)]
        explicit_frequencies: Option<Vec<f64>>,
        #[serde(default)]
        data_table_name: Option<String>,
        #[serde(default)]
        contribution_detail: NoiseContributionDetail,
        #[serde(default)]
        integration_mode: NoiseIntegrationMode,
        temperature: f64,
    },
    Sensitivity {
        output_var: String,
        ac_mode: bool,
        frequency: Option<f64>,
    },
    PoleZero {
        input_node: String,
        input_ref: String,
        output_node: String,
        output_ref: String,
        transfer_type: String,
        analysis_type: String,
    },
    Tf {
        input_source: String,
        output_expression: String,
        transfer_gain: bool,
        input_resistance: bool,
        output_resistance: bool,
        normalization: TfNormalization,
        accuracy: TfAccuracy,
    },
    Pac,
    Pxf,
    Pnoise,
    Pstb,
    Parametric,
    Corner,
    MonteCarlo,
    Reliability {
        target_years: Vec<f64>,
        enable_hci: bool,
        enable_nbti: bool,
        enable_em: bool,
        min_stress_voltage: f64,
    },
    Optimization {
        variables: Vec<OptimizationVariable>,
        objective_node: String,
        objective_ref: String,
        goal: OptimizationGoal,
        target: Option<f64>,
        algorithm: OptimizationAlgorithm,
        max_iterations: usize,
        cost_tolerance: f64,
        fd_step: f64,
        initial_step: f64,
        min_step: f64,
    },
    Soa {
        stop_time: f64,
        step_time: f64,
        check_vgs_max: bool,
        max_vgs: f64,
        check_vds_max: bool,
        max_vds: f64,
        check_vbe_max: bool,
        max_vbe: f64,
        check_vce_max: bool,
        max_vce: f64,
    },
    Stb {
        probe_node: String,
        start_freq: f64,
        stop_freq: f64,
        sweep: WorkerSweepType,
        points_per_decade: usize,
    },
    SParameter {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
        z0: f64,
        ports: Vec<SpPort>,
    },
    Disto {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
        f2_over_f1: Option<f64>,
    },
    Pss {
        #[serde(default)]
        method: PssMethod,
        fundamental_freq: f64,
        #[serde(default = "worker_default_pss_tone_sources")]
        tone_sources: Vec<String>,
        #[serde(default = "worker_default_pss_stabilization_cycles")]
        tstab_periods: usize,
        #[serde(default = "worker_default_pss_shooting_points")]
        points_per_period: usize,
        #[serde(alias = "period_tolerance")]
        tolerance: f64,
        #[serde(default)]
        oscillator_mode: bool,
        #[serde(default)]
        oscillator_node: Option<String>,
        num_harmonics: usize,
    },
    HarmonicBalance {
        tones: Vec<HbToneSpec>,
        reltol: f64,
        abstol: f64,
        max_iterations: usize,
        damping: f64,
        oversample: usize,
        #[serde(default)]
        collocation_points: Option<usize>,
        max_mixing_order: usize,
        use_krylov: bool,
        gmres_restart: usize,
        source_stepping: bool,
        verbose: bool,
    },
    Envelope {
        fundamental_freq: f64,
        #[serde(default)]
        additional_carrier_tones: Vec<f64>,
        stop_time: f64,
        num_harmonics: usize,
        #[serde(default, alias = "max_step")]
        envelope_step: Option<f64>,
        #[serde(default)]
        modulation_sources: Vec<String>,
        #[serde(default)]
        initial_periodic_solve: EnvelopeInitialPeriodicSolve,
        #[serde(default)]
        adaptive_mode: EnvelopeAdaptiveMode,
        #[serde(default)]
        extraction_path: EnvelopeExtractionPath,
    },
    Fourier {
        fundamental_freq: f64,
        num_harmonics: usize,
        output_node: String,
        output_ref: String,
        start_time: f64,
        stop_time: f64,
        #[serde(default = "worker_default_true")]
        compute_thd: bool,
        #[serde(default)]
        normalize: bool,
    },
    /// Canonical manifest analysis whose typed request is transportable but
    /// whose engine capability is not present. The worker preserves the exact
    /// request and the dispatcher rejects it fail-closed.
    ManifestPreview(AnalysisSpec),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerResponse {
    pub id: u64,
    pub outcome: WorkerOutcome,
}

impl WorkerResponse {
    pub(crate) fn from_result_for_transfer(
        id: u64,
        result: Result<SimulationResult, SimulationError>,
    ) -> Self {
        let outcome = worker_transfer_outcome_from_result(result);
        Self { id, outcome }
    }

    pub(crate) fn into_result(self) -> Result<SimulationResult, SimulationError> {
        match self.outcome {
            WorkerOutcome::Success(result) => Ok(SimulationResult::from(result)),
            WorkerOutcome::Failure(error) => Err(SimulationError::from(error)),
        }
    }
}

pub(crate) fn validate_worker_response_id(
    outer_id: u64,
    response: &WorkerResponse,
) -> Result<(), SimulationError> {
    if response.id == outer_id {
        Ok(())
    } else {
        Err(SimulationError::InvalidConfig(format!(
            "simulation worker result id mismatch: outer id {outer_id}, response id {}",
            response.id
        )))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerOutcome {
    Success(WorkerSimulationResult),
    Failure(WorkerSimulationError),
}

#[cfg(test)]
fn worker_outcome_from_result(
    result: Result<SimulationResult, SimulationError>,
    payload_limit_bytes: usize,
) -> WorkerOutcome {
    match result {
        Ok(result) => match WorkerSimulationResult::try_from(result) {
            Ok(result) => {
                let payload_bytes = result.estimated_numeric_payload_bytes();
                if payload_bytes > payload_limit_bytes {
                    WorkerOutcome::Failure(worker_payload_limit_error(
                        payload_bytes,
                        payload_limit_bytes,
                    ))
                } else {
                    WorkerOutcome::Success(result)
                }
            }
            Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
        },
        Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
    }
}

fn worker_transfer_outcome_from_result(
    result: Result<SimulationResult, SimulationError>,
) -> WorkerOutcome {
    match result {
        Ok(result) => match WorkerSimulationResult::try_from(result) {
            Ok(result) => WorkerOutcome::Success(result),
            Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
        },
        Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
    }
}

#[cfg(test)]
fn worker_payload_limit_error(payload_bytes: usize, limit_bytes: usize) -> WorkerSimulationError {
    WorkerSimulationError::InvalidConfig(format!(
        "browser worker result numeric payload is {} and exceeds the current {} transport limit; reduce saved signals/points or use the native desktop runner for dense waveforms",
        format_payload_bytes(payload_bytes),
        format_payload_bytes(limit_bytes)
    ))
}

#[cfg(test)]
fn format_payload_bytes(bytes: usize) -> String {
    const KIB: usize = 1024;
    const MIB: usize = KIB * 1024;

    if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationError {
    ParseError(String),
    CircuitError(String),
    SolverError(String),
    ConvergenceFailed {
        iterations: usize,
        message: String,
    },
    Aborted,
    AlreadyRunning,
    ThreadPanic,
    InvalidConfig(String),
    ResourceLimit {
        resource: String,
        requested: usize,
        limit: usize,
    },
}

impl From<SimulationError> for WorkerSimulationError {
    fn from(value: SimulationError) -> Self {
        match value {
            SimulationError::ParseError(message) => Self::ParseError(message),
            SimulationError::CircuitError(message) => Self::CircuitError(message),
            SimulationError::SolverError(message) => Self::SolverError(message),
            SimulationError::ConvergenceFailed {
                iterations,
                message,
            } => Self::ConvergenceFailed {
                iterations,
                message,
            },
            SimulationError::Aborted => Self::Aborted,
            SimulationError::AlreadyRunning => Self::AlreadyRunning,
            SimulationError::ThreadPanic => Self::ThreadPanic,
            SimulationError::InvalidConfig(message) => Self::InvalidConfig(message),
            SimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource,
                requested,
                limit,
            },
        }
    }
}

impl From<WorkerSimulationError> for SimulationError {
    fn from(value: WorkerSimulationError) -> Self {
        match value {
            WorkerSimulationError::ParseError(message) => Self::ParseError(message),
            WorkerSimulationError::CircuitError(message) => Self::CircuitError(message),
            WorkerSimulationError::SolverError(message) => Self::SolverError(message),
            WorkerSimulationError::ConvergenceFailed {
                iterations,
                message,
            } => Self::ConvergenceFailed {
                iterations,
                message,
            },
            WorkerSimulationError::Aborted => Self::Aborted,
            WorkerSimulationError::AlreadyRunning => Self::AlreadyRunning,
            WorkerSimulationError::ThreadPanic => Self::ThreadPanic,
            WorkerSimulationError::InvalidConfig(message) => Self::InvalidConfig(message),
            WorkerSimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource,
                requested,
                limit,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerProgressSnapshot {
    pub id: u64,
    pub status: WorkerProgressStatus,
    pub progress: Option<f32>,
    pub message: Option<String>,
    pub elapsed_ms: u64,
}

impl WorkerProgressSnapshot {
    pub(crate) fn from_progress(id: u64, progress: &SimulationProgress) -> Self {
        let elapsed_ms = progress.elapsed.as_millis().min(u128::from(u64::MAX)) as u64;
        Self {
            id,
            status: WorkerProgressStatus::from(&progress.status),
            progress: progress.status.progress(),
            message: progress.message.clone(),
            elapsed_ms,
        }
    }

    pub(crate) fn apply_to(self, progress: &mut SimulationProgress) {
        progress.elapsed = std::time::Duration::from_millis(self.elapsed_ms);
        progress.update_status(SimulationStatus::from(self.status));
        progress.message = self.message;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerProgressStatus {
    Idle,
    Queued,
    Parsing,
    Building,
    DcOperatingPoint,
    DcSweep {
        source: String,
        progress: f32,
    },
    Transient {
        time: f64,
        stop_time: f64,
    },
    AcAnalysis {
        freq: f64,
        stop_freq: f64,
    },
    NoiseAnalysis {
        freq: f64,
        stop_freq: f64,
    },
    PoleZero,
    Sensitivity,
    PostProcessing,
    Completed,
    Failed {
        message: String,
    },
    Aborted,
    ConvergenceFailed {
        iteration: usize,
        time_or_freq: Option<f64>,
    },
}

impl From<&SimulationStatus> for WorkerProgressStatus {
    fn from(value: &SimulationStatus) -> Self {
        match value {
            SimulationStatus::Idle => Self::Idle,
            SimulationStatus::Queued => Self::Queued,
            SimulationStatus::Parsing => Self::Parsing,
            SimulationStatus::Building => Self::Building,
            SimulationStatus::DcOperatingPoint => Self::DcOperatingPoint,
            SimulationStatus::DcSweep { source, progress } => Self::DcSweep {
                source: source.clone(),
                progress: *progress,
            },
            SimulationStatus::Transient { time, stop_time } => Self::Transient {
                time: *time,
                stop_time: *stop_time,
            },
            SimulationStatus::AcAnalysis { freq, stop_freq } => Self::AcAnalysis {
                freq: *freq,
                stop_freq: *stop_freq,
            },
            SimulationStatus::NoiseAnalysis { freq, stop_freq } => Self::NoiseAnalysis {
                freq: *freq,
                stop_freq: *stop_freq,
            },
            SimulationStatus::PoleZero => Self::PoleZero,
            SimulationStatus::Sensitivity => Self::Sensitivity,
            SimulationStatus::PostProcessing => Self::PostProcessing,
            SimulationStatus::Completed { .. } => Self::Completed,
            SimulationStatus::Failed { message, .. } => Self::Failed {
                message: message.clone(),
            },
            SimulationStatus::Aborted { .. } => Self::Aborted,
            SimulationStatus::ConvergenceFailed {
                iteration,
                time_or_freq,
                ..
            } => Self::ConvergenceFailed {
                iteration: *iteration,
                time_or_freq: *time_or_freq,
            },
        }
    }
}

impl From<WorkerProgressStatus> for SimulationStatus {
    fn from(value: WorkerProgressStatus) -> Self {
        match value {
            WorkerProgressStatus::Idle => Self::Idle,
            WorkerProgressStatus::Queued => Self::Queued,
            WorkerProgressStatus::Parsing => Self::Parsing,
            WorkerProgressStatus::Building => Self::Building,
            WorkerProgressStatus::DcOperatingPoint => Self::DcOperatingPoint,
            WorkerProgressStatus::DcSweep { source, progress } => {
                Self::DcSweep { source, progress }
            }
            WorkerProgressStatus::Transient { time, stop_time } => {
                Self::Transient { time, stop_time }
            }
            WorkerProgressStatus::AcAnalysis { freq, stop_freq } => {
                Self::AcAnalysis { freq, stop_freq }
            }
            WorkerProgressStatus::NoiseAnalysis { freq, stop_freq } => {
                Self::NoiseAnalysis { freq, stop_freq }
            }
            WorkerProgressStatus::PoleZero => Self::PoleZero,
            WorkerProgressStatus::Sensitivity => Self::Sensitivity,
            WorkerProgressStatus::PostProcessing => Self::PostProcessing,
            WorkerProgressStatus::Completed => Self::Completed {
                elapsed: std::time::Duration::ZERO,
            },
            WorkerProgressStatus::Failed { message } => Self::Failed {
                message,
                elapsed: std::time::Duration::ZERO,
            },
            WorkerProgressStatus::Aborted => Self::Aborted {
                elapsed: std::time::Duration::ZERO,
            },
            WorkerProgressStatus::ConvergenceFailed {
                iteration,
                time_or_freq,
            } => Self::ConvergenceFailed {
                iteration,
                time_or_freq,
                elapsed: std::time::Duration::ZERO,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationResult {
    DcOp {
        configuration: crate::simulation::dialog::OpConfig,
        validated_startup_directives: usize,
        #[serde(default)]
        mna_node_names: Vec<String>,
        #[serde(default)]
        mna_branch_names: Vec<String>,
        #[serde(default)]
        mna_solution: Vec<f64>,
        node_voltages: HashMap<String, f64>,
        branch_currents: HashMap<String, f64>,
        device_ops: Vec<WorkerDeviceOpPoint>,
        device_report: Option<WorkerDeviceOpReport>,
    },
    DcSweep {
        sweep_var: String,
        sweep_values: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
    },
    Transient {
        time: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
    },
    /// PSS numerical evidence is transported once. Display waveforms are
    /// deterministically reconstructed from this retained orbit by the
    /// receiver instead of duplicating every sample across the worker edge.
    Pss {
        measurements: Vec<WorkerMeasurement>,
        operating_point: rspice_core::engine::PssOperatingPoint,
    },
    Ac {
        frequencies: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
    },
    Noise {
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
        #[serde(default)]
        summary: Option<WorkerNoiseSummary>,
    },
    PoleZero {
        poles: Vec<(f64, f64)>,
        zeros: Vec<(f64, f64)>,
        gain: f64,
    },
    Sensitivity {
        output: String,
        ac_mode: bool,
        frequency_hz: Option<f64>,
        sensitivities: HashMap<String, f64>,
        normalized: HashMap<String, f64>,
    },
    TransferFunction {
        input_source: String,
        output_expression: String,
        input_quantity: WorkerTransferFunctionQuantity,
        output_quantity: WorkerTransferFunctionQuantity,
        input_unit: String,
        output_unit: String,
        gain_unit: String,
        normalization: TfNormalization,
        accuracy: TfAccuracy,
        gain: Option<WorkerTransferFunctionScalar>,
        input_resistance: Option<WorkerTransferFunctionScalar>,
        output_resistance: Option<WorkerTransferFunctionScalar>,
        nominal_input: Option<f64>,
        nominal_output: Option<f64>,
    },
    Parametric {
        target: String,
        sweep_values: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        num_failures: usize,
    },
    Corner {
        x_values: Vec<f64>,
        x_label: String,
        x_unit: String,
        temperatures_c: Vec<f64>,
        corner_labels: Vec<String>,
        waveforms: Vec<WorkerWaveform>,
        num_failures: usize,
    },
    MonteCarlo {
        seed: u64,
        runs_requested: usize,
        runs_completed: usize,
        num_failures: usize,
        all_converged: bool,
        variables: Vec<WorkerMonteCarloVariable>,
    },
    Reliability {
        years: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        device_results: Vec<WorkerReliabilityResult>,
    },
    Optimization {
        iterations: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        best_cost: f64,
        best_variables: HashMap<String, f64>,
        converged: bool,
    },
    Soa {
        time: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        violations: Vec<WorkerSoAViolation>,
        evaluations: Vec<WorkerSoAEvaluation>,
    },
    MeasurementsOnly {
        measurements: HashMap<String, f64>,
    },
}

#[cfg(test)]
impl WorkerSimulationResult {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        match self {
            WorkerSimulationResult::DcOp {
                configuration: _,
                validated_startup_directives: _,
                mna_node_names: _,
                mna_branch_names: _,
                mna_solution,
                node_voltages,
                branch_currents,
                device_ops,
                device_report,
            } => sum_payload_bytes([
                f64_payload_bytes(node_voltages.len()),
                f64_payload_bytes(branch_currents.len()),
                f64_payload_bytes(mna_solution.len()),
                device_ops_payload_bytes(device_ops),
                device_report
                    .as_ref()
                    .map_or(0, WorkerDeviceOpReport::estimated_numeric_payload_bytes),
            ]),
            WorkerSimulationResult::DcSweep {
                sweep_values,
                waveforms,
                measurements,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(sweep_values.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
            ]),
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
            } => sum_payload_bytes([
                f64_payload_bytes(time.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
            ]),
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => sum_payload_bytes([
                measurements_payload_bytes(measurements),
                pss_operating_point_payload_bytes(operating_point),
            ]),
            WorkerSimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
            ]),
            WorkerSimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                f64_payload_bytes(output_noise.len()),
                input_noise
                    .as_ref()
                    .map_or(0, |values| f64_payload_bytes(values.len())),
                vec_map_payload_bytes(contributors),
                summary
                    .as_ref()
                    .map_or(0, WorkerNoiseSummary::estimated_numeric_payload_bytes),
            ]),
            WorkerSimulationResult::PoleZero { poles, zeros, .. } => sum_payload_bytes([
                complex_pair_payload_bytes(poles.len()),
                complex_pair_payload_bytes(zeros.len()),
                f64_payload_bytes(1),
            ]),
            WorkerSimulationResult::Sensitivity {
                frequency_hz,
                sensitivities,
                normalized,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(usize::from(frequency_hz.is_some())),
                f64_payload_bytes(sensitivities.len()),
                f64_payload_bytes(normalized.len()),
            ]),
            WorkerSimulationResult::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
                ..
            } => f64_payload_bytes(
                [
                    gain.is_some(),
                    input_resistance.is_some(),
                    output_resistance.is_some(),
                ]
                .into_iter()
                .filter(|present| *present)
                .count()
                    + usize::from(nominal_input.is_some())
                    + usize::from(nominal_output.is_some()),
            ),
            WorkerSimulationResult::Parametric {
                sweep_values,
                waveforms,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(sweep_values.len()),
                waveforms_payload_bytes(waveforms),
            ]),
            WorkerSimulationResult::Corner {
                x_values,
                temperatures_c,
                waveforms,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(x_values.len()),
                f64_payload_bytes(temperatures_c.len()),
                waveforms_payload_bytes(waveforms),
            ]),
            WorkerSimulationResult::MonteCarlo { variables, .. } => variables
                .iter()
                .map(WorkerMonteCarloVariable::estimated_numeric_payload_bytes)
                .fold(0usize, |total, bytes| total.saturating_add(bytes)),
            WorkerSimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => sum_payload_bytes([
                f64_payload_bytes(years.len()),
                waveforms_payload_bytes(waveforms),
                reliability_results_payload_bytes(device_results),
            ]),
            WorkerSimulationResult::Optimization {
                iterations,
                waveforms,
                best_variables,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(iterations.len()),
                waveforms_payload_bytes(waveforms),
                f64_payload_bytes(best_variables.len()),
                f64_payload_bytes(1),
            ]),
            WorkerSimulationResult::Soa {
                time,
                waveforms,
                violations,
                evaluations,
            } => sum_payload_bytes([
                f64_payload_bytes(time.len()),
                waveforms_payload_bytes(waveforms),
                soa_violations_payload_bytes(violations),
                soa_evaluations_payload_bytes(evaluations),
            ]),
            WorkerSimulationResult::MeasurementsOnly { measurements } => {
                f64_payload_bytes(measurements.len())
            }
        }
    }
}

const WORKER_RESPONSE_TRANSPORT_PROTOCOL: u8 = 7;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WorkerResponseTransport {
    pub protocol: u8,
    pub response: WorkerResponseTransportMetadata,
    pub buffers: Vec<Vec<f64>>,
}

impl TryFrom<SimulationResult> for WorkerSimulationResult {
    type Error = SimulationError;

    fn try_from(value: SimulationResult) -> Result<Self, Self::Error> {
        match value {
            SimulationResult::DcOp(result) => Ok(Self::DcOp {
                configuration: result.configuration,
                validated_startup_directives: result.validated_startup_directives,
                mna_node_names: result.mna_node_names,
                mna_branch_names: result.mna_branch_names,
                mna_solution: result.mna_solution,
                node_voltages: result.node_voltages,
                branch_currents: result.branch_currents,
                device_ops: result
                    .device_ops
                    .into_iter()
                    .map(WorkerDeviceOpPoint::from)
                    .collect(),
                device_report: result.device_report.map(WorkerDeviceOpReport::from),
            }),
            SimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Ok(Self::DcSweep {
                sweep_var,
                sweep_values,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
            }),
            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
                periodic_state,
            } => match periodic_state {
                Some(operating_point) => {
                    validate_pss_display_contract(&time, &waveforms, &operating_point)?;
                    Ok(Self::Pss {
                        measurements: worker_measurements(measurements),
                        operating_point: std::sync::Arc::unwrap_or_clone(operating_point),
                    })
                }
                None => Ok(Self::Transient {
                    time,
                    waveforms: worker_waveforms(waveforms),
                    measurements: worker_measurements(measurements),
                }),
            },
            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Ok(Self::Ac {
                frequencies,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
            }),
            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => Ok(Self::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary: summary.map(WorkerNoiseSummary::from),
            }),
            SimulationResult::PoleZero { poles, zeros, gain } => {
                Ok(Self::PoleZero { poles, zeros, gain })
            }
            SimulationResult::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
            } => Ok(Self::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
            }),
            SimulationResult::TransferFunction {
                input_source,
                output_expression,
                input_quantity,
                output_quantity,
                input_unit,
                output_unit,
                gain_unit,
                normalization,
                accuracy,
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
            } => Ok(Self::TransferFunction {
                input_source,
                output_expression,
                input_quantity: WorkerTransferFunctionQuantity::from(input_quantity),
                output_quantity: WorkerTransferFunctionQuantity::from(output_quantity),
                input_unit,
                output_unit,
                gain_unit,
                normalization,
                accuracy,
                gain: gain.map(WorkerTransferFunctionScalar::from),
                input_resistance: input_resistance.map(WorkerTransferFunctionScalar::from),
                output_resistance: output_resistance.map(WorkerTransferFunctionScalar::from),
                nominal_input,
                nominal_output,
            }),
            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => Ok(Self::Parametric {
                target,
                sweep_values,
                waveforms: worker_waveforms(waveforms),
                num_failures,
            }),
            SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
            } => Ok(Self::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms: worker_waveforms(waveforms),
                num_failures,
            }),
            SimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
            } => Ok(Self::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables: variables
                    .into_iter()
                    .map(WorkerMonteCarloVariable::from)
                    .collect(),
            }),
            SimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => Ok(Self::Reliability {
                years,
                waveforms: worker_waveforms(waveforms),
                device_results: device_results
                    .into_iter()
                    .map(WorkerReliabilityResult::from)
                    .collect(),
            }),
            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Ok(Self::Optimization {
                iterations,
                waveforms: worker_waveforms(waveforms),
                best_cost,
                best_variables,
                converged,
            }),
            SimulationResult::Soa {
                time,
                waveforms,
                violations,
                evaluations,
            } => Ok(Self::Soa {
                time,
                waveforms: worker_waveforms(waveforms),
                violations: violations
                    .into_iter()
                    .map(WorkerSoAViolation::from)
                    .collect(),
                evaluations: evaluations
                    .into_iter()
                    .map(WorkerSoAEvaluation::from)
                    .collect(),
            }),
            SimulationResult::MeasurementsOnly { measurements } => {
                Ok(Self::MeasurementsOnly { measurements })
            }
        }
    }
}

impl From<WorkerSimulationResult> for SimulationResult {
    fn from(value: WorkerSimulationResult) -> Self {
        match value {
            WorkerSimulationResult::DcOp {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                node_voltages,
                branch_currents,
                device_ops,
                device_report,
            } => Self::DcOp(DcOpResult {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                node_voltages,
                branch_currents,
                device_ops: device_ops
                    .into_iter()
                    .map(|device| (device.name.clone(), DeviceOpPoint::from(device)))
                    .collect(),
                device_report: device_report.map(rspice_core::circuit::DeviceOpReport::from),
            }),
            WorkerSimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Self::DcSweep {
                sweep_var,
                sweep_values,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
            },
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
            } => Self::Transient {
                time,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
                periodic_state: None,
            },
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => simulation_result_from_worker_pss(measurements, operating_point),
            WorkerSimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Self::Ac {
                frequencies,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
            },
            WorkerSimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => Self::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary: summary.map(NoiseSummary::from),
            },
            WorkerSimulationResult::PoleZero { poles, zeros, gain } => {
                Self::PoleZero { poles, zeros, gain }
            }
            WorkerSimulationResult::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
            } => Self::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
            },
            WorkerSimulationResult::TransferFunction {
                input_source,
                output_expression,
                input_quantity,
                output_quantity,
                input_unit,
                output_unit,
                gain_unit,
                normalization,
                accuracy,
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
            } => Self::TransferFunction {
                input_source,
                output_expression,
                input_quantity: TransferFunctionQuantity::from(input_quantity),
                output_quantity: TransferFunctionQuantity::from(output_quantity),
                input_unit,
                output_unit,
                gain_unit,
                normalization,
                accuracy,
                gain: gain.map(TransferFunctionScalar::from),
                input_resistance: input_resistance.map(TransferFunctionScalar::from),
                output_resistance: output_resistance.map(TransferFunctionScalar::from),
                nominal_input,
                nominal_output,
            },
            WorkerSimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => Self::Parametric {
                target,
                sweep_values,
                waveforms: waveform_map(waveforms),
                num_failures,
            },
            WorkerSimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
            } => Self::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms: waveform_map(waveforms),
                num_failures,
            },
            WorkerSimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
            } => Self::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables: variables
                    .into_iter()
                    .map(MonteCarloVariableResult::from)
                    .collect(),
            },
            WorkerSimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => Self::Reliability {
                years,
                waveforms: waveform_map(waveforms),
                device_results: device_results
                    .into_iter()
                    .map(ReliabilityResult::from)
                    .collect(),
            },
            WorkerSimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Self::Optimization {
                iterations,
                waveforms: waveform_map(waveforms),
                best_cost,
                best_variables,
                converged,
            },
            WorkerSimulationResult::Soa {
                time,
                waveforms,
                violations,
                evaluations,
            } => Self::Soa {
                time,
                waveforms: waveform_map(waveforms),
                violations: violations.into_iter().map(SoAViolation::from).collect(),
                evaluations: evaluations.into_iter().map(SoAEvaluation::from).collect(),
            },
            WorkerSimulationResult::MeasurementsOnly { measurements } => {
                Self::MeasurementsOnly { measurements }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerTransferFunctionQuantity {
    Voltage,
    Current,
}

impl From<TransferFunctionQuantity> for WorkerTransferFunctionQuantity {
    fn from(value: TransferFunctionQuantity) -> Self {
        match value {
            TransferFunctionQuantity::Voltage => Self::Voltage,
            TransferFunctionQuantity::Current => Self::Current,
        }
    }
}

impl From<WorkerTransferFunctionQuantity> for TransferFunctionQuantity {
    fn from(value: WorkerTransferFunctionQuantity) -> Self {
        match value {
            WorkerTransferFunctionQuantity::Voltage => Self::Voltage,
            WorkerTransferFunctionQuantity::Current => Self::Current,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerTransferFunctionScalar {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl From<TransferFunctionScalar> for WorkerTransferFunctionScalar {
    fn from(value: TransferFunctionScalar) -> Self {
        match value {
            TransferFunctionScalar::Finite(value) => Self::Finite(value),
            TransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            TransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

impl From<WorkerTransferFunctionScalar> for TransferFunctionScalar {
    fn from(value: WorkerTransferFunctionScalar) -> Self {
        match value {
            WorkerTransferFunctionScalar::Finite(value) => Self::Finite(value),
            WorkerTransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            WorkerTransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNoiseSummary {
    pub rows: Vec<WorkerNoiseContributorRow>,
    #[serde(default)]
    pub total_rms: Option<f64>,
    #[serde(default)]
    pub input_rms: Option<f64>,
    pub band: (f64, f64),
}

#[cfg(test)]
impl WorkerNoiseSummary {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            self.rows
                .iter()
                .map(WorkerNoiseContributorRow::estimated_numeric_payload_bytes)
                .fold(0usize, |total, bytes| total.saturating_add(bytes)),
            f64_payload_bytes(3),
        ])
    }
}

impl From<NoiseSummary> for WorkerNoiseSummary {
    fn from(value: NoiseSummary) -> Self {
        Self {
            rows: value
                .rows
                .into_iter()
                .map(WorkerNoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

impl From<WorkerNoiseSummary> for NoiseSummary {
    fn from(value: WorkerNoiseSummary) -> Self {
        Self {
            rows: value
                .rows
                .into_iter()
                .map(NoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

#[cfg(test)]
impl WorkerNoiseContributorRow {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(2)
    }
}

impl From<NoiseContributorRow> for WorkerNoiseContributorRow {
    fn from(value: NoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism.to_string(),
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}

impl From<WorkerNoiseContributorRow> for NoiseContributorRow {
    fn from(value: WorkerNoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism,
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerMonteCarloVariable {
    pub name: String,
    pub samples: Vec<f64>,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub histogram: Vec<usize>,
    pub bin_edges: Vec<f64>,
}

#[cfg(test)]
impl WorkerMonteCarloVariable {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            f64_payload_bytes(4usize.saturating_add(self.samples.len())),
            usize_payload_bytes(self.histogram.len()),
            f64_payload_bytes(self.bin_edges.len()),
        ])
    }
}

impl From<MonteCarloVariableResult> for WorkerMonteCarloVariable {
    fn from(value: MonteCarloVariableResult) -> Self {
        Self {
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

impl From<WorkerMonteCarloVariable> for MonteCarloVariableResult {
    fn from(value: WorkerMonteCarloVariable) -> Self {
        Self {
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerReliabilityResult {
    pub device_id: String,
    pub stress: WorkerStressMetrics,
    pub shifts: HashMap<String, WorkerParamShift>,
}

#[cfg(test)]
impl WorkerReliabilityResult {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            self.stress.estimated_numeric_payload_bytes(),
            self.shifts
                .values()
                .map(WorkerParamShift::estimated_numeric_payload_bytes)
                .fold(0usize, |total, bytes| total.saturating_add(bytes)),
        ])
    }
}

impl From<ReliabilityResult> for WorkerReliabilityResult {
    fn from(value: ReliabilityResult) -> Self {
        Self {
            device_id: value.device_id,
            stress: WorkerStressMetrics::from(value.stress),
            shifts: value
                .shifts
                .into_iter()
                .map(|(label, shift)| (label, WorkerParamShift::from(shift)))
                .collect(),
        }
    }
}

impl From<WorkerReliabilityResult> for ReliabilityResult {
    fn from(value: WorkerReliabilityResult) -> Self {
        Self {
            device_id: value.device_id,
            stress: StressMetrics::from(value.stress),
            shifts: value
                .shifts
                .into_iter()
                .map(|(label, shift)| (label, ParamShift::from(shift)))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerStressMetrics {
    pub avg_vgs_stress: f64,
    pub avg_vds_stress: f64,
    pub avg_temp: f64,
    pub duration: f64,
}

#[cfg(test)]
impl WorkerStressMetrics {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(4)
    }
}

impl From<StressMetrics> for WorkerStressMetrics {
    fn from(value: StressMetrics) -> Self {
        Self {
            avg_vgs_stress: value.avg_vgs_stress,
            avg_vds_stress: value.avg_vds_stress,
            avg_temp: value.avg_temp,
            duration: value.duration,
        }
    }
}

impl From<WorkerStressMetrics> for StressMetrics {
    fn from(value: WorkerStressMetrics) -> Self {
        Self {
            avg_vgs_stress: value.avg_vgs_stress,
            avg_vds_stress: value.avg_vds_stress,
            avg_temp: value.avg_temp,
            duration: value.duration,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerParamShift {
    pub vth_shift: f64,
    pub mobility_shift: f64,
    pub rds_shift: f64,
}

#[cfg(test)]
impl WorkerParamShift {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(3)
    }
}

impl From<ParamShift> for WorkerParamShift {
    fn from(value: ParamShift) -> Self {
        Self {
            vth_shift: value.vth_shift,
            mobility_shift: value.mobility_shift,
            rds_shift: value.rds_shift,
        }
    }
}

impl From<WorkerParamShift> for ParamShift {
    fn from(value: WorkerParamShift) -> Self {
        Self {
            vth_shift: value.vth_shift,
            mobility_shift: value.mobility_shift,
            rds_shift: value.rds_shift,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerSoAEvaluation {
    pub device_id: String,
    pub parameter: WorkerSoAParameter,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: WorkerSoARuleVerdict,
}

#[cfg(test)]
impl WorkerSoAEvaluation {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(3).saturating_add(std::mem::size_of::<u64>())
    }
}

impl From<SoAEvaluation> for WorkerSoAEvaluation {
    fn from(value: SoAEvaluation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: WorkerSoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            worst_actual_value: value.worst_actual_value,
            worst_time: value.worst_time,
            sample_count: value.sample_count,
            unit: value.unit,
            description: value.description,
            verdict: WorkerSoARuleVerdict::from(value.verdict),
        }
    }
}

impl From<WorkerSoAEvaluation> for SoAEvaluation {
    fn from(value: WorkerSoAEvaluation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: SoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            worst_actual_value: value.worst_actual_value,
            worst_time: value.worst_time,
            sample_count: value.sample_count,
            unit: value.unit,
            description: value.description,
            verdict: SoARuleVerdict::from(value.verdict),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerSoARuleVerdict {
    Pass,
    Warning,
    Violation,
    Critical,
}

impl From<SoARuleVerdict> for WorkerSoARuleVerdict {
    fn from(value: SoARuleVerdict) -> Self {
        match value {
            SoARuleVerdict::Pass => Self::Pass,
            SoARuleVerdict::Warning => Self::Warning,
            SoARuleVerdict::Violation => Self::Violation,
            SoARuleVerdict::Critical => Self::Critical,
        }
    }
}

impl From<WorkerSoARuleVerdict> for SoARuleVerdict {
    fn from(value: WorkerSoARuleVerdict) -> Self {
        match value {
            WorkerSoARuleVerdict::Pass => Self::Pass,
            WorkerSoARuleVerdict::Warning => Self::Warning,
            WorkerSoARuleVerdict::Violation => Self::Violation,
            WorkerSoARuleVerdict::Critical => Self::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerSoAViolation {
    pub device_id: String,
    pub parameter: WorkerSoAParameter,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time: f64,
    pub severity: WorkerViolationSeverity,
}

#[cfg(test)]
impl WorkerSoAViolation {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(3)
    }
}

impl From<SoAViolation> for WorkerSoAViolation {
    fn from(value: SoAViolation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: WorkerSoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            actual_value: value.actual_value,
            time: value.time,
            severity: WorkerViolationSeverity::from(value.severity),
        }
    }
}

impl From<WorkerSoAViolation> for SoAViolation {
    fn from(value: WorkerSoAViolation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: SoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            actual_value: value.actual_value,
            time: value.time,
            severity: ViolationSeverity::from(value.severity),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerSoAParameter {
    Vgs,
    Vds,
    Vgd,
    Vbe,
    Vce,
    Vbc,
    Id,
    Ic,
    Pdiss,
    Temp,
}

impl From<SoAParameter> for WorkerSoAParameter {
    fn from(value: SoAParameter) -> Self {
        match value {
            SoAParameter::Vgs => Self::Vgs,
            SoAParameter::Vds => Self::Vds,
            SoAParameter::Vgd => Self::Vgd,
            SoAParameter::Vbe => Self::Vbe,
            SoAParameter::Vce => Self::Vce,
            SoAParameter::Vbc => Self::Vbc,
            SoAParameter::Id => Self::Id,
            SoAParameter::Ic => Self::Ic,
            SoAParameter::Pdiss => Self::Pdiss,
            SoAParameter::Temp => Self::Temp,
        }
    }
}

impl From<WorkerSoAParameter> for SoAParameter {
    fn from(value: WorkerSoAParameter) -> Self {
        match value {
            WorkerSoAParameter::Vgs => Self::Vgs,
            WorkerSoAParameter::Vds => Self::Vds,
            WorkerSoAParameter::Vgd => Self::Vgd,
            WorkerSoAParameter::Vbe => Self::Vbe,
            WorkerSoAParameter::Vce => Self::Vce,
            WorkerSoAParameter::Vbc => Self::Vbc,
            WorkerSoAParameter::Id => Self::Id,
            WorkerSoAParameter::Ic => Self::Ic,
            WorkerSoAParameter::Pdiss => Self::Pdiss,
            WorkerSoAParameter::Temp => Self::Temp,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerViolationSeverity {
    Warning,
    Violation,
    Critical,
}

impl From<ViolationSeverity> for WorkerViolationSeverity {
    fn from(value: ViolationSeverity) -> Self {
        match value {
            ViolationSeverity::Warning => Self::Warning,
            ViolationSeverity::Violation => Self::Violation,
            ViolationSeverity::Critical => Self::Critical,
        }
    }
}

impl From<WorkerViolationSeverity> for ViolationSeverity {
    fn from(value: WorkerViolationSeverity) -> Self {
        match value {
            WorkerViolationSeverity::Warning => Self::Warning,
            WorkerViolationSeverity::Violation => Self::Violation,
            WorkerViolationSeverity::Critical => Self::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerWaveform {
    pub name: String,
    pub x_values: Vec<f64>,
    pub y_values: Vec<f64>,
    pub y_unit: String,
    pub x_unit: String,
    pub is_complex: bool,
    pub y_imag: Option<Vec<f64>>,
}

#[cfg(test)]
impl WorkerWaveform {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            f64_payload_bytes(self.x_values.len()),
            f64_payload_bytes(self.y_values.len()),
            self.y_imag
                .as_ref()
                .map_or(0, |values| f64_payload_bytes(values.len())),
        ])
    }
}

impl From<WaveformData> for WorkerWaveform {
    fn from(value: WaveformData) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            x_unit: value.x_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}

impl From<WorkerWaveform> for WaveformData {
    fn from(value: WorkerWaveform) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            x_unit: value.x_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerMeasurement {
    pub name: String,
    pub value: Option<f64>,
    pub error: Option<String>,
    pub passed: bool,
    pub expected: Option<f64>,
    pub tolerance: Option<f64>,
    pub event_axis: Option<f64>,
}

#[cfg(test)]
impl WorkerMeasurement {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(
            usize::from(self.value.is_some())
                + usize::from(self.expected.is_some())
                + usize::from(self.tolerance.is_some())
                + usize::from(self.event_axis.is_some()),
        )
    }
}

impl From<rspice_core::MeasureResult> for WorkerMeasurement {
    fn from(value: rspice_core::MeasureResult) -> Self {
        Self {
            name: value.name,
            value: value.value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            event_axis: value.event_axis,
        }
    }
}

impl From<WorkerMeasurement> for rspice_core::MeasureResult {
    fn from(value: WorkerMeasurement) -> Self {
        Self {
            name: value.name,
            value: value.value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            event_axis: value.event_axis,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpPoint {
    pub name: String,
    pub device_type: String,
    pub parameters: HashMap<String, f64>,
}

#[cfg(test)]
impl WorkerDeviceOpPoint {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(self.parameters.len())
    }
}

impl From<(String, DeviceOpPoint)> for WorkerDeviceOpPoint {
    fn from((name, value): (String, DeviceOpPoint)) -> Self {
        Self {
            name,
            device_type: value.device_type,
            parameters: value.parameters,
        }
    }
}

impl From<WorkerDeviceOpPoint> for DeviceOpPoint {
    fn from(value: WorkerDeviceOpPoint) -> Self {
        Self {
            device_type: value.device_type,
            parameters: value.parameters,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpReport {
    pub entries: Vec<WorkerDeviceOpEntry>,
}

#[cfg(test)]
impl WorkerDeviceOpReport {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        self.entries
            .iter()
            .map(WorkerDeviceOpEntry::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpReport> for WorkerDeviceOpReport {
    fn from(value: rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(WorkerDeviceOpEntry::from)
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpReport> for rspice_core::circuit::DeviceOpReport {
    fn from(value: WorkerDeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(rspice_core::circuit::DeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    pub region: Option<String>,
    pub params: Vec<WorkerNamedValue>,
}

#[cfg(test)]
impl WorkerDeviceOpEntry {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        self.params
            .iter()
            .map(WorkerNamedValue::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpEntry> for WorkerDeviceOpEntry {
    fn from(value: rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: value.device_kind.to_string(),
            region: value.region.map(str::to_string),
            params: value
                .params
                .into_iter()
                .map(|(name, value)| WorkerNamedValue {
                    name: name.to_string(),
                    value,
                })
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpEntry> for rspice_core::circuit::DeviceOpEntry {
    fn from(value: WorkerDeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: intern_static_label(value.device_kind),
            region: value.region.map(intern_static_label),
            params: value
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNamedValue {
    pub name: String,
    pub value: f64,
}

#[cfg(test)]
impl WorkerNamedValue {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(1)
    }
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn worker_response_from_request(request: WorkerRequest) -> WorkerResponse {
    worker_response_from_request_with_progress(request, None)
}

fn worker_response_from_request_with_progress(
    request: WorkerRequest,
    progress_observer: Option<super::ProgressObserver>,
) -> WorkerResponse {
    let id = request.id;
    let (request, input) = request.into_runner_parts();
    let progress = Arc::new(Mutex::new(SimulationProgress::default()));
    let abort_flag = Arc::new(AtomicBool::new(false));

    WorkerResponse::from_result_for_transfer(
        id,
        super::run_simulation_thread_with_progress_observer(
            request,
            input,
            progress,
            abort_flag,
            progress_observer,
        ),
    )
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ACTIVE_WORKER_PROGRESS_ID: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn emit_worker_progress_snapshot(progress: &SimulationProgress) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };

    let snapshot = WorkerProgressSnapshot::from_progress(id, progress);
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("progress"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(snapshot) = serde_wasm_bindgen::to_value(&snapshot) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("progress"), &snapshot);
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = worker_request_from_value(value)?;
    let id = request.id;
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(Some(id)));
    let response =
        worker_response_from_request_with_progress(request, Some(emit_worker_progress_snapshot));
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(None));
    worker_response_transport_value(response)
}

#[cfg(target_arch = "wasm32")]
fn worker_request_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerRequest, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .map_err(worker_request_js_error)?
        .as_f64()
        .and_then(|value| {
            (value.fract() == 0.0 && (0.0..=f64::from(u8::MAX)).contains(&value))
                .then_some(value as u8)
        })
        .ok_or_else(|| {
            JsValue::from_str("worker request transport protocolVersion must be an unsigned byte")
        })?;

    let request = js_sys::Reflect::get(&value, &JsValue::from_str("request"))
        .map_err(worker_request_js_error)?;
    let request = serde_wasm_bindgen::from_value::<WorkerRequestTransportMetadata>(request)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_request_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| JsValue::from_str("worker request transport buffers must be an array"))?;
    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(JsValue::from_str(&format!(
            "worker request contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = checked_worker_request_numeric_total(
            numeric_values,
            index as usize,
            view.length() as usize,
        )
        .map_err(|error| JsValue::from_str(&error))?;
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerRequestTransport {
        protocol,
        request,
        buffers: decoded_buffers,
    }
    .into_request()
    .map_err(|error| JsValue::from_str(&error))
}

#[cfg(target_arch = "wasm32")]
fn worker_request_js_error(error: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_str(&worker_js_error(error).to_string())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_transport_value(
    response: WorkerResponse,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsValue;

    let transport = WorkerResponseTransport::from_response(response)
        .map_err(|error| JsValue::from_str(&error))?;
    let message = js_sys::Object::new();
    js_sys::Reflect::set(
        &message,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(transport.protocol)),
    )?;
    let response = serde_wasm_bindgen::to_value(&transport.response)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    js_sys::Reflect::set(&message, &JsValue::from_str("response"), &response)?;

    let buffers = js_sys::Array::new();
    for values in transport.buffers {
        let view = js_sys::Float64Array::new_with_length(values.len() as u32);
        view.copy_from(&values);
        buffers.push(&view);
    }
    js_sys::Reflect::set(&message, &JsValue::from_str("buffers"), &buffers)?;

    Ok(JsValue::from(message))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerResponse, SimulationError> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .ok()
        .and_then(|value| value.as_f64())
        .map(|value| value as u8);

    if protocol != Some(WORKER_RESPONSE_TRANSPORT_PROTOCOL) {
        return serde_wasm_bindgen::from_value::<WorkerResponse>(value)
            .map_err(|error| SimulationError::InvalidConfig(error.to_string()));
    }

    let response =
        js_sys::Reflect::get(&value, &JsValue::from_str("response")).map_err(worker_js_error)?;
    let response = serde_wasm_bindgen::from_value::<WorkerResponseTransportMetadata>(response)
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| {
            SimulationError::InvalidConfig(
                "worker response transport buffers must be an array".to_string(),
            )
        })?;

    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(SimulationError::InvalidConfig(format!(
            "worker response contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = numeric_values
            .checked_add(view.length() as usize)
            .ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "worker response numeric size overflows this platform".to_owned(),
                )
            })?;
        if numeric_values > MAX_WORKER_F64_VALUES {
            return Err(SimulationError::InvalidConfig(format!(
                "worker response contains more than {MAX_WORKER_F64_VALUES} numerical values"
            )));
        }
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerResponseTransport {
        protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
        response,
        buffers: decoded_buffers,
    }
    .into_response()
    .map_err(SimulationError::InvalidConfig)
}

#[cfg(target_arch = "wasm32")]
fn worker_js_error(error: wasm_bindgen::JsValue) -> SimulationError {
    use wasm_bindgen::JsValue;

    let message = error
        .as_string()
        .or_else(|| {
            js_sys::Reflect::get(&error, &JsValue::from_str("message"))
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "unknown JavaScript error".to_string());
    SimulationError::InvalidConfig(message)
}

#[cfg(test)]
fn sum_payload_bytes(bytes: impl IntoIterator<Item = usize>) -> usize {
    bytes
        .into_iter()
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
fn f64_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
fn pss_operating_point_payload_bytes(
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> usize {
    let analysis = operating_point.analysis();
    let values = analysis
        .result
        .time
        .len()
        .saturating_add(
            analysis
                .result
                .waveforms
                .iter()
                .map(|waveform| waveform.values.len())
                .sum::<usize>(),
        )
        .saturating_add(analysis.monodromy.iter().map(Vec::len).sum::<usize>())
        .saturating_add(analysis.result.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(analysis.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(operating_point.shooting_state().len());
    f64_payload_bytes(values)
}

#[cfg(test)]
fn usize_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<usize>())
}

#[cfg(test)]
fn complex_pair_payload_bytes(len: usize) -> usize {
    len.saturating_mul(2)
        .saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
fn waveforms_payload_bytes(waveforms: &[WorkerWaveform]) -> usize {
    waveforms
        .iter()
        .map(WorkerWaveform::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
fn measurements_payload_bytes(measurements: &[WorkerMeasurement]) -> usize {
    measurements
        .iter()
        .map(WorkerMeasurement::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
fn vec_map_payload_bytes(values_by_name: &HashMap<String, Vec<f64>>) -> usize {
    values_by_name
        .values()
        .map(|values| f64_payload_bytes(values.len()))
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
fn device_ops_payload_bytes(device_ops: &[WorkerDeviceOpPoint]) -> usize {
    device_ops
        .iter()
        .map(WorkerDeviceOpPoint::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
fn reliability_results_payload_bytes(results: &[WorkerReliabilityResult]) -> usize {
    results
        .iter()
        .map(WorkerReliabilityResult::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
fn soa_violations_payload_bytes(violations: &[WorkerSoAViolation]) -> usize {
    violations
        .iter()
        .map(WorkerSoAViolation::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
fn soa_evaluations_payload_bytes(evaluations: &[WorkerSoAEvaluation]) -> usize {
    evaluations
        .iter()
        .map(WorkerSoAEvaluation::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

fn worker_waveforms(waveforms: HashMap<String, WaveformData>) -> Vec<WorkerWaveform> {
    let mut waveforms: Vec<_> = waveforms.into_values().map(WorkerWaveform::from).collect();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));
    waveforms
}

fn validate_pss_display_contract(
    time: &[f64],
    waveforms: &HashMap<String, WaveformData>,
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> Result<(), SimulationError> {
    let result = &operating_point.analysis().result;
    if time != result.time.as_slice() {
        return Err(SimulationError::InvalidConfig(
            "PSS display time axis does not match its retained numerical orbit".to_owned(),
        ));
    }
    let expected_count = result
        .node_names
        .iter()
        .filter(|name| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .count();
    if waveforms.len() != expected_count {
        return Err(SimulationError::InvalidConfig(format!(
            "PSS display contains {} waveforms, but its retained orbit requires {expected_count}",
            waveforms.len()
        )));
    }
    for (name, periodic) in result.node_names.iter().zip(&result.waveforms) {
        if name == "0" || name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        let display_name = format!("V({name})");
        let display = waveforms.get(&display_name).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "PSS display is missing retained-orbit waveform '{display_name}'"
            ))
        })?;
        if display.name != display_name
            || display.x_values.as_slice() != result.time.as_slice()
            || display.y_values.as_slice() != periodic.values.as_slice()
            || display.x_unit != "s"
            || display.y_unit != "V"
            || display.is_complex
            || display.y_imag.is_some()
        {
            return Err(SimulationError::InvalidConfig(format!(
                "PSS display waveform '{display_name}' does not exactly match its retained numerical orbit"
            )));
        }
    }
    Ok(())
}

fn simulation_result_from_worker_pss(
    measurements: Vec<WorkerMeasurement>,
    operating_point: rspice_core::engine::PssOperatingPoint,
) -> SimulationResult {
    let result = &operating_point.analysis().result;
    let time = result.time.clone();
    let mut waveforms = HashMap::with_capacity(result.waveforms.len());
    for (name, periodic) in result.node_names.iter().zip(&result.waveforms) {
        if name == "0" || name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        let display_name = format!("V({name})");
        waveforms.insert(
            display_name.clone(),
            WaveformData::new_time_domain(display_name, time.clone(), periodic.values.clone()),
        );
    }
    SimulationResult::Transient {
        time,
        waveforms,
        measurements: measure_results(measurements),
        periodic_state: Some(std::sync::Arc::new(operating_point)),
    }
}

fn waveform_map(waveforms: Vec<WorkerWaveform>) -> HashMap<String, WaveformData> {
    waveforms
        .into_iter()
        .map(|waveform| {
            let name = waveform.name.clone();
            (name, WaveformData::from(waveform))
        })
        .collect()
}

fn worker_measurements(measurements: Vec<rspice_core::MeasureResult>) -> Vec<WorkerMeasurement> {
    measurements
        .into_iter()
        .map(WorkerMeasurement::from)
        .collect()
}

fn measure_results(measurements: Vec<WorkerMeasurement>) -> Vec<rspice_core::MeasureResult> {
    measurements
        .into_iter()
        .map(rspice_core::MeasureResult::from)
        .collect()
}

fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

fn known_static_label(value: &str) -> Option<&'static str> {
    match value {
        "MOSFET" => "MOSFET",
        "BSIM3" => "BSIM3",
        "BSIM4" => "BSIM4",
        "BJT" => "BJT",
        "DIODE" => "DIODE",
        "JFET" => "JFET",
        "MESFET" => "MESFET",
        "id" => "id",
        "vgs" => "vgs",
        "vds" => "vds",
        "vbs" => "vbs",
        "vth" => "vth",
        "vdsat" => "vdsat",
        "gm" => "gm",
        "gds" => "gds",
        "gmb" => "gmb",
        "gmbs" => "gmbs",
        "ic" => "ic",
        "ib" => "ib",
        "vbe" => "vbe",
        "vce" => "vce",
        "beta" => "beta",
        "vd" => "vd",
        "gd" => "gd",
        "igs" => "igs",
        "igd" => "igd",
        "saturation" => "saturation",
        "linear" => "linear",
        "cutoff" => "cutoff",
        "triode" => "triode",
        "subthreshold" => "subthreshold",
        "active" => "active",
        "reverse" => "reverse",
        "forward" => "forward",
        "thermal" => "thermal",
        "flicker" => "flicker",
        "shot" => "shot",
        "burst" => "burst",
        "white" => "white",
        "table" => "table",
        "unknown" => "unknown",
        _ => return None,
    }
    .into()
}
