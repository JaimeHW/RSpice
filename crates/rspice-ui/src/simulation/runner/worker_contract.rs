#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::config::{
        AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
        PoleZeroConfig, PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
    };
    use crate::simulation::multi_run::{
        AnalysisSpec, FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal,
        OptimizationVariable, SpPort,
    };
    use crate::simulation::results::{DcOpResult, SimulationResult, WaveformData};
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
        };

        let encoded = serde_json::to_string(&request).expect("request serializes");
        let decoded: WorkerRequest = serde_json::from_str(&encoded).expect("request deserializes");

        assert_eq!(decoded, request);
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
            }],
        };

        let encoded = serde_json::to_string(&result).expect("result serializes");
        let decoded: WorkerSimulationResult =
            serde_json::from_str(&encoded).expect("result deserializes");

        assert_eq!(decoded, result);
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

        let transport = WorkerResponseTransport::from_response(transfer_response.clone());
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

        let transport = WorkerResponseTransport::from_response(response.clone());

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

        let transport = WorkerResponseTransport::from_response(response.clone());

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
        let ac_transport = WorkerResponseTransport::from_response(ac.clone());
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
        let noise_transport = WorkerResponseTransport::from_response(noise.clone());
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

        let mut missing = WorkerResponseTransport::from_response(response.clone());
        missing.buffers.pop();
        let error = missing
            .into_response()
            .expect_err("missing buffer must fail");
        assert!(error.contains("missing transferable buffer 2"));

        let mut mismatched = WorkerResponseTransport::from_response(response);
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
            AnalysisConfig::DcOp,
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
                num_points: 5,
                start_freq: 1.0,
                stop_freq: 100.0,
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
            let reconstructed = AnalysisConfig::from(worker);

            assert_analysis_configs_match(&reconstructed, &config);
        }
    }

    #[test]
    fn analysis_spec_round_trips_supported_variants() {
        let specs = vec![
            AnalysisSpec::DcOp,
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
                start_freq: 10.0,
                stop_freq: 1e6,
                points_per_decade: 8,
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
            AnalysisSpec::Tf,
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
                fundamental_freq: 1e6,
                num_harmonics: 5,
                tolerance: 1e-6,
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
                stop_time: 10e-6,
                num_harmonics: 7,
                max_step: Some(10e-9),
            },
            AnalysisSpec::Fourier {
                fundamental_freq: 1e6,
                num_harmonics: 9,
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                start_time: 1e-6,
                stop_time: 10e-6,
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
            let reconstructed = AnalysisSpec::from(worker);

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
                assert!(options.tf.is_none());
                assert!(options.pnoise.is_none());
                assert!(options.pstb.is_none());
            }
            other => panic!("expected Monte Carlo spec request, got {other:?}"),
        }
    }

    #[test]
    fn worker_spec_request_preserves_tf_execution_options() {
        let tf = crate::services::simulation_runner::TfRunConfig {
            start_freq: 10.0,
            stop_freq: 10e6,
            points_per_unit: 17,
            sweep: crate::services::simulation_runner::TfFrequencySweep::Octave,
            input_source: "Vstim".to_string(),
            output_node: "out".to_string(),
            output_ref: Some("ref".to_string()),
            group_delay: true,
            input_impedance: true,
            output_impedance: true,
        };
        let request = SimulationRequest::Spec {
            spec: Box::new(AnalysisSpec::Tf),
            options: Box::new(SpecExecutionOptions {
                tf: Some(tf.clone()),
                ..SpecExecutionOptions::default()
            }),
        };
        let input = NetlistInput {
            netlist: "Vstim in 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) Vstim\n.end\n"
                .to_string(),
            source_path: None,
        };

        let worker =
            WorkerRequest::from_runner_parts(51, &request, &input).expect("request converts");
        let (round_tripped, _) = worker.into_runner_parts();

        match round_tripped {
            SimulationRequest::Spec { spec, options } => {
                assert!(matches!(*spec, AnalysisSpec::Tf));
                let actual = options.tf.expect("TF options survive worker contract");
                assert_eq!(actual.start_freq, tf.start_freq);
                assert_eq!(actual.stop_freq, tf.stop_freq);
                assert_eq!(actual.points_per_unit, tf.points_per_unit);
                assert_eq!(actual.sweep, tf.sweep);
                assert_eq!(actual.input_source, tf.input_source);
                assert_eq!(actual.output_node, tf.output_node);
                assert_eq!(actual.output_ref, tf.output_ref);
                assert_eq!(actual.group_delay, tf.group_delay);
                assert_eq!(actual.input_impedance, tf.input_impedance);
                assert_eq!(actual.output_impedance, tf.output_impedance);
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
        };
        let transient = round_trip_result(transient);
        match transient {
            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
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
        };
        let soa = round_trip_result(soa);
        match soa {
            SimulationResult::Soa {
                time,
                waveforms,
                violations,
            } => {
                assert_eq!(time, vec![0.0, 1e-6]);
                assert_eq!(waveforms["SOA_VIOLATION_COUNT"].y_values, vec![0.0, 1.0]);
                assert_eq!(violations[0].device_id, "M1");
                assert_eq!(
                    violations[0].severity,
                    crate::services::safety::ViolationSeverity::Critical
                );
            }
            other => panic!("expected SOA result, got {other:?}"),
        }

        let noise_summary = crate::state::NoiseSummary {
            rows: vec![
                crate::state::NoiseContributorRow {
                    device: "R1".to_string(),
                    mechanism: "thermal",
                    power: 2.5e-18,
                    share_pct: 75.0,
                },
                crate::state::NoiseContributorRow {
                    device: "BNOISE1".to_string(),
                    mechanism: "white",
                    power: 0.5e-18,
                    share_pct: 15.0,
                },
                crate::state::NoiseContributorRow {
                    device: "ATABLE1".to_string(),
                    mechanism: "table",
                    power: 0.25e-18,
                    share_pct: 10.0,
                },
            ],
            total_rms: 1.2e-6,
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
        let request = SimulationRequest::Config(Box::new(AnalysisConfig::DcOp));
        let input = NetlistInput {
            netlist: "V1 in 0 1\nR1 in 0 1k\n.op\n.end\n".to_string(),
            source_path: Some(std::path::PathBuf::from("deck.cir")),
        };

        let worker =
            WorkerRequest::from_runner_parts(41, &request, &input).expect("request converts");

        assert_eq!(worker.id, 41);
        assert!(matches!(
            worker.request,
            WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp)
        ));
        assert_eq!(worker.netlist, input.netlist);
        assert_eq!(worker.source_path.as_deref(), Some("deck.cir"));
    }

    #[test]
    fn worker_request_runs_dc_op() {
        let request = WorkerRequest {
            id: 12,
            request: WorkerSimulationRequest::Config(WorkerAnalysisConfig::DcOp),
            netlist: "* worker op\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n".to_string(),
            source_path: None,
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
    fn worker_request_runs_tf_spec_with_options() {
        let request = SimulationRequest::Spec {
            spec: Box::new(AnalysisSpec::Tf),
            options: Box::new(SpecExecutionOptions {
                tf: Some(crate::services::simulation_runner::TfRunConfig {
                    start_freq: 10.0,
                    stop_freq: 100.0,
                    points_per_unit: 3,
                    sweep: crate::services::simulation_runner::TfFrequencySweep::Linear,
                    input_source: "Vstim".to_string(),
                    output_node: "out".to_string(),
                    output_ref: None,
                    group_delay: false,
                    input_impedance: false,
                    output_impedance: false,
                }),
                ..SpecExecutionOptions::default()
            }),
        };
        let input = NetlistInput {
            netlist: "* worker tf\nVstim in 0 DC 0\nR1 in out 1k\nR2 out 0 1k\n.end\n".to_string(),
            source_path: None,
        };
        let worker =
            WorkerRequest::from_runner_parts(13, &request, &input).expect("request converts");

        let response = worker_response_from_request(worker);
        assert_eq!(response.id, 13);

        let result = response.into_result().expect("worker TF succeeds");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
                ..
            } => {
                assert!(!frequencies.is_empty());
                assert!(
                    waveforms.contains_key("H(V(out)/Vstim)")
                        || waveforms.contains_key("H(out/Vstim)")
                );
            }
            other => panic!("expected AC-like TF result, got {other:?}"),
        }
    }

    fn round_trip_result(result: SimulationResult) -> SimulationResult {
        let worker = WorkerSimulationResult::try_from(result).expect("result is supported");
        SimulationResult::from(worker)
    }

    fn assert_analysis_configs_match(actual: &AnalysisConfig, expected: &AnalysisConfig) {
        match (actual, expected) {
            (AnalysisConfig::DcOp, AnalysisConfig::DcOp) => {}
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
use crate::services::safety::{SoAParameter, SoAViolation, ViolationSeverity};
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    PoleZeroConfig, PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{
    AnalysisSpec, FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal,
    OptimizationVariable, SpPort,
};
use crate::simulation::reliability_engine::{ParamShift, ReliabilityResult, StressMetrics};
use crate::simulation::results::{
    DcOpResult, DeviceOpPoint, MonteCarloVariableResult, SimulationResult, WaveformData,
};
use crate::simulation::status::{SimulationProgress, SimulationStatus};
use crate::state::{NoiseContributorRow, NoiseSummary};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRequest {
    pub id: u64,
    pub request: WorkerSimulationRequest,
    pub netlist: String,
    pub source_path: Option<String>,
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
        })
    }

    pub(crate) fn into_runner_parts(self) -> (SimulationRequest, NetlistInput) {
        (
            SimulationRequest::from(self.request),
            NetlistInput {
                netlist: self.netlist,
                source_path: self.source_path.map(PathBuf::from),
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

impl TryFrom<&SimulationRequest> for WorkerSimulationRequest {
    type Error = SimulationError;

    fn try_from(value: &SimulationRequest) -> Result<Self, Self::Error> {
        match value {
            SimulationRequest::Config(config) => {
                Ok(Self::Config(WorkerAnalysisConfig::from(config.as_ref())))
            }
            SimulationRequest::Spec { spec, options } => Ok(Self::Spec {
                spec: Box::new(WorkerAnalysisSpec::try_from(spec.as_ref())?),
                options: Box::new(WorkerSpecExecutionOptions::from(options.as_ref())),
            }),
        }
    }
}

impl From<WorkerSimulationRequest> for SimulationRequest {
    fn from(value: WorkerSimulationRequest) -> Self {
        match value {
            WorkerSimulationRequest::Config(config) => {
                SimulationRequest::Config(Box::new(AnalysisConfig::from(config)))
            }
            WorkerSimulationRequest::Spec { spec, options } => SimulationRequest::Spec {
                spec: Box::new(AnalysisSpec::from(*spec)),
                options: Box::new(SpecExecutionOptions::from(*options)),
            },
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerSpecExecutionOptions {
    pub temp: Option<WorkerTempRunConfig>,
    pub corner: Option<WorkerCornerRunConfig>,
    pub pac: Option<WorkerPacRunConfig>,
    pub pxf: Option<WorkerPxfRunConfig>,
    pub tf: Option<WorkerTfRunConfig>,
    pub pnoise: Option<WorkerPnoiseRunConfig>,
    pub pstb: Option<WorkerPstbRunConfig>,
}

impl From<&SpecExecutionOptions> for WorkerSpecExecutionOptions {
    fn from(value: &SpecExecutionOptions) -> Self {
        Self {
            temp: value.temp.as_ref().map(WorkerTempRunConfig::from),
            corner: value.corner.as_ref().map(WorkerCornerRunConfig::from),
            pac: value.pac.as_ref().map(WorkerPacRunConfig::from),
            pxf: value.pxf.as_ref().map(WorkerPxfRunConfig::from),
            tf: value.tf.as_ref().map(WorkerTfRunConfig::from),
            pnoise: value.pnoise.as_ref().map(WorkerPnoiseRunConfig::from),
            pstb: value.pstb.as_ref().map(WorkerPstbRunConfig::from),
        }
    }
}

impl From<WorkerSpecExecutionOptions> for SpecExecutionOptions {
    fn from(value: WorkerSpecExecutionOptions) -> Self {
        Self {
            temp: value
                .temp
                .map(crate::services::simulation_runner::TempRunConfig::from),
            corner: value
                .corner
                .map(crate::services::simulation_runner::CornerRunConfig::from),
            pac: value
                .pac
                .map(crate::services::simulation_runner::PacRunConfig::from),
            pxf: value
                .pxf
                .map(crate::services::simulation_runner::PxfRunConfig::from),
            tf: value
                .tf
                .map(crate::services::simulation_runner::TfRunConfig::from),
            pnoise: value
                .pnoise
                .map(crate::services::simulation_runner::PnoiseRunConfig::from),
            pstb: value
                .pstb
                .map(crate::services::simulation_runner::PstbRunConfig::from),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerTempRunConfig {
    pub temperatures_c: Vec<f64>,
    pub base_mode: WorkerCornerBaseMode,
}

impl From<&crate::services::simulation_runner::TempRunConfig> for WorkerTempRunConfig {
    fn from(value: &crate::services::simulation_runner::TempRunConfig) -> Self {
        Self {
            temperatures_c: value.temperatures_c.clone(),
            base_mode: WorkerCornerBaseMode::from(&value.base_mode),
        }
    }
}

impl From<WorkerTempRunConfig> for crate::services::simulation_runner::TempRunConfig {
    fn from(value: WorkerTempRunConfig) -> Self {
        Self {
            temperatures_c: value.temperatures_c,
            base_mode: crate::services::simulation_runner::CornerBaseMode::from(value.base_mode),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerCornerRunConfig {
    pub process_corners: Vec<WorkerCornerProcess>,
    pub voltages: Vec<f64>,
    pub temperatures_c: Vec<f64>,
    pub full_matrix: bool,
    pub nominal_voltage: Option<f64>,
    pub base_mode: WorkerCornerBaseMode,
    pub model_bindings: Vec<WorkerCornerModelBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WorkerCornerModelBinding {
    pub process: WorkerCornerProcess,
    pub source_label: String,
    pub section: Option<String>,
    pub materialized_model_cards: String,
}

impl From<&crate::services::simulation_runner::CornerModelBinding> for WorkerCornerModelBinding {
    fn from(value: &crate::services::simulation_runner::CornerModelBinding) -> Self {
        Self {
            process: WorkerCornerProcess::from(value.process),
            source_label: value.source_label.clone(),
            section: value.section.clone(),
            materialized_model_cards: value.materialized_model_cards.clone(),
        }
    }
}

impl From<WorkerCornerModelBinding> for crate::services::simulation_runner::CornerModelBinding {
    fn from(value: WorkerCornerModelBinding) -> Self {
        Self {
            process: crate::services::simulation_runner::CornerProcess::from(value.process),
            source_label: value.source_label,
            section: value.section,
            materialized_model_cards: value.materialized_model_cards,
        }
    }
}

impl From<&crate::services::simulation_runner::CornerRunConfig> for WorkerCornerRunConfig {
    fn from(value: &crate::services::simulation_runner::CornerRunConfig) -> Self {
        Self {
            process_corners: value
                .process_corners
                .iter()
                .copied()
                .map(WorkerCornerProcess::from)
                .collect(),
            voltages: value.voltages.clone(),
            temperatures_c: value.temperatures_c.clone(),
            full_matrix: value.full_matrix,
            nominal_voltage: value.nominal_voltage,
            base_mode: WorkerCornerBaseMode::from(&value.base_mode),
            model_bindings: value
                .model_bindings
                .iter()
                .map(WorkerCornerModelBinding::from)
                .collect(),
        }
    }
}

impl From<WorkerCornerRunConfig> for crate::services::simulation_runner::CornerRunConfig {
    fn from(value: WorkerCornerRunConfig) -> Self {
        Self {
            process_corners: value
                .process_corners
                .into_iter()
                .map(crate::services::simulation_runner::CornerProcess::from)
                .collect(),
            voltages: value.voltages,
            temperatures_c: value.temperatures_c,
            full_matrix: value.full_matrix,
            nominal_voltage: value.nominal_voltage,
            base_mode: crate::services::simulation_runner::CornerBaseMode::from(value.base_mode),
            model_bindings: value
                .model_bindings
                .into_iter()
                .map(crate::services::simulation_runner::CornerModelBinding::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerCornerBaseMode {
    Op,
    DcSweep {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
    },
    Transient {
        stop_time: f64,
        step_time: f64,
    },
    Ac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
    },
}

impl From<&crate::services::simulation_runner::CornerBaseMode> for WorkerCornerBaseMode {
    fn from(value: &crate::services::simulation_runner::CornerBaseMode) -> Self {
        match value {
            crate::services::simulation_runner::CornerBaseMode::Op => Self::Op,
            crate::services::simulation_runner::CornerBaseMode::DcSweep {
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
            },
            crate::services::simulation_runner::CornerBaseMode::Transient {
                stop_time,
                step_time,
            } => Self::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
            },
            crate::services::simulation_runner::CornerBaseMode::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
            },
        }
    }
}

impl From<WorkerCornerBaseMode> for crate::services::simulation_runner::CornerBaseMode {
    fn from(value: WorkerCornerBaseMode) -> Self {
        match value {
            WorkerCornerBaseMode::Op => Self::Op,
            WorkerCornerBaseMode::DcSweep {
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                source_name,
                start,
                stop,
                step,
            },
            WorkerCornerBaseMode::Transient {
                stop_time,
                step_time,
            } => Self::Transient {
                stop_time,
                step_time,
            },
            WorkerCornerBaseMode::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: crate::services::simulation_runner::CornerFrequencySweep::from(sweep),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPacRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub max_sideband: i32,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub pac_magnitude: f64,
    pub include_dc: bool,
    pub reltol: f64,
    pub abstol: f64,
}

impl From<&crate::services::simulation_runner::PacRunConfig> for WorkerPacRunConfig {
    fn from(value: &crate::services::simulation_runner::PacRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            max_sideband: value.max_sideband,
            input_source: value.input_source.clone(),
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

impl From<WorkerPacRunConfig> for crate::services::simulation_runner::PacRunConfig {
    fn from(value: WorkerPacRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: crate::services::simulation_runner::PacFrequencySweep::from(value.sweep),
            max_sideband: value.max_sideband,
            input_source: value.input_source,
            output_node: value.output_node,
            output_ref: value.output_ref,
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPxfRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub input_source: String,
    pub input_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub output_sideband: i32,
    pub max_sideband: i32,
    pub reltol: f64,
    pub abstol: f64,
}

impl From<&crate::services::simulation_runner::PxfRunConfig> for WorkerPxfRunConfig {
    fn from(value: &crate::services::simulation_runner::PxfRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            input_source: value.input_source.clone(),
            input_sideband: value.input_sideband,
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            output_sideband: value.output_sideband,
            max_sideband: value.max_sideband,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

impl From<WorkerPxfRunConfig> for crate::services::simulation_runner::PxfRunConfig {
    fn from(value: WorkerPxfRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: crate::services::simulation_runner::PxfFrequencySweep::from(value.sweep),
            input_source: value.input_source,
            input_sideband: value.input_sideband,
            output_node: value.output_node,
            output_ref: value.output_ref,
            output_sideband: value.output_sideband,
            max_sideband: value.max_sideband,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPnoiseRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub max_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub noise_ref: WorkerPnoiseReference,
    pub integrated_noise: bool,
    pub noise_summary: bool,
    pub reltol: f64,
    pub abstol: f64,
}

impl From<&crate::services::simulation_runner::PnoiseRunConfig> for WorkerPnoiseRunConfig {
    fn from(value: &crate::services::simulation_runner::PnoiseRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            max_sideband: value.max_sideband,
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            input_source: value.input_source.clone(),
            noise_ref: WorkerPnoiseReference::from(value.noise_ref),
            integrated_noise: value.integrated_noise,
            noise_summary: value.noise_summary,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

impl From<WorkerPnoiseRunConfig> for crate::services::simulation_runner::PnoiseRunConfig {
    fn from(value: WorkerPnoiseRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: crate::services::simulation_runner::PnoiseFrequencySweep::from(value.sweep),
            max_sideband: value.max_sideband,
            output_node: value.output_node,
            output_ref: value.output_ref,
            input_source: value.input_source,
            noise_ref: crate::services::simulation_runner::PnoiseReference::from(value.noise_ref),
            integrated_noise: value.integrated_noise,
            noise_summary: value.noise_summary,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPstbRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: f64,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: f64,
}

impl From<&crate::services::simulation_runner::PstbRunConfig> for WorkerPstbRunConfig {
    fn from(value: &crate::services::simulation_runner::PstbRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            probe_instance: value.probe_instance.clone(),
            max_harmonics: value.max_harmonics,
            num_multipliers: value.num_multipliers,
            stability_threshold: value.stability_threshold,
            detect_subharmonics: value.detect_subharmonics,
            eigenvalue_tolerance: value.eigenvalue_tolerance,
        }
    }
}

impl From<WorkerPstbRunConfig> for crate::services::simulation_runner::PstbRunConfig {
    fn from(value: WorkerPstbRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            probe_instance: value.probe_instance,
            max_harmonics: value.max_harmonics,
            num_multipliers: value.num_multipliers,
            stability_threshold: value.stability_threshold,
            detect_subharmonics: value.detect_subharmonics,
            eigenvalue_tolerance: value.eigenvalue_tolerance,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerTfRunConfig {
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub group_delay: bool,
    pub input_impedance: bool,
    pub output_impedance: bool,
}

impl From<&crate::services::simulation_runner::TfRunConfig> for WorkerTfRunConfig {
    fn from(value: &crate::services::simulation_runner::TfRunConfig) -> Self {
        Self {
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            input_source: value.input_source.clone(),
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            group_delay: value.group_delay,
            input_impedance: value.input_impedance,
            output_impedance: value.output_impedance,
        }
    }
}

impl From<WorkerTfRunConfig> for crate::services::simulation_runner::TfRunConfig {
    fn from(value: WorkerTfRunConfig) -> Self {
        Self {
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: crate::services::simulation_runner::TfFrequencySweep::from(value.sweep),
            input_source: value.input_source,
            output_node: value.output_node,
            output_ref: value.output_ref,
            group_delay: value.group_delay,
            input_impedance: value.input_impedance,
            output_impedance: value.output_impedance,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerAnalysisConfig {
    DcOp,
    DcSweep {
        source: String,
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
        sweep_type: WorkerSweepType,
        num_points: usize,
        start_freq: f64,
        stop_freq: f64,
    },
    Noise {
        output_node: String,
        reference_node: String,
        input_source: String,
        sweep_type: WorkerSweepType,
        num_points: usize,
        start_freq: f64,
        stop_freq: f64,
    },
    PoleZero {
        input_node: String,
        input_ref: String,
        output_node: String,
        output_ref: String,
        transfer_type: String,
        analysis_type: WorkerPzAnalysisType,
    },
    Sensitivity {
        output_var: String,
        ac_mode: bool,
        frequency: Option<f64>,
    },
}

impl From<&AnalysisConfig> for WorkerAnalysisConfig {
    fn from(value: &AnalysisConfig) -> Self {
        match value {
            AnalysisConfig::DcOp => Self::DcOp,
            AnalysisConfig::DcSweep(config) => Self::DcSweep {
                source: config.source.clone(),
                start: config.start,
                stop: config.stop,
                step: config.step,
                source2: config.source2.clone(),
                start2: config.start2,
                stop2: config.stop2,
                step2: config.step2,
            },
            AnalysisConfig::Transient(config) => Self::Transient {
                stop_time: config.stop_time,
                step_time: config.step_time,
                start_time: config.start_time,
                max_timestep: config.max_timestep,
                uic: config.uic,
            },
            AnalysisConfig::Ac(config) => Self::Ac {
                sweep_type: WorkerSweepType::from(config.sweep_type),
                num_points: config.num_points,
                start_freq: config.start_freq,
                stop_freq: config.stop_freq,
            },
            AnalysisConfig::Noise(config) => Self::Noise {
                output_node: config.output_node.clone(),
                reference_node: config.reference_node.clone(),
                input_source: config.input_source.clone(),
                sweep_type: WorkerSweepType::from(config.sweep_type),
                num_points: config.num_points,
                start_freq: config.start_freq,
                stop_freq: config.stop_freq,
            },
            AnalysisConfig::PoleZero(config) => Self::PoleZero {
                input_node: config.input_node.clone(),
                input_ref: config.input_ref.clone(),
                output_node: config.output_node.clone(),
                output_ref: config.output_ref.clone(),
                transfer_type: config.transfer_type.clone(),
                analysis_type: WorkerPzAnalysisType::from(config.analysis_type),
            },
            AnalysisConfig::Sensitivity(config) => Self::Sensitivity {
                output_var: config.output_var.clone(),
                ac_mode: config.ac_mode,
                frequency: config.frequency,
            },
        }
    }
}

impl From<WorkerAnalysisConfig> for AnalysisConfig {
    fn from(value: WorkerAnalysisConfig) -> Self {
        match value {
            WorkerAnalysisConfig::DcOp => Self::DcOp,
            WorkerAnalysisConfig::DcSweep {
                source,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweep(DcSweepConfig {
                source,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            }),
            WorkerAnalysisConfig::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::Transient(TransientAnalysisConfig {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            }),
            WorkerAnalysisConfig::Ac {
                sweep_type,
                num_points,
                start_freq,
                stop_freq,
            } => Self::Ac(AcAnalysisConfig {
                sweep_type: AcSweepType::from(sweep_type),
                num_points,
                start_freq,
                stop_freq,
            }),
            WorkerAnalysisConfig::Noise {
                output_node,
                reference_node,
                input_source,
                sweep_type,
                num_points,
                start_freq,
                stop_freq,
            } => Self::Noise(NoiseAnalysisConfig {
                output_node,
                reference_node,
                input_source,
                sweep_type: AcSweepType::from(sweep_type),
                num_points,
                start_freq,
                stop_freq,
            }),
            WorkerAnalysisConfig::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => Self::PoleZero(PoleZeroConfig {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type: PzAnalysisType::from(analysis_type),
            }),
            WorkerAnalysisConfig::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => Self::Sensitivity(SensitivityConfig {
                output_var,
                ac_mode,
                frequency,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerAnalysisSpec {
    DcOp,
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
        start_freq: f64,
        stop_freq: f64,
        points_per_decade: usize,
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
    Tf,
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
        fundamental_freq: f64,
        num_harmonics: usize,
        tolerance: f64,
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
        stop_time: f64,
        num_harmonics: usize,
        max_step: Option<f64>,
    },
    Fourier {
        fundamental_freq: f64,
        num_harmonics: usize,
        output_node: String,
        output_ref: String,
        start_time: f64,
        stop_time: f64,
    },
    /// Canonical manifest analysis whose typed request is transportable but
    /// whose engine capability is not present. The worker preserves the exact
    /// request and the dispatcher rejects it fail-closed.
    ManifestPreview(AnalysisSpec),
}

impl TryFrom<&AnalysisSpec> for WorkerAnalysisSpec {
    type Error = SimulationError;

    fn try_from(value: &AnalysisSpec) -> Result<Self, Self::Error> {
        match value {
            AnalysisSpec::DcOp => Ok(Self::DcOp),
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Ok(Self::DcSweep {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: source2.clone(),
                start2: *start2,
                stop2: *stop2,
                step2: *step2,
            }),
            AnalysisSpec::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Ok(Self::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: *start_time,
                max_timestep: *max_timestep,
                uic: *uic,
            }),
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Ok(Self::Ac {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
            }),
            AnalysisSpec::AcData {
                table_name,
                frequencies,
            } => Ok(Self::AcData {
                table_name: table_name.clone(),
                frequencies: frequencies.clone(),
            }),
            AnalysisSpec::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                temperature,
            } => Ok(Self::Noise {
                output_node: output_node.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_decade: *points_per_decade,
                temperature: *temperature,
            }),
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => Ok(Self::Sensitivity {
                output_var: output_var.clone(),
                ac_mode: *ac_mode,
                frequency: *frequency,
            }),
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => Ok(Self::PoleZero {
                input_node: input_node.clone(),
                input_ref: input_ref.clone(),
                output_node: output_node.clone(),
                output_ref: output_ref.clone(),
                transfer_type: transfer_type.clone(),
                analysis_type: analysis_type.clone(),
            }),
            AnalysisSpec::Tf => Ok(Self::Tf),
            AnalysisSpec::Pac => Ok(Self::Pac),
            AnalysisSpec::Pxf => Ok(Self::Pxf),
            AnalysisSpec::Pnoise => Ok(Self::Pnoise),
            AnalysisSpec::Pstb => Ok(Self::Pstb),
            AnalysisSpec::Parametric => Ok(Self::Parametric),
            AnalysisSpec::Corner => Ok(Self::Corner),
            AnalysisSpec::MonteCarlo => Ok(Self::MonteCarlo),
            AnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => Ok(Self::Reliability {
                target_years: target_years.clone(),
                enable_hci: *enable_hci,
                enable_nbti: *enable_nbti,
                enable_em: *enable_em,
                min_stress_voltage: *min_stress_voltage,
            }),
            AnalysisSpec::Optimization {
                variables,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            } => Ok(Self::Optimization {
                variables: variables.clone(),
                objective_node: objective_node.clone(),
                objective_ref: objective_ref.clone(),
                goal: *goal,
                target: *target,
                algorithm: *algorithm,
                max_iterations: *max_iterations,
                cost_tolerance: *cost_tolerance,
                fd_step: *fd_step,
                initial_step: *initial_step,
                min_step: *min_step,
            }),
            AnalysisSpec::Soa {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            } => Ok(Self::Soa {
                stop_time: *stop_time,
                step_time: *step_time,
                check_vgs_max: *check_vgs_max,
                max_vgs: *max_vgs,
                check_vds_max: *check_vds_max,
                max_vds: *max_vds,
                check_vbe_max: *check_vbe_max,
                max_vbe: *max_vbe,
                check_vce_max: *check_vce_max,
                max_vce: *max_vce,
            }),
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                sweep,
                points_per_decade,
            } => Ok(Self::Stb {
                probe_node: probe_node.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                sweep: WorkerSweepType::from(*sweep),
                points_per_decade: *points_per_decade,
            }),
            AnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
            } => Ok(Self::SParameter {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
                z0: *z0,
                ports: ports.clone(),
            }),
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => Ok(Self::Disto {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
                f2_over_f1: *f2_over_f1,
            }),
            AnalysisSpec::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            } => Ok(Self::Pss {
                fundamental_freq: *fundamental_freq,
                num_harmonics: *num_harmonics,
                tolerance: *tolerance,
            }),
            AnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            } => Ok(Self::HarmonicBalance {
                tones: tones.clone(),
                reltol: *reltol,
                abstol: *abstol,
                max_iterations: *max_iterations,
                damping: *damping,
                oversample: *oversample,
                collocation_points: *collocation_points,
                max_mixing_order: *max_mixing_order,
                use_krylov: *use_krylov,
                gmres_restart: *gmres_restart,
                source_stepping: *source_stepping,
                verbose: *verbose,
            }),
            AnalysisSpec::Envelope {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            } => Ok(Self::Envelope {
                fundamental_freq: *fundamental_freq,
                stop_time: *stop_time,
                num_harmonics: *num_harmonics,
                max_step: *max_step,
            }),
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            } => Ok(Self::Fourier {
                fundamental_freq: *fundamental_freq,
                num_harmonics: *num_harmonics,
                output_node: output_node.clone(),
                output_ref: output_ref.clone(),
                start_time: *start_time,
                stop_time: *stop_time,
            }),
            AnalysisSpec::Qpss { .. }
            | AnalysisSpec::Hbsp { .. }
            | AnalysisSpec::Hbnoise { .. }
            | AnalysisSpec::Psp { .. }
            | AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpnoise { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::TransientNoise { .. }
            | AnalysisSpec::DcMismatch { .. } => Ok(Self::ManifestPreview(value.clone())),
        }
    }
}

impl From<WorkerAnalysisSpec> for AnalysisSpec {
    fn from(value: WorkerAnalysisSpec) -> Self {
        match value {
            WorkerAnalysisSpec::DcOp => Self::DcOp,
            WorkerAnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            },
            WorkerAnalysisSpec::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            },
            WorkerAnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::from(sweep),
            },
            WorkerAnalysisSpec::AcData {
                table_name,
                frequencies,
            } => Self::AcData {
                table_name,
                frequencies,
            },
            WorkerAnalysisSpec::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                temperature,
            } => Self::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                temperature,
            },
            WorkerAnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => Self::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            },
            WorkerAnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => Self::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            },
            WorkerAnalysisSpec::Tf => Self::Tf,
            WorkerAnalysisSpec::Pac => Self::Pac,
            WorkerAnalysisSpec::Pxf => Self::Pxf,
            WorkerAnalysisSpec::Pnoise => Self::Pnoise,
            WorkerAnalysisSpec::Pstb => Self::Pstb,
            WorkerAnalysisSpec::Parametric => Self::Parametric,
            WorkerAnalysisSpec::Corner => Self::Corner,
            WorkerAnalysisSpec::MonteCarlo => Self::MonteCarlo,
            WorkerAnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => Self::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            },
            WorkerAnalysisSpec::Optimization {
                variables,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            } => Self::Optimization {
                variables,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            },
            WorkerAnalysisSpec::Soa {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            } => Self::Soa {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            },
            WorkerAnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                sweep,
                points_per_decade,
            } => Self::Stb {
                probe_node,
                start_freq,
                stop_freq,
                sweep: FrequencySweep::from(sweep),
                points_per_decade,
            },
            WorkerAnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
            } => Self::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::from(sweep),
                z0,
                ports,
            },
            WorkerAnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => Self::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::from(sweep),
                f2_over_f1,
            },
            WorkerAnalysisSpec::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            } => Self::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            },
            WorkerAnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            } => Self::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            },
            WorkerAnalysisSpec::Envelope {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            } => Self::Envelope {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            },
            WorkerAnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            } => Self::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            },
            WorkerAnalysisSpec::ManifestPreview(spec) => spec,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerSweepType {
    Decade,
    Octave,
    Linear,
}

impl From<AcSweepType> for WorkerSweepType {
    fn from(value: AcSweepType) -> Self {
        match value {
            AcSweepType::Decade => Self::Decade,
            AcSweepType::Octave => Self::Octave,
            AcSweepType::Linear => Self::Linear,
        }
    }
}

impl From<FrequencySweep> for WorkerSweepType {
    fn from(value: FrequencySweep) -> Self {
        match value {
            FrequencySweep::Decade => Self::Decade,
            FrequencySweep::Octave => Self::Octave,
            FrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::TfFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::TfFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::TfFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::TfFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::TfFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::PacFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::PacFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::PacFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::PacFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::PacFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::PxfFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::PxfFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::PxfFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::PxfFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::PxfFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::PnoiseFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::PnoiseFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::PnoiseFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::PnoiseFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::PnoiseFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::CornerFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::CornerFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::CornerFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::CornerFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::CornerFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for AcSweepType {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for FrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::TfFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::PacFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::PxfFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::PnoiseFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::CornerFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerCornerProcess {
    TT,
    SS,
    FF,
    SF,
    FS,
}

impl From<crate::services::simulation_runner::CornerProcess> for WorkerCornerProcess {
    fn from(value: crate::services::simulation_runner::CornerProcess) -> Self {
        match value {
            crate::services::simulation_runner::CornerProcess::TT => Self::TT,
            crate::services::simulation_runner::CornerProcess::SS => Self::SS,
            crate::services::simulation_runner::CornerProcess::FF => Self::FF,
            crate::services::simulation_runner::CornerProcess::SF => Self::SF,
            crate::services::simulation_runner::CornerProcess::FS => Self::FS,
        }
    }
}

impl From<WorkerCornerProcess> for crate::services::simulation_runner::CornerProcess {
    fn from(value: WorkerCornerProcess) -> Self {
        match value {
            WorkerCornerProcess::TT => Self::TT,
            WorkerCornerProcess::SS => Self::SS,
            WorkerCornerProcess::FF => Self::FF,
            WorkerCornerProcess::SF => Self::SF,
            WorkerCornerProcess::FS => Self::FS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerPnoiseReference {
    Output,
    Input,
    Phase,
}

impl From<crate::services::simulation_runner::PnoiseReference> for WorkerPnoiseReference {
    fn from(value: crate::services::simulation_runner::PnoiseReference) -> Self {
        match value {
            crate::services::simulation_runner::PnoiseReference::Output => Self::Output,
            crate::services::simulation_runner::PnoiseReference::Input => Self::Input,
            crate::services::simulation_runner::PnoiseReference::Phase => Self::Phase,
        }
    }
}

impl From<WorkerPnoiseReference> for crate::services::simulation_runner::PnoiseReference {
    fn from(value: WorkerPnoiseReference) -> Self {
        match value {
            WorkerPnoiseReference::Output => Self::Output,
            WorkerPnoiseReference::Input => Self::Input,
            WorkerPnoiseReference::Phase => Self::Phase,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerPzAnalysisType {
    PoleZero,
    PolesOnly,
    ZerosOnly,
}

impl From<PzAnalysisType> for WorkerPzAnalysisType {
    fn from(value: PzAnalysisType) -> Self {
        match value {
            PzAnalysisType::PoleZero => Self::PoleZero,
            PzAnalysisType::PolesOnly => Self::PolesOnly,
            PzAnalysisType::ZerosOnly => Self::ZerosOnly,
        }
    }
}

impl From<WorkerPzAnalysisType> for PzAnalysisType {
    fn from(value: WorkerPzAnalysisType) -> Self {
        match value {
            WorkerPzAnalysisType::PoleZero => Self::PoleZero,
            WorkerPzAnalysisType::PolesOnly => Self::PolesOnly,
            WorkerPzAnalysisType::ZerosOnly => Self::ZerosOnly,
        }
    }
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
    ConvergenceFailed { iterations: usize, message: String },
    Aborted,
    AlreadyRunning,
    ThreadPanic,
    InvalidConfig(String),
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
        sensitivities: HashMap<String, f64>,
        normalized: HashMap<String, f64>,
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
                node_voltages,
                branch_currents,
                device_ops,
                device_report,
            } => sum_payload_bytes([
                f64_payload_bytes(node_voltages.len()),
                f64_payload_bytes(branch_currents.len()),
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
                sensitivities,
                normalized,
            } => sum_payload_bytes([
                f64_payload_bytes(sensitivities.len()),
                f64_payload_bytes(normalized.len()),
            ]),
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
            } => sum_payload_bytes([
                f64_payload_bytes(time.len()),
                waveforms_payload_bytes(waveforms),
                soa_violations_payload_bytes(violations),
            ]),
            WorkerSimulationResult::MeasurementsOnly { measurements } => {
                f64_payload_bytes(measurements.len())
            }
        }
    }
}

const WORKER_RESPONSE_TRANSPORT_PROTOCOL: u8 = 2;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WorkerResponseTransport {
    pub protocol: u8,
    pub response: WorkerResponseTransportMetadata,
    pub buffers: Vec<Vec<f64>>,
}

impl WorkerResponseTransport {
    fn from_response(response: WorkerResponse) -> Self {
        let mut buffers = Vec::new();
        let response = WorkerResponseTransportMetadata {
            id: response.id,
            outcome: WorkerOutcomeTransport::from_outcome(response.outcome, &mut buffers),
        };
        Self {
            protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
            response,
            buffers,
        }
    }

    fn into_response(self) -> Result<WorkerResponse, String> {
        if self.protocol != WORKER_RESPONSE_TRANSPORT_PROTOCOL {
            return Err(format!(
                "unsupported worker response transport protocol {}",
                self.protocol
            ));
        }

        Ok(WorkerResponse {
            id: self.response.id,
            outcome: self.response.outcome.into_outcome(&self.buffers)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerResponseTransportMetadata {
    pub id: u64,
    pub outcome: WorkerOutcomeTransport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerOutcomeTransport {
    Success(WorkerSimulationResultTransport),
    Failure(WorkerSimulationError),
}

impl WorkerOutcomeTransport {
    fn from_outcome(outcome: WorkerOutcome, buffers: &mut Vec<Vec<f64>>) -> Self {
        match outcome {
            WorkerOutcome::Success(result) => Self::Success(
                WorkerSimulationResultTransport::from_result(result, buffers),
            ),
            WorkerOutcome::Failure(error) => Self::Failure(error),
        }
    }

    fn into_outcome(self, buffers: &[Vec<f64>]) -> Result<WorkerOutcome, String> {
        match self {
            Self::Success(result) => Ok(WorkerOutcome::Success(result.into_result(buffers)?)),
            Self::Failure(error) => Ok(WorkerOutcome::Failure(error)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerF64Series {
    Inline(Vec<f64>),
    Buffer { buffer: usize, len: usize },
}

impl WorkerF64Series {
    fn from_vec(values: Vec<f64>, buffers: &mut Vec<Vec<f64>>) -> Self {
        let len = values.len();
        let buffer = buffers.len();
        buffers.push(values);
        Self::Buffer { buffer, len }
    }

    fn into_vec(self, buffers: &[Vec<f64>]) -> Result<Vec<f64>, String> {
        match self {
            Self::Inline(values) => Ok(values),
            Self::Buffer { buffer, len } => {
                let values = buffers
                    .get(buffer)
                    .ok_or_else(|| format!("missing transferable buffer {buffer}"))?;
                if values.len() != len {
                    return Err(format!(
                        "transferable buffer {buffer} has length {}, expected {len}",
                        values.len()
                    ));
                }
                Ok(values.clone())
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Inline(values) => values.len(),
            Self::Buffer { len, .. } => *len,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationResultTransport {
    Inline(WorkerSimulationResult),
    DcSweep {
        sweep_var: String,
        sweep_values: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        measurements: Vec<WorkerMeasurement>,
    },
    Transient {
        time: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        measurements: Vec<WorkerMeasurement>,
    },
    Ac {
        frequencies: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        measurements: Vec<WorkerMeasurement>,
    },
    Noise {
        frequencies: WorkerF64Series,
        output_noise: WorkerF64Series,
        input_noise: Option<WorkerF64Series>,
        contributors: HashMap<String, WorkerF64Series>,
        #[serde(default)]
        summary: Option<WorkerNoiseSummary>,
    },
    Parametric {
        target: String,
        sweep_values: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        num_failures: usize,
    },
    Corner {
        x_values: WorkerF64Series,
        x_label: String,
        x_unit: String,
        temperatures_c: WorkerF64Series,
        corner_labels: Vec<String>,
        waveforms: Vec<WorkerWaveformTransport>,
        num_failures: usize,
    },
    Reliability {
        years: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        device_results: Vec<WorkerReliabilityResult>,
    },
    Optimization {
        iterations: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        best_cost: f64,
        best_variables: HashMap<String, f64>,
        converged: bool,
    },
    Soa {
        time: WorkerF64Series,
        waveforms: Vec<WorkerWaveformTransport>,
        violations: Vec<WorkerSoAViolation>,
    },
}

impl WorkerSimulationResultTransport {
    fn from_result(result: WorkerSimulationResult, buffers: &mut Vec<Vec<f64>>) -> Self {
        match result {
            WorkerSimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Self::DcSweep {
                sweep_var,
                sweep_values: WorkerF64Series::from_vec(sweep_values, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
            },
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
            } => Self::Transient {
                time: WorkerF64Series::from_vec(time, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
            },
            WorkerSimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Self::Ac {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
            },
            WorkerSimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => Self::Noise {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                output_noise: WorkerF64Series::from_vec(output_noise, buffers),
                input_noise: input_noise.map(|values| WorkerF64Series::from_vec(values, buffers)),
                contributors: contributors
                    .into_iter()
                    .map(|(name, values)| (name, WorkerF64Series::from_vec(values, buffers)))
                    .collect(),
                summary,
            },
            WorkerSimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => Self::Parametric {
                target,
                sweep_values: WorkerF64Series::from_vec(sweep_values, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
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
                x_values: WorkerF64Series::from_vec(x_values, buffers),
                x_label,
                x_unit,
                temperatures_c: WorkerF64Series::from_vec(temperatures_c, buffers),
                corner_labels,
                waveforms: transport_waveforms(waveforms, buffers),
                num_failures,
            },
            WorkerSimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => Self::Reliability {
                years: WorkerF64Series::from_vec(years, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                device_results,
            },
            WorkerSimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Self::Optimization {
                iterations: WorkerF64Series::from_vec(iterations, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                best_cost,
                best_variables,
                converged,
            },
            WorkerSimulationResult::Soa {
                time,
                waveforms,
                violations,
            } => Self::Soa {
                time: WorkerF64Series::from_vec(time, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                violations,
            },
            other => Self::Inline(other),
        }
    }

    fn into_result(self, buffers: &[Vec<f64>]) -> Result<WorkerSimulationResult, String> {
        match self {
            Self::Inline(result) => Ok(result),
            Self::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Ok(WorkerSimulationResult::DcSweep {
                sweep_var,
                sweep_values: sweep_values.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
            }),
            Self::Transient {
                time,
                waveforms,
                measurements,
            } => Ok(WorkerSimulationResult::Transient {
                time: time.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
            }),
            Self::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Ok(WorkerSimulationResult::Ac {
                frequencies: frequencies.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
            }),
            Self::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => Ok(WorkerSimulationResult::Noise {
                frequencies: frequencies.into_vec(buffers)?,
                output_noise: output_noise.into_vec(buffers)?,
                input_noise: input_noise
                    .map(|values| values.into_vec(buffers))
                    .transpose()?,
                contributors: contributors
                    .into_iter()
                    .map(|(name, values)| values.into_vec(buffers).map(|values| (name, values)))
                    .collect::<Result<_, _>>()?,
                summary,
            }),
            Self::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => Ok(WorkerSimulationResult::Parametric {
                target,
                sweep_values: sweep_values.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                num_failures,
            }),
            Self::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
            } => Ok(WorkerSimulationResult::Corner {
                x_values: x_values.into_vec(buffers)?,
                x_label,
                x_unit,
                temperatures_c: temperatures_c.into_vec(buffers)?,
                corner_labels,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                num_failures,
            }),
            Self::Reliability {
                years,
                waveforms,
                device_results,
            } => Ok(WorkerSimulationResult::Reliability {
                years: years.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                device_results,
            }),
            Self::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Ok(WorkerSimulationResult::Optimization {
                iterations: iterations.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                best_cost,
                best_variables,
                converged,
            }),
            Self::Soa {
                time,
                waveforms,
                violations,
            } => Ok(WorkerSimulationResult::Soa {
                time: time.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                violations,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerWaveformTransport {
    pub name: String,
    pub x_values: WorkerF64Series,
    pub y_values: WorkerF64Series,
    pub y_unit: String,
    pub x_unit: String,
    pub is_complex: bool,
    pub y_imag: Option<WorkerF64Series>,
}

impl WorkerWaveformTransport {
    fn from_waveform(waveform: WorkerWaveform, buffers: &mut Vec<Vec<f64>>) -> Self {
        Self {
            name: waveform.name,
            x_values: WorkerF64Series::from_vec(waveform.x_values, buffers),
            y_values: WorkerF64Series::from_vec(waveform.y_values, buffers),
            y_unit: waveform.y_unit,
            x_unit: waveform.x_unit,
            is_complex: waveform.is_complex,
            y_imag: waveform
                .y_imag
                .map(|values| WorkerF64Series::from_vec(values, buffers)),
        }
    }

    fn into_waveform(self, buffers: &[Vec<f64>]) -> Result<WorkerWaveform, String> {
        let x_len = self.x_values.len();
        let y_len = self.y_values.len();
        let imag_len = self.y_imag.as_ref().map(WorkerF64Series::len);

        if x_len != y_len {
            return Err(format!(
                "waveform {} x/y length mismatch: x length {x_len}, y length {y_len}",
                self.name
            ));
        }
        match (self.is_complex, imag_len) {
            (true, Some(len)) if len == y_len => {}
            (true, Some(len)) => {
                return Err(format!(
                    "complex waveform {} imaginary length {len} does not match y length {y_len}",
                    self.name
                ));
            }
            (true, None) => {
                return Err(format!(
                    "complex waveform {} is missing an imaginary buffer",
                    self.name
                ));
            }
            (false, Some(_)) => {
                return Err(format!(
                    "non-complex waveform {} must not include an imaginary buffer",
                    self.name
                ));
            }
            (false, None) => {}
        }

        Ok(WorkerWaveform {
            name: self.name,
            x_values: self.x_values.into_vec(buffers)?,
            y_values: self.y_values.into_vec(buffers)?,
            y_unit: self.y_unit,
            x_unit: self.x_unit,
            is_complex: self.is_complex,
            y_imag: self
                .y_imag
                .map(|values| values.into_vec(buffers))
                .transpose()?,
        })
    }
}

fn transport_waveforms(
    waveforms: Vec<WorkerWaveform>,
    buffers: &mut Vec<Vec<f64>>,
) -> Vec<WorkerWaveformTransport> {
    waveforms
        .into_iter()
        .map(|waveform| WorkerWaveformTransport::from_waveform(waveform, buffers))
        .collect()
}

fn worker_waveforms_from_transport(
    waveforms: Vec<WorkerWaveformTransport>,
    buffers: &[Vec<f64>],
) -> Result<Vec<WorkerWaveform>, String> {
    waveforms
        .into_iter()
        .map(|waveform| waveform.into_waveform(buffers))
        .collect()
}

impl TryFrom<SimulationResult> for WorkerSimulationResult {
    type Error = SimulationError;

    fn try_from(value: SimulationResult) -> Result<Self, Self::Error> {
        match value {
            SimulationResult::DcOp(result) => Ok(Self::DcOp {
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
            } => Ok(Self::Transient {
                time,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
            }),
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
                sensitivities,
                normalized,
            } => Ok(Self::Sensitivity {
                sensitivities,
                normalized,
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
            } => Ok(Self::Soa {
                time,
                waveforms: worker_waveforms(waveforms),
                violations: violations
                    .into_iter()
                    .map(WorkerSoAViolation::from)
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
                node_voltages,
                branch_currents,
                device_ops,
                device_report,
            } => Self::DcOp(DcOpResult {
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
            },
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
                sensitivities,
                normalized,
            } => Self::Sensitivity {
                sensitivities,
                normalized,
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
            } => Self::Soa {
                time,
                waveforms: waveform_map(waveforms),
                violations: violations.into_iter().map(SoAViolation::from).collect(),
            },
            WorkerSimulationResult::MeasurementsOnly { measurements } => {
                Self::MeasurementsOnly { measurements }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNoiseSummary {
    pub rows: Vec<WorkerNoiseContributorRow>,
    pub total_rms: f64,
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
            mechanism: intern_static_label(value.mechanism),
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
}

#[cfg(test)]
impl WorkerMeasurement {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(
            usize::from(self.value.is_some())
                + usize::from(self.expected.is_some())
                + usize::from(self.tolerance.is_some()),
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
    let request: WorkerRequest = serde_wasm_bindgen::from_value(value)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let id = request.id;
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(Some(id)));
    let response =
        worker_response_from_request_with_progress(request, Some(emit_worker_progress_snapshot));
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(None));
    worker_response_transport_value(response)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_transport_value(
    response: WorkerResponse,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsValue;

    let transport = WorkerResponseTransport::from_response(response);
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

    let mut decoded_buffers = Vec::with_capacity(buffers.length() as usize);
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

fn worker_waveforms(waveforms: HashMap<String, WaveformData>) -> Vec<WorkerWaveform> {
    let mut waveforms: Vec<_> = waveforms.into_values().map(WorkerWaveform::from).collect();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));
    waveforms
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
