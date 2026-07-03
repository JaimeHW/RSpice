//! CLI ergonomics: stdin netlists, --save output selection, SPICE-suffix
//! flag values, static topology checks, and config-file layering.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_ergo_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

#[test]
fn run_reads_netlist_from_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["run", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn rspice");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"* stdin divider\nV1 in 0 5\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n")
        .expect("write deck");
    let output = child.wait_with_output().expect("wait");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("V(OUT) = 2.500000"),
        "divider OP expected in output: {stdout}"
    );
}

#[test]
fn save_flag_restricts_exported_signals() {
    let dir = test_dir("save");
    let deck = dir.join("rc.sp");
    std::fs::write(
        &deck,
        "* rc\nV1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\nR1 in out 1k\nC1 out 0 100p\n.tran 1n 1u\n.end\n",
    )
    .expect("write deck");
    let out = dir.join("out.csv");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
        "--save",
        "V(out)",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let header = std::fs::read_to_string(&out)
        .expect("csv")
        .lines()
        .next()
        .unwrap()
        .to_string();
    assert_eq!(
        header.to_uppercase(),
        "TIME,V(OUT)",
        "--save must restrict the exported columns"
    );

    // An unparseable probe is a usage error.
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "--save",
        "V(",
    ]);
    assert_eq!(output.status.code(), Some(2));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_flag_exports_differential_voltage_waveform() {
    let dir = test_dir("save_diff");
    let deck = dir.join("divider.sp");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 5\nR1 in out 1k\nR2 out 0 1k\n.tran 1n 2n\n.end\n",
    )
    .expect("write deck");
    let out = dir.join("diff.csv");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
        "--save",
        "V(in,out)",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let csv = std::fs::read_to_string(&out).expect("csv");
    let mut lines = csv.lines();
    assert_eq!(
        lines.next(),
        Some("time,\"V(in,out)\""),
        "differential voltage names must be exported as one quoted CSV column"
    );
    let values: Vec<f64> = lines
        .map(|line| {
            line.split_once(',')
                .expect("time,value row")
                .1
                .parse::<f64>()
                .expect("numeric differential voltage")
        })
        .collect();
    assert!(!values.is_empty(), "transient output must include samples");
    for value in values {
        assert!(
            (value - 2.5).abs() < 1e-9,
            "V(in,out) should equal 2.5 V, got {value}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_flag_quotes_differential_voltage_in_dc_csv() {
    let dir = test_dir("save_diff_op");
    let deck = dir.join("divider.sp");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 5\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");
    let out = dir.join("diff-op.csv");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        "csv",
        "--save",
        "V(in,out)",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let csv = std::fs::read_to_string(&out).expect("csv");
    assert!(
        csv.lines().any(|line| line.starts_with("\"V(in,out)\",")),
        "DC OP CSV must quote differential probe names as one field: {csv}"
    );
    let value = csv
        .lines()
        .skip(1)
        .find_map(|line| line.rsplit_once(',').map(|(_, value)| value))
        .and_then(|field| field.parse::<f64>().ok())
        .unwrap_or_else(|| panic!("missing numeric V(in,out) value: {csv}"));
    assert!(
        (value - 2.5).abs() < 1.0e-9,
        "V(in,out) should equal 2.5 V, got {value:.17e}: {csv}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn numeric_flags_accept_spice_suffixes() {
    let dir = test_dir("suffix");
    let deck = dir.join("rc.sp");
    std::fs::write(
        &deck,
        "* rc\nV1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\nR1 in out 1k\nC1 out 0 100p\n.tran 1n 1u\n.end\n",
    )
    .expect("write deck");

    // 1u = 1e-6 as a max-step value; plain parse would reject "1u".
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--max-step",
        "1u",
        "--temp",
        "85",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn check_flags_voltage_source_loops() {
    let dir = test_dir("vloop");
    let deck = dir.join("vloop.sp");
    std::fs::write(
        &deck,
        "* conflicting sources\nV1 a 0 5\nV2 a 0 3\n.op\n.end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["check", deck.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "singular topology must fail check"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("loop of voltage sources"),
        "check should name the problem: {stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn check_rejects_xspice_port_type_mismatch() {
    let dir = test_dir("xspice_bad_io");
    let deck = dir.join("xspice_bad_io.sp");
    std::fs::write(
        &deck,
        "* invalid XSPICE typed port\n\
         V1 1 0 0\n\
         A1 1 %d 2 gain_block\n\
         .model gain_block gain (gain=10)\n\
         R2 2 0 1k\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["check", deck.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "XSPICE port mismatch must fail check"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("XSPICE build validation failed")
            && stdout.contains("does not allow explicit Digital"),
        "check should report the XSPICE port-type problem: {stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn check_validates_xspice_without_launching_external_runtimes() {
    let dir = test_dir("xspice_external_check");
    let deck = dir.join("xspice_external_check.sp");
    std::fs::write(
        &deck,
        "* external XSPICE static check\n\
         A1 [din] [dout] co\n\
         .model co d_cosim simulation=\"./missing_cosim_runtime\"\n\
         A2 [din] clk null [pout] proc\n\
         .model proc d_process process_file=\"\"\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "check", deck.to_str().unwrap()]);
    assert!(
        output.status.success(),
        "static check must not launch d_cosim runtime; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A config value explicitly set to the built-in default must still
/// override a lower layer (the old merge compared against defaults and
/// could not).
#[test]
fn config_file_sets_values_by_presence() {
    let dir = test_dir("config");
    let deck = dir.join("rc.sp");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 5\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");

    let config = dir.join("rspice.toml");
    std::fs::write(
        &config,
        "[output]\nformat = \"csv\"\n\n[simulation]\ntemperature = 27.0\n",
    )
    .expect("write config");

    let out = dir.join("res.csv");
    let output = run_rspice(&[
        "--quiet",
        "--config",
        config.to_str().unwrap(),
        "run",
        deck.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let body = std::fs::read_to_string(&out).expect("output exists");
    assert!(
        body.starts_with("signal,"),
        "config output.format=csv must apply: {body}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
