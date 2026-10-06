//! Independent-tone execution, retained carrier binding, and native exports.
mod common;
use common::{read_json, test_dir};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const CIRCUIT: &str = "QP CLI integration\n.param resistance=1k\nV1 in 0 SIN(1 .1 1k)\nR1 in out {resistance}\nR2 out 0 1k\nC1 out 0 100n\n";
const QPSS: &str = ".QPSS 1k 1.4142135623730951k HARMS=1\n";
const QPAC: &str =
    ".QPAC LIN 2 10 100 SOURCE=V1 OUT=V(out) INLATTICE=(0,0) OUTLATTICE=(0,0) MAG=.3 PHASE=40\n";
const QPXF: &str = ".QPXF LIN 2 10 100 SOURCES=(V1) OUT=V(out) INLATTICES=((0,0)) OUTLATTICE=(0,0) GROUPDELAY=YES\n";
const QPNOISE: &str =
    ".QPNOISE LIN 2 10 100 OUT=V(out) OUTLATTICE=(0,0) SOURCE=V1 INLATTICE=(0,0)\n";

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .unwrap()
}
fn run(dir: &Path, cards: &str, format: &str, extra: &[&str]) -> Output {
    let input = dir.join("deck.cir");
    let output = dir.join(format!("result.{format}"));
    std::fs::write(&input, format!("{CIRCUIT}{cards}.end\n")).unwrap();
    let mut args = vec![
        "run",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        "-f",
        format,
    ];
    args.extend(extra);
    cli(&args)
}
fn artifact(dir: &Path, tag: &str, format: &str) -> PathBuf {
    dir.join(format!("result.{tag}.{format}"))
}
fn signal<'a>(document: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    document["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|signal| signal["descriptor"]["canonicalName"] == name)
        .unwrap()
}
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs().max(1e-30) * 2e-7 + 1e-28,
        "{actual:.17e} != {expected:.17e}"
    );
}

#[test]
fn quasiperiodic_small_signal_and_noise_match_the_analytic_rc_circuit() {
    let dir = test_dir("analytic");
    let output = run(&dir, &format!("{QPSS}{QPAC}{QPXF}{QPNOISE}"), "json", &[]);
    assert!(output.status.success(), "{output:?}");
    let qpss = read_json(&artifact(&dir, "qpss-001", "json"));
    let qpac = read_json(&artifact(&dir, "qpac-001", "json"));
    let qpxf = read_json(&artifact(&dir, "qpxf-001", "json"));
    let noise = read_json(&artifact(&dir, "qpnoise-001", "json"));
    for document in [&qpac, &qpxf, &noise] {
        assert_eq!(document["parentAnalysis"]["tag"], "qpss-001");
        assert_eq!(
            document["payload"]["result"]["metadata"]["operating_point_identity"],
            qpss["payload"]["operatingPoint"]["retained_identity"]
        );
    }
    assert_eq!(qpss["pointCount"], 9);
    assert!(signal(&qpss, "tone_index(1)").is_object());
    assert!(signal(&qpss, "tone_index(2)").is_object());
    for (index, frequency) in [10.0, 100.0].into_iter().enumerate() {
        let x = std::f64::consts::TAU * frequency * 500.0 * 100e-9;
        let re = 0.5 / (1.0 + x * x);
        let im = -0.5 * x / (1.0 + x * x);
        for (document, name) in [(&qpac, "output_transfer"), (&qpxf, "transfer(v1;0,0)")] {
            let value = &signal(document, name)["values"]["samples"][index];
            close(value["real"].as_f64().unwrap(), re);
            close(value["imaginary"].as_f64().unwrap(), im);
        }
        let response = &signal(&qpac, "v(out)")["values"]["samples"][index];
        let phase = 40_f64.to_radians();
        close(
            response["real"].as_f64().unwrap(),
            0.3 * (re * phase.cos() - im * phase.sin()),
        );
        close(
            response["imaginary"].as_f64().unwrap(),
            0.3 * (re * phase.sin() + im * phase.cos()),
        );
        close(
            signal(&noise, "output(1).noise_psd")["values"]["samples"][index]
                .as_f64()
                .unwrap(),
            4.0 * 1.380649e-23 * 300.15 * 500.0 / (1.0 + x * x),
        );
    }
}

#[test]
fn native_flat_formats_preserve_the_same_quasiperiodic_primary_values() {
    let dir = test_dir("formats");
    let cards = format!("{QPSS}{QPAC}{QPXF}{QPNOISE}");
    assert!(run(&dir, &cards, "json", &[]).status.success());
    for format in ["csv", "tsv", "raw", "ascii", "hdf5"] {
        let output = run(&dir, &cards, format, &[]);
        assert!(output.status.success(), "{format}: {output:?}");
        for tag in ["qpss-001", "qpac-001", "qpxf-001", "qpnoise-001"] {
            let source = artifact(&dir, tag, format);
            let converted = dir.join(format!("flat-{tag}-{format}.json"));
            let output = cli(&[
                "convert",
                source.to_str().unwrap(),
                converted.to_str().unwrap(),
                "--from",
                format,
                "--to",
                "json",
            ]);
            assert!(output.status.success(), "{tag}/{format}: {output:?}");
            let expected = dir.join(format!("expected-{tag}.json"));
            let reference = artifact(&dir, tag, "json");
            assert!(
                cli(&[
                    "convert",
                    reference.to_str().unwrap(),
                    expected.to_str().unwrap(),
                    "--to",
                    "json"
                ])
                .status
                .success()
            );
            let output = cli(&[
                "compare",
                converted.to_str().unwrap(),
                expected.to_str().unwrap(),
                "--abstol",
                "0",
                "--reltol",
                "0",
            ]);
            assert!(output.status.success(), "{tag}/{format}: {output:?}");
        }
    }
}

#[test]
fn two_carriers_and_step_coordinates_keep_separate_qpss_state_identities() {
    let dir = test_dir("carriers");
    let cards = format!(
        ".step param resistance list 1k 2k\n{QPSS}{QPAC}.QPSS 1k 1.4142135623730951k HARMS=2\n{QPAC}"
    );
    let output = run(&dir, &cards, "json", &[]);
    assert!(output.status.success(), "{output:?}");
    let documents = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .map(|path| read_json(&path))
        .filter(|document| document["schema"] == "rspice-analysis-result")
        .collect::<Vec<_>>();
    assert_eq!(documents.len(), 8);
    for result in documents
        .iter()
        .filter(|document| document["resultKind"] == "qpac")
    {
        let expected_tag = if result["analysis"]["tag"] == "qpac-001" {
            "qpss-001"
        } else {
            "qpss-002"
        };
        let carrier = documents
            .iter()
            .find(|document| {
                document["analysis"]["tag"] == expected_tag
                    && document["coordinate"] == result["coordinate"]
            })
            .unwrap();
        assert_eq!(result["parentAnalysis"]["tag"], expected_tag);
        assert_eq!(
            result["payload"]["result"]["metadata"]["operating_point_identity"],
            carrier["payload"]["operatingPoint"]["retained_identity"]
        );
    }
}

#[test]
fn authored_save_selects_qpss_and_qpac_voltages_without_losing_tone_coordinates() {
    let dir = test_dir("save");
    let output = run(&dir, &format!(".save V(out)\n{QPSS}{QPAC}"), "csv", &[]);
    assert!(output.status.success(), "{output:?}");
    let qpss = std::fs::read_to_string(artifact(&dir, "qpss-001", "csv")).unwrap();
    let header = qpss.lines().next().unwrap();
    assert!(
        header.contains("tone_index(1)")
            && header.contains("tone_index(2)")
            && header.contains("frequency"),
        "{header}"
    );
    assert!(
        header.to_ascii_lowercase().contains("v(out)")
            && !header.to_ascii_lowercase().contains("v(in)"),
        "{header}"
    );
    let qpac = std::fs::read_to_string(artifact(&dir, "qpac-001", "csv")).unwrap();
    let header = qpac.lines().next().unwrap();
    assert!(
        header.to_ascii_lowercase().contains("v(out)")
            && !header.to_ascii_lowercase().contains("v(in)"),
        "{header}"
    );
}

#[test]
fn absent_carriers_and_resource_limits_fail_before_publishing_results() {
    let dir = test_dir("preflight");
    for card in [QPAC, QPXF, QPNOISE] {
        let output = run(&dir, &format!(".op\n{card}"), "json", &[]);
        assert!(!output.status.success(), "{output:?}");
        assert!(!artifact(&dir, "op-001", "json").exists());
    }
    let config = dir.join("limited.toml");
    std::fs::write(&config, "[resources]\nmax_result_values=4\n").unwrap();
    let output = run(&dir, QPSS, "json", &["--config", config.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    assert!(!dir.join("result.json").exists());
}
