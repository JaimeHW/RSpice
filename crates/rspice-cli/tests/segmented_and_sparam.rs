//! Checkpoint/resume segmentation and two-port S-parameter extraction.

use std::path::PathBuf;
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_seg_sp_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

fn last_vout(path: &std::path::Path) -> (f64, f64) {
    let text = std::fs::read_to_string(path).expect("read csv");
    let header: Vec<&str> = text.lines().next().expect("header").split(',').collect();
    let vout_col = header
        .iter()
        .position(|c| c.eq_ignore_ascii_case("V(OUT)"))
        .expect("V(OUT) column");
    let last = text.lines().last().expect("data");
    let fields: Vec<&str> = last.split(',').collect();
    (
        fields[0].parse().expect("time"),
        fields[vout_col].parse().expect("vout"),
    )
}

/// A run segmented through --checkpoint/--resume must land on the same
/// final state as one uninterrupted run; resuming against a different deck
/// must be refused.
#[test]
fn checkpoint_resume_matches_uninterrupted_run() {
    let dir = test_dir("ckpt");
    let deck = dir.join("rc.sp");
    std::fs::write(
        &deck,
        "* RC driven by a sine\n\
         V1 in 0 SIN(0 1 100k)\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .tran 10n 20u\n\
         .end\n",
    )
    .expect("write deck");

    let full = dir.join("full.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        full.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(output.status.success());

    let state = dir.join("state.ckpt");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--tran-stop",
        "10u",
        "--checkpoint",
        state.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(state.exists(), "checkpoint file must be written");

    let resumed = dir.join("resumed.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--resume",
        state.to_str().unwrap(),
        "-o",
        resumed.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let (t_full, v_full) = last_vout(&full);
    let (t_seg, v_seg) = last_vout(&resumed);
    assert!((t_full - 2e-5).abs() < 1e-9 && (t_seg - 2e-5).abs() < 1e-9);
    assert!(
        (v_full - v_seg).abs() < 1e-6,
        "segmented run must match the full run: full={v_full}, resumed={v_seg}"
    );

    // Fingerprint safety: a different deck cannot consume this state.
    let other = dir.join("other.sp");
    std::fs::write(
        &other,
        "* different circuit\nV1 in 0 SIN(0 1 100k)\nR1 in out 2k\nC1 out 0 100n\n.tran 10n 20u\n.end\n",
    )
    .expect("write deck");
    let output = run_rspice(&[
        "--quiet",
        "run",
        other.to_str().unwrap(),
        "--resume",
        state.to_str().unwrap(),
    ]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "mismatched checkpoint must be refused"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("different netlist"),
        "refusal should explain the fingerprint mismatch: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Series 50Ω two-port at Z0 = 50Ω: S11 = 1/3 and S21 = 2/3, exactly and
/// at every frequency.
#[test]
fn sparam_matches_series_resistor_analytics() {
    let dir = test_dir("sparam");
    let deck = dir.join("twoport.sp");
    std::fs::write(
        &deck,
        "* series 50 ohm two-port\nR1 p1 p2 50\n.ac lin 3 1k 100k\n.end\n",
    )
    .expect("write deck");

    let s2p = dir.join("out.s2p");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--sparam",
        "p1,0,p2,0",
        "-o",
        s2p.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let text = std::fs::read_to_string(&s2p).expect("touchstone file");
    assert!(text.contains("# HZ S RI R 50"), "touchstone header: {text}");
    for line in text.lines().filter(|l| !l.starts_with(['!', '#'])) {
        let fields: Vec<f64> = line
            .split_whitespace()
            .map(|f| f.parse().expect("numeric"))
            .collect();
        assert_eq!(fields.len(), 9, "freq + 4 complex pairs: {line}");
        let (s11re, s21re, s12re, s22re) = (fields[1], fields[3], fields[5], fields[7]);
        assert!((s11re - 1.0 / 3.0).abs() < 1e-9, "S11: {line}");
        assert!((s21re - 2.0 / 3.0).abs() < 1e-9, "S21: {line}");
        assert!((s12re - s21re).abs() < 1e-12, "reciprocity: {line}");
        assert!((s22re - s11re).abs() < 1e-12, "symmetry: {line}");
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn native_sp_card_matches_series_resistor_analytics() {
    let dir = test_dir("native_sp");
    let deck = dir.join("twoport_native_sp.cir");
    std::fs::write(
        &deck,
        "* native .sp two-port\n\
         V1 p1 0 dc 0 ac 1 portnum 1 z0 50\n\
         R1 p1 p2 50\n\
         V2 p2 0 dc 0 ac 0 portnum 2 z0 50\n\
         .sp lin 3 1k 100k\n\
         .end\n",
    )
    .expect("write deck");

    let csv = dir.join("out.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "-o",
        csv.to_str().unwrap(),
        "-f",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let text = std::fs::read_to_string(&csv).expect("csv output");
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().expect("header").split(',').collect();
    let idx = |name: &str| {
        header
            .iter()
            .position(|column| *column == name)
            .unwrap_or_else(|| panic!("missing {name} in header {header:?}"))
    };
    let s11_re = idx("Re(S_1_1)");
    let s21_re = idx("Re(S_2_1)");
    let s12_re = idx("Re(S_1_2)");
    let s22_re = idx("Re(S_2_2)");

    for line in lines {
        let fields: Vec<f64> = line
            .split(',')
            .map(|field| field.parse().expect("numeric csv field"))
            .collect();
        assert!((fields[s11_re] - 1.0 / 3.0).abs() < 1e-9, "S11: {line}");
        assert!((fields[s21_re] - 2.0 / 3.0).abs() < 1e-9, "S21: {line}");
        assert!(
            (fields[s12_re] - fields[s21_re]).abs() < 1e-12,
            "reciprocity: {line}"
        );
        assert!(
            (fields[s22_re] - fields[s11_re]).abs() < 1e-12,
            "symmetry: {line}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}
