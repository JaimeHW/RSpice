use super::*;
use std::path::PathBuf;

fn write_runner_include_fixture() -> (tempfile::TempDir, PathBuf, String) {
    let temp_dir = tempfile::tempdir().expect("temp dir should be created");
    let source_path = temp_dir.path().join("project").join("amp_top.rsch");
    std::fs::create_dir_all(
        source_path
            .parent()
            .expect("source path should have parent directory"),
    )
    .expect("project directory should be created");
    std::fs::create_dir_all(temp_dir.path().join("models"))
        .expect("models directory should be created");

    std::fs::write(
        temp_dir.path().join("models").join("rc_section.inc"),
        "R1 in out 1k\nC1 out 0 1u\n",
    )
    .expect("include file should be written");

    let netlist = "* include-backed service test\n\
V1 in 0 DC 1 AC 1\n\
.include ../models/rc_section.inc\n\
.end\n"
        .to_string();

    (temp_dir, source_path, netlist)
}

#[test]
fn test_generate_freq_points_linear() {
    let freqs = generate_freq_points(1.0, 101.0, 11, "lin");
    // Linear sweep from 1 to 101 Hz with 11 points
    assert_eq!(freqs.len(), 11);
    assert!((freqs[0] - 1.0).abs() < 1e-6);
    assert!((freqs[10] - 101.0).abs() < 1e-6);
}

#[test]
fn test_generate_freq_points_decade() {
    let freqs = generate_freq_points(1.0, 1000.0, 10, "dec");
    // 3 decades, 10 points per decade = 30 points
    assert!(freqs.len() >= 2);
    assert!((freqs[0] - 1.0).abs() < 1e-6);
    assert!((freqs[freqs.len() - 1] - 1000.0).abs() < 1.0);
}

#[test]
fn test_interpolate_magnitude_at_log_frequency() {
    let frequencies = vec![1.0, 10.0, 100.0];
    let magnitudes = vec![1.0, 10.0, 100.0];
    let mid = interpolate_magnitude_at_for_tests(&frequencies, &magnitudes, 31.622776601683793)
        .expect("interpolation should succeed");
    assert!((mid - 31.622776601683793).abs() < 1e-9);
}

#[test]
fn test_build_engine_config_applies_netlist_options_when_ui_is_none() {
    let netlist = rspice_core::netlist::parse_netlist(
        r#"Test
.OPTIONS TEMP=85 ITL1=120 METHOD=GEAR RELTOL=2e-4 VNTOL=3e-6 IABSTOL=4e-12 GMIN=1e-11
.END
"#,
    )
    .expect("netlist should parse");

    let config = build_engine_config(&netlist, None);

    assert!((config.temperature - 358.15).abs() < 1e-12);
    assert_eq!(config.max_iterations, 120);
    assert_eq!(
        config.integration_method,
        rspice_core::analysis::IntegrationMethod::Gear2
    );
    assert!((config.tolerance - 2e-4).abs() < 1e-15);
    assert!((config.convergence_config.voltage_reltol - 2e-4).abs() < 1e-15);
    assert!((config.convergence_config.residual_reltol - 2e-4).abs() < 1e-15);
    assert!((config.convergence_config.voltage_abstol - 3e-6).abs() < 1e-18);
    assert!((config.convergence_config.current_abstol - 4e-12).abs() < 1e-24);
    assert!((config.convergence_config.gmin_initial - 1e-11).abs() < 1e-24);
}

#[test]
fn test_build_engine_config_ui_options_override_netlist_options() {
    let netlist = rspice_core::netlist::parse_netlist(
        r#"Test
.OPTIONS TEMP=125 ITL1=200 METHOD=GEAR RELTOL=2e-4 VNTOL=2e-6 IABSTOL=2e-12 GMIN=1e-11
.END
"#,
    )
    .expect("netlist should parse");

    let mut ui = crate::simulation::dialog::SimulationOptions::default();
    ui.temp = 27.0;
    ui.itl1 = 90;
    ui.method = crate::simulation::dialog::IntegrationMethod::Trap;
    ui.reltol = 7e-4;
    ui.residual_reltol = 4e-4;
    ui.vntol = 9e-6;
    ui.iabstol = 8e-12;
    ui.gmin = 3e-10;

    let config = build_engine_config(&netlist, Some(&ui));

    assert!((config.temperature - 300.15).abs() < 1e-12);
    assert_eq!(config.max_iterations, 90);
    assert_eq!(
        config.integration_method,
        rspice_core::analysis::IntegrationMethod::Trapezoidal
    );
    assert!((config.tolerance - 7e-4).abs() < 1e-15);
    assert!((config.convergence_config.voltage_reltol - 7e-4).abs() < 1e-15);
    assert!((config.convergence_config.residual_reltol - 4e-4).abs() < 1e-15);
    assert!((config.convergence_config.voltage_abstol - 9e-6).abs() < 1e-18);
    assert!((config.convergence_config.current_abstol - 8e-12).abs() < 1e-24);
    assert!((config.convergence_config.gmin_initial - 3e-10).abs() < 1e-22);
}

#[test]
fn test_transient_analysis_validation() {
    let netlist = "* test\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p\n.end\n";

    let err = run_transient_analysis(netlist, 0.0, 1e-9).expect_err("expected validation error");
    assert!(err.contains("stop_time"));

    let err = run_transient_analysis(netlist, 1e-6, 0.0).expect_err("expected validation error");
    assert!(err.contains("step_time"));
}

#[test]
fn test_run_simulation_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_runner_include_fixture();

    let without_source = run_simulation(&netlist);
    assert!(
        !without_source.success,
        "relative include should fail without source path"
    );

    let with_source = run_simulation_with_source_path(&netlist, Some(source_path.as_path()));
    assert!(
        with_source.success,
        "source-aware runner should resolve include"
    );
    assert!(with_source.dc_op.is_some());
}

#[test]
fn test_run_transient_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_runner_include_fixture();

    let data = run_transient_analysis_with_source_path(&netlist, 2e-3, 10e-6, Some(&source_path))
        .expect("source-aware transient analysis should resolve include");
    assert!(!data.time.is_empty());
    assert!(data
        .voltages
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("V(OUT)")));
}

#[test]
fn test_run_ac_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_runner_include_fixture();

    let data =
        run_ac_analysis_with_source_path(&netlist, 10.0, 1.0e6, 5, "dec", Some(&source_path))
            .expect("source-aware AC analysis should resolve include");
    assert!(data.num_points >= 2);
    assert!(data
        .responses
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("V(OUT)")));
}

#[test]
fn test_run_dc_sweep_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_runner_include_fixture();

    let data = run_dc_sweep_with_source_path(&netlist, "V1", 0.0, 1.0, 0.25, Some(&source_path))
        .expect("source-aware DC sweep should resolve include");
    assert_eq!(data.source_name, "V1");
    assert_eq!(data.num_points, data.sweep_values.len());
    assert!(data
        .voltages
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("V(OUT)")));
}

#[test]
fn test_parse_output_node_helper() {
    let names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
    assert_eq!(
        crate::output_spec::parse_output_node("V(OUT)", &names),
        Some(2)
    );
    assert_eq!(
        crate::output_spec::parse_output_node("out", &names),
        Some(2)
    );
    assert_eq!(crate::output_spec::parse_output_node("2", &names), Some(2));
    assert_eq!(
        crate::output_spec::parse_output_node("V(OUT,IN)", &names),
        None
    );
    assert_eq!(crate::output_spec::parse_output_node("I(R1)", &names), None);
}

#[test]
fn test_parse_output_voltage_spec_helper() {
    let names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
    assert_eq!(
        crate::output_spec::parse_output_voltage_spec("V(OUT)", &names),
        Some(OutputVoltageSpec { pos: 2, neg: None })
    );
    assert_eq!(
        crate::output_spec::parse_output_voltage_spec("V(OUT,IN)", &names),
        Some(OutputVoltageSpec {
            pos: 2,
            neg: Some(1)
        })
    );
    assert_eq!(
        crate::output_spec::parse_output_voltage_spec("V(OUT,GND)", &names),
        Some(OutputVoltageSpec {
            pos: 2,
            neg: Some(0)
        })
    );
    assert_eq!(
        crate::output_spec::parse_output_voltage_spec("I(R1)", &names),
        None
    );
}

#[test]
fn test_parse_output_spec_current_helper() {
    let netlist = rspice_core::netlist::parse_netlist("* t\nV1 in 0 1\nR1 in 0 1k\n")
        .expect("netlist should parse");
    let engine = Engine::new(SimulationConfig::default());
    let circuit = engine
        .build_circuit(&netlist)
        .expect("circuit build should succeed");
    let node_names = vec!["0".to_string(), "IN".to_string()];

    let spec = crate::output_spec::parse_output_spec("I(V1)", &node_names, &circuit);
    assert!(matches!(
        spec,
        Some(OutputSpec::BranchCurrent {
            branch_ordinal: 1,
            ..
        })
    ));
}

#[test]
fn test_expand_step_sweep_values_linear_descending() {
    let values = expand_step_sweep_values(&StepSweep::Linear {
        start: 5.0,
        stop: 1.0,
        step: -2.0,
    })
    .expect("descending linear sweep should expand");
    assert_eq!(values, vec![5.0, 3.0, 1.0]);
}

#[test]
fn test_expand_step_sweep_values_rejects_wrong_direction() {
    let err = expand_step_sweep_values(&StepSweep::Linear {
        start: 1.0,
        stop: 5.0,
        step: -1.0,
    })
    .expect_err("mismatched direction should fail");
    assert!(err.contains("direction"));
}

#[test]
fn test_extract_temp_points_deduplicates_values() {
    let netlist = rspice_core::netlist::parse_netlist(
        "* dedupe\nV1 in 0 1\nR1 in 0 1k\n.TEMP -40 27\n.TEMP 27 125\n.end\n",
    )
    .expect("netlist should parse");

    let temps = extract_temp_points(&netlist);
    assert_eq!(temps, vec![-40.0, 27.0, 125.0]);
}

#[test]
fn test_netlist_has_independent_source_named_matches_case_insensitive() {
    let netlist = rspice_core::netlist::parse_netlist(
        "* source lookup\nV1 in 0 1\nI_BIAS out 0 1m\nE1 x 0 in out 10\n.end\n",
    )
    .expect("netlist should parse");
    assert!(netlist_has_independent_source_named(&netlist, "v1"));
    assert!(netlist_has_independent_source_named(&netlist, "I_BIAS"));
    assert!(!netlist_has_independent_source_named(&netlist, "E1"));
    assert!(!netlist_has_independent_source_named(
        &netlist,
        "NOT_PRESENT"
    ));
}
