use super::*;
use std::path::PathBuf;

fn write_pole_sensitivity_include_fixture() -> (tempfile::TempDir, PathBuf, String) {
    let temp_dir = tempfile::tempdir().expect("temp dir should be created");
    let source_path = temp_dir.path().join("project").join("analysis_top.rsch");
    std::fs::create_dir_all(
        source_path
            .parent()
            .expect("source path should have parent directory"),
    )
    .expect("project directory should be created");
    std::fs::create_dir_all(temp_dir.path().join("models"))
        .expect("models directory should be created");
    std::fs::write(
        temp_dir.path().join("models").join("analysis_stage.inc"),
        ".param RVAL=1k\nR1 in out {RVAL}\nC1 out 0 1n\n",
    )
    .expect("include file should be written");

    let netlist = "* analysis include fixture\n\
V1 in 0 DC 1 AC 1\n\
.include ../models/analysis_stage.inc\n\
.end\n"
        .to_string();

    (temp_dir, source_path, netlist)
}

#[test]
fn test_run_pole_zero_analysis_validation() {
    let netlist = "* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n";

    let err = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "BAD", "PZ")
        .expect_err("expected transfer_type validation");
    assert!(err.contains("transfer_type"));

    let err = run_pole_zero_analysis(netlist, "in", "nref", "out", "0", "VOL", "PZ")
        .expect_err("expected reference validation");
    assert!(err.contains("not found"));

    let err = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "BAD")
        .expect_err("expected analysis_type validation");
    assert!(err.contains("analysis_type"));

    run_pole_zero_analysis(netlist, "in", "0", "out", "0", "CUR", "PZ")
        .expect("CUR transfer_type should be accepted");
}

#[test]
fn test_run_pole_zero_analysis_filters_analysis_type() {
    let netlist = "* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n";

    let pol = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "POL")
        .expect("POL run should succeed");
    assert!(pol.zeros.is_empty());

    let zer = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "ZER")
        .expect("ZER run should succeed");
    assert!(zer.poles.is_empty());
}

#[test]
fn test_run_pole_zero_analysis_supports_non_ground_references() {
    let netlist =
        "* pz diff\nV1 in 0 1\nR1 in out 1k\nR2 out ref 500\nC1 out ref 1n\nR3 ref 0 1k\n";

    let diff = run_pole_zero_analysis(netlist, "in", "ref", "out", "ref", "VOL", "PZ")
        .expect("differential pole-zero should succeed");

    let h11 = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "PZ")
        .expect("h11 should succeed")
        .gain;
    let h12 = run_pole_zero_analysis(netlist, "ref", "0", "out", "0", "VOL", "PZ")
        .expect("h12 should succeed")
        .gain;
    let h21 = run_pole_zero_analysis(netlist, "in", "0", "ref", "0", "VOL", "PZ")
        .expect("h21 should succeed")
        .gain;
    let h22 = run_pole_zero_analysis(netlist, "ref", "0", "ref", "0", "VOL", "PZ")
        .expect("h22 should succeed")
        .gain;
    let expected = h11 - h12 - h21 + h22;

    assert!((diff.gain - expected).abs() < 1e-9);
}

#[test]
fn test_run_pole_zero_analysis_voltage_mode_reports_highpass_zero() {
    let netlist = "* hp\nC1 in out 1n\nR1 out 0 1k\n";

    let result = run_pole_zero_analysis(netlist, "in", "0", "out", "0", "VOL", "ZER")
        .expect("voltage-mode zero analysis should succeed");

    assert!(
        result
            .zeros
            .iter()
            .any(|(re, im)| (re * re + im * im).sqrt() < 1e-2),
        "expected zero near origin, got {:?}",
        result.zeros
    );
}

#[test]
fn test_run_sensitivity_analysis_validation() {
    let netlist = "* sens\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n";

    let err = run_sensitivity_analysis(netlist, "", false, None)
        .expect_err("expected output variable validation");
    assert!(err.contains("output_var"));

    let err = run_sensitivity_analysis(netlist, "V(out)", true, Some(0.0))
        .expect_err("expected AC frequency validation");
    assert!(err.contains("frequency"));

    let err = run_sensitivity_analysis(netlist, "V(out)", false, Some(1e6))
        .expect_err("expected mode/frequency validation");
    assert!(err.contains("only valid"));

    let err = run_sensitivity_analysis(netlist, "I(NO_SUCH_BRANCH)", false, None)
        .expect_err("expected branch output resolution failure");
    assert!(err.contains("resolved"));
}

#[test]
fn test_run_sensitivity_analysis_no_parameters_returns_empty() {
    let netlist = "* sens\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n";

    let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
        .expect("sensitivity run should succeed");
    assert_eq!(result.output_var, "V(out)");
    assert!(result.sensitivities.is_empty());
}

#[test]
fn test_run_sensitivity_analysis_with_parameter() {
    let netlist = "* sens\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n";

    let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
        .expect("sensitivity run should succeed");
    assert_eq!(result.output_var, "V(out)");
    assert!(!result.sensitivities.is_empty());
    assert!(result
        .sensitivities
        .iter()
        .any(|(name, _, _)| name.eq_ignore_ascii_case("RVAL")));
}

#[test]
fn test_run_sensitivity_analysis_filters_internal_side_channel_parameters() {
    let netlist = "* sens params\n.param RVAL=1k\n.param IC_START=0.1\n.param NODESET_OUT=0.2\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n";

    let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
        .expect("sensitivity run should succeed");

    assert!(result
        .sensitivities
        .iter()
        .any(|(name, _, _)| name.eq_ignore_ascii_case("RVAL")));
    assert!(result
        .sensitivities
        .iter()
        .all(|(name, _, _)| !name.starts_with("IC_") && !name.starts_with("NODESET_")));
}

#[test]
fn test_run_sensitivity_analysis_ac_mode_with_parameter() {
    let netlist = "* sens ac\n.param RVAL=1k\nV1 in 0 AC 1\nR1 in out RVAL\nC1 out 0 1n\n";

    let result = run_sensitivity_analysis(netlist, "V(out)", true, Some(1e6))
        .expect("ac sensitivity run should succeed");
    assert_eq!(result.output_var, "V(out)");
    let (_name, raw, normalized) = result
        .sensitivities
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
        .expect("expected RVAL sensitivity");
    assert!(raw.is_finite());
    assert!(normalized.is_finite());
}

#[test]
fn test_run_sensitivity_analysis_supports_differential_output() {
    let netlist = "* sens diff\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n";

    let result = run_sensitivity_analysis(netlist, "V(out,in)", false, None)
        .expect("differential sensitivity run should succeed");
    assert!(!result.sensitivities.is_empty());
    assert!(result
        .sensitivities
        .iter()
        .all(|(_, raw, norm)| raw.is_finite() && norm.is_finite()));
}

#[test]
fn test_run_sensitivity_analysis_supports_current_output_dc() {
    let netlist = "* sens i\n.param RVAL=1k\nV1 in 0 1\nR1 in 0 {RVAL}\n";

    let result = run_sensitivity_analysis(netlist, "I(V1)", false, None)
        .expect("current-output dc sensitivity should succeed");
    assert_eq!(result.output_var, "I(V1)");
    assert!(!result.sensitivities.is_empty());
    let (_name, raw, normalized) = result
        .sensitivities
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
        .expect("expected RVAL sensitivity");
    assert!(raw.is_finite());
    assert!(normalized.is_finite());
}

#[test]
fn test_run_sensitivity_analysis_supports_current_output_ac() {
    let netlist = "* sens iac\n.param RVAL=1k\nV1 in 0 AC 1\nR1 in 0 {RVAL}\n";

    let result = run_sensitivity_analysis(netlist, "I(V1)", true, Some(1e3))
        .expect("current-output ac sensitivity should succeed");
    assert_eq!(result.output_var, "I(V1)");
    assert!(!result.sensitivities.is_empty());
    assert!(result
        .sensitivities
        .iter()
        .all(|(_, raw, norm)| raw.is_finite() && norm.is_finite()));
}

#[test]
fn test_run_sensitivity_analysis_current_output_handles_multiple_parameters() {
    let netlist =
        "* sens i2\n.param RA=1k\n.param RB=2k\nV1 in 0 1\nR1 in mid {RA}\nR2 mid 0 {RB}\n";

    let result = run_sensitivity_analysis(netlist, "I(V1)", false, None)
        .expect("multi-parameter current-output sensitivity should succeed");
    let ra = result
        .sensitivities
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case("RA"))
        .expect("expected RA sensitivity");
    let rb = result
        .sensitivities
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case("RB"))
        .expect("expected RB sensitivity");

    assert!(ra.1.is_finite() && ra.2.is_finite());
    assert!(rb.1.is_finite() && rb.2.is_finite());
    assert!((ra.1 - rb.1).abs() < 1e-12);
}

#[test]
fn test_run_sensitivity_analysis_normalized_reports_zero_when_nominal_is_near_zero() {
    let netlist = "* sens tiny\n.param RVAL=1k\nV1 in 0 1e-16\nR1 in out {RVAL}\nR2 out 0 1k\n";

    let result = run_sensitivity_analysis(netlist, "V(out)", false, None)
        .expect("sensitivity run should succeed");
    let (_name, raw, normalized) = result
        .sensitivities
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case("RVAL"))
        .expect("expected RVAL sensitivity");
    assert!(raw.is_finite());
    assert_eq!(*normalized, 0.0);
}

#[test]
fn test_run_pole_zero_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_pole_sensitivity_include_fixture();

    let without_source = run_pole_zero_analysis(&netlist, "in", "0", "out", "0", "VOL", "PZ");
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_pole_zero_analysis_with_source_path(
        &netlist,
        "in",
        "0",
        "out",
        "0",
        "VOL",
        "PZ",
        Some(&source_path),
    )
    .expect("source-aware pole-zero analysis should resolve include");
    assert!(with_source.gain.is_finite());
}

#[test]
fn test_run_sensitivity_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_pole_sensitivity_include_fixture();

    let without_source = run_sensitivity_analysis(&netlist, "V(out)", false, None);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_sensitivity_analysis_with_source_path(
        &netlist,
        "V(out)",
        false,
        None,
        Some(&source_path),
    )
    .expect("source-aware sensitivity analysis should resolve include");
    assert!(!with_source.sensitivities.is_empty());
    assert!(with_source
        .sensitivities
        .iter()
        .any(|(name, _, _)| name.eq_ignore_ascii_case("RVAL")));
}
