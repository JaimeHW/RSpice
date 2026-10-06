mod common;
use std::process::Command;

#[test]
fn check_keeps_the_included_analysis_path_and_physical_line() {
    let directory = common::test_dir("analysis_error_origins");
    let root = directory.join("root.cir");
    let child = directory.join("analysis.inc");
    std::fs::write(
        &root,
        "Analysis origin\nV1 out 0 1\nR1 out 0 1k\n.include analysis.inc\n.param count=0\n.end\n",
    )
    .unwrap();
    std::fs::write(
        &child,
        "* included\n.subckt child p\n.PSS FUND=1G\n+ HARMS={count}\n.ends\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json", "check", "--json"])
        .arg(&root)
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["valid"], false, "{report}");
    let error = &report["errors"][0];
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("analysis.inc:3"),
        "{error}"
    );
    assert!(
        error["message"].as_str().unwrap().contains("HARMS"),
        "{error}"
    );
    assert_eq!(error["details"]["line"], 3, "{error}");
}
