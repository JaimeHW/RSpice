//! Scripted STB publishes the solver's typed margins and natural-pole evidence.
mod common;
use rspice_core::execution::AnalysisResultDocument;
use rspice_core::execution::result_document::{ResultScalar, ScalarUnavailability, ScalarValue};
use std::path::Path;
use std::process::{Command, Output};

const DECK: &str = "Stability control\nE1 out 0 sense 0 -100\nVprobe out drive 0\nR1 drive sense 1k\nC1 sense 0 159.154943091895n\n";

fn run(deck: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(output)
        .args(["-f", "json"])
        .output()
        .unwrap()
}

#[test]
fn direct_explicit_and_run_routes_preserve_stability_evidence_and_presentations() {
    let dir = common::test_dir("control-stb-routes");
    for nyquist in ["yes", "no"] {
        for zero in [false, true] {
            let source = if zero {
                DECK.replace("-100", "0")
            } else {
                DECK.into()
            };
            let analysis = format!("stb lin 3 10 1meg probe=Vprobe nyquist={nyquist}");
            let presentation = "print loopgain gain_margin_db dc_loop_gain_db\nplot abs(loopgain)";
            let mut documents = Vec::new();
            for (route, cards) in [
                ("direct", format!(".{analysis}")),
                (
                    "explicit",
                    format!(".control\n{analysis}\n{presentation}\n.endc"),
                ),
                (
                    "run",
                    format!(".{analysis}\n.control\nrun\n{presentation}\n.endc"),
                ),
            ] {
                let stem = format!("{nyquist}-{zero}-{route}");
                let deck = dir.join(format!("{stem}.cir"));
                std::fs::write(&deck, format!("{source}{cards}\n.end\n")).unwrap();
                let checked = Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "check"])
                    .arg(&deck)
                    .arg("--json")
                    .output()
                    .unwrap();
                assert!(checked.status.success(), "{stem}: {checked:?}");
                let report: serde_json::Value = serde_json::from_slice(&checked.stdout).unwrap();
                assert_eq!(report["valid"], true, "{stem}: {report}");
                let outcome = run(&deck, &dir.join(format!("{stem}.json")));
                assert!(outcome.status.success(), "{stem}: {outcome:?}");
                let path = dir.join(if route == "direct" {
                    format!("{stem}.json")
                } else {
                    format!("{stem}.stb-001.json")
                });
                let document =
                    AnalysisResultDocument::from_json(&std::fs::read_to_string(path).unwrap())
                        .unwrap();
                assert_eq!(document.point_count(), 3);
                documents.push(document);
                if route != "direct" {
                    let print: serde_json::Value = serde_json::from_slice(
                        &std::fs::read(dir.join(format!("{stem}.control-001.json"))).unwrap(),
                    )
                    .unwrap();
                    assert_eq!(print["traces"][0]["y"]["unit"], "1");
                    assert_eq!(print["traces"][0]["x"]["unit"], "Hz");
                    assert_eq!(print["scalars"][0]["position"], 1);
                    assert_eq!(print["scalars"][1]["position"], 2);
                    let gain: ResultScalar =
                        serde_json::from_value(print["scalars"][0]["scalar"].clone()).unwrap();
                    assert_eq!(
                        gain.value(),
                        &ScalarValue::Unavailable {
                            reason: ScalarUnavailability::NoCrossover
                        }
                    );
                    let dc: ResultScalar =
                        serde_json::from_value(print["scalars"][1]["scalar"].clone()).unwrap();
                    if zero {
                        assert_eq!(
                            dc.value(),
                            &ScalarValue::Unavailable {
                                reason: ScalarUnavailability::NegativeInfinity
                            }
                        );
                    } else {
                        let ScalarValue::Real { value: Some(value) } = dc.value() else {
                            panic!("DC gain");
                        };
                        assert!((value - 40.0).abs() < 1e-8);
                        assert!(
                            print["traces"][0]["y"]["samples"][0][1]
                                .as_f64()
                                .unwrap()
                                .abs()
                                > 0.1
                        );
                    }
                    assert!(dir.join(format!("{stem}.control-002.svg")).exists());
                }
            }
            for document in &documents[1..] {
                assert_eq!(document.payload(), documents[0].payload());
                assert_eq!(document.signals(), documents[0].signals());
                assert_eq!(document.axes(), documents[0].axes());
                assert_eq!(document.scalars(), documents[0].scalars());
            }
        }
    }
}

#[test]
fn invalid_later_stability_probe_preserves_existing_outputs() {
    let dir = common::test_dir("control-stb-rollback");
    let deck = dir.join("invalid.cir");
    std::fs::write(&deck,format!("{DECK}.control\nstb lin 3 10 1meg probe=Vprobe\nplot abs(loopgain)\nstb lin 3 10 1meg probe=missing\n.endc\n.end\n")).unwrap();
    let existing = dir.join("invalid.stb-001.json");
    std::fs::write(&existing, "previous result\n").unwrap();
    let output = run(&deck, &dir.join("invalid.json"));
    assert!(!output.status.success());
    assert_eq!(
        std::fs::read_to_string(existing).unwrap(),
        "previous result\n"
    );
    assert!(!dir.join("invalid.control-001.svg").exists());
    assert!(!dir.join("invalid.stb-002.json").exists());
}
