mod common;
use std::process::Command;
#[test]
fn qpss_executes_after_an_earlier_op_with_distinct_artifacts() {
    let dir = common::test_dir("preflight");
    let (name, analysis, format, expected) = (
        "qpss",
        ".qpss 1meg 1.4142135623730951meg HARMS=(1,1)",
        "json",
        0,
    );
    let deck = dir.join(format!("{name}.sp"));
    std::fs::write(
        &deck,
        format!("* mixed deck\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n{analysis}\n.end\n"),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(&deck)
        .args(["-f", format, "-o"])
        .arg(dir.join(format!("{name}.result")))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(expected), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let files = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    assert!(
        files
            .iter()
            .any(|path| path.to_string_lossy().contains("op-001")),
        "{files:?}"
    );
    assert!(
        files
            .iter()
            .any(|path| path.to_string_lossy().contains("qpss-001")),
        "{files:?}"
    );
}
