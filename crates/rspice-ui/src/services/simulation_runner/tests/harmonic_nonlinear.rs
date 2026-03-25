use super::*;
use std::path::PathBuf;

fn write_nonlinear_include_fixture(
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

#[test]
fn test_run_disto_analysis_generates_harmonic_metrics() {
    let netlist = r#"
* disto transfer-estimation test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#;

    let cfg = DistoRunConfig {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 8,
        sweep: DistoFrequencySweep::Decade,
        f2_over_f1: Some(1.5),
        allow_linearized_fallback: false,
    };

    let data = run_disto_analysis(netlist, &cfg).expect("DISTO should execute");
    assert!(!data.frequencies.is_empty());
    assert!(!data.traces.is_empty());
    assert!(
        data.warnings.is_empty(),
        "nonlinear HB DISTO path should not emit fallback warnings"
    );

    let trace = &data.traces[0];
    assert_eq!(trace.fundamental_gain_db.len(), data.frequencies.len());
    assert_eq!(trace.hd2_db.len(), data.frequencies.len());
    assert_eq!(trace.hd3_db.len(), data.frequencies.len());
    assert_eq!(trace.thd_percent.len(), data.frequencies.len());
    assert!(trace.imd2_db.is_some());
    assert!(trace.imd3_db.is_some());
    assert_eq!(
        trace.imd2_db.as_ref().expect("imd2 should exist").len(),
        data.frequencies.len()
    );
    assert_eq!(
        trace.imd3_db.as_ref().expect("imd3 should exist").len(),
        data.frequencies.len()
    );
}

#[test]
fn test_run_disto_analysis_rejects_invalid_f2_ratio() {
    let cfg = DistoRunConfig {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 8,
        sweep: DistoFrequencySweep::Decade,
        f2_over_f1: Some(1.0),
        allow_linearized_fallback: false,
    };
    let err = run_disto_analysis("* invalid\nV1 in 0 AC 1\nR1 in 0 1k\n.end\n", &cfg)
        .expect_err("f2 ratio <= 1 should fail validation");
    assert!(err.contains("f2_over_f1"));
}

#[test]
fn test_run_disto_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_nonlinear_include_fixture(
        "disto_top.rsch",
        "disto_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* disto include fixture\n\
V1 in 0 DC 1 AC 1\n\
.include ../models/disto_stage.inc\n\
.end\n",
    );
    let cfg = DistoRunConfig {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 8,
        sweep: DistoFrequencySweep::Decade,
        f2_over_f1: Some(1.5),
        allow_linearized_fallback: false,
    };

    let without_source = run_disto_analysis(&netlist, &cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_disto_analysis_with_source_path(&netlist, &cfg, Some(&source_path))
        .expect("source-aware DISTO analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert!(!with_source.traces.is_empty());
}

#[test]
fn test_build_disto_two_tone_harmonic_plan_rational_ratio() {
    let plan = build_disto_two_tone_harmonic_plan(1.5).expect("1.5 should map to 3/2");
    assert_eq!(plan.tone1_harmonic, 2);
    assert_eq!(plan.tone2_harmonic, 3);
    assert!(plan.max_harmonic >= 9);
}

#[test]
fn test_build_multi_tone_hb_layout_reports_harmonic_mapping() {
    let tones = vec![HbToneRunConfig::new(2e6, 2), HbToneRunConfig::new(3e6, 1)];
    let layout = build_multi_tone_hb_layout(&tones, 3)
        .expect("2 MHz and 3 MHz should map to a commensurate 1 MHz basis");
    assert!((layout.base_frequency - 1e6).abs() < 1e-9);
    assert_eq!(layout.tone_harmonics, vec![2, 3]);
    assert!(layout.max_harmonic >= 9);
}

#[test]
fn test_build_disto_two_tone_plan_uses_shared_hb_layout_mapping() {
    let plan = build_disto_two_tone_harmonic_plan(2.5).expect("2.5 should map to 5/2");
    assert_eq!(plan.tone1_harmonic, 2);
    assert_eq!(plan.tone2_harmonic, 5);
    assert!(plan.max_harmonic >= 15);
}

#[test]
fn test_build_disto_two_tone_harmonic_plan_rejects_unstable_ratio() {
    let err = build_disto_two_tone_harmonic_plan(2f64.sqrt())
        .expect_err("irrational ratio should not map to low-order harmonic basis");
    assert!(err.contains("low-order rational"));
}

#[test]
fn test_run_disto_analysis_fallbacks_for_unstable_two_tone_ratio() {
    let netlist = r#"
* disto fallback ratio test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#;
    let cfg = DistoRunConfig {
        start_freq: 1e3,
        stop_freq: 1e5,
        points_per_unit: 5,
        sweep: DistoFrequencySweep::Decade,
        f2_over_f1: Some(2f64.sqrt()),
        allow_linearized_fallback: true,
    };

    let data = run_disto_analysis(netlist, &cfg).expect("DISTO should fallback when needed");
    assert!(
        data.warnings
            .iter()
            .any(|warning| warning.contains("linearized transfer-based fallback")),
        "expected explicit nonlinear fallback warning"
    );
    assert!(
        data.traces
            .iter()
            .all(|trace| trace.imd2_db.is_some() && trace.imd3_db.is_some()),
        "linearized fallback should still produce IMD traces"
    );
}

#[test]
fn test_run_disto_analysis_unstable_two_tone_ratio_errors_without_fallback() {
    let netlist = r#"
* disto strict ratio test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#;
    let cfg = DistoRunConfig {
        start_freq: 1e3,
        stop_freq: 1e5,
        points_per_unit: 5,
        sweep: DistoFrequencySweep::Decade,
        f2_over_f1: Some(2f64.sqrt()),
        allow_linearized_fallback: false,
    };

    let err = run_disto_analysis(netlist, &cfg)
        .expect_err("DISTO should fail in strict mode when HB ratio mapping is unstable");
    assert!(err.contains("allow_linearized_fallback=true"));
    assert!(err.contains("nonlinear HB path failed"));
}

#[test]
fn test_run_pss_analysis_executes_for_driven_rc() {
    let netlist = "* pss\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let result =
        run_pss_analysis(netlist, 1e6, 8, 1e-4).expect("PSS analysis should execute for driven RC");
    assert!(result.period > 0.0);
    assert!(result.frequency > 0.0);
    assert!(!result.time.is_empty());
    assert!(!result.waveforms.is_empty());
    assert!(
        result
            .harmonics
            .iter()
            .any(|(_, harmonics)| !harmonics.is_empty()),
        "expected harmonic content for at least one waveform"
    );
}

#[test]
fn test_run_pss_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_nonlinear_include_fixture(
        "pss_top.rsch",
        "pss_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* pss include fixture\n\
V1 in 0 1\n\
.include ../models/pss_stage.inc\n\
.end\n",
    );

    let without_source = run_pss_analysis(&netlist, 1e6, 8, 1e-4);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_pss_analysis_with_source_path(&netlist, 1e6, 8, 1e-4, Some(&source_path))
        .expect("source-aware PSS analysis should resolve include");
    assert!(with_source.period > 0.0);
    assert!(!with_source.waveforms.is_empty());
}

#[test]
fn test_run_hb_analysis_executes_for_driven_rc() {
    let netlist = "* hb\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let hb_cfg = HbRunConfig {
        tones: vec![HbToneRunConfig::new(1e6, 5)],
        ..HbRunConfig::default()
    };
    let result =
        run_hb_analysis(netlist, &hb_cfg).expect("HB analysis should execute for driven RC");
    assert_eq!(result.fundamentals, vec![1e6]);
    assert_eq!(result.harmonics_per_tone, vec![5]);
    assert!(result.converged);
    assert!(result.num_components >= 2);
    assert!(!result.dc_voltages.is_empty());
    assert!(
        result
            .spectra
            .iter()
            .any(|(_, spectrum)| !spectrum.is_empty()),
        "expected at least one non-empty HB spectrum"
    );
}

#[test]
fn test_run_hb_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_nonlinear_include_fixture(
        "hb_top.rsch",
        "hb_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* hb include fixture\n\
V1 in 0 1\n\
.include ../models/hb_stage.inc\n\
.end\n",
    );
    let hb_cfg = HbRunConfig {
        tones: vec![HbToneRunConfig::new(1e6, 5)],
        ..HbRunConfig::default()
    };

    let without_source = run_hb_analysis(&netlist, &hb_cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_hb_analysis_with_source_path(&netlist, &hb_cfg, Some(&source_path))
        .expect("source-aware HB analysis should resolve include");
    assert_eq!(with_source.fundamentals, vec![1e6]);
    assert!(with_source.converged);
}

#[test]
fn test_run_hb_analysis_executes_with_two_tone_layout() {
    let netlist = "* hb two-tone\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let hb_cfg = HbRunConfig {
        tones: vec![HbToneRunConfig::new(2e6, 4), HbToneRunConfig::new(3e6, 3)],
        ..HbRunConfig::default()
    };
    let result = run_hb_analysis(netlist, &hb_cfg)
        .expect("HB two-tone analysis should execute for commensurate tones");
    assert_eq!(result.fundamentals, vec![2e6, 3e6]);
    assert_eq!(result.harmonics_per_tone, vec![4, 3]);
    assert!(result.converged);
    assert!(result.num_components >= 10);
    let first_spectrum = result
        .spectra
        .first()
        .expect("expected at least one HB spectrum");
    assert!(
        first_spectrum
            .1
            .iter()
            .any(|(freq, _, _)| (*freq - 1e6).abs() < 1e-6),
        "two-tone HB should use the derived 1 MHz basis frequency"
    );
}

#[test]
fn test_run_hb_analysis_executes_with_three_tone_layout() {
    let netlist = "* hb three-tone\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let hb_cfg = HbRunConfig {
        tones: vec![
            HbToneRunConfig::new(2e6, 2),
            HbToneRunConfig::new(3e6, 2),
            HbToneRunConfig::new(5e6, 1),
        ],
        max_mixing_order: 4,
        ..HbRunConfig::default()
    };
    let result = run_hb_analysis(netlist, &hb_cfg)
        .expect("HB three-tone analysis should execute for commensurate tones");
    assert_eq!(result.fundamentals, vec![2e6, 3e6, 5e6]);
    assert_eq!(result.harmonics_per_tone, vec![2, 2, 1]);
    assert!(result.converged);
    assert!(
        result.num_components >= 21,
        "max_mixing_order should increase harmonic budget for three-tone solve"
    );
    let first_spectrum = result
        .spectra
        .first()
        .expect("expected at least one HB spectrum");
    assert!(
        first_spectrum
            .1
            .iter()
            .any(|(freq, _, _)| (*freq - 1e6).abs() < 1e-6),
        "three-tone HB should use the derived 1 MHz basis frequency"
    );
}

#[test]
fn test_run_hb_analysis_routes_source_filtered_tones() {
    let netlist = "* hb source-routed\nVRF 1 0 DC 0 AC 1\nVLO 2 0 DC 0 AC 1\nR1 1 0 1k\nR2 2 0 1k\nC1 1 0 1n\nC2 2 0 1n\n.end\n";
    let hb_cfg = HbRunConfig {
        tones: vec![
            HbToneRunConfig::new(2e6, 1)
                .with_name("rf")
                .with_source("VRF"),
            HbToneRunConfig::new(3e6, 1)
                .with_name("lo")
                .with_source("VLO"),
        ],
        ..HbRunConfig::default()
    };

    let data = run_hb_analysis(netlist, &hb_cfg)
        .expect("HB should route source-filtered tones to matching sources");
    assert!(data.converged);

    let magnitude_at = |spectrum: &[(Value, Value, Value)], target_freq: Value| -> Value {
        spectrum
            .iter()
            .find(|(freq, _, _)| (*freq - target_freq).abs() < 1e-6)
            .map(|(_, magnitude, _)| *magnitude)
            .unwrap_or(0.0)
    };

    let vrf = data
        .spectra
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("V(1)"))
        .expect("expected V(1) spectrum")
        .1
        .as_slice();
    let vlo = data
        .spectra
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("V(2)"))
        .expect("expected V(2) spectrum")
        .1
        .as_slice();

    assert!(
        magnitude_at(vrf, 2e6) > 0.9,
        "VRF should be driven at 2 MHz"
    );
    assert!(
        magnitude_at(vrf, 3e6) < 1e-9,
        "VRF should not be driven at 3 MHz"
    );
    assert!(
        magnitude_at(vlo, 3e6) > 0.9,
        "VLO should be driven at 3 MHz"
    );
    assert!(
        magnitude_at(vlo, 2e6) < 1e-9,
        "VLO should not be driven at 2 MHz"
    );
}

#[test]
fn test_run_hb_analysis_rejects_invalid_runtime_controls() {
    let netlist = "* hb invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let hb_cfg = HbRunConfig {
        tones: vec![HbToneRunConfig::new(1e6, 5)],
        reltol: 0.0,
        ..HbRunConfig::default()
    };
    let err = run_hb_analysis(netlist, &hb_cfg)
        .expect_err("invalid HB runtime controls should be rejected");
    assert!(err.contains("reltol"));
}
