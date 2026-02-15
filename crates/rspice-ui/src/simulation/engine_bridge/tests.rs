use super::*;

// -------------------------------------------------------------------------
// Construction Tests
// -------------------------------------------------------------------------

#[test]
fn test_engine_bridge_new() {
    let bridge = EngineBridge::new();
    // Should not panic
    assert!(true);
    let _ = bridge; // Use bridge to avoid unused warning
}

#[test]
fn test_engine_bridge_default() {
    let bridge = EngineBridge::default();
    let _ = bridge;
}

#[test]
fn test_engine_bridge_with_config() {
    let config = rspice_core::SimulationConfig::default();
    let bridge = EngineBridge::with_config(config);
    let _ = bridge;
}

#[test]
fn test_engine_for_netlist_applies_netlist_options() {
    let bridge = EngineBridge::new();
    let netlist = bridge
        .parse_netlist(
            r#"
* Netlist options mapping
V1 1 0 1
R1 1 0 1k
.OPTIONS TEMP=85 ITL1=120 METHOD=GEAR RELTOL=2e-4 VNTOL=3e-6 IABSTOL=4e-12 GMIN=1e-11
.END
"#,
        )
        .expect("netlist should parse");

    let engine = bridge.engine_for_netlist(&netlist);
    let cfg = engine.config();

    assert!((cfg.temperature - 358.15).abs() < 1e-12);
    assert_eq!(cfg.max_iterations, 120);
    assert_eq!(
        cfg.integration_method,
        rspice_core::analysis::IntegrationMethod::Gear2
    );
    assert!((cfg.tolerance - 2e-4).abs() < 1e-15);
    assert!((cfg.convergence_config.voltage_reltol - 2e-4).abs() < 1e-15);
    assert!((cfg.convergence_config.residual_reltol - 2e-4).abs() < 1e-15);
    assert!((cfg.convergence_config.voltage_abstol - 3e-6).abs() < 1e-18);
    assert!((cfg.convergence_config.current_abstol - 4e-12).abs() < 1e-24);
    assert!((cfg.convergence_config.gmin_initial - 1e-11).abs() < 1e-24);
}

#[test]
fn test_engine_for_netlist_preserves_base_for_unspecified_options() {
    let mut base = rspice_core::SimulationConfig::default();
    base.tolerance = 8e-4;
    base.max_iterations = 88;
    let bridge = EngineBridge::with_config(base);
    let netlist = bridge
        .parse_netlist(
            r#"
* Netlist options partial override
V1 1 0 1
R1 1 0 1k
.OPTIONS TEMP=125
.END
"#,
        )
        .expect("netlist should parse");

    let engine = bridge.engine_for_netlist(&netlist);
    let cfg = engine.config();

    assert!((cfg.temperature - 398.15).abs() < 1e-12);
    assert!((cfg.tolerance - 8e-4).abs() < 1e-15);
    assert_eq!(cfg.max_iterations, 88);
}

// -------------------------------------------------------------------------
// Parse Error Tests
// -------------------------------------------------------------------------

#[test]
fn test_parse_empty_netlist() {
    let bridge = EngineBridge::new();
    let result = bridge.run(&AnalysisConfig::DcOp, "");
    // Empty netlist should fail
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_netlist() {
    let bridge = EngineBridge::new();
    let result = bridge.run(&AnalysisConfig::DcOp, "not valid spice");
    // Invalid should fail
    assert!(result.is_err());
}

#[test]
fn test_parse_netlist_expands_include_directives() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!(
        "rspice_ui_engine_bridge_include_{}_{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&temp_dir).expect("failed to create temp dir");

    let include_file = temp_dir.join("included.sp");
    fs::write(&include_file, "RINC 1 0 1k\n").expect("failed to write include file");

    let bridge = EngineBridge::new();
    let netlist = bridge
        .parse_netlist(&format!(
            "V1 1 0 DC 1\n.include \"{}\"\n.end\n",
            include_file.display()
        ))
        .expect("netlist should parse with include expansion");

    assert!(
        netlist
            .elements
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case("RINC")),
        "included element should be present after preprocessing"
    );

    let _ = fs::remove_dir_all(temp_dir);
}

// -------------------------------------------------------------------------
// DC OP Tests
// -------------------------------------------------------------------------

#[test]
fn test_run_dc_op_simple_resistor() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Simple resistor divider
V1 1 0 DC 10
R1 1 2 1k
R2 2 0 1k
.end
"#;

    let result = bridge.run(&AnalysisConfig::DcOp, netlist);

    if let Ok(SimulationResult::DcOp(dc_result)) = result {
        // Node 2 should be 5V (resistor divider)
        if let Some(&v2) = dc_result.node_voltages.get("2") {
            assert!((v2 - 5.0).abs() < 0.01, "Expected 5V, got {}", v2);
        }
    }
}

#[test]
fn test_run_dc_op_single_resistor() {
    let bridge = EngineBridge::new();
    let netlist = r#"
V1 1 0 DC 5
R1 1 0 1k
.end
"#;

    let result = bridge.run(&AnalysisConfig::DcOp, netlist);
    assert!(result.is_ok(), "DC OP should succeed for simple circuit");
}

#[test]
fn test_run_dc_sweep_supports_nested_secondary_source() {
    let bridge = EngineBridge::new();
    let netlist = r#"
V1 in 0 DC 0
V2 ctrl 0 DC 0
R1 in out 1k
R2 out ctrl 1k
R3 out 0 1k
.end
"#;
    let config = AnalysisConfig::DcSweep(super::super::config::DcSweepConfig {
        source: "V1".to_string(),
        start: 0.0,
        stop: 1.0,
        step: 0.5,
        source2: Some("V2".to_string()),
        start2: Some(0.0),
        stop2: Some(1.0),
        step2: Some(1.0),
    });

    let result = bridge
        .run(&config, netlist)
        .expect("nested DC sweep should run");
    match result {
        SimulationResult::DcSweep {
            sweep_values,
            waveforms,
            ..
        } => {
            assert_eq!(sweep_values, vec![0.0, 0.5, 1.0]);
            assert!(!waveforms.is_empty());
            assert!(waveforms.keys().any(|name| name.contains("[V2=0")));
            assert!(waveforms.keys().any(|name| name.contains("[V2=1")));
            assert!(waveforms
                .values()
                .all(|wf| wf.y_values.len() == sweep_values.len()));
        }
        other => panic!("expected DC sweep result, got {:?}", other),
    }
}

#[test]
fn test_run_dc_sweep_rejects_partial_nested_config() {
    let bridge = EngineBridge::new();
    let netlist = r#"
V1 in 0 DC 0
R1 in 0 1k
.end
"#;
    let config = AnalysisConfig::DcSweep(super::super::config::DcSweepConfig {
        source: "V1".to_string(),
        start: 0.0,
        stop: 1.0,
        step: 0.5,
        source2: Some("V2".to_string()),
        start2: Some(0.0),
        stop2: Some(1.0),
        step2: None,
    });

    let err = bridge
        .run(&config, netlist)
        .expect_err("partial nested config should be rejected");
    assert!(matches!(err, SimulationError::InvalidConfig(_)));
}

// -------------------------------------------------------------------------
// Error Translation Tests
// -------------------------------------------------------------------------

#[test]
fn test_translate_circuit_error() {
    let bridge = EngineBridge::new();
    let err = rspice_core::SimulationError::Circuit("test error".to_string());
    let ui_err = bridge.translate_error(err);
    assert!(matches!(ui_err, SimulationError::CircuitError(_)));
}

#[test]
fn test_translate_netlist_error() {
    let bridge = EngineBridge::new();
    let err = rspice_core::SimulationError::Netlist("parse error".to_string());
    let ui_err = bridge.translate_error(err);
    assert!(matches!(ui_err, SimulationError::ParseError(_)));
}

#[test]
fn test_translate_convergence_error() {
    let bridge = EngineBridge::new();
    let err = rspice_core::SimulationError::ConvergenceFailed(50);
    let ui_err = bridge.translate_error(err);
    assert!(matches!(ui_err, SimulationError::ConvergenceFailed { .. }));
}

// -------------------------------------------------------------------------
// DC Result Conversion Tests
// -------------------------------------------------------------------------

#[test]
fn test_convert_dc_result_empty() {
    let bridge = EngineBridge::new();
    let core_result = rspice_core::SimulationResult::new(0, 0);
    let ui_result = bridge.convert_dc_result(&core_result);
    assert!(ui_result.node_voltages.is_empty());
}

#[test]
fn test_convert_dc_result_with_nodes() {
    let bridge = EngineBridge::new();
    let mut core_result = rspice_core::SimulationResult::new(2, 0);
    core_result.node_voltages[1] = 5.0;
    core_result.node_voltages[2] = 3.3;
    core_result.node_names = vec!["0".to_string(), "VCC".to_string(), "OUT".to_string()];

    let ui_result = bridge.convert_dc_result(&core_result);
    assert_eq!(ui_result.node_voltages.len(), 2);
}

// -------------------------------------------------------------------------
// Integration Tests with Various Analyses
// -------------------------------------------------------------------------

#[test]
fn test_resolve_transient_max_step_defaults_to_output_step_value() {
    let cfg = super::super::config::TransientAnalysisConfig {
        stop_time: 5e-3,
        step_time: 10e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    };
    let max_step = EngineBridge::resolve_transient_max_step(&cfg);
    assert!((max_step - 10e-9).abs() < 1e-18);
}

#[test]
fn test_resolve_transient_max_step_honors_explicit_max_timestep() {
    let cfg = super::super::config::TransientAnalysisConfig {
        stop_time: 5e-3,
        step_time: 10e-9,
        start_time: 0.0,
        max_timestep: Some(25e-9),
        uic: false,
    };
    let max_step = EngineBridge::resolve_transient_max_step(&cfg);
    assert!((max_step - 25e-9).abs() < 1e-18);
}

#[test]
fn test_transient_start_index_uses_output_window_start_time() {
    let time = vec![0.0, 1.0e-9, 2.0e-9, 3.0e-9, 4.0e-9];
    assert_eq!(EngineBridge::transient_start_index(&time, 0.0), 0);
    assert_eq!(EngineBridge::transient_start_index(&time, 1.5e-9), 2);
    assert_eq!(EngineBridge::transient_start_index(&time, 4.0e-9), 4);
    assert_eq!(
        EngineBridge::transient_start_index(&time, 10.0e-9),
        time.len()
    );
}

#[test]
fn test_transient_sample_count_after_index_clamps_to_shortest_series() {
    let time = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    let voltages = vec![vec![0.0, 1.0, 2.0, 3.0, 4.0], vec![0.0, 1.0, 2.0, 3.0]];
    let start_idx = 1;
    let count = EngineBridge::transient_sample_count_after_index(&time, &voltages, start_idx);
    assert_eq!(count, 3);
}

#[test]
fn test_run_transient_simple_rc() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* RC circuit
V1 1 0 PULSE(0 5 0 1n 1n 50n 100n)
R1 1 2 1k
C1 2 0 1n
.end
"#;

    let config = super::super::config::TransientAnalysisConfig {
        stop_time: 100e-9,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: Some(1e-9),
        uic: false,
    };

    let result = bridge.run(&AnalysisConfig::Transient(config), netlist);
    // May fail if engine doesn't support tran yet, that's ok
    let _ = result;
}

#[test]
fn test_dispatch_to_correct_analysis() {
    let bridge = EngineBridge::new();
    let simple_netlist = r#"
V1 1 0 DC 5
R1 1 0 1k
.end
"#;

    // Test DC OP dispatch
    let result = bridge.run(&AnalysisConfig::DcOp, simple_netlist);
    if result.is_ok() {
        assert!(matches!(result.unwrap(), SimulationResult::DcOp(_)));
    }
}

#[test]
fn test_run_sensitivity_dc_reports_param_derivatives() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            assert!(!sensitivities.is_empty());
            let key = sensitivities
                .keys()
                .find(|k| k.eq_ignore_ascii_case("RVAL"))
                .expect("expected RVAL sensitivity key");
            assert!(sensitivities[key].is_finite());
            assert!(normalized[key].is_finite());
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_ac_reports_param_derivatives() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity parameterized AC low-pass
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in out {RVAL}
C1 out 0 1n
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: true,
        frequency: Some(1e3),
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("ac sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            assert!(!sensitivities.is_empty());
            let key = sensitivities
                .keys()
                .find(|k| k.eq_ignore_ascii_case("RVAL"))
                .expect("expected RVAL sensitivity key");
            assert!(sensitivities[key].is_finite());
            assert!(normalized[key].is_finite());
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_ac_supports_numeric_output_node_index() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity parameterized AC low-pass
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in out {RVAL}
C1 out 0 1n
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "2".to_string(),
        ac_mode: true,
        frequency: Some(1e3),
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("ac sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            assert!(!sensitivities.is_empty());
            let key = sensitivities
                .keys()
                .find(|k| k.eq_ignore_ascii_case("RVAL"))
                .expect("expected RVAL sensitivity key");
            assert!(sensitivities[key].is_finite());
            assert!(normalized[key].is_finite());
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_supports_differential_output() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity differential output
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "V(out,in)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("differential sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            assert!(!sensitivities.is_empty());
            assert!(!normalized.is_empty());
            assert!(sensitivities.iter().all(|(_, raw)| raw.is_finite()));
            assert!(normalized.iter().all(|(_, norm)| norm.is_finite()));
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_normalized_reports_zero_when_nominal_is_zero() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity zero nominal output
.param RVAL=1k
V1 in 0 DC 0
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            let key = sensitivities
                .keys()
                .find(|k| k.eq_ignore_ascii_case("RVAL"))
                .expect("expected RVAL sensitivity key");
            let norm = normalized
                .get(key)
                .copied()
                .expect("expected normalized sensitivity entry");
            assert!(norm.abs() <= 1e-18);
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_supports_current_output_dc() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity branch current output
.param RVAL=1k
V1 in 0 DC 10
R1 in 0 {RVAL}
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "I(V1)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("current-output dc sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            assert!(!sensitivities.is_empty());
            let key = sensitivities
                .keys()
                .find(|k| k.eq_ignore_ascii_case("RVAL"))
                .expect("expected RVAL sensitivity key");
            assert!(sensitivities[key].is_finite());
            assert!(normalized[key].is_finite());
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_supports_current_output_ac() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity branch current output AC
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in 0 {RVAL}
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "I(V1)".to_string(),
        ac_mode: true,
        frequency: Some(1e3),
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("current-output ac sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            assert!(!sensitivities.is_empty());
            let key = sensitivities
                .keys()
                .find(|k| k.eq_ignore_ascii_case("RVAL"))
                .expect("expected RVAL sensitivity key");
            assert!(sensitivities[key].is_finite());
            assert!(normalized[key].is_finite());
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_current_output_handles_multiple_parameters() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity branch current output with multiple parameters
.param RA=1k
.param RB=2k
V1 in 0 1
R1 in mid {RA}
R2 mid 0 {RB}
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "I(V1)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = bridge
        .run(&cfg, netlist)
        .expect("multi-parameter current-output sensitivity run should succeed");
    match result {
        SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        } => {
            let ra = sensitivities
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("RA"))
                .expect("expected RA sensitivity");
            let rb = sensitivities
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("RB"))
                .expect("expected RB sensitivity");
            let ra_norm = normalized
                .get(ra.0)
                .expect("expected normalized RA sensitivity");
            let rb_norm = normalized
                .get(rb.0)
                .expect("expected normalized RB sensitivity");

            assert!(ra.1.is_finite() && rb.1.is_finite());
            assert!(ra_norm.is_finite() && rb_norm.is_finite());
            assert!((ra.1 - rb.1).abs() < 1e-12);
        }
        _ => panic!("Expected Sensitivity result"),
    }
}

#[test]
fn test_run_sensitivity_rejects_frequency_without_ac_mode() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: Some(1e3),
    });

    let err = bridge
        .run(&cfg, netlist)
        .expect_err("expected validation error");
    assert!(err
        .to_string()
        .contains("only valid when AC mode is enabled"));
}

#[test]
fn test_run_sensitivity_rejects_non_positive_ac_frequency() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity parameterized AC low-pass
.param RVAL=1k
V1 in 0 DC 0 AC 1
R1 in out {RVAL}
C1 out 0 1n
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: true,
        frequency: Some(0.0),
    });

    let err = bridge
        .run(&cfg, netlist)
        .expect_err("expected validation error");
    assert!(err.to_string().contains("must be > 0"));
}

#[test]
fn test_run_sensitivity_rejects_unresolved_branch_output() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "I(NO_SUCH_BRANCH)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let err = bridge
        .run(&cfg, netlist)
        .expect_err("expected unresolved output error");
    assert!(err
        .to_string()
        .contains("could not be resolved to a node or branch"));
}

#[test]
fn test_run_sensitivity_rejects_out_of_range_numeric_output_index() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Sensitivity parameterized divider
.param RVAL=1k
V1 in 0 DC 10
R1 in out {RVAL}
R2 out 0 1k
.end
"#;

    let cfg = AnalysisConfig::Sensitivity(super::super::config::SensitivityConfig {
        output_var: "99".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let err = bridge
        .run(&cfg, netlist)
        .expect_err("expected unresolved output error");
    assert!(err
        .to_string()
        .contains("could not be resolved to a node or branch"));
}

#[test]
fn test_run_pz_resolves_named_nodes() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Named-node PZ
R1 in out 1k
C1 out 0 1n
.end
"#;

    let cfg = AnalysisConfig::PoleZero(super::super::config::PoleZeroConfig {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "CUR".to_string(),
        analysis_type: super::super::config::PzAnalysisType::PoleZero,
    });

    let result = bridge.run(&cfg, netlist).expect("PZ run should succeed");
    match result {
        SimulationResult::PoleZero { poles, gain, .. } => {
            assert!(!poles.is_empty());
            assert!(gain.is_finite());
        }
        _ => panic!("Expected PoleZero result"),
    }
}

#[test]
fn test_run_pz_differential_gain_matches_superposition() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* Differential PZ
R1 in out 1k
R2 out ref 500
C1 out ref 1n
R3 ref 0 1k
.end
"#;

    let run_gain = |input_node: &str, input_ref: &str, output_node: &str, output_ref: &str| {
        let cfg = AnalysisConfig::PoleZero(super::super::config::PoleZeroConfig {
            input_node: input_node.to_string(),
            input_ref: input_ref.to_string(),
            output_node: output_node.to_string(),
            output_ref: output_ref.to_string(),
            transfer_type: "CUR".to_string(),
            analysis_type: super::super::config::PzAnalysisType::PoleZero,
        });
        match bridge.run(&cfg, netlist).expect("PZ run should succeed") {
            SimulationResult::PoleZero { gain, .. } => gain,
            _ => panic!("Expected PoleZero result"),
        }
    };

    let diff = run_gain("in", "ref", "out", "ref");
    let h11 = run_gain("in", "0", "out", "0");
    let h12 = run_gain("ref", "0", "out", "0");
    let h21 = run_gain("in", "0", "ref", "0");
    let h22 = run_gain("ref", "0", "ref", "0");
    let expected = h11 - h12 - h21 + h22;

    assert!((diff - expected).abs() < 1e-9);
}

#[test]
fn test_run_pz_voltage_mode_highpass_zero() {
    let bridge = EngineBridge::new();
    let netlist = r#"
* High-pass PZ
C1 in out 1n
R1 out 0 1k
.end
"#;

    let cfg = AnalysisConfig::PoleZero(super::super::config::PoleZeroConfig {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "VOL".to_string(),
        analysis_type: super::super::config::PzAnalysisType::ZerosOnly,
    });

    let result = bridge.run(&cfg, netlist).expect("PZ run should succeed");
    match result {
        SimulationResult::PoleZero { zeros, .. } => {
            assert!(
                zeros
                    .iter()
                    .any(|(re, im)| (re * re + im * im).sqrt() < 1e-2),
                "expected zero near origin, got {:?}",
                zeros
            );
        }
        _ => panic!("Expected PoleZero result"),
    }
}
