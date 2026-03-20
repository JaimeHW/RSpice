use super::*;

#[test]
fn test_extract_reliability_stress_data_maps_transistor_biases() {
    let elements = vec![
        Element {
            name: "M1".to_string(),
            kind: ElementKind::Mosfet {
                model: "NM".to_string(),
                mos_type: rspice_core::netlist::MosType::Nmos,
                instance_params: Vec::new(),
            },
            nodes: vec![
                "d".to_string(),
                "g".to_string(),
                "s".to_string(),
                "0".to_string(),
            ],
        },
        Element {
            name: "Q1".to_string(),
            kind: ElementKind::Bjt {
                model: "NPN".to_string(),
                bjt_type: rspice_core::netlist::BjtType::Npn,
                instance_params: Vec::new(),
            },
            nodes: vec!["c".to_string(), "b".to_string(), "e".to_string()],
        },
    ];
    let node_voltages = std::collections::HashMap::from([
        ("D".to_string(), 1.8),
        ("G".to_string(), 1.1),
        ("S".to_string(), 0.2),
        ("C".to_string(), 2.5),
        ("B".to_string(), 0.9),
        ("E".to_string(), 0.0),
    ]);

    let stress = extract_reliability_stress_data(&elements, &node_voltages, 300.0, 0.05);
    let m1 = stress.get("M1").expect("M1 stress should be extracted");
    assert!((m1.avg_vgs_stress - 0.9).abs() < 1e-12);
    assert!((m1.avg_vds_stress - 1.6).abs() < 1e-12);

    let q1 = stress.get("Q1").expect("Q1 stress should be extracted");
    assert!((q1.avg_vgs_stress - 0.9).abs() < 1e-12);
    assert!((q1.avg_vds_stress - 2.5).abs() < 1e-12);
}

#[test]
fn test_run_reliability_analysis_with_config_produces_device_results() {
    let netlist = r#"
* Reliability smoke
VDD vdd 0 1.8
VG g 0 1.2
R1 vdd d 1k
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#;
    let cfg = ReliabilityRunConfig {
        target_years: vec![1.0, 5.0, 10.0],
        enable_hci: true,
        enable_nbti: true,
        enable_em: false,
        min_stress_voltage: 0.05,
    };

    let data = run_reliability_analysis_with_config(netlist, &cfg)
        .expect("reliability analysis should execute for stressed MOSFET");
    assert_eq!(data.years, vec![1.0, 5.0, 10.0]);
    assert!(!data.device_results.is_empty());
    let m1 = data
        .device_results
        .iter()
        .find(|result| result.device_id.eq_ignore_ascii_case("M1"))
        .expect("M1 should be present in reliability results");
    assert!(m1.shifts.contains_key("1y"));
    assert!(m1.shifts.contains_key("10y"));
    assert!(m1.shifts["10y"].vth_shift > m1.shifts["1y"].vth_shift);
}

#[test]
fn test_run_reliability_analysis_with_config_rejects_if_no_stressed_devices() {
    let netlist = r#"
* No stressed semiconductor devices
V1 in 0 1
R1 in out 1k
R2 out 0 1k
.end
"#;
    let cfg = ReliabilityRunConfig {
        target_years: vec![1.0, 5.0],
        enable_hci: true,
        enable_nbti: false,
        enable_em: false,
        min_stress_voltage: 0.1,
    };
    let err = run_reliability_analysis_with_config(netlist, &cfg)
        .expect_err("reliability should fail when no qualifying devices exist");
    assert!(err.contains("no stressed semiconductor devices"));
}

#[test]
fn test_apply_reliability_mechanism_scaling_em_only_suppresses_vth_mobility_shift() {
    let mut results = vec![ReliabilityResult {
        device_id: "M1".to_string(),
        stress: StressMetrics {
            avg_vgs_stress: 1.0,
            avg_vds_stress: 1.0,
            avg_temp: 300.0,
            duration: 3600.0,
        },
        shifts: std::collections::HashMap::from([(
            "1y".to_string(),
            ParamShift {
                vth_shift: 1.0,
                mobility_shift: -1.0,
                rds_shift: 0.5,
            },
        )]),
    }];
    let cfg = ReliabilityRunConfig {
        target_years: vec![1.0],
        enable_hci: false,
        enable_nbti: false,
        enable_em: true,
        min_stress_voltage: 0.0,
    };

    apply_reliability_mechanism_scaling(&mut results, &cfg);
    let shift = results[0]
        .shifts
        .get("1y")
        .expect("scaled shift should still be present");
    assert!((shift.vth_shift - 0.0).abs() < 1e-12);
    assert!((shift.mobility_shift - 0.0).abs() < 1e-12);
    assert!(shift.rds_shift > 1.0, "EM scaling should amplify Rds shift");
}

#[test]
fn test_run_optimization_analysis_with_config_targets_resistive_divider_voltage() {
    let netlist = r#"
* optimization smoke
.param RTOP=1k
.param RBOT=1k
V1 in 0 2
R1 in out {RTOP}
R2 out 0 {RBOT}
.end
"#;
    let cfg = OptimizationRunConfig {
        variables: vec![OptimizationVariable {
            name: "RBOT".to_string(),
            min: 500.0,
            max: 3000.0,
            initial: 1000.0,
        }],
        objective_node: "out".to_string(),
        objective_ref: "0".to_string(),
        goal: OptimizationGoalMode::Target,
        target: Some(1.2),
        algorithm: OptimizationAlgorithmMode::PatternSearch,
        max_iterations: 80,
        cost_tolerance: 1e-8,
        fd_step: 1e-4,
        initial_step: 0.2,
        min_step: 1e-8,
    };

    let data = run_optimization_analysis_with_config(netlist, &cfg)
        .expect("optimization should run for divider netlist");
    assert!(!data.iterations.is_empty());
    assert_eq!(data.iterations.len(), data.costs.len());
    let rbot_trace = data
        .variable_traces
        .get("RBOT")
        .expect("RBOT trace should exist");
    assert_eq!(rbot_trace.len(), data.iterations.len());
    assert!(data.best_cost.is_finite());
    let best_rbot = data
        .best_variables
        .get("RBOT")
        .copied()
        .expect("best RBOT should be present");
    assert!(
        (best_rbot - 1500.0).abs() < 250.0,
        "expected optimizer to approach ideal RBOT ~1500 ohm, got {}",
        best_rbot
    );
}

#[test]
fn test_run_optimization_analysis_rejects_invalid_variable_name() {
    let cfg = OptimizationRunConfig {
        variables: vec![OptimizationVariable {
            name: "1BAD".to_string(),
            min: 1.0,
            max: 2.0,
            initial: 1.5,
        }],
        ..OptimizationRunConfig::default()
    };
    let err = run_optimization_analysis_with_config("* invalid\nR1 in 0 1k\n.end\n", &cfg)
        .expect_err("invalid variable name must be rejected");
    assert!(err.contains("Invalid optimization variable name"));
}

#[test]
fn test_inject_param_overrides_inserts_before_last_end() {
    let netlist = "* test\n.param A=1\nR1 in 0 {A}\n.end\n";
    let vars = std::collections::HashMap::from([("A".to_string(), 2.5)]);
    let overridden = inject_param_overrides(netlist, &vars);
    let lowered = overridden.to_ascii_lowercase();
    let end_pos = lowered
        .rfind(".end")
        .expect("overridden netlist should include .end");
    let param_pos = lowered
        .rfind("a=2.5000000000000000e+00")
        .expect("override assignment should be present");
    assert!(param_pos < end_pos);
}

#[test]
fn test_inject_param_overrides_rewrites_existing_param_lines_in_place() {
    let netlist = "* opt\n.param A=1 B=2\n.param A=3\nR1 out 0 {A}\n.end\n";
    let vars = std::collections::HashMap::from([("A".to_string(), 4.25)]);
    let overridden = inject_param_overrides(netlist, &vars);
    let lowered = overridden.to_ascii_lowercase();
    let expected = ".param a=1 b=2 a=4.2500000000000000e+00";
    assert!(
        lowered.contains(expected),
        "first .param line should include override assignment: {}",
        overridden
    );
    assert!(
        lowered.contains(".param a=3 a=4.2500000000000000e+00"),
        "every matching .param line should include override assignment"
    );
}

#[test]
fn test_inject_param_overrides_inserts_missing_params_after_title_line() {
    let netlist = "optimization deck\nR1 out 0 {A}\n.end\n";
    let vars = std::collections::HashMap::from([("A".to_string(), 1.75)]);
    let overridden = inject_param_overrides(netlist, &vars);
    let lines: Vec<&str> = overridden.lines().collect();
    assert_eq!(lines[0], "optimization deck");
    assert!(
        lines[1]
            .to_ascii_lowercase()
            .contains(".param a=1.7500000000000000e+00"),
        "missing overrides should be inserted directly after title line"
    );
}

#[test]
fn test_format_param_override_value_uses_explicit_signed_exponent() {
    assert_eq!(
        format_param_override_value(2.5),
        "2.5000000000000000e+00".to_string()
    );
    assert_eq!(
        format_param_override_value(1e-9),
        "1.0000000000000001e-09".to_string()
    );
}

#[test]
fn test_run_soa_analysis_with_config_detects_mos_voltage_violation() {
    let netlist = r#"
* soa smoke
VDD d 0 3.3
VG g 0 PULSE(0 2.5 0 1n 1n 8n 16n)
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#;
    let cfg = SoaRunConfig {
        stop_time: 32e-9,
        step_time: 1e-9,
        check_vgs_max: true,
        max_vgs: 1.2,
        check_vds_max: true,
        max_vds: 3.0,
        check_vbe_max: false,
        max_vbe: 1.0,
        check_vce_max: false,
        max_vce: 5.0,
    };
    let data = run_soa_analysis_with_config(netlist, &cfg)
        .expect("SOA should execute for MOS transient netlist");
    assert!(!data.time.is_empty());
    assert_eq!(data.time.len(), data.violation_count.len());
    assert!(
        !data.violations.is_empty(),
        "expected SOA voltage violations with aggressive limits"
    );
    let last = *data
        .violation_count
        .last()
        .expect("violation count trace should have data");
    assert!(last >= 1.0);
}

#[test]
fn test_run_soa_analysis_rejects_netlist_without_supported_devices() {
    let netlist = "* soa none\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n";
    let err = run_soa_analysis(netlist)
        .expect_err("SOA should fail when no supported semiconductor devices exist");
    assert!(err.contains("supported semiconductor devices"));
}
