//! With several analysis cards in one deck, `-o` must produce one file per
//! analysis instead of silently overwriting the same path.

use std::path::PathBuf;
use std::process::Command;

fn test_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_multi_analysis_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

#[test]
fn each_analysis_gets_its_own_output_file() {
    let dir = test_dir();
    let deck = dir.join("multi.sp");
    std::fs::write(
        &deck,
        "* op + transient in one deck\n\
         v1 in 0 sin(0 1 1k)\n\
         r1 in out 1k\n\
         c1 out 0 1u\n\
         .op\n\
         .tran 10u 100u\n\
         .end\n",
    )
    .expect("write deck");

    let out = dir.join("results.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let op = dir.join("results.op.csv");
    let tran = dir.join("results.tran.csv");
    assert!(
        op.exists(),
        "missing per-analysis OP output {}",
        op.display()
    );
    assert!(
        tran.exists(),
        "missing per-analysis transient output {}",
        tran.display()
    );
    assert!(
        !out.exists(),
        "untagged output should not exist when several analyses run"
    );

    let op_text = std::fs::read_to_string(&op).expect("read op csv");
    assert!(op_text.starts_with("signal,"), "op file has op schema");
    let tran_text = std::fs::read_to_string(&tran).expect("read tran csv");
    assert!(
        tran_text.starts_with("time,"),
        "tran file has waveform schema"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn single_analysis_keeps_exact_output_path() {
    let dir = test_dir().join("single");
    std::fs::create_dir_all(&dir).unwrap();
    let deck = dir.join("single.sp");
    std::fs::write(
        &deck,
        "* single analysis\nv1 in 0 dc 5\nr1 in out 1k\nr2 out 0 1k\n.op\n.end\n",
    )
    .expect("write deck");

    let out = dir.join("exact.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(output.status.success());
    assert!(
        out.exists(),
        "single-analysis output must keep the exact path"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
