//! Every run mode must produce machine-readable output under `-o`:
//! .STEP sweeps, Monte Carlo samples, transfer function, pole-zero, and
//! sensitivity used to print summaries and discard the data.

use std::path::PathBuf;
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_adv_out_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

fn write_deck(dir: &std::path::Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, body).expect("write deck");
    path
}

#[test]
fn step_sweep_exports_table() {
    let dir = test_dir("step");
    let deck = write_deck(
        &dir,
        "step.sp",
        "* stepped divider\n\
         V1 in 0 5\n\
         R1 in out {rval}\n\
         R2 out 0 1k\n\
         .param rval=1k\n\
         .step param rval 1k 3k 1k\n\
         .end\n",
    );
    let out = dir.join("sweep.csv");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let csv = std::fs::read_to_string(&out).expect("step sweep table must be written");
    let mut lines = csv.lines();
    let header = lines.next().expect("header");
    assert!(
        header.to_uppercase().starts_with("RVAL"),
        "scale column should be the stepped parameter: {header}"
    );
    let rows: Vec<&str> = lines.collect();
    assert_eq!(rows.len(), 3, "three step values: {csv}");

    // 5V * 1k/(rval+1k): 2.5, 1.667, 1.25
    let vout_idx = header
        .split(',')
        .position(|c| c.eq_ignore_ascii_case("V(OUT)"))
        .expect("V(OUT) column");
    let vout_at = |row: &str| -> f64 { row.split(',').nth(vout_idx).unwrap().parse().unwrap() };
    assert!((vout_at(rows[0]) - 2.5).abs() < 1e-9);
    assert!((vout_at(rows[2]) - 1.25).abs() < 1e-9);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn monte_carlo_exports_samples_and_is_seed_deterministic() {
    let dir = test_dir("mc");
    let deck = write_deck(
        &dir,
        "mc.sp",
        "* divider for MC\n\
         V1 in 0 5\n\
         R1 in out {rtop}\n\
         R2 out 0 1k\n\
         .param rtop=1k\n\
         .op\n\
         .end\n",
    );

    let json_out = dir.join("mc.json");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--monte-carlo",
        "6",
        "--seed",
        "11",
        "--mc-spread",
        "0.05",
        "-o",
        json_out.to_str().unwrap(),
        "-f",
        "json",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&json_out).expect("mc json")).expect("parse");
    assert_eq!(json["analysis"], "monte_carlo");
    assert_eq!(json["runs"], 6);
    assert_eq!(json["seed"], 11);
    let variables = json["variables"].as_array().expect("variables array");
    let vout = variables
        .iter()
        .find(|v| {
            v["name"]
                .as_str()
                .unwrap_or("")
                .eq_ignore_ascii_case("V(OUT)")
        })
        .expect("V(OUT) variable");
    assert_eq!(vout["samples"].as_array().unwrap().len(), 6);
    let std_dev = vout["std_dev"].as_f64().unwrap();
    assert!(
        std_dev > 1e-3,
        "5% spread must move the divider output, std={std_dev}"
    );

    // Same seed, same samples — byte-identical CSV exports.
    let csv_a = dir.join("a.csv");
    let csv_b = dir.join("b.csv");
    for out in [&csv_a, &csv_b] {
        let output = run_rspice(&[
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "--monte-carlo",
            "4",
            "--seed",
            "42",
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ]);
        assert!(output.status.success());
    }
    assert_eq!(
        std::fs::read(&csv_a).unwrap(),
        std::fs::read(&csv_b).unwrap(),
        "same seed must reproduce identical samples"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn monte_carlo_zero_spread_exports_nominal_samples() {
    let dir = test_dir("mc_zero");
    let deck = write_deck(
        &dir,
        "mc_zero.sp",
        "* deterministic MC plumbing check\n\
         V1 in 0 5\n\
         R1 in out {rtop}\n\
         R2 out 0 1k\n\
         .param rtop=1k\n\
         .op\n\
         .end\n",
    );
    let json_out = dir.join("mc_zero.json");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--monte-carlo",
        "4",
        "--mc-spread",
        "0",
        "-o",
        json_out.to_str().unwrap(),
        "-f",
        "json",
    ]);
    assert!(
        output.status.success(),
        "zero spread should mean deterministic nominal samples; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&json_out).expect("mc json")).expect("parse");
    let variables = json["variables"].as_array().expect("variables array");
    let vout = variables
        .iter()
        .find(|v| {
            v["name"]
                .as_str()
                .unwrap_or("")
                .eq_ignore_ascii_case("V(OUT)")
        })
        .expect("V(OUT) variable");
    assert_eq!(vout["std_dev"].as_f64().unwrap(), 0.0);
    let samples = vout["samples"].as_array().unwrap();
    assert_eq!(samples.len(), 4);
    assert!(
        samples
            .iter()
            .all(|sample| (sample.as_f64().unwrap() - 2.5).abs() < 1e-9)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transfer_function_exports_scalars() {
    let dir = test_dir("tf");
    let deck = write_deck(
        &dir,
        "tf.sp",
        "* divider\n\
         V1 in 0 5\n\
         R1 in out 1k\n\
         R2 out 0 1k\n\
         .tf V(out) V1\n\
         .end\n",
    );
    let out = dir.join("tf.csv");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let csv = std::fs::read_to_string(out.with_file_name("tf.tf.csv").as_path())
        .or_else(|_| std::fs::read_to_string(&out))
        .expect("tf table");
    let mut lines = csv.lines();
    let header = lines.next().expect("header").to_lowercase();
    let row = lines.next().expect("data row");
    let col = |name: &str| -> f64 {
        let idx = header
            .split(',')
            .position(|c| c.contains(name))
            .unwrap_or_else(|| panic!("missing column {name} in {header}"));
        row.split(',').nth(idx).unwrap().parse().unwrap()
    };
    assert!((col("transfer_function") - 0.5).abs() < 1e-9);
    assert!((col("input_impedance") - 2000.0).abs() < 1e-6);
    assert!((col("output_impedance") - 500.0).abs() < 1e-6);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn pole_zero_exports_complex_singularities() {
    let dir = test_dir("pz");
    // RC low-pass: single pole at -1/(RC) = -1e4 rad/s.
    let deck = write_deck(
        &dir,
        "rc.sp",
        "* rc\n\
         V1 in 0 SIN(0 1 1k)\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .end\n",
    );
    let out = dir.join("pz.csv");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--pz-input",
        "in",
        "--pz-output",
        "out",
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let csv = std::fs::read_to_string(&out).expect("pz table");
    let mut lines = csv.lines();
    let header = lines.next().expect("header");
    assert!(
        header.contains("Re(pole(1))"),
        "pole column expected: {header}"
    );
    let row = lines.next().expect("data row");
    let re_idx = header.split(',').position(|c| c == "Re(pole(1))").unwrap();
    let pole_re: f64 = row.split(',').nth(re_idx).unwrap().parse().unwrap();
    assert!(
        (pole_re + 1e4).abs() < 1.0,
        "RC pole should be at -1e4 rad/s, got {pole_re}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn sensitivity_exports_table() {
    let dir = test_dir("sens");
    let deck = write_deck(
        &dir,
        "div.sp",
        "* divider\n\
         V1 in 0 5\n\
         R1 in out {rtop}\n\
         R2 out 0 1k\n\
         .param rtop=1k\n\
         .op\n\
         .end\n",
    );
    let out = dir.join("sens.csv");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--sens-output",
        "out",
        "--sens-param",
        "rtop",
        "--sens-value",
        "1000",
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let csv = std::fs::read_to_string(&out).expect("sensitivity table");
    assert!(csv.contains("dV/d(rtop)"), "column header: {csv}");
    // dV(out)/dRtop = -V*R2/(R1+R2)^2 = -1.25e-3 V/ohm
    let row = csv.lines().nth(1).expect("data row");
    let value: f64 = row.split(',').nth(1).unwrap().parse().unwrap();
    assert!(
        (value + 1.25e-3).abs() < 5e-5,
        "divider sensitivity should be ~-1.25e-3, got {value}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
