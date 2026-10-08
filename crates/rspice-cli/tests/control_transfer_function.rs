//! Direct, declarative and explicit TF runs share typed results and exports.
mod common;
use rspice_core::execution::{AnalysisResultDocument, SignalUnit};
use std::path::Path;
use std::process::{Command, Output};

fn success(output: Output) -> Output {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn run(input: &Path, output: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(input)
        .arg("-o")
        .arg(output)
        .args(["-f", format])
        .output()
        .unwrap()
}

#[test]
fn all_transfer_quantities_match_direct_and_both_control_routes() {
    let directory = common::test_dir("tf-control-quantities");
    for (case, body, command, gain, unit) in [
        (
            "vv",
            "V1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n",
            "tf V(out) V1",
            2.0 / 3.0,
            SignalUnit::Dimensionless,
        ),
        (
            "iv",
            "I1 0 out 0\nR1 out 0 3k\n",
            "tf V(out) I1",
            3000.0,
            SignalUnit::Ohm,
        ),
        (
            "vi",
            "V1 in 0 1\nR1 in mid 1k\nVm mid out 0\nR2 out 0 2k\n",
            "tf I(Vm) V1",
            1.0 / 3000.0,
            SignalUnit::Siemens,
        ),
        (
            "ii",
            "I1 0 mid 0\nVm mid out 0\nR1 out 0 3k\n",
            "tf I(Vm) I1",
            1.0,
            SignalUnit::Dimensionless,
        ),
    ] {
        let mut documents = Vec::new();
        for (route, cards) in [
            ("direct", format!(".{command}")),
            ("explicit", format!(".control\n{command}\n.endc")),
            ("run", format!(".{command}\n.control\nrun\n.endc")),
        ] {
            let stem = format!("{case}-{route}");
            let input = directory.join(format!("{stem}.cir"));
            let output = directory.join(format!("{stem}.json"));
            std::fs::write(&input, format!("TF\n{body}{cards}\n.end\n")).unwrap();
            success(
                Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "check"])
                    .arg(&input)
                    .output()
                    .unwrap(),
            );
            success(run(&input, &output, "json"));
            let published = if route == "direct" {
                output
            } else {
                directory.join(format!("{stem}.tf-001.json"))
            };
            let document =
                AnalysisResultDocument::from_json(&std::fs::read_to_string(published).unwrap())
                    .unwrap();
            assert_eq!(document.scalars()[0].unit(), Some(&unit));
            let wire = serde_json::to_value(&document).unwrap();
            let actual = wire["scalars"][0]["value"]["value"].as_f64().unwrap();
            assert!((actual - gain).abs() < gain.abs() * 1e-9 + 1e-14);
            documents.push(document);
        }
        for document in &documents[1..] {
            assert_eq!(document.scalars(), documents[0].scalars());
            assert_eq!(document.payload(), documents[0].payload());
        }
    }
}

#[test]
fn transfer_export_and_conversion_keep_gain_and_impedance_units() {
    let directory = common::test_dir("tf-export-units");
    for control in [false, true] {
        let input = directory.join(format!("{control}.cir"));
        let cards = if control {
            ".control\ntf V(out) I1\n.endc"
        } else {
            ".tf V(out) I1"
        };
        std::fs::write(
            &input,
            format!("TF\nI1 0 out 0\nR1 out 0 3k\n{cards}\n.end\n"),
        )
        .unwrap();
        for format in ["json", "raw", "ascii", "hdf5"] {
            let stem = format!("{control}-{format}");
            let output = directory.join(format!("{stem}.data"));
            success(run(&input, &output, format));
            let published = if control {
                directory.join(format!("{stem}.tf-001.data"))
            } else {
                output
            };
            let converted = directory.join(format!("{stem}-converted.json"));
            success(
                Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "convert"])
                    .arg(published)
                    .arg(&converted)
                    .args(["--from", format, "--to", "json"])
                    .output()
                    .unwrap(),
            );
            let wire: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(converted).unwrap()).unwrap();
            let signals = wire["signals"].as_array().unwrap();
            assert_eq!(signals.len(), 3, "{stem}: {wire}");
            for signal in signals {
                assert_eq!(signal["unit"], "ohm", "{stem}: {wire}");
            }
        }
    }
}

#[test]
fn unbounded_control_print_keeps_typed_evidence_and_authored_order() {
    let directory = common::test_dir("tf-print-infinity");
    let input = directory.join("open.cir");
    std::fs::write(&input, "TF\nV1 in 0 1\nR1 out 0 1k\n.control\ntf I(V1) V1\nprint transfer_function input_impedance transfer_gain V1#output_impedance\n.endc\n.end\n").unwrap();
    let output = success(run(&input, &directory.join("open.json"), "json"));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let gain = stdout.find("# transfer_function").unwrap();
    let input = stdout.find("# input_impedance").unwrap();
    let again = stdout.find("# transfer_gain").unwrap();
    let output = stdout.find("# V1#output_impedance").unwrap();
    assert!(gain < input && input < again && again < output, "{stdout}");
    assert_eq!(stdout.lines().filter(|line| *line == "inf").count(), 2);
    let wire: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(directory.join("open.control-001.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(wire["version"], 3);
    assert_eq!(wire["traces"].as_array().unwrap().len(), 2);
    for (scalar, position) in wire["scalars"].as_array().unwrap().iter().zip([1, 3]) {
        assert_eq!(scalar["position"], position);
        assert_eq!(scalar["dataset"], "tf1");
        assert_eq!(scalar["scalar"]["value"]["representation"], "unavailable");
        assert_eq!(scalar["scalar"]["value"]["reason"], "positive_infinity");
    }
}

#[test]
fn transfer_scalar_assignments_drive_subsequent_analyses() {
    let directory = common::test_dir("tf-scalar-workflow");
    let input = directory.join("scalar.cir");
    std::fs::write(&input, "TF\nI1 0 out 0\nR1 out 0 3k\n.control\ntf V(out) I1\nlet resistance = tf1.transfer_function*2\nif resistance > 5000\nalter R1 $resistance\ntf V(out) I1\nend\n.endc\n.end\n").unwrap();
    success(run(&input, &directory.join("scalar.json"), "json"));
    let wire: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(directory.join("scalar.tf-002.json")).unwrap(),
    )
    .unwrap();
    let gain = wire["scalars"][0]["value"]["value"].as_f64().unwrap();
    assert!((gain - 6000.0).abs() < 1e-7, "{wire}");
}

#[test]
fn failed_transfer_after_an_ordinary_analysis_rolls_back_output() {
    let directory = common::test_dir("tf-control-rollback");
    let input = directory.join("failed.cir");
    std::fs::write(
        &input,
        "TF\nV1 in 0 1\nR1 in 0 1k\n.control\nop\ntf V(in) missing\n.endc\n.end\n",
    )
    .unwrap();
    let output = run(&input, &directory.join("failed.json"), "json");
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("MISSING"), "{error}");
    assert!(!directory.join("failed.op-001.json").exists());
    assert!(!directory.join("failed.tf-001.json").exists());
}
