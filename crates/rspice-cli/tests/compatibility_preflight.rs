mod common;
use std::process::Command;
#[test]
fn unsupported_later_analysis_does_not_publish_an_earlier_op() {
    let dir = common::test_dir("preflight");
    for (name, analysis, format, expected) in [
        ("tf", ".tf V(out) V1", "hdf5", 2),
        (
            "qpss",
            ".qpss 1meg 1.4142135623730951meg HARMS=(1,1)",
            "json",
            69,
        ),
    ] {
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
        let _: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert!(!dir.join(format!("{name}.op.result")).exists());
    }
}
