//! Every result variant, out through the worker contract and back.
//!
//! One test rather than one per variant, deliberately: what it proves is that
//! the boundary is total. A variant that gained a field and lost it in transit
//! fails here, and a variant nobody remembered to encode fails here too, which
//! is the failure a per-variant suite misses because nobody writes the test for
//! the case they forgot.

use super::*;

#[test]
fn transient_convergence_survives_worker_round_trip() {
    let mut convergence = rspice_core::diagnostics::ConvergenceQuality {
        total_iterations: 19,
        gmin_stepping_count: 2,
        source_stepping_count: 3,
        max_residual: 1.25e-9,
        avg_iterations_per_solve: 3.5,
        timestep_reductions: 7,
        lte_rejections: 8,
        bypassed_device_evaluations: 11,
        ..Default::default()
    };
    convergence.record_force_accept(1);
    let convergence = crate::state::TransientConvergenceEvidence::capture(
        convergence,
        &[0.0, 1e-9],
        &rspice_core::abort_signal::NoAbort,
    )
    .unwrap();
    let expected = convergence.clone();
    let result = SimulationResult::Transient {
        spectra: Vec::new(),
        time: vec![0.0, 1e-9],
        waveforms: HashMap::from([(
            "V(out)".to_owned(),
            WaveformData::new_time_domain("V(out)", vec![0.0, 1e-9], vec![0.0, 0.8]),
        )]),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: Some(std::sync::Arc::new(convergence)),
        events: Default::default(),
    };
    let response = WorkerResponse {
        id: 42,
        outcome: WorkerOutcome::Success(Box::new(
            WorkerSimulationResult::try_from(result).unwrap(),
        )),
    };
    let transport = WorkerResponseTransport::from_response(response).unwrap();
    let json = serde_json::to_string(&transport.response).unwrap();
    let transport = WorkerResponseTransport {
        response: serde_json::from_str(&json).unwrap(),
        ..transport
    };
    let WorkerOutcome::Success(result) = transport.into_response().unwrap().outcome else {
        panic!("worker must retain the successful result")
    };
    let SimulationResult::Transient { convergence, .. } = SimulationResult::from(*result) else {
        panic!("transient result must retain its kind")
    };
    let convergence = convergence.unwrap();
    assert_eq!(*convergence, expected);
    let convergence = &convergence.transient;
    assert_eq!(convergence.total_iterations, 19);
    assert_eq!(convergence.gmin_stepping_count, 2);
    assert_eq!(convergence.source_stepping_count, 3);
    assert_eq!(convergence.force_accepted_points, 1);
    assert_eq!(convergence.force_accepted_indices, vec![1]);
    assert_eq!(convergence.max_residual, 1.25e-9);
    assert_eq!(convergence.avg_iterations_per_solve, 3.5);
    assert_eq!(convergence.timestep_reductions, 7);
    assert_eq!(convergence.lte_rejections, 8);
    assert_eq!(convergence.bypassed_device_evaluations, 11);
}

#[test]
fn worker_result_round_trip() {
    let dc_op = SimulationResult::DcOp(Box::new(DcOpResult {
        configuration: crate::simulation::dialog::OpConfig::default(),
        validated_startup_directives: 0,
        mna_node_names: vec!["out".to_owned()],
        mna_branch_names: vec!["V1".to_owned()],
        mna_solution: vec![1.2, -0.01],
        node_voltages: HashMap::from([("out".to_string(), 1.2)]),
        branch_currents: HashMap::from([("V1".to_string(), -0.01)]),
        device_report: None,
    }));
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
        spectra: Vec::new(),
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
        convergence: Default::default(),
        events: Default::default(),
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
        member_measurements: Vec::new(),
        target: "TEMP".to_string(),
        sweep_values: vec![-40.0, 25.0, 125.0],
        waveforms: HashMap::from([(
            "V(out)".to_string(),
            WaveformData::new_time_domain("V(out)", vec![-40.0, 25.0, 125.0], vec![0.8, 0.9, 1.0]),
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
            ..
        } => {
            assert_eq!(target, "TEMP");
            assert_eq!(sweep_values, vec![-40.0, 25.0, 125.0]);
            assert_eq!(waveforms["V(out)"].y_values, vec![0.8, 0.9, 1.0]);
            assert_eq!(num_failures, 1);
        }
        other => panic!("expected parametric result, got {other:?}"),
    }

    let corner = SimulationResult::Corner {
        member_measurements: Vec::new(),
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
            ..
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
        member_measurements: Vec::new(),
        seed: 0x1234_5678_9abc_def0,
        runs_requested: 20,
        runs_completed: 18,
        num_failures: 2,
        all_converged: false,
        variables: vec![crate::simulation::results::MonteCarloVariableResult {
            mean_confidence: None,
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
            ..
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
        convergence: None,
        time: vec![0.0, 1e-6],
        waveforms: HashMap::from([(
            "SOA_VIOLATION_COUNT".to_string(),
            WaveformData::new_time_domain("SOA_VIOLATION_COUNT", vec![0.0, 1e-6], vec![0.0, 1.0]),
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
            duration: None,
            thresholds: Default::default(),
            derating: None,
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
            convergence,
        } => {
            assert!(convergence.is_none());
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
        input_quantity: None,
        conversion: Some(crate::state::PeriodicNoiseConversionEvidence {
            input_source: "V1".into(),
            carrier_hz: 1e6,
            input_sideband: 2,
            output_sideband: -1,
            max_sideband: 3,
        }),
        noise_figure: Some(std::sync::Arc::new(crate::state::NoiseFigureEvidence {
            input_source: "V1".into(),
            source_resistor: "Rs".into(),
            source_resistance_ohm: 75.0,
            source_temperature_kelvin: 325.0,
            reference_temperature_kelvin: 290.0,
            frequencies: vec![1.0, 10.0],
            decibels: vec![2.1, 2.5],
        })),
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
        measurements: Vec::new(),
    };
    let noise = round_trip_result(noise);
    match noise {
        SimulationResult::Noise {
            frequencies,
            output_noise,
            input_noise,
            contributors,
            summary,
            measurements,
        } => {
            assert_eq!(frequencies, vec![1.0, 10.0]);
            assert_eq!(output_noise, vec![1.0e-18, 2.0e-18]);
            assert_eq!(input_noise, Some(vec![3.0e-18, 4.0e-18]));
            assert_eq!(contributors["R1"], vec![0.7e-18, 1.4e-18]);
            assert_eq!(summary, Some(noise_summary));
            assert!(measurements.is_empty());
        }
        other => panic!("expected noise result, got {other:?}"),
    }

    let pole_zero = SimulationResult::PoleZero {
        poles: vec![(-1.0, 2.0)],
        zeros: vec![(-3.0, 0.0)],
        pole_evidence: crate::state::PoleZeroRootSetEvidence::Qualified {
            certificate: crate::state::PoleZeroSpectrumCertificate {
                problem_order: 1,
                infinite_count: 0,
                max_backward_error: 1.0e-14,
                qualification_tolerance:
                    crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                        .unwrap(),
            },
        },
        zero_evidence: crate::state::PoleZeroRootSetEvidence::Approximate {
            certificate: crate::state::PoleZeroSpectrumCertificate {
                problem_order: 1,
                infinite_count: 0,
                max_backward_error: 1.0e-9,
                qualification_tolerance:
                    crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1)
                        .unwrap(),
            },
        },
        gain: None,
    };
    let pole_zero = round_trip_result(pole_zero);
    match pole_zero {
        SimulationResult::PoleZero {
            poles,
            zeros,
            pole_evidence,
            zero_evidence,
            gain,
        } => {
            assert_eq!(poles, vec![(-1.0, 2.0)]);
            assert_eq!(zeros, vec![(-3.0, 0.0)]);
            assert!(matches!(
                pole_evidence,
                crate::state::PoleZeroRootSetEvidence::Qualified { .. }
            ));
            assert!(matches!(
                zero_evidence,
                crate::state::PoleZeroRootSetEvidence::Approximate { .. }
            ));
            assert_eq!(gain, None);
        }
        other => panic!("expected pole-zero result, got {other:?}"),
    }

    let mut pac_current = WaveformData::new_complex(
        "I(V1)[sb=+0]",
        vec![1.0, 10.0],
        vec![-5.0e-4, f64::from_bits(1)],
        vec![0.0, -f64::from_bits(2)],
    );
    pac_current.y_unit = "A".to_owned();
    let ac = SimulationResult::Ac {
        convergence: None,
        noise_reference_temperature_kelvin: None,
        reference_impedances_ohm: None,
        frequencies: vec![1.0, 10.0],
        waveforms: HashMap::from([
            (
                "V(out)".to_string(),
                WaveformData::new_complex(
                    "V(out)",
                    vec![1.0, 10.0],
                    vec![0.5, 0.25],
                    vec![-0.1, -0.2],
                ),
            ),
            ("I(V1)[sb=+0]".to_owned(), pac_current),
        ]),
        measurements: vec![rspice_core::MeasureResult::success("gain", 0.5)],
    };
    let ac = round_trip_result(ac);
    match ac {
        SimulationResult::Ac {
            frequencies,
            waveforms,
            measurements,
            reference_impedances_ohm,
            noise_reference_temperature_kelvin,
            convergence,
        } => {
            assert!(convergence.is_none());
            assert_eq!(reference_impedances_ohm, None);
            assert_eq!(noise_reference_temperature_kelvin, None);
            assert_eq!(frequencies, vec![1.0, 10.0]);
            let waveform = waveforms.get("V(out)").expect("waveform is preserved");
            assert!(waveform.is_complex);
            assert_eq!(waveform.y_values, vec![0.5, 0.25]);
            assert_eq!(waveform.y_imag.as_deref(), Some(&[-0.1, -0.2][..]));
            let current = waveforms
                .get("I(V1)[sb=+0]")
                .expect("PAC branch current is preserved");
            assert_eq!(current.y_unit, "A");
            assert_eq!(current.y_values[1].to_bits(), f64::from_bits(1).to_bits());
            assert_eq!(
                current.y_imag.as_ref().expect("current remains complex")[1].to_bits(),
                (-f64::from_bits(2)).to_bits()
            );
            assert_eq!(measurements[0].name, "gain");
            assert_eq!(measurements[0].value, Some(0.5));
            assert!(measurements[0].passed);
        }
        other => panic!("expected ac result, got {other:?}"),
    }
}

#[test]
fn convergence_source_evidence_survives_derived_worker_transports() {
    use crate::state::{
        ConvergenceReport, PeriodicConvergenceEvidence, PeriodicInitializationMethod,
        TransientConvergenceEvidence,
    };
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::diagnostics::{
        ConvergenceDiagnostic, ConvergenceFailureClass, ConvergenceQuality, ConvergenceSite,
        ConvergenceSiteKind,
    };
    let mut metrics = ConvergenceQuality {
        total_iterations: 23,
        bypassed_device_evaluations: (1_u64 << 53) + 1,
        failure_diagnostic: Some(ConvergenceDiagnostic {
            class: ConvergenceFailureClass::NewtonNonConvergence,
            sites: vec![ConvergenceSite {
                name: "OUT".to_owned(),
                kind: ConvergenceSiteKind::Node,
                residual: Some(2.75),
            }],
            elided_sites: 2,
            failure_message: "initial attempt exhausted iterations".to_owned(),
        }),
        ..Default::default()
    };
    metrics.record_force_accept(1);
    let mut quality =
        TransientConvergenceEvidence::capture(metrics, &[0.0, 0.5, 1.0], &NoAbort).unwrap();
    let mut initialization = ConvergenceQuality::default();
    initialization.record_force_accept(4);
    quality.initialization = Some(PeriodicConvergenceEvidence {
        method: PeriodicInitializationMethod::Shooting,
        solver_iterations: (1_u64 << 53) + 3,
        final_residual: 2.75e-12,
        report: ConvergenceReport::capture(initialization, None, &NoAbort).unwrap(),
    });
    let quality = std::sync::Arc::new(quality);
    for result in [
        SimulationResult::Transient {
            spectra: Vec::new(),
            time: vec![0.6, 1.0],
            waveforms: HashMap::new(),
            measurements: Vec::new(),
            periodic_state: None,
            events: Default::default(),
            convergence: Some(quality.clone()),
        },
        SimulationResult::Ac {
            frequencies: vec![1.0],
            waveforms: HashMap::new(),
            measurements: Vec::new(),
            reference_impedances_ohm: None,
            noise_reference_temperature_kelvin: None,
            convergence: Some(quality.clone()),
        },
        SimulationResult::Soa {
            time: vec![0.6, 1.0],
            waveforms: HashMap::new(),
            violations: Vec::new(),
            evaluations: Vec::new(),
            convergence: Some(quality.clone()),
        },
    ] {
        let transport = WorkerResponseTransport::from_response(
            WorkerResponse::from_result_for_transfer(7, Ok(result)),
        )
        .unwrap();
        let json = serde_json::to_string(&transport.response).unwrap();
        assert!(json.contains("\"9007199254740993\""));
        let restored = WorkerResponseTransport {
            response: serde_json::from_str(&json).unwrap(),
            ..transport
        }
        .into_response()
        .unwrap()
        .into_result()
        .unwrap();
        assert_eq!(restored.transient_convergence(), Some(&quality));
    }
}

#[test]
fn convergence_worker_transport_rejects_inline_and_malformed_quality_buffers() {
    let mut metrics = rspice_core::diagnostics::ConvergenceQuality::default();
    metrics.record_force_accept(1);
    let quality = crate::state::TransientConvergenceEvidence::capture(
        metrics,
        &[0.0, 1.0],
        &rspice_core::abort_signal::NoAbort,
    )
    .unwrap();
    let result = SimulationResult::Transient {
        spectra: Vec::new(),
        time: vec![0.0, 1.0],
        waveforms: HashMap::new(),
        measurements: Vec::new(),
        periodic_state: None,
        events: Default::default(),
        convergence: Some(std::sync::Arc::new(quality)),
    };
    let transport = WorkerResponseTransport::from_response(
        WorkerResponse::from_result_for_transfer(8, Ok(result)),
    )
    .unwrap();
    let mut metadata = serde_json::to_value(&transport.response).unwrap();
    let index = metadata["outcome"]["Success"]["Transient"]["convergence"]["transient_indices"]["Buffer"]["buffer"].as_u64().unwrap() as usize;
    let mut malformed = transport.clone();
    malformed.buffers[index][0] = 0.5;
    assert!(
        malformed
            .into_response()
            .unwrap_err()
            .contains("integer limb")
    );
    metadata["outcome"]["Success"]["Transient"]["convergence"]["transient_indices"] =
        serde_json::json!({"Inline": [1.0, 0.0]});
    let mut inline = transport.clone();
    inline.response = serde_json::from_value(metadata).unwrap();
    assert!(
        inline
            .into_response()
            .unwrap_err()
            .contains("dedicated transfer buffers")
    );
    let mut obsolete = transport;
    obsolete.protocol = 17;
    assert!(
        obsolete
            .into_response()
            .unwrap_err()
            .contains("unsupported")
    );
}

/// A DC mismatch result survives the worker wire intact.
///
/// Every number the sheet draws and every trimming control that explains the
/// list crosses in one typed evidence value, so this walks the whole payload
/// rather than a sigma or two: a field dropped in transport would leave a
/// browser run drawing a cumulative share against the wrong denominator.
#[test]
fn a_dc_mismatch_result_survives_the_worker_wire() {
    use crate::state::{
        DcMismatchContributorEvidence, DcMismatchEvidence, DcMismatchScopeEvidence,
    };

    let evidence = DcMismatchEvidence {
        output: "V(OUT,IN)".to_owned(),
        output_unit: "V".to_owned(),
        nominal_value: 2.0 / 3.0,
        sigma_multiplier: 3.0,
        sigma_total: (5.0_f64).sqrt() * 1.0e-3,
        sigma_mismatch: 2.0e-3,
        sigma_process: 1.0e-3,
        include_mismatch: true,
        include_process: true,
        contributor_limit: 0,
        threshold: 0.05,
        normalized_contributions: false,
        applied_correlations_mismatch: 0,
        applied_correlations_process: 1,
        evaluated_contributors: 6,
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
                instance: "(design)".to_owned(),
                parameter: "XL".to_owned(),
                scope: DcMismatchScopeEvidence::Process,
                sigma_parameter: 1.0,
                sensitivity: -1.0e-3,
                contribution: -1.0e-3,
                share: -0.2,
            },
        ],
    };
    let result = round_trip_result(SimulationResult::DcMismatch {
        evidence: std::sync::Arc::new(evidence.clone()),
    });
    match result {
        SimulationResult::DcMismatch { evidence: restored } => {
            assert_eq!(*restored, evidence);
            // The signs are what make the allocation readable, so they are
            // checked as bit patterns rather than as magnitudes.
            assert_eq!(
                restored.contributors[1].share.to_bits(),
                (-0.2_f64).to_bits()
            );
        }
        other => panic!("expected dc mismatch result, got {other:?}"),
    }
}

/// A recorded FFT crosses the worker boundary with every bit intact.
///
/// The coefficients ride the transfer-buffer channel, so this goes through the
/// transport encoding rather than only the in-process conversion: the JSON
/// envelope carries lengths and indices, and the numbers are compared by bit
/// pattern, not by tolerance.
#[test]
fn a_recorded_fft_spectrum_survives_the_worker_boundary_bit_for_bit() {
    use crate::simulation::results::RecordedFftSpectrum;
    use crate::state::{
        FftSpectrumEvidence, FftSpectrumFormatEvidence, FftSpectrumModeEvidence,
        FftSpectrumStatusEvidence,
    };

    let point_count = 8usize;
    let bins = point_count / 2 + 1;
    let resolution = 125.0_f64;
    let spectrum = RecordedFftSpectrum {
        request_key: ".fft V(OUT) NP=8 WINDOW=RECT".to_owned(),
        evidence: FftSpectrumEvidence {
            status: FftSpectrumStatusEvidence::Complete,
            output: "V(OUT)".to_owned(),
            physical_type: "voltage".to_owned(),
            start_time_s: 0.0,
            stop_time_s: 1.0 / resolution,
            sample_interval_s: (1.0 / resolution) / point_count as f64,
            point_count,
            accurate_sampling: true,
            format: FftSpectrumFormatEvidence::Unnormalized,
            mode: FftSpectrumModeEvidence::HspiceCompatible,
            window: "RECT".to_owned(),
            alpha: 3.0,
            coherent_gain: 1.0,
            frequency_resolution_hz: resolution,
            fundamental_bin: 1,
            minimum_metric_bin: 1,
            maximum_metric_bin: point_count / 2,
            metrics: None,
        },
        frequency: (0..bins).map(|bin| bin as f64 * resolution).collect(),
        real: vec![0.1, 0.987_654_321_012_345_6, -0.25, 1.0e-17, 0.0],
        imaginary: vec![0.0, -0.123_456_789_012_345_6, 0.5, -1.0e-17, 0.0],
    };
    spectrum
        .validate()
        .expect("the fixture is a valid spectrum");
    let recorded = std::sync::Arc::new(spectrum.clone());

    // The transient that computed it carries it out.
    let carried = SimulationResult::Transient {
        time: vec![0.0, 1e-9, 2e-9],
        waveforms: HashMap::new(),
        measurements: Vec::new(),
        periodic_state: None,
        convergence: None,
        events: Default::default(),
        spectra: vec![std::sync::Arc::clone(&recorded)],
    };
    let SimulationResult::Transient {
        spectra: restored, ..
    } = round_trip_result(carried)
    else {
        panic!("a transient result");
    };
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].as_ref(), &spectrum);

    // And the FFT task's own result, through the transfer-buffer transport.
    let result = SimulationResult::Fft {
        spectrum: std::sync::Arc::clone(&recorded),
        convergence: None,
    };
    let transport = WorkerResponseTransport::from_response(
        WorkerResponse::from_result_for_transfer(9, Ok(result)),
    )
    .expect("the response encodes");
    let response = transport.into_response().expect("the response decodes");
    let WorkerOutcome::Success(worker) = response.outcome else {
        panic!("a successful response");
    };
    let SimulationResult::Fft { spectrum: back, .. } = SimulationResult::from(*worker) else {
        panic!("an FFT result");
    };
    for (actual, expected) in back
        .frequency
        .iter()
        .chain(&back.real)
        .chain(&back.imaginary)
        .zip(
            spectrum
                .frequency
                .iter()
                .chain(&spectrum.real)
                .chain(&spectrum.imaginary),
        )
    {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    assert_eq!(back.evidence, spectrum.evidence);
    assert_eq!(back.request_key, spectrum.request_key);
}

#[test]
fn monte_carlo_mean_confidence_survives_worker_transport_and_rejects_wrong_population() {
    use crate::state::{MonteCarloMeanConfidence, MonteCarloMeanInterval, MonteCarloMeanMethod};
    let confidence = MonteCarloMeanConfidence {
        level_pct: 90.0,
        method: MonteCarloMeanMethod::PercentileBootstrap {
            resamples: 1000,
            seed: u64::MAX,
        },
        successful_samples: 3,
        conditional_on_successful_trials: true,
        interval: MonteCarloMeanInterval::Available {
            lower: 0.9,
            upper: 1.1,
        },
    };
    let members = [Some(0.9), Some(1.0), None, Some(1.1)]
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            crate::state::FamilyMemberMeasurements::new(
                crate::state::FamilyMemberId::MonteCarloSequenceTrial {
                    index,
                    seed: 7,
                    policy: "parameter-xoroshiro128plus-2018-v1".into(),
                },
                vec![crate::state::FamilyMeasurementEvidence {
                    name: "V(out)".into(),
                    value,
                    passed: value.is_some(),
                    error: value.is_none().then(|| "did not converge".into()),
                }],
            )
        })
        .collect::<Vec<_>>();
    let result = SimulationResult::MonteCarlo {
        seed: 7,
        runs_requested: 4,
        runs_completed: 3,
        num_failures: 1,
        all_converged: false,
        member_measurements: members.clone(),
        variables: vec![crate::simulation::results::MonteCarloVariableResult {
            mean_confidence: Some(confidence),
            name: "V(out)".into(),
            samples: vec![0.9, 1.0, 1.1],
            mean: 1.0,
            std_dev: 0.1,
            min: 0.9,
            max: 1.1,
            histogram: vec![1, 2],
            bin_edges: vec![0.9, 1.0, 1.1],
        }],
    };
    let SimulationResult::MonteCarlo {
        variables,
        member_measurements,
        ..
    } = round_trip_result(result)
    else {
        panic!("Monte Carlo")
    };
    assert_eq!(variables[0].mean_confidence, Some(confidence));
    assert_eq!(member_measurements, members);
    let mut misattributed = members;
    misattributed[3].member = misattributed[0].member.clone();
    let response = WorkerResponse {
        id: 1,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::MonteCarlo {
            seed: 7,
            runs_requested: 4,
            runs_completed: 3,
            num_failures: 1,
            all_converged: false,
            member_measurements: misattributed,
            variables: vec![WorkerMonteCarloVariable::from(variables[0].clone())],
        })),
    };
    assert!(
        WorkerResponseTransport::from_response(response)
            .unwrap_err()
            .contains("trial population")
    );
    let mut worker = WorkerMonteCarloVariable::from(variables[0].clone());
    worker.mean_confidence.as_mut().unwrap().successful_samples = 4;
    let response = WorkerResponse {
        id: 1,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::MonteCarlo {
            seed: 7,
            runs_requested: 4,
            runs_completed: 3,
            num_failures: 1,
            all_converged: false,
            member_measurements: Vec::new(),
            variables: vec![worker],
        })),
    };
    assert!(
        WorkerResponseTransport::from_response(response)
            .unwrap_err()
            .contains("population")
    );
}
