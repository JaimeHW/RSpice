//! PZ control results use the ordinary exports and atomic script publication.
mod common;
use rspice_core::execution::{AnalysisResultDocument, ResultPayload, SignalUnit};
use std::path::Path;
use std::process::{Command, Output};

fn run(deck: &Path, output: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(output)
        .args(["-f", format])
        .output()
        .unwrap()
}

const RLC: &str = "PZ\nV1 in 0 0\nR1 in mid 1\nL1 mid out 1\nC1 out 0 1\n";

#[test]
fn direct_and_control_pz_publish_identical_typed_roots_and_complex_prints() {
    let dir = common::test_dir("control-pz-routes");
    let mut payloads = Vec::new();
    for (route, cards) in [
        ("direct", ".pz in 0 out 0 vol pz"),
        (
            "explicit",
            ".control\npz in 0 out 0 vol pz\nlet saved = pz1.pole(1)\nprint pole(1) saved\n.endc",
        ),
        (
            "run",
            ".pz in 0 out 0 vol pz\n.control\nrun\nprint pz1.pole(1)\n.endc",
        ),
    ] {
        let deck = dir.join(format!("{route}.cir"));
        std::fs::write(&deck, format!("{RLC}{cards}\n.end\n")).unwrap();
        let result = run(&deck, &dir.join(format!("{route}.json")), "json");
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let path = dir.join(if route == "direct" {
            "direct.json".into()
        } else {
            format!("{route}.pz-001.json")
        });
        let document =
            AnalysisResultDocument::from_json(&std::fs::read_to_string(path).unwrap()).unwrap();
        let ResultPayload::PoleZero(payload) = document.payload() else {
            panic!("PZ")
        };
        assert_eq!(payload.root_unit, Some(SignalUnit::RadianPerSecond));
        assert_eq!(payload.gain_unit, Some(SignalUnit::Dimensionless));
        assert_eq!(payload.poles.len(), 2);
        assert!((payload.poles[0].real + 0.5).abs() < 1e-9);
        assert!((payload.poles[0].imaginary - 3.0_f64.sqrt() / 2.0).abs() < 1e-9);
        payloads.push(payload.clone());
        if route != "direct" {
            let print: serde_json::Value = serde_json::from_slice(
                &std::fs::read(dir.join(format!("{route}.control-001.json"))).unwrap(),
            )
            .unwrap();
            assert_eq!(print["traces"][0]["y"]["unit"], "rad/s");
            assert!(
                (print["traces"][0]["y"]["samples"][0][1].as_f64().unwrap() - 3.0_f64.sqrt() / 2.0)
                    .abs()
                    < 1e-9
            );
        }
    }
    assert_eq!(payloads[0], payloads[1]);
    assert_eq!(payloads[0], payloads[2]);
}

#[test]
fn invalid_root_after_a_successful_pz_preserves_existing_outputs() {
    let dir = common::test_dir("control-pz-rollback");
    let deck = dir.join("invalid.cir");
    std::fs::write(
        &deck,
        format!("{RLC}.control\npz in 0 out 0 vol pz\nprint pole(9)\n.endc\n.end\n"),
    )
    .unwrap();
    let existing = dir.join("result.pz-001.json");
    std::fs::write(&existing, "previous result\n").unwrap();
    let output = run(&deck, &dir.join("result.json"), "json");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unavailable"));
    assert_eq!(
        std::fs::read_to_string(existing).unwrap(),
        "previous result\n"
    );
    assert!(!dir.join("result.control-001.json").exists());
}
