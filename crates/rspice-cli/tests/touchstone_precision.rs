//! Invalid RF inputs must fail before conversion and baseline publication.
mod common;
use std::process::{Command, Output};

fn assert_underflow(result: Output) {
    assert_eq!(result.status.code(), Some(1), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("underflow"),
        "{result:?}"
    );
}

#[test]
fn touchstone_underflow_cannot_replace_outputs_or_bless_baselines() {
    let dir = common::test_dir("touchstone_precision_publication");
    let network = "# Hz S RI R 50\n1 1 0 1 0 1 0 1 0\n";
    for (name, extension, source, baseline) in [
        (
            "coordinate",
            "s1p",
            "# Hz S RI R 50\n1e-999 1 0\n".to_owned(),
            "# Hz S RI R 50\n0 1 0\n".to_owned(),
        ),
        (
            "coefficient",
            "s1p",
            "# Hz S RI R 50\n1 1e-999 0\n".to_owned(),
            "# Hz S RI R 50\n1 0 0\n".to_owned(),
        ),
        (
            "decibels",
            "s1p",
            "# Hz S DB R 50\n1 -10000 0\n".to_owned(),
            "# Hz S RI R 50\n1 0 0\n".to_owned(),
        ),
        (
            "noise",
            "s2p",
            format!("{network}0.5 3 0.5 45 1e-999\n"),
            format!("{network}0.5 3 0.5 45 0\n"),
        ),
    ] {
        let input = dir.join(format!("{name}.{extension}"));
        let golden = dir.join(format!("{name}.golden.{extension}"));
        let missing = dir.join(format!("{name}.missing.{extension}"));
        let output = dir.join("protected.json");
        std::fs::write(&input, &source).unwrap();
        std::fs::write(&golden, &baseline).unwrap();
        std::fs::write(&output, "predecessor").unwrap();
        // An invalid unselected noise section must not be discarded.
        assert_underflow(
            Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "convert"])
                .arg(&input)
                .arg(&output)
                .args(["--to", "json", "--section", "network"])
                .output()
                .unwrap(),
        );
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
        for (destination, bless) in [(&golden, false), (&golden, true), (&missing, true)] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command
                .args(["--quiet", "compare"])
                .arg(&input)
                .arg(destination);
            if bless {
                command.arg("--bless");
            } else {
                command.args(["--section", "network"]);
            }
            assert_underflow(command.output().unwrap());
            assert_eq!(std::fs::read_to_string(&golden).unwrap(), baseline);
            assert!(!missing.exists());
        }
    }
}

#[test]
fn touchstone_conversion_preserves_tiny_physical_frequencies() {
    let dir = common::test_dir("touchstone_precision_units");
    let input = dir.join("source.s1p");
    let output = dir.join("converted.json");
    std::fs::write(&input, "# GHz S RI R 50\n1D-330 5e-324 -0e-999\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&input)
        .arg(&output)
        .args(["--to", "json"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let table = common::read_json(&output);
    assert_eq!(table["scale"]["values"][0].as_f64().unwrap(), 1e-321);
    assert_eq!(
        table["signals"][0]["real"][0].as_f64().unwrap().to_bits(),
        5e-324_f64.to_bits()
    );
    assert_eq!(
        table["signals"][0]["imag"][0].as_f64().unwrap().to_bits(),
        (-0.0_f64).to_bits()
    );
}
