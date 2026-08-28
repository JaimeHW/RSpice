//! Every result variant, out through the worker contract and back.
//!
//! One test rather than one per variant, deliberately: what it proves is that
//! the boundary is total. A variant that gained a field and lost it in transit
//! fails here, and a variant nobody remembered to encode fails here too, which
//! is the failure a per-variant suite misses because nobody writes the test for
//! the case they forgot.

use super::*;

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

    let ac = SimulationResult::Ac {
        frequencies: vec![1.0, 10.0],
        waveforms: HashMap::from([(
            "V(out)".to_string(),
            WaveformData::new_complex("V(out)", vec![1.0, 10.0], vec![0.5, 0.25], vec![-0.1, -0.2]),
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
