use super::*;

#[test]
fn test_run_monte_carlo_analysis_executes_mc_command() {
    let netlist = "* mc\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 6 SEED 3 DIST GAUSS SPREAD 0.01 PARAMS RVAL\n.end\n";
    let result = run_monte_carlo_analysis(netlist).expect("Monte Carlo analysis should execute");
    assert_eq!(result.runs_requested, 6);
    assert!(result.runs_completed > 0);
    assert!(!result.variables.is_empty());
    assert!(result.variables.iter().all(|var| var.mean.is_finite()));
}

#[test]
fn test_run_monte_carlo_analysis_supports_worst_case_distribution() {
    let netlist = "* mc worst\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 8 DIST WORSTCASE SPREAD 0.03 PARAMS RVAL\n.end\n";
    let result =
        run_monte_carlo_analysis(netlist).expect("Monte Carlo WORSTCASE analysis should execute");
    assert_eq!(result.runs_requested, 8);
    assert!(result.runs_completed > 0);
    assert!(!result.variables.is_empty());
}

#[test]
fn test_run_monte_carlo_analysis_requires_command() {
    let err = run_monte_carlo_analysis("* no mc\nV1 in 0 1\nR1 in 0 1k\n")
        .expect_err("missing .MC command should fail");
    assert!(err.contains(".MC command"));
}

#[test]
fn test_run_parametric_analysis_executes_step_param_command() {
    let netlist = "* step param\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.STEP PARAM RVAL 1k 4k 1k\n.end\n";
    let result = run_parametric_analysis(netlist).expect("parametric .STEP PARAM should execute");
    assert_eq!(result.target, "PARAM RVAL");
    assert_eq!(result.sweep_values.len(), 4);
    assert_eq!(result.num_points, 4);
    assert!(result
        .voltages
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("V(out)")));
}

#[test]
fn test_run_parametric_analysis_executes_step_temp_command() {
    let netlist =
        "* step temp\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.STEP TEMP LIST -40 27 125\n.end\n";
    let result = run_parametric_analysis(netlist).expect("parametric .STEP TEMP should execute");
    assert_eq!(result.target, "TEMP");
    assert_eq!(result.sweep_values, vec![-40.0, 27.0, 125.0]);
    assert_eq!(result.num_points, 3);
}

#[test]
fn test_run_parametric_analysis_with_config_dc_base_mode() {
    let netlist = "* step temp dc\nVDD in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n";
    let cfg = TempRunConfig {
        temperatures_c: vec![-40.0, 25.0, 125.0],
        base_mode: CornerBaseMode::DcSweep {
            source_name: "VDD".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.25,
        },
    };

    let result = run_parametric_analysis_with_config(netlist, &cfg)
        .expect("temperature sweep DC base mode should execute");
    assert_eq!(result.target, "TEMP");
    assert_eq!(result.sweep_values, cfg.temperatures_c);
    assert_eq!(result.num_points, 3);
    let trace = result
        .voltages
        .iter()
        .find(|(_, values)| values.len() == 3 && values.iter().all(|value| value.is_finite()))
        .expect("expected finite temperature trace");
    assert!(trace.0.starts_with("V("));
}

#[test]
fn test_run_parametric_analysis_with_config_transient_base_mode() {
    let netlist = "* step temp tran\nVDD vdd 0 1.0\nR1 vdd out 1k\nC1 out 0 1n\n.end\n";
    let cfg = TempRunConfig {
        temperatures_c: vec![-40.0, 25.0, 125.0],
        base_mode: CornerBaseMode::Transient {
            stop_time: 2e-6,
            step_time: 2e-8,
        },
    };

    let result = run_parametric_analysis_with_config(netlist, &cfg)
        .expect("temperature sweep transient base mode should execute");
    assert_eq!(result.target, "TEMP");
    assert_eq!(result.num_points, 3);
    let out = result
        .voltages
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
        .expect("expected V(out) waveform");
    assert_eq!(out.1.len(), 3);
    assert!(out.1.iter().all(|value| value.is_finite()));
}

#[test]
fn test_run_parametric_analysis_with_config_ac_base_mode() {
    let netlist = "* step temp ac\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = TempRunConfig {
        temperatures_c: vec![-40.0, 25.0, 125.0],
        base_mode: CornerBaseMode::Ac {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: CornerFrequencySweep::Decade,
        },
    };

    let result = run_parametric_analysis_with_config(netlist, &cfg)
        .expect("temperature sweep AC base mode should execute");
    assert_eq!(result.target, "TEMP");
    assert_eq!(result.num_points, 3);
    assert!(result
        .voltages
        .iter()
        .any(|(name, values)| name.eq_ignore_ascii_case("|V(out)|")
            && values.len() == 3
            && values.iter().all(|v| v.is_finite() && *v >= 0.0)));
}

#[test]
fn test_run_parametric_analysis_requires_step_command() {
    let err = run_parametric_analysis("* no step\nV1 in 0 1\nR1 in 0 1k\n")
        .expect_err("missing .STEP command should fail");
    assert!(err.contains(".STEP command"));
}

#[test]
fn test_run_corner_analysis_executes_temp_directives() {
    let netlist = "* corners\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.TEMP -40 27 125\n.end\n";
    let result = run_corner_analysis(netlist).expect("corner analysis should execute");
    assert_eq!(result.x_label, "Temperature");
    assert_eq!(result.x_unit, "C");
    assert_eq!(result.x_values, vec![-40.0, 27.0, 125.0]);
    assert_eq!(result.temperatures_c, vec![-40.0, 27.0, 125.0]);
    assert_eq!(result.corner_labels.len(), 3);
    assert!(result.corner_labels[0].starts_with("TT_1.000000V_"));
    assert_eq!(result.num_points, 3);
    assert!(result
        .voltages
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("V(out)")));
}

#[test]
fn test_run_corner_analysis_requires_temp_command() {
    let err = run_corner_analysis("* no corners\nV1 in 0 1\nR1 in 0 1k\n")
        .expect_err("missing .TEMP command should fail");
    assert!(err.contains(".TEMP"));
}

#[test]
fn test_run_corner_analysis_with_config_executes_full_matrix() {
    let netlist = "* corners cfg\nVDD vdd 0 1.0\nR1 vdd out 1k\nR2 out 0 1k\n.end\n";
    let cfg = CornerRunConfig {
        process_corners: vec![CornerProcess::TT, CornerProcess::FF],
        voltages: vec![0.9, 1.1],
        temperatures_c: vec![-40.0, 125.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::Op,
    };

    let result = run_corner_analysis_with_config(netlist, &cfg)
        .expect("corner analysis with explicit config should execute");
    assert_eq!(result.num_points, 8);
    assert_eq!(result.x_label, "Corner Index");
    assert_eq!(result.x_unit, "");
    assert_eq!(
        result.x_values,
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]
    );
    assert_eq!(result.temperatures_c.len(), 8);
    assert_eq!(result.corner_labels.len(), 8);
    assert_eq!(result.num_failures, 0);
    assert!(result
        .corner_labels
        .iter()
        .any(|label| label.contains("FF_1.100000V_125.000000C")));
}

#[test]
fn test_run_corner_analysis_with_config_executes_diagonal_mode() {
    let netlist = "* corners diag\nVDD vdd 0 1.0\nR1 vdd out 1k\nR2 out 0 1k\n.end\n";
    let cfg = CornerRunConfig {
        process_corners: vec![CornerProcess::SS, CornerProcess::TT, CornerProcess::FF],
        voltages: vec![0.95, 1.0],
        temperatures_c: vec![25.0],
        full_matrix: false,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::Op,
    };

    let result = run_corner_analysis_with_config(netlist, &cfg)
        .expect("diagonal corner analysis should execute");
    assert_eq!(result.num_points, 3);
    assert_eq!(result.corner_labels.len(), 3);
    assert!(result.corner_labels[0].starts_with("SS_0.950000V_25.000000C"));
    assert!(result.corner_labels[1].starts_with("TT_1.000000V_25.000000C"));
    assert!(result.corner_labels[2].starts_with("FF_0.950000V_25.000000C"));
}

#[test]
fn test_run_corner_analysis_with_config_rejects_invalid_voltage() {
    let netlist = "* corners invalid\nV1 in 0 1\nR1 in 0 1k\n.end\n";
    let cfg = CornerRunConfig {
        process_corners: vec![CornerProcess::TT],
        voltages: vec![0.0],
        temperatures_c: vec![25.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::Op,
    };
    let err = run_corner_analysis_with_config(netlist, &cfg)
        .expect_err("invalid voltage corner must be rejected");
    assert!(err.contains("voltage corners"));
}

#[test]
fn test_run_corner_analysis_with_config_transient_base_mode() {
    let netlist = "* corners tran\nVDD vdd 0 1.0\nR1 vdd out 1k\nC1 out 0 1n\n.end\n";
    let cfg = CornerRunConfig {
        process_corners: vec![CornerProcess::TT],
        voltages: vec![0.9, 1.1],
        temperatures_c: vec![25.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::Transient {
            stop_time: 2e-6,
            step_time: 2e-8,
        },
    };

    let result = run_corner_analysis_with_config(netlist, &cfg)
        .expect("corner transient base mode should execute");
    assert_eq!(result.num_points, 2);
    let out = result
        .voltages
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
        .expect("expected V(out) waveform");
    assert_eq!(out.1.len(), 2);
    assert!(out.1.iter().all(|value| value.is_finite()));
}

#[test]
fn test_run_corner_analysis_with_config_ac_base_mode() {
    let netlist = "* corners ac\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = CornerRunConfig {
        process_corners: vec![CornerProcess::TT],
        voltages: vec![1.0],
        temperatures_c: vec![-40.0, 25.0, 125.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::Ac {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 10,
            sweep: CornerFrequencySweep::Decade,
        },
    };

    let result =
        run_corner_analysis_with_config(netlist, &cfg).expect("corner AC base mode should run");
    assert_eq!(result.num_points, 3);
    assert_eq!(result.x_label, "Temperature");
    assert_eq!(result.x_unit, "C");
    assert_eq!(result.x_values, vec![-40.0, 25.0, 125.0]);
    assert!(result
        .voltages
        .iter()
        .any(|(name, values)| name.eq_ignore_ascii_case("|V(out)|")
            && values.len() == 3
            && values.iter().all(|v| v.is_finite() && *v >= 0.0)));
}

#[test]
fn test_run_corner_analysis_with_config_dc_sweep_base_mode() {
    let netlist = "* corners dc\nVDD in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n";
    let cfg = CornerRunConfig {
        process_corners: vec![CornerProcess::TT, CornerProcess::FF],
        voltages: vec![1.0],
        temperatures_c: vec![25.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::DcSweep {
            source_name: "VDD".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.2,
        },
    };

    let result = run_corner_analysis_with_config(netlist, &cfg)
        .expect("corner DC sweep base mode should execute");
    assert_eq!(result.num_points, 2);
    assert_eq!(result.x_label, "Corner Index");
    assert_eq!(result.x_unit, "");
    assert_eq!(result.x_values, vec![0.0, 1.0]);
    let trace = result
        .voltages
        .iter()
        .find(|(_, values)| values.len() == 2 && values.iter().all(|value| value.is_finite()))
        .expect("expected at least one finite corner trace");
    assert!(trace.0.starts_with("V("));
}

#[test]
fn test_run_corner_analysis_with_config_rejects_invalid_dc_base_mode_step() {
    let netlist = "* corners invalid dc\nV1 in 0 1\nR1 in 0 1k\n.end\n";
    let cfg = CornerRunConfig {
        process_corners: vec![CornerProcess::TT],
        voltages: vec![1.0],
        temperatures_c: vec![25.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        base_mode: CornerBaseMode::DcSweep {
            source_name: "V1".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.0,
        },
    };
    let err = run_corner_analysis_with_config(netlist, &cfg)
        .expect_err("invalid corner DC step must be rejected");
    assert!(err.contains("step cannot be zero"));
}
