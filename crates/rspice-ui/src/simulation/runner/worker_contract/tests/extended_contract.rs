//! Extended worker-wire contract tests across analysis kinds and failure paths.

use super::*;

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
            hysteresis: false,
        }),
        // A retracing sweep is a different analysis from the one-way sweep over
        // the same range, so the flag has to survive the worker boundary rather
        // than being reconstructed as a default on the far side.
        AnalysisConfig::DcSweep(DcSweepConfig {
            source: "VIN".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.5,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
            hysteresis: true,
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
            // Retracing, so the round trip proves the flag crosses rather than
            // being defaulted back to a one-way sweep on the far side.
            hysteresis: true,
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

pub(super) fn round_trip_result(result: SimulationResult) -> SimulationResult {
    let worker = WorkerSimulationResult::try_from(result).expect("result is supported");
    SimulationResult::from(worker)
}

pub(super) fn assert_analysis_configs_match(actual: &AnalysisConfig, expected: &AnalysisConfig) {
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
