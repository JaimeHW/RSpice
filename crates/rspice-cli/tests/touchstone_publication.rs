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

#[test]
fn generic_network_extensions_are_self_describing_and_readable() {
    let dir = test_dir("touchstone_generic_names");
    let deck = dir.join("network.cir");
    std::fs::write(&deck, format!("{CIRCUIT}.END\n")).unwrap();
    for extension in ["snp", "SNP", "ts", "TS"] {
        let path = dir.join(format!("network.{extension}"));
        let result = run(&deck, &path);
        assert!(result.status.success(), "{extension}: {result:?}");
        let decoded = dir.join("decoded.json");
        let result = convert(&path, &decoded);
        assert!(result.status.success(), "{extension}: {result:?}");
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("[Version] 2.0"));
        assert!(text.contains("[Number of Ports] 2"));
        assert!(text.contains("[Two-Port Data Order] 21_12"));
        assert!(text.contains("[Number of Frequencies] 3"));
        assert!(text.trim_end().ends_with("[End]"));
        let table = read_json(&decoded);
        assert_eq!(table["signals"].as_array().unwrap().len(), 6);
        assert_eq!(
            table["scale"]["values"],
            serde_json::json!([1000.0, 2000.0, 3000.0])
        );
    }
}

#[test]
fn version_two_preserves_unequal_port_references_and_every_coefficient() {
    let dir = test_dir("touchstone_mixed_references");
    let deck = dir.join("network.cir");
    std::fs::write(
        &deck,
        format!(
            "{}.END\n",
            CIRCUIT.replace("PORTNUM 2 Z0 50", "PORTNUM 2 Z0 75")
        ),
    )
    .unwrap();
    let flat = dir.join("network.csv");
    assert!(run(&deck, &flat).status.success());
    let reference = dir.join("reference.json");
    assert!(convert(&flat, &reference).status.success());
    let expected = read_json(&reference);
    for extension in ["snp", "ts"] {
        let path = dir.join(format!("network.{extension}"));
        let result = run(&deck, &path);
        assert!(result.status.success(), "{extension}: {result:?}");
        let decoded = dir.join("decoded.json");
        let result = convert(&path, &decoded);
        assert!(result.status.success(), "{extension}: {result:?}");
        let table = read_json(&decoded);
        assert_eq!(table["scale"]["values"], expected["scale"]["values"]);
        for signal in expected["signals"].as_array().unwrap() {
            let name = signal["name"].as_str().unwrap().replace('_', "");
            let actual = table["signals"]
                .as_array()
                .unwrap()
                .iter()
                .find(|column| column["name"] == name)
                .unwrap();
            for field in ["values", "real", "imag"] {
                assert_eq!(actual[field], signal[field], "{extension}: {name}/{field}");
            }
        }
    }
}

#[test]
fn version_two_retains_asymmetric_matrices_across_all_encodings() {
    use rspice_core::analysis::s_param::{
        TouchstoneFormat, TouchstoneFrequencyUnit, TouchstoneInput, TouchstoneVersion,
        touchstone_with_version,
    };
    for ports in [1, 2, 3, 5, 10] {
        let frequencies = [1e6, 2e6, 3e6];
        let references = (0..ports)
            .map(|port| 50.0 + port as f64 * 25.0)
            .collect::<Vec<_>>();
        let parameters = (0..ports)
            .map(|row| {
                (0..ports)
                    .map(|column| {
                        (0..frequencies.len())
                            .map(|point| {
                                let value = (row * ports + column + 1) as f64 / 100.0
                                    + point as f64 / 10_000.0;
                                rspice_core::Complex64::new(value, -value / 3.0)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for format in [
            TouchstoneFormat::RealImaginary,
            TouchstoneFormat::MagnitudeAngle,
            TouchstoneFormat::DecibelAngle,
        ] {
            let text = touchstone_with_version(
                &TouchstoneInput {
                    frequencies: &frequencies,
                    parameters: &parameters,
                    reference_impedances: &references,
                    comments: &[],
                },
                format,
                TouchstoneFrequencyUnit::MHz,
                TouchstoneVersion::V2,
            )
            .unwrap();
            let decoded =
                rspice_formats::read_touchstone_bytes("network.snp", text.as_bytes()).unwrap();
            assert_eq!(decoded.x_signal.as_ref().unwrap().data, frequencies);
            assert_eq!(
                decoded.metadata["z0_ports"],
                references
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            );
            for (row, columns) in parameters.iter().enumerate() {
                for (column, values) in columns.iter().enumerate() {
                    let name = if ports <= 9 {
                        format!("S{}{}", row + 1, column + 1)
                    } else {
                        format!("S{}_{}", row + 1, column + 1)
                    };
                    for (suffix, expected) in [
                        (
                            "RE",
                            values.iter().map(|value| value.re).collect::<Vec<_>>(),
                        ),
                        (
                            "IM",
                            values.iter().map(|value| value.im).collect::<Vec<_>>(),
                        ),
                    ] {
                        let signal = decoded
                            .signals
                            .iter()
                            .find(|signal| signal.name == format!("{name}_{suffix}"))
                            .unwrap();
                        for (actual, expected) in signal.data.iter().zip(expected) {
                            assert!(
                                (actual - expected).abs() < 1e-12,
                                "{ports}/{format:?}/{name}/{suffix}"
                            );
                        }
                    }
                }
            }
        }
    }
}
