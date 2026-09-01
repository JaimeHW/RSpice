//! Public-CLI contracts for `.STEP` as a Cartesian decorator around authored
//! analyses. Every coordinate owns a complete child analysis and a distinct
//! output artifact; the first authored STEP dimension varies fastest.

use std::path::{Path, PathBuf};
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "rspice_step_cartesian_{}_{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("create test directory");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

fn step_output(base: &Path, one_based_index: usize) -> PathBuf {
    let stem = base.file_stem().expect("output stem").to_string_lossy();
    base.with_file_name(format!(
        "{stem}.step_{one_based_index:06}.{}",
        base.extension()
            .expect("output extension")
            .to_string_lossy()
    ))
}

fn csv_column(csv: &str, name: &str) -> Vec<f64> {
    let mut lines = csv.lines();
    let header = lines.next().expect("CSV header");
    let index = header
        .split(',')
        .position(|column| column.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing {name} in {header}"));
    lines
        .map(|line| {
            line.split(',')
                .nth(index)
                .expect("CSV value")
                .parse()
                .expect("numeric CSV value")
        })
        .collect()
}

fn scalar_csv_value(csv: &str, name: &str) -> f64 {
    csv.lines()
        .skip(1)
        .find_map(|line| {
            let (signal, value) = line.split_once(',')?;
            signal
                .eq_ignore_ascii_case(name)
                .then(|| value.parse().expect("numeric scalar CSV value"))
        })
        .unwrap_or_else(|| panic!("missing {name} in {csv}"))
}

fn complex_csv_value_at(csv: &str, name: &str, frequency: f64) -> (f64, f64) {
    let frequencies = csv_column(csv, "frequency");
    let real = csv_column(csv, &format!("Re({name})"));
    let imaginary = csv_column(csv, &format!("Im({name})"));
    let row = frequencies
        .iter()
        .position(|value| (*value - frequency).abs() <= frequency.abs().max(1.0) * 1.0e-12)
        .unwrap_or_else(|| panic!("missing {frequency} Hz row in {csv}"));
    (real[row], imaginary[row])
}

fn complex_csv_magnitude_at(csv: &str, name: &str, frequency: f64) -> f64 {
    let (real, imaginary) = complex_csv_value_at(csv, name, frequency);
    real.hypot(imaginary)
}

#[test]
fn two_step_dimensions_wrap_the_complete_dc_sweep_in_xyce_order() {
    let dir = test_dir("dc");
    let deck = dir.join("cartesian.sp");
    let output_path = dir.join("sweep.csv");
    std::fs::write(
        &deck,
        "* Cartesian STEP around DC\n\
         V1 in 0 0\n\
         R1 in out {rleft}\n\
         R2 out 0 {rright}\n\
         .param rleft=1k rright=1k\n\
         .step param rleft list 1k 2k\n\
         .step param rright list 1k 2k 3k\n\
         .dc V1 0 1 1\n\
         .end\n",
    )
    .expect("write Cartesian deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        !output_path.exists(),
        "the unqualified path must not be reused"
    );

    let expected_vout = [0.5, 1.0 / 3.0, 2.0 / 3.0, 0.5, 0.75, 0.6];
    for (index, expected) in expected_vout.into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let source = csv_column(&csv, "V1");
        let vout = csv_column(&csv, "V(OUT)");
        assert_eq!(source, vec![0.0, 1.0], "complete DC grid in {csv}");
        assert_eq!(vout.len(), 2, "one full child sweep in {csv}");
        assert!(
            (vout[1] - expected).abs() < 1.0e-9,
            "coordinate {} violates first-dimension-fastest order: {csv}",
            index + 1
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn femto_step_runs_exactly_six_fresh_transient_analyses() {
    let dir = test_dir("tran");
    let deck = dir.join("transient.sp");
    let output_path = dir.join("wave.csv");
    std::fs::write(
        &deck,
        "* Femto STEP around transient\n\
         V1 in 0 PULSE(0 1 0 10p 10p 1n 2n)\n\
         R1 in out 1k\n\
         C1 out 0 {cap}\n\
         .param cap=100f\n\
         .step param cap 100f 150f 10f\n\
         .tran 50p 500p\n\
         .end\n",
    )
    .expect("write transient deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    for index in 1..=6 {
        let path = step_output(&output_path, index);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let time = csv_column(&csv, "time");
        assert_eq!(time.first().copied(), Some(0.0), "fresh transient: {csv}");
        assert!(time.last().is_some_and(|value| *value >= 500.0e-12));
    }
    assert!(
        !step_output(&output_path, 7).exists(),
        "the femto grid must contain exactly six coordinates"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn authored_hb_runs_once_per_step_coordinate_with_distinct_spectra() {
    let dir = test_dir("hb");
    let deck = dir.join("harmonic_balance.sp");
    let output_path = dir.join("spectrum.csv");
    std::fs::write(
        &deck,
        "* STEP around authored harmonic balance\n\
         .param amplitude=100m\n\
         V1 out 0 SIN(0 {amplitude} 1meg)\n\
         R1 out 0 1k\n\
         .step param amplitude list 100m 300m\n\
         .hb 1meg\n\
         .print hb V(out) I(V1)\n\
         .end\n",
    )
    .expect("write stepped HB deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        !output_path.exists(),
        "the unqualified path must not be reused"
    );

    for (index, expected) in [0.1, 0.3].into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let fundamental = complex_csv_magnitude_at(&csv, "V(OUT)", 1.0e6);
        assert!(
            (fundamental - expected).abs() <= expected * 1.0e-8,
            "coordinate {} must bind amplitude={expected}: {csv}",
            index + 1
        );
        let (voltage_real, voltage_imaginary) = complex_csv_value_at(&csv, "V(OUT)", 1.0e6);
        let (current_real, current_imaginary) = complex_csv_value_at(&csv, "I(V1)", 1.0e6);
        let current_magnitude = current_real.hypot(current_imaginary);
        assert!(
            (current_magnitude - expected / 1_000.0).abs() <= expected * 1.0e-11,
            "coordinate {} must retain the V1 branch-current spectrum: {csv}",
            index + 1
        );
        assert!(
            (voltage_real + 1_000.0 * current_real).abs() <= expected * 1.0e-8
                && (voltage_imaginary + 1_000.0 * current_imaginary).abs() <= expected * 1.0e-8,
            "coordinate {} must satisfy V(out)/R1 + I(V1) = 0: {csv}",
            index + 1
        );
    }
    assert!(
        !step_output(&output_path, 3).exists(),
        "the bounded plan must contain exactly two HB coordinates"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn conditional_hb_signature_change_fails_before_any_step_output() {
    let dir = test_dir("conditional_hb");
    let deck = dir.join("conditional_hb.sp");
    let output_path = dir.join("conditional_hb.csv");
    std::fs::write(
        &deck,
        "* STEP must not conditionally add harmonic balance\n\
         .param mode=0\n\
         V1 out 0 SIN(0 100m 1meg)\n\
         R1 out 0 1k\n\
         .step param mode list 0 1\n\
         .if (mode==1)\n\
         .hb 1meg\n\
         .endif\n\
         .end\n",
    )
    .expect("write conditional HB deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert_eq!(
        run.status.code(),
        Some(2),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stderr).contains("conditionally changes"),
        "stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    assert!(!step_output(&output_path, 1).exists());
    assert!(!step_output(&output_path, 2).exists());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn default_ngspice_cli_does_not_apply_xyce_print_voltage_wildcard() {
    let dir = test_dir("ngspice_tran_print_vstar");
    let deck = dir.join("transient_vstar.sp");
    let output_path = dir.join("wave.csv");
    std::fs::write(
        &deck,
        "* STEP transient PRINT V(*)\n\
         VDrive Supply 0 PULSE(0 1 0 1p 1p 20p 50p)\n\
         RLoad Supply Load {resistance}\n\
         CLoad Load 0 1p\n\
         .param resistance=1k\n\
         .step param resistance list 1k 2k\n\
         .tran 5p 20p\n\
         .print tran V(Load) V(*)\n\
         .end\n",
    )
    .expect("write stepped wildcard deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        !run.status.success(),
        "default CLI mode must not reinterpret ngspice V(*) as Xyce: stdout={}; stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stderr).contains("V(*)"),
        "failure must retain the unsupported authored probe: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    assert!(!step_output(&output_path, 1).exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn step_data_table_reaches_the_typed_planner_and_binds_each_row() {
    let dir = test_dir("data");
    let deck = dir.join("data_step.sp");
    let output_path = dir.join("operating_point.csv");
    std::fs::write(
        &deck,
        "* Typed STEP DATA ownership\n\
         V1 in 0 1\n\
         R1 in out {rleft}\n\
         R2 out 0 1k\n\
         .param rleft=1k\n\
         .data coordinates rleft\n\
         1k\n\
         3k\n\
         .enddata\n\
         .step data=coordinates\n\
         .op\n\
         .end\n",
    )
    .expect("write STEP DATA deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        run.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    for (index, expected) in [0.5, 0.25].into_iter().enumerate() {
        let path = step_output(&output_path, index + 1);
        let csv = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let vout = scalar_csv_value(&csv, "V(OUT)");
        assert!((vout - expected).abs() < 1.0e-9, "{csv}");
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn conditional_child_analysis_change_fails_before_any_step_output() {
    let dir = test_dir("conditional_analysis");
    let deck = dir.join("conditional.sp");
    let output_path = dir.join("conditional.csv");
    std::fs::write(
        &deck,
        "* STEP must not change its child analysis set\n\
         .param mode=0\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .step param mode list 0 1\n\
         .if (mode==1)\n\
         .tran 1n 2n\n\
         .endif\n\
         .end\n",
    )
    .expect("write conditional deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert_eq!(
        run.status.code(),
        Some(2),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stderr).contains("conditionally changes"),
        "stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    assert!(!step_output(&output_path, 1).exists());
    assert!(!step_output(&output_path, 2).exists());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn outer_alter_step_incompatibility_is_rejected_before_base_output() {
    let dir = test_dir("alter_preflight");
    let deck = dir.join("alter_step.sp");
    let output_path = dir.join("alter_step.csv");
    std::fs::write(
        &deck,
        "* outer ALTER must be globally preflighted\n\
         .param r=1k\n\
         V1 in 0 1\n\
         R1 in 0 {r}\n\
         .op\n\
         .alter stepped\n\
         .step param r list 1k 2k\n\
         .end\n",
    )
    .expect("write ALTER STEP deck");

    let run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        output_path.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert_eq!(
        run.status.code(),
        Some(2),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(
        String::from_utf8_lossy(&run.stderr).contains("cannot yet be composed with .STEP"),
        "stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(!output_path.exists());
    assert_eq!(
        std::fs::read_dir(&dir)
            .expect("read test directory")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "csv"))
            .count(),
        0,
        "global preflight must prevent every outer-run output"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn step_batch_limit_retains_typed_resource_metadata() {
    let dir = test_dir("batch_limit");
    let deck = dir.join("bounded.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(
        &deck,
        "* bounded STEP\n\
         .param r=1k\n\
         V1 in 0 1\n\
         R1 in 0 {r}\n\
         .step param r list 1k 2k\n\
         .op\n\
         .end\n",
    )
    .expect("write bounded STEP deck");
    std::fs::write(&config, "[resources]\nmax_batch_runs = 1\n").expect("write resource config");

    let run = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--error-format",
        "json",
        "--quiet",
        "run",
        deck.to_str().unwrap(),
    ]);
    assert_eq!(run.status.code(), Some(1));
    let diagnostic: serde_json::Value =
        serde_json::from_slice(&run.stderr).expect("structured diagnostic");
    assert_eq!(diagnostic["error"]["code"], "resource_limit");
    assert_eq!(diagnostic["error"]["resource"], "batch_runs");
    assert_eq!(diagnostic["error"]["requested"], 2);
    assert_eq!(diagnostic["error"]["limit"], 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn implicit_step_charges_its_single_aggregate_report_once() {
    let dir = test_dir("aggregate_report_limit");
    let deck = dir.join("aggregate.sp");
    let config = dir.join("rspice.toml");
    std::fs::write(
        &deck,
        "* one aggregate report for implicit STEP compatibility\n\
         .param p=0\n\
         .step param p list 1 2 3 4\n\
         .meas op constant param='1'\n\
         .end\n",
    )
    .expect("write aggregate STEP deck");
    std::fs::write(&config, "[resources]\nmax_result_values = 8\n").expect("write resource config");

    let run = run_rspice(&[
        "--config",
        config.to_str().unwrap(),
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--allow-failed-meas",
    ]);
    assert!(
        run.status.success(),
        "the four OP payloads and single aggregate report fit exactly; stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn stepped_measurement_exports_identify_every_coordinate_run() {
    let dir = test_dir("measurement_identity");
    let deck = dir.join("measured.sp");
    let json_path = dir.join("measurements.json");
    let csv_path = dir.join("measurements.csv");
    std::fs::write(
        &deck,
        "* STEP measurement identity\n\
         .param rleft=1k\n\
         V1 in 0 0\n\
         R1 in out {rleft}\n\
         R2 out 0 1k\n\
         .step param rleft list 1k 2k\n\
         .dc V1 0 1 1\n\
         .meas dc vfinal find V(out) at=1\n\
         .end\n",
    )
    .expect("write measured STEP deck");

    let json_run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--meas-file",
        json_path.to_str().unwrap(),
    ]);
    assert!(
        json_run.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&json_run.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&json_path).expect("read measurement JSON"))
            .expect("parse measurement JSON");
    let measurements = json["measurements"].as_array().expect("measurement array");
    assert_eq!(measurements.len(), 2);
    assert!(
        measurements[0]["run"]
            .as_str()
            .is_some_and(|run| run.ends_with("[step-000001]"))
    );
    assert!(
        measurements[1]["run"]
            .as_str()
            .is_some_and(|run| run.ends_with("[step-000002]"))
    );

    let csv_run = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--meas-file",
        csv_path.to_str().unwrap(),
        "--meas-format",
        "csv",
    ]);
    assert!(
        csv_run.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&csv_run.stderr)
    );
    let csv = std::fs::read_to_string(&csv_path).expect("read measurement CSV");
    let lines = csv.lines().collect::<Vec<_>>();
    assert_eq!(
        lines[0],
        "netlist,name,value,expected,tolerance,passed,error,run,raw_value,failure_limit,failure_limit_exceeded"
    );
    let run_column = lines[0]
        .split(',')
        .position(|column| column == "run")
        .expect("measurement CSV has a run column");
    assert!(
        lines[1]
            .split(',')
            .nth(run_column)
            .is_some_and(|run| run.ends_with("[step-000001]"))
    );
    assert!(
        lines[2]
            .split(',')
            .nth(run_column)
            .is_some_and(|run| run.ends_with("[step-000002]"))
    );

    let _ = std::fs::remove_dir_all(&dir);
}
