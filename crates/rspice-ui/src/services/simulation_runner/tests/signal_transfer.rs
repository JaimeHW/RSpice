use super::*;
use std::path::PathBuf;

fn write_signal_include_fixture(
    source_file_name: &str,
    include_file_name: &str,
    include_contents: &str,
    netlist: &str,
) -> (tempfile::TempDir, PathBuf, String) {
    let temp_dir = tempfile::tempdir().expect("temp dir should be created");
    let source_path = temp_dir.path().join("project").join(source_file_name);
    std::fs::create_dir_all(
        source_path
            .parent()
            .expect("source path should have parent directory"),
    )
    .expect("project directory should be created");
    std::fs::create_dir_all(temp_dir.path().join("models"))
        .expect("models directory should be created");
    std::fs::write(
        temp_dir.path().join("models").join(include_file_name),
        include_contents,
    )
    .expect("include file should be written");

    (temp_dir, source_path, netlist.to_string())
}

fn write_tf_include_fixture() -> (tempfile::TempDir, PathBuf, String) {
    write_signal_include_fixture(
        "tf_top.rsch",
        "tf_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* tf include fixture\n\
V1 in 0 DC 1 AC 1\n\
.include ../models/tf_stage.inc\n\
.end\n",
    )
}

#[test]
fn test_ac_data_magnitude_db() {
    let data = AcData {
        frequencies: vec![1e3, 1e4],
        responses: vec![(
            "V(out)".to_string(),
            vec![Complex64::new(1.0, 0.0), Complex64::new(10.0, 0.0)],
        )],
        num_points: 2,
    };

    let mag = data.magnitude_db(0);
    assert_eq!(mag.len(), 2);
    assert!((mag[0] - 0.0).abs() < 0.01); // 0 dB
    assert!((mag[1] - 20.0).abs() < 0.01); // 20 dB
}

#[test]
fn test_ac_data_phase_deg() {
    let data = AcData {
        frequencies: vec![1e3],
        responses: vec![(
            "V(out)".to_string(),
            vec![Complex64::new(0.0, 1.0)], // 90 degrees
        )],
        num_points: 1,
    };

    let phase = data.phase_deg(0);
    assert_eq!(phase.len(), 1);
    assert!((phase[0] - 90.0).abs() < 0.01);
}

#[test]
fn test_ac_data_from_results_node_mapping() {
    let results = vec![
        AcResult {
            frequency: 1e3,
            node_names: vec!["IN".to_string(), "OUT".to_string()],
            branch_names: vec![],
            voltages: vec![Complex64::new(1.0, 0.0), Complex64::new(0.5, 0.0)],
            currents: vec![],
        },
        AcResult {
            frequency: 1e6,
            node_names: vec!["IN".to_string(), "OUT".to_string()],
            branch_names: vec![],
            voltages: vec![Complex64::new(2.0, 0.0), Complex64::new(1.0, 0.0)],
            currents: vec![],
        },
    ];

    let data = AcData::from_results(results);
    assert_eq!(data.responses.len(), 2);
    assert_eq!(data.responses[0].0, "V(IN)");
    assert_eq!(data.responses[0].1[0], Complex64::new(1.0, 0.0));
    assert_eq!(data.responses[1].0, "V(OUT)");
    assert_eq!(data.responses[1].1[0], Complex64::new(0.5, 0.0));
}

#[test]
fn test_run_tf_analysis_with_config_executes_for_driven_rc() {
    let netlist = "* tf\nV1 in 0 DC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = TfRunConfig {
        start_freq: 10.0,
        stop_freq: 1e6,
        points_per_unit: 6,
        sweep: TfFrequencySweep::Decade,
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        output_ref: None,
        group_delay: true,
        input_impedance: true,
        output_impedance: true,
    };

    let result = run_tf_analysis_with_config(netlist, &cfg)
        .expect("TF analysis should execute for driven RC");
    assert!(!result.frequencies.is_empty());
    assert_eq!(result.transfer.len(), result.frequencies.len());
    assert_eq!(result.magnitude_db.len(), result.frequencies.len());
    assert_eq!(result.phase_deg.len(), result.frequencies.len());
    assert!(
        result
            .transfer
            .iter()
            .all(|value| value.re.is_finite() && value.im.is_finite())
    );
    assert!(
        result
            .group_delay
            .as_ref()
            .is_some_and(|curve| !curve.is_empty())
    );
    assert!(
        result
            .input_impedance
            .as_ref()
            .is_some_and(|curve| curve.iter().all(|value| value.re.is_finite()))
    );
    assert!(
        result
            .output_impedance
            .as_ref()
            .is_some_and(|curve| curve.iter().all(|value| value.re.is_finite()))
    );
}

#[test]
fn test_run_tf_analysis_with_config_rejects_unknown_input_source() {
    let netlist = "* tf invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = TfRunConfig {
        input_source: "V_NOT_PRESENT".to_string(),
        output_node: "out".to_string(),
        ..TfRunConfig::default()
    };
    let err = run_tf_analysis_with_config(netlist, &cfg).expect_err("missing source must fail TF");
    assert!(err.contains("not found"));
}

#[test]
fn test_run_tf_analysis_auto_infers_configuration() {
    let netlist = "* tf auto\nVDRIVE in 0 1\nR1 in out 2k\nC1 out 0 2n\n.end\n";
    let result = run_tf_analysis(netlist).expect("TF auto mode should infer source and output");
    assert!(!result.frequencies.is_empty());
    assert_eq!(result.transfer.len(), result.frequencies.len());
    assert_eq!(result.input_source, "VDRIVE");
    assert!(result.output_label.contains("out") || result.output_label.contains("OUT"));
}

#[test]
fn test_run_tf_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_tf_include_fixture();

    let without_source = run_tf_analysis(&netlist);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_tf_analysis_with_source_path(&netlist, Some(&source_path))
        .expect("source-aware TF analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert_eq!(with_source.transfer.len(), with_source.frequencies.len());
    assert_eq!(with_source.input_source, "V1");
}

#[test]
fn test_run_tf_analysis_with_config_and_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_tf_include_fixture();
    let cfg = TfRunConfig {
        start_freq: 10.0,
        stop_freq: 1e6,
        points_per_unit: 6,
        sweep: TfFrequencySweep::Decade,
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        output_ref: None,
        group_delay: true,
        input_impedance: true,
        output_impedance: true,
    };

    let without_source = run_tf_analysis_with_config(&netlist, &cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source =
        run_tf_analysis_with_config_and_source_path(&netlist, &cfg, Some(&source_path))
            .expect("source-aware configured TF analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert_eq!(with_source.transfer.len(), with_source.frequencies.len());
    assert!(
        with_source
            .group_delay
            .as_ref()
            .is_some_and(|curve| !curve.is_empty())
    );
    assert!(
        with_source
            .input_impedance
            .as_ref()
            .is_some_and(|curve| curve.len() == with_source.frequencies.len())
    );
    assert!(
        with_source
            .output_impedance
            .as_ref()
            .is_some_and(|curve| curve.len() == with_source.frequencies.len())
    );
}

#[test]
fn test_run_sparameter_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_signal_include_fixture(
        "sparameter_top.rsch",
        "sparameter_stage.inc",
        "R1 in 0 50\nR2 out 0 50\n",
        "* sparameter include fixture\n\
.include ../models/sparameter_stage.inc\n\
.end\n",
    );
    let cfg = SParameterRunConfig {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 5,
        sweep: SParameterSweep::Decade,
        z0: 50.0,
        ports: vec![
            SParameterPort::single_ended("in"),
            SParameterPort::single_ended("out"),
        ],
    };

    let without_source = run_sparameter_analysis(&netlist, &cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_sparameter_analysis_with_source_path(&netlist, &cfg, Some(&source_path))
        .expect("source-aware S-parameter analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert_eq!(with_source.num_ports, 2);
    assert_eq!(with_source.s.len(), 2);
}

#[test]
fn test_run_stb_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_signal_include_fixture(
        "stb_top.rsch",
        "stb_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* stb include fixture\n\
V1 in 0 DC 1 AC 1\n\
.include ../models/stb_stage.inc\n\
.end\n",
    );

    let without_source = run_stb_analysis(&netlist, "2", 1.0, 1e6, 5);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source =
        run_stb_analysis_with_source_path(&netlist, "2", 1.0, 1e6, 5, Some(&source_path))
            .expect("source-aware STB analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert_eq!(
        with_source.loop_gain_db.len(),
        with_source.frequencies.len()
    );
    assert_eq!(
        with_source.loop_phase_deg.len(),
        with_source.frequencies.len()
    );
}

#[test]
fn test_run_sparameter_analysis_returns_expected_shapes_for_decoupled_matched_ports() {
    let netlist = "* S-parameter decoupled matched ports\nR1 in 0 50\nR2 out 0 50\n.end\n";
    let cfg = SParameterRunConfig {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 5,
        sweep: SParameterSweep::Decade,
        z0: 50.0,
        ports: vec![
            SParameterPort::single_ended("in"),
            SParameterPort::single_ended("out"),
        ],
    };

    let result = run_sparameter_analysis(netlist, &cfg)
        .expect("S-parameter analysis should execute for simple two-port");
    assert!(!result.frequencies.is_empty());
    assert_eq!(result.num_ports, 2);
    assert_eq!(result.s.len(), 2);
    assert_eq!(result.s[0].len(), 2);
    assert_eq!(result.s[1].len(), 2);
    assert_eq!(result.frequencies.len(), result.s[0][0].len());
    for idx in 0..result.frequencies.len() {
        assert!(
            result.s[1][0][idx].norm() < 1e-8,
            "S21 should be near 0 for decoupled ports"
        );
        assert!(
            result.s[0][1][idx].norm() < 1e-8,
            "S12 should be near 0 for decoupled ports"
        );
        assert!(
            (result.s[0][0][idx] - result.s[1][1][idx]).norm() < 1e-9,
            "symmetric ports should have matching reflections"
        );
        assert!(result.s[0][0][idx].norm().is_finite() && result.s[0][0][idx].norm() <= 1.0 + 1e-6);
        assert!(result.s[1][1][idx].norm().is_finite() && result.s[1][1][idx].norm() <= 1.0 + 1e-6);
    }
}

#[test]
fn test_run_sparameter_analysis_preserves_per_port_reference_impedance_overrides() {
    let netlist = "* S-parameter decoupled matched ports\nR1 in 0 50\nR2 out 0 50\n.end\n";
    let cfg = SParameterRunConfig {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 5,
        sweep: SParameterSweep::Decade,
        z0: 50.0,
        ports: vec![
            SParameterPort::single_ended("in"),
            SParameterPort {
                node_pos: "out".to_string(),
                node_neg: "0".to_string(),
                z0: Some(75.0),
            },
        ],
    };

    let result = run_sparameter_analysis(netlist, &cfg)
        .expect("S-parameter analysis should execute with per-port z0");
    assert_eq!(result.z0, vec![50.0, 75.0]);
}

#[test]
fn test_run_sparameter_analysis_supports_three_port_matrices() {
    let netlist = "* S-parameter 3-port matched\nR1 p1 0 50\nR2 p2 0 50\nR3 p3 0 50\n.end\n";
    let cfg = SParameterRunConfig {
        start_freq: 1e3,
        stop_freq: 1e5,
        points_per_unit: 3,
        sweep: SParameterSweep::Decade,
        z0: 50.0,
        ports: vec![
            SParameterPort::single_ended("p1"),
            SParameterPort::single_ended("p2"),
            SParameterPort::single_ended("p3"),
        ],
    };

    let result = run_sparameter_analysis(netlist, &cfg)
        .expect("S-parameter analysis should execute for simple three-port");
    assert_eq!(result.num_ports, 3);
    assert_eq!(result.s.len(), 3);
    for row in &result.s {
        assert_eq!(row.len(), 3);
        for trace in row {
            assert_eq!(trace.len(), result.frequencies.len());
        }
    }
    for idx in 0..result.frequencies.len() {
        for row in 0..3 {
            for col in 0..3 {
                assert!(
                    result.s[row][col][idx].norm().is_finite(),
                    "S{}{} should be finite",
                    row + 1,
                    col + 1
                );
            }
        }
        for row in 0..3 {
            for col in 0..3 {
                if row == col {
                    continue;
                }
                assert!(
                    result.s[row][col][idx].norm() <= 1e-8,
                    "S{}{} should be near 0 for decoupled ports",
                    row + 1,
                    col + 1
                );
            }
        }
        for row in 0..3 {
            assert!(
                result.s[row][row][idx].norm() <= 1.0 + 1e-6,
                "S{}{} should stay passive and bounded",
                row + 1,
                row + 1
            );
        }
        let s11 = result.s[0][0][idx];
        assert!((s11 - result.s[1][1][idx]).norm() < 1e-9);
        assert!((s11 - result.s[2][2][idx]).norm() < 1e-9);
    }
}

#[test]
fn test_run_sparameter_analysis_rejects_single_port_config() {
    let netlist = "* S-parameter invalid\nR1 p1 0 50\n.end\n";
    let cfg = SParameterRunConfig {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 5,
        sweep: SParameterSweep::Decade,
        z0: 50.0,
        ports: vec![SParameterPort::single_ended("p1")],
    };

    let err = run_sparameter_analysis(netlist, &cfg)
        .expect_err("single-port S-parameter config should be rejected");
    assert!(err.contains("at least 2 ports"));
}

#[test]
fn test_run_envelope_analysis_produces_envelope_traces() {
    let netlist = "* Envelope sine\nV1 out 0 SIN(0 1 1Meg 0 0 0)\nR1 out 0 1k\n.end\n";
    let cfg = EnvelopeRunConfig {
        fundamental_freq: 1e6,
        stop_time: 10e-6,
        num_harmonics: 9,
        max_step: None,
    };

    let result = run_envelope_analysis(netlist, &cfg)
        .expect("Envelope analysis should run for simple sinusoid");
    assert!(!result.time.is_empty());
    assert!(!result.waveforms.is_empty());
    for (name, values) in &result.waveforms {
        assert!(name.starts_with("ENV("));
        assert_eq!(values.len(), result.time.len());
        assert!(values.iter().all(|v| v.is_finite() && *v >= 0.0));
        let max_env = values.iter().copied().fold(0.0, Value::max);
        assert!(
            max_env > 1e-4,
            "envelope should contain non-trivial amplitude"
        );
    }
}

#[test]
fn test_run_fourier_analysis_detects_fundamental_and_low_thd_for_sine() {
    let netlist = "* Fourier sine\nV1 out 0 SIN(0 1 1k 0 0 0)\nR1 out 0 1k\n.end\n";
    let cfg = FourierRunConfig {
        fundamental_freq: 1e3,
        num_harmonics: 8,
        output_node: "out".to_string(),
        output_ref: None,
        start_time: 0.0,
        stop_time: 20e-3,
    };

    let result = run_fourier_analysis(netlist, &cfg).expect("Fourier analysis should run for sine");
    assert_eq!(result.frequencies.len(), result.response.len());
    assert_eq!(result.frequencies.len(), cfg.num_harmonics + 1);
    assert!(!result.response.is_empty());
    let fundamental = result
        .response
        .get(1)
        .expect("fundamental component should exist");
    assert!(
        fundamental.norm() > 0.7 && fundamental.norm() < 1.3,
        "fundamental magnitude should be near 1V, got {}",
        fundamental.norm()
    );
    assert!(
        result.thd_percent < 1.0,
        "pure sine THD should be low, got {}%",
        result.thd_percent
    );
}
