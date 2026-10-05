//! Analysis results must survive the CLI's own readers and converters.
mod common;

use common::test_dir;
use std::path::Path;
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .unwrap()
}

fn simulate(
    dir: &Path,
    name: &str,
    bottom: &str,
    analysis: &str,
    format: &str,
) -> std::path::PathBuf {
    let deck = dir.join(format!("{name}.cir"));
    let file = dir.join(format!("{name}.{format}"));
    std::fs::write(
        &deck,
        format!("Readback\nV1 in 0 1\nR1 in out 1k\nR2 out 0 {bottom}\n{analysis}\n.end\n"),
    )
    .unwrap();
    let result = run(&[
        "run",
        deck.to_str().unwrap(),
        "-f",
        format,
        "-o",
        file.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");
    file
}

#[test]
fn transfer_function_scalars_survive_conversion_and_comparison() {
    let dir = test_dir("tf_readback");
    let a = simulate(&dir, "a", "1k", ".tf V(out) V1", "json");
    let b = simulate(&dir, "b", "3k", ".tf V(out) V1", "json");
    let result = run(&[
        "compare",
        a.to_str().unwrap(),
        b.to_str().unwrap(),
        "--abstol",
        "0",
        "--reltol",
        "0",
        "--json",
    ]);
    assert_eq!(result.status.code(), Some(3), "{result:?}");
    let json: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(json["num_variables"], 4);
    assert_eq!(json["num_differences"], 3);

    let converted = dir.join("converted.csv");
    let result = run(&[
        "convert",
        a.to_str().unwrap(),
        converted.to_str().unwrap(),
        "--to",
        "csv",
    ]);
    assert!(result.status.success(), "{result:?}");
    let direct = simulate(&dir, "direct", "1k", ".tf V(out) V1", "csv");
    let result = run(&[
        "compare",
        converted.to_str().unwrap(),
        direct.to_str().unwrap(),
        "--abstol",
        "0",
        "--reltol",
        "0",
    ]);
    assert!(result.status.success(), "{result:?}");
}

#[test]
fn operating_point_raw_preserves_the_first_signal_and_matches_csv() {
    let dir = test_dir("op_readback");
    let csv = simulate(&dir, "reference", "1k", ".op", "csv");
    for format in ["raw", "ascii"] {
        let raw = simulate(&dir, format, "1k", ".op", format);
        let result = run(&["compare", raw.to_str().unwrap(), csv.to_str().unwrap()]);
        assert!(result.status.success(), "{result:?}");
        let selected = dir.join(format!("selected-{format}.csv"));
        let result = run(&[
            "convert",
            raw.to_str().unwrap(),
            selected.to_str().unwrap(),
            "--to",
            "csv",
            "--variables",
            "V(IN)",
        ]);
        assert!(result.status.success(), "{result:?}");
        let content = std::fs::read_to_string(selected).unwrap();
        assert!(
            content.lines().next().unwrap().contains("V(IN)"),
            "{content}"
        );
    }
}
