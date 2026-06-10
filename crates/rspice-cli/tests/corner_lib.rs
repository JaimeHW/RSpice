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
            "--quiet",
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
    assert!((v_ss - 7.5).abs() < 1e-6, "ss corner must use its own models: {v_ss}");

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
