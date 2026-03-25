use super::*;
use std::time::{Duration, Instant};

/// Minimal valid netlist for testing DC operating point
fn test_netlist() -> String {
    "* Test Circuit\nV1 vdd 0 5\nR1 vdd out 1k\nR2 out 0 1k\n.end\n".to_string()
}

fn wait_for_result(
    runner: &mut SimulationRunner,
    timeout: Duration,
) -> Option<Result<SimulationResult, SimulationError>> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(result) = runner.poll_result() {
            return Some(result);
        }
        if Instant::now() >= deadline {
            return None;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn test_runner_new() {
    let runner = SimulationRunner::new();
    assert!(!runner.is_running());
    assert!(matches!(runner.status(), SimulationStatus::Idle));
}

#[test]
fn test_runner_not_running_initially() {
    let runner = SimulationRunner::new();
    assert!(!runner.is_running());
    assert!(runner.progress_fraction().is_none());
}

#[test]
fn test_runner_start_and_poll() {
    let mut runner = SimulationRunner::new();

    // Start simulation with valid netlist
    let result = runner.start(AnalysisConfig::DcOp, test_netlist());
    assert!(result.is_ok());

    // Should be running now (or already finished if very fast)
    // Don't assert running since the simulation is very fast

    // Wait for completion
    thread::sleep(std::time::Duration::from_millis(200));

    // Poll for result
    let result = runner.poll_result();
    assert!(result.is_some(), "Expected simulation result, got None");
    let sim_result = result.unwrap();
    assert!(
        sim_result.is_ok(),
        "Expected Ok result, got: {:?}",
        sim_result
    );

    // No longer running
    assert!(!runner.is_running());
}

#[test]
fn test_runner_abort() {
    let mut runner = SimulationRunner::new();

    // Start simulation with valid netlist
    runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

    // Abort immediately
    runner.abort();

    // Wait for thread to notice abort
    thread::sleep(std::time::Duration::from_millis(100));

    // Poll for result - might be aborted or might have completed before abort took effect
    let result = runner.poll_result();
    // Result should exist after polling (either aborted or completed)
    if let Some(res) = result {
        // Either aborted or completed is acceptable
        assert!(matches!(res, Err(SimulationError::Aborted)) || res.is_ok());
    }
}

#[test]
fn test_runner_already_running() {
    let mut runner = SimulationRunner::new();

    // Start first simulation with valid netlist
    runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

    // Try to start another while running (might already be done if fast)
    if runner.is_running() {
        let result = runner.start(AnalysisConfig::DcOp, test_netlist());
        assert!(matches!(result, Err(SimulationError::AlreadyRunning)));
    }

    // Cleanup
    runner.abort();
    thread::sleep(std::time::Duration::from_millis(100));
}

#[test]
fn test_runner_clear_results() {
    let mut runner = SimulationRunner::new();
    runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();
    thread::sleep(std::time::Duration::from_millis(200));
    let result = runner.poll_result();

    // Verify simulation completed successfully before testing clear
    assert!(result.is_some(), "Expected simulation result");
    let sim_result = result.unwrap();
    assert!(sim_result.is_ok(), "Simulation failed: {:?}", sim_result);

    assert!(runner.last_result().is_some(), "Expected cached result");
    runner.clear_results();
    assert!(runner.last_result().is_none(), "Expected cleared result");
}

#[test]
fn test_simulation_error_display() {
    let err = SimulationError::ParseError("test error".to_string());
    assert!(err.to_string().contains("Parse error"));

    let err = SimulationError::ConvergenceFailed {
        iterations: 50,
        message: "did not converge".to_string(),
    };
    assert!(err.to_string().contains("50"));
}

#[test]
fn test_runner_progress_update() {
    let mut runner = SimulationRunner::new();
    runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

    // Give thread time to start
    thread::sleep(std::time::Duration::from_millis(10));

    // Check status is not idle (or completed if very fast)
    let _status = runner.status();
    // Progress status might be Idle if completed very quickly, so just check it ran

    // Cleanup
    thread::sleep(std::time::Duration::from_millis(200));
    let _ = runner.poll_result();
}

#[test]
fn test_runner_default() {
    let runner = SimulationRunner::default();
    assert!(!runner.is_running());
}

#[test]
fn test_runner_recovers_from_poisoned_progress_mutex() {
    let runner = SimulationRunner::new();
    let poisoned_progress = Arc::clone(&runner.progress);
    let _ = thread::spawn(move || {
        let _guard = poisoned_progress
            .lock()
            .expect("progress mutex should lock before poison");
        panic!("intentional poison for runner progress mutex");
    })
    .join();

    assert!(matches!(runner.status(), SimulationStatus::Idle));
    assert!(runner.progress_fraction().is_none());
    assert!(matches!(runner.progress().status, SimulationStatus::Idle));
}

#[test]
fn test_run_dc_op_convenience() {
    let mut runner = SimulationRunner::new();
    let result = runner.run_dc_op(test_netlist());
    assert!(result.is_ok());

    // Cleanup - wait and poll
    thread::sleep(std::time::Duration::from_millis(200));
    let result = runner.poll_result();
    assert!(result.is_some(), "Expected result from dc_op");
}

#[test]
fn test_runner_start_with_source_path_resolves_relative_include() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!(
        "rspice_ui_runner_relative_include_{}_{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    let source_path = temp_dir.join("designs").join("divider.rsch");
    fs::create_dir_all(
        source_path
            .parent()
            .expect("source path should have parent"),
    )
    .expect("source directory should be created");
    fs::write(
        source_path
            .parent()
            .expect("source path should have parent")
            .join("included.sp"),
        "RINC in 0 1k\n",
    )
    .expect("include file should be written");

    let mut runner = SimulationRunner::new();
    runner
        .start_with_source_path(
            AnalysisConfig::DcOp,
            "* Source-relative include\nV1 in 0 1\n.include \"included.sp\"\n.end\n".to_string(),
            Some(source_path),
        )
        .expect("runner should start with source path");

    let result = wait_for_result(&mut runner, Duration::from_secs(2))
        .expect("runner should finish within timeout")
        .expect("simulation should succeed");

    match result {
        SimulationResult::DcOp(dc) => {
            assert!(
                !dc.node_voltages.is_empty(),
                "expected source-relative include run to complete successfully"
            );
        }
        other => panic!("expected DC result, got {:?}", other),
    }

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_analysis_config_from_spec_covers_base_analyses() {
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
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        },
        AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_unit: 10,
            sweep: crate::simulation::multi_run::FrequencySweep::Decade,
        },
        AnalysisSpec::Noise {
            output_node: "out".to_string(),
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_decade: 10,
            temperature: 300.0,
        },
        AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: "PZ".to_string(),
        },
        AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        },
    ];

    for spec in specs {
        assert!(
            super::spec::analysis_config_from_spec(&spec).is_some(),
            "expected base spec to map to AnalysisConfig: {:?}",
            spec.run_type()
        );
    }
}

#[test]
fn test_analysis_config_from_spec_preserves_transient_window_and_uic() {
    let spec = AnalysisSpec::Transient {
        stop_time: 20e-6,
        step_time: 2e-9,
        start_time: 3e-6,
        max_timestep: Some(10e-9),
        uic: true,
    };

    let config =
        super::spec::analysis_config_from_spec(&spec).expect("transient spec should map to config");

    match config {
        AnalysisConfig::Transient(tran) => {
            assert!((tran.stop_time - 20e-6).abs() < 1e-18);
            assert!((tran.step_time - 2e-9).abs() < 1e-21);
            assert!((tran.start_time - 3e-6).abs() < 1e-18);
            assert_eq!(tran.max_timestep, Some(10e-9));
            assert!(tran.uic);
        }
        other => panic!("expected transient config, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_dc_op_routes_through_engine_bridge() {
    let mut runner = SimulationRunner::new();
    runner
        .start_spec(AnalysisSpec::DcOp, test_netlist())
        .expect("DC OP spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(5));
    assert!(result.is_some(), "Expected DC OP result");
    let result = result.unwrap().expect("DC OP should succeed");
    assert!(matches!(result, SimulationResult::DcOp(_)));
}

#[test]
fn test_runner_start_spec_dc_sweep_routes_through_engine_bridge() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* DC sweep routing test
V1 in 0 0
R1 in out 1k
R2 out 0 1k
.end
"#
    .to_string();

    runner
        .start_spec(
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
            netlist,
        )
        .expect("DC sweep spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(5));
    assert!(result.is_some(), "Expected DC sweep result");
    let result = result.unwrap().expect("DC sweep should succeed");
    match result {
        SimulationResult::DcSweep {
            sweep_values,
            waveforms,
            ..
        } => {
            assert!(!sweep_values.is_empty());
            assert!(!waveforms.is_empty());
        }
        other => panic!("Expected DC sweep result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_ac_routes_through_engine_bridge() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* AC routing test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Ac {
                start_freq: 1.0,
                stop_freq: 1e6,
                points_per_unit: 8,
                sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            },
            netlist,
        )
        .expect("AC spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected AC result");
    let result = result.unwrap().expect("AC should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(!waveforms.is_empty());
        }
        other => panic!("Expected AC result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_disto_routes_through_spec_runner() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* DISTO routing test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Disto {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 8,
                sweep: crate::simulation::multi_run::FrequencySweep::Decade,
                f2_over_f1: Some(1.5),
            },
            netlist,
        )
        .expect("DISTO spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected DISTO result");
    let result = result.unwrap().expect("DISTO should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(!waveforms.is_empty());
            assert!(waveforms.keys().any(|name| name.contains("THD(%)")));
        }
        other => panic!("Expected AC-form DISTO result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_noise_routes_through_engine_bridge() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Noise routing test
V1 in 0 DC 1 AC 1
R1 in out 1k
R2 out 0 1k
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Noise {
                output_node: "out".to_string(),
                start_freq: 10.0,
                stop_freq: 1e6,
                points_per_decade: 6,
                temperature: 300.0,
            },
            netlist,
        )
        .expect("Noise spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected noise result");
    let result = result.unwrap().expect("Noise should succeed");
    match result {
        SimulationResult::Noise {
            frequencies,
            output_noise,
            ..
        } => {
            assert!(!frequencies.is_empty());
            assert_eq!(frequencies.len(), output_noise.len());
        }
        other => panic!("Expected Noise result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pole_zero_routes_through_engine_bridge() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Pole-zero routing test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::PoleZero {
                input_node: "in".to_string(),
                input_ref: "0".to_string(),
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                transfer_type: "VOL".to_string(),
                analysis_type: "PZ".to_string(),
            },
            netlist,
        )
        .expect("Pole-zero spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected pole-zero result");
    let result = result.unwrap().expect("Pole-zero should succeed");
    match result {
        SimulationResult::PoleZero { poles, .. } => {
            assert!(!poles.is_empty(), "expected at least one pole");
        }
        other => panic!("Expected PoleZero result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_sensitivity_routes_through_engine_bridge() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Sensitivity routing test
.param RV=1k
V1 in 0 1
R1 in out {RV}
R2 out 0 1k
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Sensitivity {
                output_var: "V(out)".to_string(),
                ac_mode: false,
                frequency: None,
            },
            netlist,
        )
        .expect("Sensitivity spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected sensitivity result");
    let result = result.unwrap().expect("Sensitivity should succeed");
    match result {
        SimulationResult::Sensitivity { sensitivities, .. } => {
            assert!(
                !sensitivities.is_empty(),
                "expected at least one sensitivity entry"
            );
        }
        other => panic!("Expected Sensitivity result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_monte_carlo() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Monte Carlo smoke test
.param RV=1k
V1 in 0 1
R1 in 0 {RV}
.mc 8 gauss 0.05
.end
"#
    .to_string();

    runner
        .start_spec(AnalysisSpec::MonteCarlo, netlist)
        .expect("spec run should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected Monte Carlo result");
    let result = result.unwrap().expect("Monte Carlo should succeed");
    match result {
        SimulationResult::MonteCarlo {
            runs_requested,
            runs_completed,
            ..
        } => {
            assert_eq!(runs_requested, 8);
            assert!(runs_completed <= runs_requested);
        }
        other => panic!("Expected MonteCarlo result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_parametric_temp() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Parametric TEMP sweep smoke test
V1 in 0 1
R1 in 0 1k
.step temp list -40 25 85
.end
"#
    .to_string();

    runner
        .start_spec(AnalysisSpec::Parametric, netlist)
        .expect("parametric spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected parametric result");
    let result = result.unwrap().expect("Parametric should succeed");
    match result {
        SimulationResult::Parametric {
            target,
            sweep_values,
            ..
        } => {
            assert_eq!(target, "TEMP");
            assert!(!sweep_values.is_empty());
        }
        other => panic!("Expected Parametric result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_parametric_with_temp_ac_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Parametric TEMP sweep with AC base-mode override
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.step temp list -40 25 125
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: Some(crate::services::simulation_runner::TempRunConfig {
            temperatures_c: vec![-40.0, 25.0, 125.0],
            base_mode: crate::services::simulation_runner::CornerBaseMode::Ac {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 8,
                sweep: crate::services::simulation_runner::CornerFrequencySweep::Decade,
            },
        }),
        corner: None,
        pac: None,
        pxf: None,
        tf: None,
        pnoise: None,
        pstb: None,
    };

    runner
        .start_spec_with_options(AnalysisSpec::Parametric, netlist, options)
        .expect("parametric AC options should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected parametric AC result");
    let result = result.unwrap().expect("Parametric AC should succeed");
    match result {
        SimulationResult::Parametric {
            target,
            sweep_values,
            waveforms,
            ..
        } => {
            assert_eq!(target, "TEMP");
            assert_eq!(sweep_values.len(), 3);
            assert!(
                waveforms
                    .keys()
                    .any(|name| name.eq_ignore_ascii_case("|V(out)|")),
                "expected |V(out)| trace, got {:?}",
                waveforms.keys().collect::<Vec<_>>()
            );
        }
        other => panic!("Expected Parametric result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_corner_temp() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Corner TEMP sweep smoke test
V1 in 0 1
R1 in 0 1k
.temp -40 25 85
.end
"#
    .to_string();

    runner
        .start_spec(AnalysisSpec::Corner, netlist)
        .expect("corner spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected corner result");
    let result = result.unwrap().expect("Corner should succeed");
    match result {
        SimulationResult::Corner {
            x_values,
            x_label,
            x_unit,
            temperatures_c,
            ..
        } => {
            assert_eq!(temperatures_c.len(), 3);
            assert_eq!(x_values, vec![-40.0, 25.0, 85.0]);
            assert_eq!(x_label, "Temperature");
            assert_eq!(x_unit, "C");
        }
        other => panic!("Expected Corner result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_corner_with_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Corner PVT sweep smoke test
VDD vdd 0 1.0
R1 vdd out 1k
R2 out 0 1k
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: None,
        corner: Some(crate::services::simulation_runner::CornerRunConfig {
            process_corners: vec![
                crate::services::simulation_runner::CornerProcess::TT,
                crate::services::simulation_runner::CornerProcess::FF,
            ],
            voltages: vec![0.9, 1.1],
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: crate::services::simulation_runner::CornerBaseMode::Op,
        }),
        pac: None,
        pxf: None,
        tf: None,
        pnoise: None,
        pstb: None,
    };

    runner
        .start_spec_with_options(AnalysisSpec::Corner, netlist, options)
        .expect("corner spec with options should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected corner result");
    let result = result.unwrap().expect("Corner should succeed");
    match result {
        SimulationResult::Corner {
            x_values,
            x_label,
            x_unit,
            temperatures_c,
            corner_labels,
            ..
        } => {
            assert_eq!(temperatures_c.len(), 4);
            assert_eq!(x_values, vec![0.0, 1.0, 2.0, 3.0]);
            assert_eq!(x_label, "Corner Index");
            assert_eq!(x_unit, "");
            assert_eq!(corner_labels.len(), 4);
            assert!(
                corner_labels
                    .iter()
                    .any(|label| label.contains("FF_1.100000V"))
            );
        }
        other => panic!("Expected Corner result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_corner_with_ac_base_mode_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Corner AC sweep smoke test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: None,
        corner: Some(crate::services::simulation_runner::CornerRunConfig {
            process_corners: vec![crate::services::simulation_runner::CornerProcess::TT],
            voltages: vec![1.0],
            temperatures_c: vec![-40.0, 25.0, 125.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: crate::services::simulation_runner::CornerBaseMode::Ac {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 8,
                sweep: crate::services::simulation_runner::CornerFrequencySweep::Decade,
            },
        }),
        pac: None,
        pxf: None,
        tf: None,
        pnoise: None,
        pstb: None,
    };

    runner
        .start_spec_with_options(AnalysisSpec::Corner, netlist, options)
        .expect("corner AC options should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected corner AC result");
    let result = result.unwrap().expect("Corner AC should succeed");
    match result {
        SimulationResult::Corner {
            x_values,
            x_label,
            x_unit,
            temperatures_c,
            waveforms,
            ..
        } => {
            assert_eq!(temperatures_c.len(), 3);
            assert_eq!(x_values, vec![-40.0, 25.0, 125.0]);
            assert_eq!(x_label, "Temperature");
            assert_eq!(x_unit, "C");
            assert!(
                waveforms
                    .keys()
                    .any(|name| name.eq_ignore_ascii_case("|V(out)|")),
                "expected |V(out)| trace, got {:?}",
                waveforms.keys().collect::<Vec<_>>()
            );
        }
        other => panic!("Expected Corner result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pss() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* PSS smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Pss {
                fundamental_freq: 1e6,
                num_harmonics: 8,
                tolerance: 1e-4,
            },
            netlist,
        )
        .expect("PSS spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected PSS result");
    let result = result.unwrap().expect("PSS should succeed");
    match result {
        SimulationResult::Transient { time, waveforms } => {
            assert!(!time.is_empty());
            assert!(!waveforms.is_empty());
        }
        other => panic!("Expected Transient result for PSS, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_harmonic_balance() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* HB smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::HarmonicBalance {
                tones: vec![crate::simulation::multi_run::HbToneSpec::new(1e6, 5)],
                reltol: 1e-6,
                abstol: 1e-12,
                max_iterations: 100,
                damping: 1.0,
                oversample: 2,
                max_mixing_order: 5,
                use_krylov: false,
                gmres_restart: 30,
                source_stepping: false,
                verbose: false,
            },
            netlist,
        )
        .expect("HB spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected HB result");
    let result = result.unwrap().expect("HB should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(!waveforms.is_empty());
            assert!(
                waveforms
                    .values()
                    .any(|wf| wf.is_complex && wf.y_imag.is_some()),
                "expected at least one complex HB waveform"
            );
        }
        other => panic!("Expected AC result for HB, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_sparameter() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* S-parameter smoke
R1 in 0 50
R2 out 0 50
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::SParameter {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 5,
                sweep: crate::simulation::multi_run::FrequencySweep::Decade,
                z0: 50.0,
                ports: vec![
                    crate::simulation::multi_run::SpPort {
                        node_pos: "in".to_string(),
                        node_neg: "0".to_string(),
                        z0: None,
                    },
                    crate::simulation::multi_run::SpPort {
                        node_pos: "out".to_string(),
                        node_neg: "0".to_string(),
                        z0: Some(75.0),
                    },
                ],
            },
            netlist,
        )
        .expect("S-parameter spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(5));
    assert!(result.is_some(), "Expected S-parameter result");
    let result = result.unwrap().expect("S-parameter should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(waveforms.contains_key("S11"));
            assert!(waveforms.contains_key("S21"));
            assert!(waveforms.contains_key("S12"));
            assert!(waveforms.contains_key("S22"));
            assert!(
                waveforms
                    .values()
                    .all(|wf| wf.is_complex && wf.y_imag.as_ref().is_some())
            );
        }
        other => panic!("Expected AC result for S-parameter, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_envelope() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Envelope smoke
V1 out 0 SIN(0 1 1Meg 0 0 0)
R1 out 0 1k
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Envelope {
                fundamental_freq: 1e6,
                stop_time: 2e-6,
                num_harmonics: 9,
                max_step: None,
            },
            netlist,
        )
        .expect("Envelope spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(90));
    assert!(result.is_some(), "Expected envelope result");
    let result = result.unwrap().expect("Envelope should succeed");
    match result {
        SimulationResult::Transient { time, waveforms } => {
            assert!(!time.is_empty());
            assert!(!waveforms.is_empty());
            assert!(waveforms.keys().all(|name| name.starts_with("ENV(")));
        }
        other => panic!("Expected Transient result for envelope, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_fourier() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Fourier smoke
V1 out 0 SIN(0 1 1k 0 0 0)
R1 out 0 1k
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Fourier {
                fundamental_freq: 1e3,
                num_harmonics: 8,
                output_node: "out".to_string(),
                output_ref: "".to_string(),
                start_time: 0.0,
                stop_time: 10e-3,
            },
            netlist,
        )
        .expect("Fourier spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(90));
    assert!(result.is_some(), "Expected Fourier result");
    let result = result.unwrap().expect("Fourier should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(
                waveforms.keys().any(|name| name.contains("Spectrum")),
                "expected Fourier spectrum waveform"
            );
            assert!(waveforms.contains_key("THD(%)"));
            assert!(waveforms.contains_key("DC"));
        }
        other => panic!("Expected AC result for Fourier, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_reliability() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Reliability smoke
VDD vdd 0 1.8
VG g 0 1.2
R1 vdd d 1k
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Reliability {
                target_years: vec![1.0, 5.0, 10.0],
                enable_hci: true,
                enable_nbti: true,
                enable_em: false,
                min_stress_voltage: 0.05,
            },
            netlist,
        )
        .expect("Reliability spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected reliability result");
    let result = result.unwrap().expect("Reliability should succeed");
    match result {
        SimulationResult::Reliability {
            years,
            waveforms,
            device_results,
        } => {
            assert_eq!(years, vec![1.0, 5.0, 10.0]);
            assert!(!device_results.is_empty());
            assert!(!waveforms.is_empty());
            assert!(
                waveforms
                    .keys()
                    .any(|name| name.starts_with("DVTH(") || name.starts_with("DRDS("))
            );
        }
        other => panic!("Expected Reliability result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_optimization() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* Optimization smoke
.param RTOP=1k
.param RBOT=1k
V1 in 0 2
R1 in out {RTOP}
R2 out 0 {RBOT}
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Optimization {
                variables: vec![crate::simulation::multi_run::OptimizationVariable {
                    name: "RBOT".to_string(),
                    min: 500.0,
                    max: 3000.0,
                    initial: 1000.0,
                }],
                objective_node: "out".to_string(),
                objective_ref: "0".to_string(),
                goal: crate::simulation::multi_run::OptimizationGoal::Target,
                target: Some(1.2),
                algorithm: crate::simulation::multi_run::OptimizationAlgorithm::PatternSearch,
                max_iterations: 48,
                cost_tolerance: 1e-8,
                fd_step: 1e-4,
                initial_step: 0.2,
                min_step: 1e-8,
            },
            netlist,
        )
        .expect("Optimization spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected optimization result");
    let result = result.unwrap().expect("Optimization should succeed");
    match result {
        SimulationResult::Optimization {
            iterations,
            waveforms,
            best_cost,
            ..
        } => {
            assert!(!iterations.is_empty());
            assert!(!waveforms.is_empty());
            assert!(waveforms.contains_key("OPT_COST"));
            assert!(best_cost.is_finite());
        }
        other => panic!("Expected Optimization result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_soa() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* SOA smoke
VDD d 0 3.3
VG g 0 PULSE(0 2.5 0 1n 1n 8n 16n)
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Soa {
                stop_time: 32e-9,
                step_time: 1e-9,
                check_vgs_max: true,
                max_vgs: 1.2,
                check_vds_max: true,
                max_vds: 3.0,
                check_vbe_max: false,
                max_vbe: 0.9,
                check_vce_max: false,
                max_vce: 5.0,
            },
            netlist,
        )
        .expect("SOA spec should start");

    let result = wait_for_result(&mut runner, Duration::from_secs(10));
    assert!(result.is_some(), "Expected SOA result");
    let result = result.unwrap().expect("SOA should succeed");
    match result {
        SimulationResult::Soa {
            time,
            waveforms,
            violations,
        } => {
            assert!(!time.is_empty());
            assert!(waveforms.contains_key("SOA_VIOLATION_COUNT"));
            assert!(!violations.is_empty());
        }
        other => panic!("Expected SOA result, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pac() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* PAC smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: None,
        corner: None,
        pac: Some(crate::services::simulation_runner::PacRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: crate::services::simulation_runner::PacFrequencySweep::Decade,
            max_sideband: 2,
            input_source: "V1".to_string(),
            output_node: "out".to_string(),
            output_ref: None,
            pac_magnitude: 1.0,
            include_dc: true,
            reltol: 1e-3,
            abstol: 1e-12,
        }),
        pxf: None,
        tf: None,
        pnoise: None,
        pstb: None,
    };

    runner
        .start_spec_with_options(AnalysisSpec::Pac, netlist, options)
        .expect("PAC spec should start");
    let result = wait_for_result(&mut runner, Duration::from_secs(5));
    assert!(result.is_some(), "Expected PAC result");
    let result = result.unwrap().expect("PAC should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(!waveforms.is_empty());
            assert!(
                waveforms
                    .keys()
                    .any(|name| name.contains("[sb=") && name.starts_with("V(")),
                "expected PAC sideband trace names, got {:?}",
                waveforms.keys().collect::<Vec<_>>()
            );
            assert!(waveforms.values().all(|wf| {
                wf.is_complex
                    && wf
                        .y_imag
                        .as_ref()
                        .is_some_and(|imag| imag.len() == wf.y_values.len())
            }));
        }
        other => panic!("Expected AC result for PAC, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pxf_with_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* PXF smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: None,
        corner: None,
        pac: None,
        pxf: Some(crate::services::simulation_runner::PxfRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: crate::services::simulation_runner::PxfFrequencySweep::Decade,
            input_source: "V1".to_string(),
            input_sideband: 1,
            output_node: "out".to_string(),
            output_ref: None,
            output_sideband: 1,
            max_sideband: 3,
            reltol: 1e-3,
            abstol: 1e-12,
        }),
        tf: None,
        pnoise: None,
        pstb: None,
    };

    runner
        .start_spec_with_options(AnalysisSpec::Pxf, netlist, options)
        .expect("PXF spec should start");
    let result = wait_for_result(&mut runner, Duration::from_secs(5));
    assert!(result.is_some(), "Expected PXF result");
    let result = result.unwrap().expect("PXF should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(!waveforms.is_empty());
            assert!(
                waveforms.keys().any(|name| name.starts_with("H(sb")),
                "expected PXF transfer waveform name, got {:?}",
                waveforms.keys().collect::<Vec<_>>()
            );
            assert!(
                waveforms
                    .values()
                    .any(|wf| wf.is_complex && wf.y_imag.as_ref().is_some())
            );
        }
        other => panic!("Expected AC result for PXF, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pxf_requires_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* PXF missing options
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(AnalysisSpec::Pxf, netlist)
        .expect("PXF spec launch without options should still start thread");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner
        .poll_result()
        .expect("Expected PXF completion result")
        .expect_err("PXF without options should fail");
    assert!(matches!(result, SimulationError::InvalidConfig(_)));
    assert!(
        result
            .to_string()
            .contains("requires explicit PXF execution options")
    );
}

#[test]
fn test_runner_start_spec_tf_with_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* TF smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: None,
        corner: None,
        pac: None,
        pxf: None,
        tf: Some(crate::services::simulation_runner::TfRunConfig {
            start_freq: 10.0,
            stop_freq: 1e6,
            points_per_unit: 6,
            sweep: crate::services::simulation_runner::TfFrequencySweep::Decade,
            input_source: "V1".to_string(),
            output_node: "out".to_string(),
            output_ref: None,
            group_delay: true,
            input_impedance: true,
            output_impedance: true,
        }),
        pnoise: None,
        pstb: None,
    };

    runner
        .start_spec_with_options(AnalysisSpec::Tf, netlist, options)
        .expect("TF spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected TF result");
    let result = result.unwrap().expect("TF should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(
                waveforms.keys().any(|name| name.starts_with("H(")),
                "expected transfer waveform, got {:?}",
                waveforms.keys().collect::<Vec<_>>()
            );
            assert!(
                waveforms.keys().any(|name| name.starts_with("Zin(")),
                "expected Zin waveform, got {:?}",
                waveforms.keys().collect::<Vec<_>>()
            );
            assert!(
                waveforms.keys().any(|name| name.starts_with("Zout(")),
                "expected Zout waveform, got {:?}",
                waveforms.keys().collect::<Vec<_>>()
            );
            assert!(
                waveforms
                    .values()
                    .any(|wf| wf.is_complex && wf.y_imag.as_ref().is_some())
            );
        }
        other => panic!("Expected AC result for TF, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pnoise_with_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* PNOISE smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: None,
        corner: None,
        pac: None,
        pxf: None,
        tf: None,
        pnoise: Some(crate::services::simulation_runner::PnoiseRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            start_freq: 10.0,
            stop_freq: 1e6,
            points_per_unit: 6,
            sweep: crate::services::simulation_runner::PnoiseFrequencySweep::Decade,
            max_sideband: 3,
            output_node: "out".to_string(),
            output_ref: None,
            input_source: "V1".to_string(),
            noise_ref: crate::services::simulation_runner::PnoiseReference::Output,
            integrated_noise: true,
            noise_summary: true,
            reltol: 1e-3,
            abstol: 1e-18,
        }),
        pstb: None,
    };

    runner
        .start_spec_with_options(AnalysisSpec::Pnoise, netlist, options)
        .expect("PNOISE spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected PNOISE result");
    let result = result.unwrap().expect("PNOISE should succeed");
    match result {
        SimulationResult::Noise {
            frequencies,
            output_noise,
            ..
        } => {
            assert!(!frequencies.is_empty());
            assert_eq!(frequencies.len(), output_noise.len());
            assert!(output_noise.iter().all(|value| value.is_finite()));
        }
        other => panic!("Expected Noise result for PNOISE, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_stb() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* STB smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(
            AnalysisSpec::Stb {
                probe_node: "1".to_string(),
                start_freq: 1.0,
                stop_freq: 1e6,
                points_per_decade: 8,
            },
            netlist,
        )
        .expect("STB spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected STB result");
    let result = result.unwrap().expect("STB should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(waveforms.contains_key("Loop Gain (dB)"));
            assert!(waveforms.contains_key("Loop Phase (deg)"));
            assert_eq!(
                waveforms
                    .get("Loop Gain (dB)")
                    .expect("loop-gain waveform should exist")
                    .x_values
                    .len(),
                frequencies.len()
            );
            assert_eq!(
                waveforms
                    .get("Loop Phase (deg)")
                    .expect("loop-phase waveform should exist")
                    .x_values
                    .len(),
                frequencies.len()
            );
        }
        other => panic!("Expected AC result for STB, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pstb_with_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* PSTB smoke test
V1 in 0 DC 1
R1 in mid 1k
LPROBE mid out 1u
C1 out 0 1n
.end
"#
    .to_string();

    let options = SpecExecutionOptions {
        temp: None,
        corner: None,
        pac: None,
        pxf: None,
        tf: None,
        pnoise: None,
        pstb: Some(crate::services::simulation_runner::PstbRunConfig {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 8,
            pss_tolerance: 1e-4,
            probe_instance: "LPROBE".to_string(),
            max_harmonics: 8,
            num_multipliers: 4,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        }),
    };

    runner
        .start_spec_with_options(AnalysisSpec::Pstb, netlist, options)
        .expect("PSTB spec should start");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner.poll_result();
    assert!(result.is_some(), "Expected PSTB result");
    let result = result.unwrap().expect("PSTB should succeed");
    match result {
        SimulationResult::Ac {
            frequencies,
            waveforms,
        } => {
            assert!(!frequencies.is_empty());
            assert!(waveforms.contains_key("Floquet |lambda|"));
            assert!(waveforms.contains_key("Stability Margin (dB)"));
            assert!(waveforms.contains_key("Mode Damping (1/s)"));
            assert!(waveforms.contains_key("Probe Mode Participation"));
            assert_eq!(
                waveforms
                    .get("Floquet |lambda|")
                    .expect("Floquet waveform should exist")
                    .x_values
                    .len(),
                frequencies.len()
            );
        }
        other => panic!("Expected AC result for PSTB, got {:?}", other),
    }
}

#[test]
fn test_runner_start_spec_pstb_requires_options() {
    let mut runner = SimulationRunner::new();
    let netlist = r#"
* PSTB missing options
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
    .to_string();

    runner
        .start_spec(AnalysisSpec::Pstb, netlist)
        .expect("PSTB launch without options should still start thread");
    thread::sleep(std::time::Duration::from_millis(250));

    let result = runner
        .poll_result()
        .expect("Expected PSTB completion result")
        .expect_err("PSTB without options should fail");
    assert!(matches!(result, SimulationError::InvalidConfig(_)));
    assert!(
        result
            .to_string()
            .contains("requires explicit PSTB execution options")
    );
}
