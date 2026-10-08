mod common;

use common::{read_json, test_dir};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

fn run(dir: &Path, circuit: &str, cards: &str) -> Value {
    let deck = dir.join("network.cir");
    let output = dir.join("result.json");
    std::fs::write(&deck, format!("{circuit}\n{cards}\n.END\n")).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "--error-format",
            "json",
            "run",
            deck.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "-f",
            "json",
        ])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    read_json(&dir.join("result.step_schema.json"))
}

fn name(signal: &Value) -> String {
    let display = signal["descriptor"]["displayName"].as_str().unwrap();
    match signal["qualifier"]["kind"].as_str() {
        Some("pac-sideband") => format!("{display}:sb{}", signal["qualifier"]["sideband"]),
        None => display.to_string(),
        other => panic!("unexpected qualifier {other:?}"),
    }
}

fn check_manifest(dir: &Path, manifest: &Value) {
    assert_eq!(manifest["schema_version"], 3);
    for entry in manifest["analyses"].as_array().unwrap() {
        let schema = entry["union_schema"].as_array().unwrap();
        let coordinates = entry["coordinates"].as_array().unwrap();
        let documents = coordinates
            .iter()
            .map(|coordinate| read_json(&dir.join(coordinate["artifact"].as_str().unwrap())))
            .collect::<Vec<_>>();
        let expected = documents
            .iter()
            .flat_map(|document| {
                document["signals"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|signal| name(signal).to_ascii_lowercase())
            })
            .collect::<BTreeSet<_>>();
        let actual = schema
            .iter()
            .map(|descriptor| {
                descriptor["display_name"]
                    .as_str()
                    .unwrap()
                    .to_ascii_lowercase()
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            actual, expected,
            "{}: manifest does not describe its artifacts",
            entry["analysis_id"]
        );
        for (coordinate, document) in coordinates.iter().zip(&documents) {
            let indices = coordinate["source_signal_indices"]
                .as_array()
                .expect("typed manifest maps its union to document signals");
            let validity = coordinate["validity"].as_array().unwrap();
            assert_eq!(indices.len(), schema.len());
            assert_eq!(validity.len(), schema.len());
            let sources = document["signals"].as_array().unwrap();
            let mut used = BTreeSet::new();
            for ((descriptor, index), valid) in schema.iter().zip(indices).zip(validity) {
                let Some(index) = index.as_u64() else {
                    assert_eq!(*valid, false);
                    continue;
                };
                assert!(
                    used.insert(index),
                    "different responses share an artifact index"
                );
                let signal = &sources[index as usize];
                assert!(
                    descriptor["display_name"]
                        .as_str()
                        .unwrap()
                        .eq_ignore_ascii_case(&name(signal))
                );
                let unit = &signal["descriptor"]["unit"];
                let unit_name = if unit["unit"] == "custom" {
                    format!("custom:{}", unit["symbol"].as_str().unwrap())
                } else {
                    unit["unit"].as_str().unwrap().replace('-', "_")
                };
                assert_eq!(descriptor["unit"], unit_name);
                assert_eq!(
                    *valid,
                    signal["values"]["samples"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|sample| !sample.is_null())
                );
                assert_eq!(
                    descriptor["shape"], "scalar",
                    "manifest describes one sample per signal"
                );
            }
            assert_eq!(used.len(), sources.len(), "manifest omits artifact signals");
        }
    }
}

const CIRCUIT: &str = "* typed stepped inventory\n.param r=1k\nV1 in 0 DC 1 AC 1 SIN(1 .1 1k)\nR1 in out {r}\nR2 out 0 1k\nC1 out 0 100n\n.step param r list 1k 2k\n.save V(out)\n";

#[test]
fn typed_op_and_ac_manifests_use_the_published_inventory() {
    for cards in [".OP", ".AC LIN 3 1k 2k"] {
        let dir = test_dir("typed_schema_basic");
        let manifest = run(&dir, CIRCUIT, cards);
        check_manifest(&dir, &manifest);
    }
}

#[test]
fn typed_transient_manifest_preserves_projection_availability() {
    let dir = test_dir("typed_schema_transient");
    let manifest = run(&dir, CIRCUIT, ".TRAN 10u 100u");
    check_manifest(&dir, &manifest);
    let coordinates = manifest["analyses"][0]["coordinates"].as_array().unwrap();
    for coordinate in coordinates {
        let validity = coordinate["validity"].as_array().unwrap();
        assert!(validity.iter().any(|value| value == false));
        assert!(validity.iter().any(|value| value == true));
    }
}

#[test]
fn qualified_periodic_series_have_distinct_manifest_indices() {
    let dir = test_dir("typed_schema_pac");
    let manifest = run(
        &dir,
        CIRCUIT,
        ".HB 1k\n.PAC LIN 3 100 300 INPUT=V1 OUT=V(out) MAXSIDEBAND=1",
    );
    check_manifest(&dir, &manifest);
}

#[test]
fn quasiperiodic_manifests_use_the_published_inventory() {
    let dir = test_dir("typed_schema_qpss");
    let manifest = run(&dir, CIRCUIT, ".QPSS 1k 1.4142135623730951k HARMS=1");
    check_manifest(&dir, &manifest);
}

#[test]
fn point_count_changes_do_not_conflict_with_signal_identity() {
    let dir = test_dir("typed_schema_point_count");
    let circuit = CIRCUIT.replace(
        ".step param r list 1k 2k",
        ".param n=1\n.step param n list 1 3",
    );
    let manifest = run(&dir, &circuit, ".AC LIN {n} 1k 2k");
    check_manifest(&dir, &manifest);
    let coordinates = manifest["analyses"][0]["coordinates"].as_array().unwrap();
    let counts = coordinates
        .iter()
        .map(|coordinate| {
            read_json(&dir.join(coordinate["artifact"].as_str().unwrap()))["pointCount"]
                .as_u64()
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(counts, [1, 3]);
}

#[test]
fn implicit_coordinate_documents_use_their_actual_inventory() {
    let dir = test_dir("typed_schema_implicit");
    let circuit = "* conditional implicit OP\n.param mode=0\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.if (mode==1)\nR3 out extra 1k\nR4 extra 0 1k\n.endif\n.step param mode list 0 1\n.save V(out)\n";
    let manifest = run(&dir, circuit, "");
    check_manifest(&dir, &manifest);
}

#[test]
fn stepped_sp_noise_manifest_covers_both_result_kinds() {
    let dir = test_dir("typed_schema_sp_noise");
    let circuit = "* stepped port noise\n.param r=50\nV1 p1 0 DC 0 AC 1 portnum=1 z0=50\nV2 p2 0 DC 0 portnum=2 z0=50\nR1 p1 p2 {r}\n.step param r list 50 100\n";
    let manifest = run(&dir, circuit, ".SP LIN 3 10 30 DONOISE");
    let analyses = manifest["analyses"].as_array().unwrap();
    assert_eq!(analyses.len(), 2, "port noise needs its own schema entry");
    check_manifest(&dir, &manifest);
    let kinds = analyses
        .iter()
        .map(|entry| entry["result_kind"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(kinds, BTreeSet::from(["sp", "port-noise"]));
    let mut covered = BTreeSet::new();
    for entry in analyses {
        assert_eq!(entry["analysis_id"], "sp-001");
        let coordinates = entry["coordinates"].as_array().unwrap();
        assert_eq!(coordinates.len(), 2);
        for coordinate in coordinates {
            let artifact = coordinate["artifact"].as_str().unwrap();
            assert!(covered.insert(artifact));
            let document = read_json(&dir.join(artifact));
            assert_eq!(entry["result_kind"], document["resultKind"]);
        }
    }
    let published = std::fs::read_dir(&dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .filter(|path| read_json(path).get("resultKind").is_some())
        .count();
    assert_eq!(covered.len(), published);
}
