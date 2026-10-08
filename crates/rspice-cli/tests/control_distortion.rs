//! Scripted distortion uses the same typed results and atomic exports as decks.
mod common;
use rspice_core::execution::AnalysisResultDocument;
use std::path::Path;
use std::process::{Command, Output};

const DECK: &str = "Distortion control\nV1 out 0 DC .5 DISTOF1 1m 30 DISTOF2 .5m -20\nD1 out 0 DM\n.model DM D(IS=1e-12 N=1 CJO=0 TT=0)\n";

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
fn direct_explicit_and_run_routes_preserve_physical_spectra_and_presentations() {
    let dir = common::test_dir("control-disto-routes");
    for (mode, ratio, product, frequencies) in [
        ("harmonic", "", "2f1", [2000.0, 3000.0, 4000.0]),
        ("mixing", " .9", "f1-f2", [100.0, 600.0, 1100.0]),
    ] {
        let analysis = format!("disto lin 3 1k 2k{ratio}");
        let presentation = format!(
            "print disto(\"{product}\",i(V1)) vs disto(\"{product}\",frequency)\nplot abs(disto(\"{product}\",i(V1)))"
        );
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
            let stem = format!("{mode}-{route}");
            let deck = dir.join(format!("{stem}.cir"));
            std::fs::write(&deck, format!("{DECK}{cards}\n.end\n")).unwrap();
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
                format!("{stem}.disto-001.json")
            });
            let document =
                AnalysisResultDocument::from_json(&std::fs::read_to_string(path).unwrap()).unwrap();
            assert_eq!(document.point_count(), 3);
            documents.push(document);
            if route != "direct" {
                let print: serde_json::Value = serde_json::from_slice(
                    &std::fs::read(dir.join(format!("{stem}.control-001.json"))).unwrap(),
                )
                .unwrap();
                assert_eq!(print["traces"][0]["y"]["unit"], "A");
                assert_eq!(print["traces"][0]["x"]["unit"], "Hz");
                assert_eq!(
                    print["traces"][0]["x"]["samples"],
                    serde_json::json!(frequencies.map(|frequency| [frequency, 0.0]))
                );
                assert!(
                    print["traces"][0]["y"]["samples"][0][1]
                        .as_f64()
                        .unwrap()
                        .abs()
                        > 1e-9
                );
                assert!(dir.join(format!("{stem}.control-002.svg")).exists());
            }
        }
        for document in &documents[1..] {
            assert_eq!(document.payload(), documents[0].payload());
            assert_eq!(document.signals(), documents[0].signals());
            assert_eq!(document.axes(), documents[0].axes());
        }
    }
}

#[test]
fn unavailable_product_and_invalid_later_analysis_preserve_existing_outputs() {
    let dir = common::test_dir("control-disto-rollback");
    for (name, failure) in [
        ("product", "print disto(\"f2\",v(0))"),
        ("analysis", "disto lin 3 1k 2k 1.1"),
    ] {
        let deck = dir.join(format!("{name}.cir"));
        std::fs::write(&deck, format!(
            "{DECK}.control\ndisto lin 3 1k 2k\nplot abs(disto(\"2f1\",i(V1)))\n{failure}\n.endc\n.end\n"
        )).unwrap();
        let existing = dir.join(format!("{name}.disto-001.json"));
        std::fs::write(&existing, "previous result\n").unwrap();
        let output = run(&deck, &dir.join(format!("{name}.json")));
        assert!(!output.status.success());
        assert_eq!(
            std::fs::read_to_string(existing).unwrap(),
            "previous result\n"
        );
        assert!(!dir.join(format!("{name}.control-001.json")).exists());
        assert!(!dir.join(format!("{name}.control-001.svg")).exists());
    }
}

#[test]
fn check_rejects_malformed_explicit_distortion_arguments() {
    let dir = common::test_dir("control-disto-check");
    for (index, arguments) in ["lin 0 1k 2k", "lin 3 1k 2k junk"].iter().enumerate() {
        let deck = dir.join(format!("invalid-{index}.cir"));
        std::fs::write(
            &deck,
            format!("{DECK}.control\ndisto {arguments}\n.endc\n.end\n"),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "check"])
            .arg(&deck)
            .arg("--json")
            .output()
            .unwrap();
        assert!(!output.status.success(), "{arguments}: {output:?}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["valid"], false);
        assert_eq!(report["errors"][0]["details"]["line"], 6);
    }
}
