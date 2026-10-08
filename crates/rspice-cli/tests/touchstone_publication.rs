mod common;

use common::{read_json, test_dir};
use std::path::Path;
use std::process::{Command, Output};

const CIRCUIT: &str = "* two-port export\n.param r=50\nV1 p1 0 DC 0 AC 1 PORTNUM 1 Z0 50\nV2 p2 0 DC 0 PORTNUM 2 Z0 50\nR1 p1 p2 {r}\n.SP LIN 3 1k 3k\n";

fn run(deck: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "run"])
        .arg(deck)
        .arg("-o")
        .arg(output)
        .args(["-f", "csv"])
        .output()
        .unwrap()
}

fn convert(input: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", "json"])
        .output()
        .unwrap()
}

#[test]
fn mismatched_port_count_extensions_cannot_replace_results() {
    let dir = test_dir("touchstone_wrong_ports");
    let deck = dir.join("network.cir");
    std::fs::write(&deck, format!("{CIRCUIT}.END\n")).unwrap();
    for extension in ["s0p", "s1p", "S3P", "s99999999999999999999999999999999999p"] {
        let path = dir.join(format!("network.{extension}"));
        std::fs::write(&path, "previous network").unwrap();
        let result = run(&deck, &path);
        assert_eq!(result.status.code(), Some(2), "{extension}: {result:?}");
        let error: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
        assert!(
            error["error"]["message"]
                .as_str()
                .unwrap()
                .contains("Touchstone")
        );
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "previous network");
    }
}

#[test]
fn equivalent_port_count_extensions_write_readable_networks() {
    let dir = test_dir("touchstone_port_spelling");
    let deck = dir.join("network.cir");
    std::fs::write(&deck, format!("{CIRCUIT}.END\n")).unwrap();
    for extension in ["s2p", "S2P", "s02p"] {
        let path = dir.join(format!("network.{extension}"));
        let result = run(&deck, &path);
        assert!(result.status.success(), "{extension}: {result:?}");
        let decoded = dir.join("decoded.json");
        let result = convert(&path, &decoded);
        assert!(result.status.success(), "{extension}: {result:?}");
        let table = read_json(&decoded);
        assert_eq!(
            table["scale"]["values"],
            serde_json::json!([1000.0, 2000.0, 3000.0])
        );
        assert_eq!(table["signals"].as_array().unwrap().len(), 6);
    }
}

#[test]
fn stepped_touchstone_manifests_describe_every_network() {
    let dir = test_dir("touchstone_step_schema");
    let deck = dir.join("network.cir");
    let output = dir.join("network.s2p");
    std::fs::write(&deck, format!("{CIRCUIT}.STEP PARAM r LIST 50 100\n.END\n")).unwrap();
    let result = run(&deck, &output);
    assert!(result.status.success(), "{result:?}");
    let manifest = read_json(&dir.join("network.step_schema.json"));
    let entries = manifest["analyses"].as_array().unwrap();
    assert_eq!(entries.len(), 1, "the network needs a schema entry");
    let entry = &entries[0];
    assert_eq!(entry["analysis_id"], "sp-001");
    let schema = entry["union_schema"].as_array().unwrap();
    let names = schema
        .iter()
        .map(|descriptor| descriptor["display_name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, ["S11", "S12", "S21", "S22"]);
    for descriptor in schema {
        assert_eq!(descriptor["unit"], "dimensionless");
        assert_eq!(descriptor["value_type"], "complex");
    }
    let coordinates = entry["coordinates"].as_array().unwrap();
    let set = common::AxisRunSet::read(&output);
    assert_eq!(coordinates.len(), 2);
    assert_eq!(set.coordinates.len(), 2);
    for (index, (coordinate, run)) in coordinates.iter().zip(&set.coordinates).enumerate() {
        let path = dir.join(coordinate["artifact"].as_str().unwrap());
        assert_eq!(path, run.only_artifact());
        assert_eq!(
            coordinate["validity"],
            serde_json::json!([true, true, true, true])
        );
        assert!(coordinate["source_signal_indices"].is_null());
        let decoded = dir.join("decoded.json");
        let result = convert(&path, &decoded);
        assert!(result.status.success(), "{result:?}");
        let table = read_json(&decoded);
        let s11 = table["signals"]
            .as_array()
            .unwrap()
            .iter()
            .find(|signal| signal["name"] == "S11")
            .unwrap();
        let expected = if index == 0 { 1.0 / 3.0 } else { 0.5 };
        for value in s11["real"].as_array().unwrap() {
            assert!((value.as_f64().unwrap() - expected).abs() < 1e-12);
        }
    }
}
