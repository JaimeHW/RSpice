//! `--corners` with `--corner-lib` must re-elaborate the deck per corner so
//! each corner's `.lib` section actually changes the simulated circuit.

use std::path::PathBuf;
use std::process::Command;

fn test_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_corner_lib_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn read_op_voltage(path: &std::path::Path, signal: &str) -> f64 {
    let text = std::fs::read_to_string(path).expect("read op csv");
    for line in text.lines().skip(1) {
        let mut fields = line.split(',');
        let name = fields.next().unwrap_or_default();
        if name.eq_ignore_ascii_case(signal) {
            return fields.next().unwrap().trim().parse().expect("numeric");
        }
    }
    panic!("signal {signal} not found in {}", path.display());
}

#[test]
fn corner_lib_sections_change_results() {
    let dir = test_dir();

    std::fs::write(
        dir.join("corners.lib"),
        "* process corner library\n\
         .lib tt\n\
         .param rload=1k\n\
         .endl tt\n\
         .lib ss\n\
         .param rload=3k\n\
         .endl ss\n",
    )
    .expect("write corner lib");

    let deck = dir.join("divider.sp");
    std::fs::write(
        &deck,
        "* corner-dependent divider\n\
         v1 in 0 dc 10\n\
         r1 in out 1k\n\
         r2 out 0 {rload}\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    let out = dir.join("res.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "run",
            deck.to_str().unwrap(),
            "--corners",
            "tt,ss",
            "--corner-lib",
            dir.join("corners.lib").to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "corner sweep failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // tt: 10 * 1k/2k = 5V; ss: 10 * 3k/4k = 7.5V
    let v_tt = read_op_voltage(&dir.join("res.tt.csv"), "V(OUT)");
    let v_ss = read_op_voltage(&dir.join("res.ss.csv"), "V(OUT)");
    assert!((v_tt - 5.0).abs() < 1e-6, "tt corner wrong: {v_tt}");
    assert!(
        (v_ss - 7.5).abs() < 1e-6,
        "ss corner must use its own models: {v_ss}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Parallel corner workers must produce byte-identical outputs to the
/// serial sweep — corners are independent, only the scheduling changes.
#[test]
fn parallel_corners_match_serial() {
    // Own top-level dir: the sibling tests remove the shared pid-keyed
    // directory when they finish, racing anything nested inside it.
    let dir = std::env::temp_dir().join(format!("rspice_corner_parallel_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create test dir");
    let lib = dir.join("corners.lib");
    std::fs::write(
        &lib,
        "* corners\n.lib tt\n.param rbot=1k\n.endl\n.lib ss\n.param rbot=3k\n.endl\n.lib ff\n.param rbot=500\n.endl\n",
    )
    .expect("write lib");
    let deck = dir.join("div.sp");
    std::fs::write(
        &deck,
        "* divider\nV1 in 0 10\nR1 in out 1k\nR2 out 0 {rbot}\n.param rbot=1k\n.op\n.end\n",
    )
    .expect("write deck");

    let run = |out: &std::path::Path, extra: &[&str]| {
        let mut args = vec![
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "--corners",
            "tt,ss,ff",
            "--corner-lib",
            lib.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ];
        args.extend_from_slice(extra);
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(&args)
            .output()
            .expect("run rspice");
        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };

    run(&dir.join("ser.csv"), &[]);
    run(&dir.join("par.csv"), &["-j", "3"]);

    for corner in ["tt", "ss", "ff"] {
        let serial = std::fs::read(dir.join(format!("ser.{corner}.csv"))).expect("serial out");
        let parallel = std::fs::read(dir.join(format!("par.{corner}.csv"))).expect("parallel out");
        assert_eq!(
            serial, parallel,
            "corner {corner} must be identical under -j"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn nominal_corners_record_measurements_per_corner() {
    let dir =
        std::env::temp_dir().join(format!("rspice_nominal_corner_meas_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create test dir");

    let deck = dir.join("nominal.sp");
    let summary = dir.join("summary.json");
    std::fs::write(
        &deck,
        "* nominal corner measurement accounting\n\
         R1 in out 50\n\
         R2 out 0 50\n\
         .AC LIN 1 1k 1k\n\
         .MEAS TRAN SHOULD_FAIL MAX V(out)\n\
         .END\n",
    )
    .expect("write deck");

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "--corners",
            "tt,ss",
            "--summary",
            summary.to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert_eq!(
        output.status.code(),
        Some(3),
        "failed corner measurements must fail the command; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).expect("summary file"))
            .expect("valid json");
    let measurements = json["runs"][0]["measurements"]
        .as_array()
        .expect("measurements array");
    let names: Vec<&str> = measurements
        .iter()
        .map(|m| m["name"].as_str().expect("measurement name"))
        .collect();
    assert_eq!(
        names,
        vec!["tt:SHOULD_FAIL", "ss:SHOULD_FAIL"],
        "nominal corners must report isolated, tagged measurements: {json}"
    );
    assert!(
        measurements
            .iter()
            .all(|m| m["passed"].as_bool() == Some(false)),
        "each corner measurement should fail explicitly: {json}"
    );

    let display = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["run", deck.to_str().unwrap(), "--corners", "tt,ss"])
        .output()
        .expect("run rspice display");
    assert_eq!(
        display.status.code(),
        Some(3),
        "failed corner measurements must fail the display run; stderr: {}",
        String::from_utf8_lossy(&display.stderr)
    );
    let stdout = String::from_utf8_lossy(&display.stdout);
    assert!(
        stdout.contains("FAIL"),
        "corner summary should mark failed measurements as failed; stdout: {stdout}"
    );
    assert!(
        !stdout.contains("PASS"),
        "corner summary must not report PASS when measurements failed; stdout: {stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn missing_corner_section_fails_the_corner() {
    let dir = test_dir().join("missing");
    std::fs::create_dir_all(&dir).unwrap();

    std::fs::write(
        dir.join("corners.lib"),
        "* lib with only tt\n.lib tt\n.param rload=1k\n.endl tt\n",
    )
    .unwrap();
    let deck = dir.join("divider.sp");
    std::fs::write(
        &deck,
        "* divider\nv1 in 0 dc 10\nr1 in out 1k\nr2 out 0 {rload}\n.op\n.end\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "--corners",
            "tt,nosuchcorner",
            "--corner-lib",
            dir.join("corners.lib").to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert!(
        !output.status.success(),
        "a corner with no library section must fail the run"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
