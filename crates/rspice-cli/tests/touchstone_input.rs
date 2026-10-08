mod common;
use common::test_dir;
use std::path::Path;
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json"])
        .args(args)
        .output()
        .unwrap()
}

fn convert(path: &Path, output: &Path, flags: &[&str]) -> Output {
    let mut args = vec![
        "convert",
        path.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        "json",
    ];
    args.extend_from_slice(flags);
    cli(&args)
}

fn network(reference: f64, forward: f64) -> String {
    format!("# Hz S RI R {reference}\n1 0.1 0.2 {forward} 0.4 0.5 0.6 0.7 0.8\n")
}

#[test]
fn comparison_checks_all_network_coefficients_and_port_references() {
    let dir = test_dir("touchstone_compare");
    let left = dir.join("left.s2p");
    let right = dir.join("right.s2p");
    std::fs::write(&left, network(50.0, 0.3)).unwrap();
    for (reference, forward, status) in [(50.0, 0.3, 0), (50.0, 0.9, 3), (75.0, 0.3, 3)] {
        std::fs::write(&right, network(reference, forward)).unwrap();
        let result = cli(&[
            "compare",
            left.to_str().unwrap(),
            right.to_str().unwrap(),
            "--json",
        ]);
        assert_eq!(result.status.code(), Some(status), "{result:?}");
    }
}

#[test]
fn complex_formats_and_frequency_units_are_decoded_before_conversion() {
    let dir = test_dir("touchstone_complex");
    let input = dir.join("source.s1p");
    let output = dir.join("converted.json");
    for (format, pair) in [
        ("RI", "0 2"),
        ("MA", "2 90"),
        ("DB", "6.020599913279624 90"),
    ] {
        std::fs::write(&input, format!("# MHz S {format} R 50\n1 {pair}\n")).unwrap();
        let result = convert(&input, &output, &[]);
        assert!(result.status.success(), "{result:?}");
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
        assert_eq!(json["scale"]["values"][0], 1e6);
        assert_eq!(json["scale"]["type"], "frequency");
        let signal = &json["signals"][0];
        assert_eq!(signal["name"], "S11");
        assert_eq!(signal["type"], "dimensionless");
        assert!(signal["real"][0].as_f64().unwrap().abs() < 1e-14);
        assert!((signal["imag"][0].as_f64().unwrap() - 2.0).abs() < 1e-14);
    }
}

#[test]
fn noise_uses_its_own_selected_frequency_grid() {
    let dir = test_dir("touchstone_noise");
    let input = dir.join("source.s2p");
    let output = dir.join("noise.json");
    std::fs::write(&input, "# MHz S RI R 50\n1 0 0 0.5 0 0.5 0 0 0\n3 0 0 0.5 0 0.5 0 0 0\n1 3 0.2 0 0.1\n2 3 0.2 0 0.1\n").unwrap();
    assert!(!convert(&input, &output, &[]).status.success());
    let result = convert(&input, &output, &["--section", "noise"]);
    assert!(result.status.success(), "{result:?}");
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
    assert_eq!(json["scale"]["values"], serde_json::json!([1e6, 2e6]));
    let resistance = json["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|signal| signal["name"] == "Rn")
        .unwrap();
    assert_eq!(resistance["values"], serde_json::json!([5.0, 5.0]));
    let temperature = json["signals"]
        .as_array()
        .unwrap()
        .iter()
        .find(|signal| signal["name"] == "noise_reference_temperature")
        .unwrap();
    assert_eq!(temperature["values"], serde_json::json!([290.0, 290.0]));
}

#[test]
fn touchstone_admission_is_typed_and_preserves_the_destination() {
    let dir = test_dir("touchstone_budget");
    let input = dir.join("source.s2p");
    let output = dir.join("result.json");
    let config = dir.join("limited.toml");
    std::fs::write(&input, network(50.0, 0.3)).unwrap();
    std::fs::write(&config, "[resources]\nmax_external_data_values=8\n").unwrap();
    std::fs::write(&output, "predecessor").unwrap();
    let result = convert(&input, &output, &["--config", config.to_str().unwrap()]);
    assert_eq!(result.status.code(), Some(75), "{result:?}");
    assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
}

#[test]
fn a_touchstone_file_written_by_run_can_be_compared_and_converted() {
    let dir = test_dir("own_touchstone");
    let deck = dir.join("network.sp");
    let output = dir.join("network.s2p");
    let json = dir.join("network.json");
    std::fs::write(&deck, "* two-port\nV1 in 0 DC 0 AC 1 PORTNUM 1 Z0 50\nV2 out 0 DC 0 AC 0 PORTNUM 2 Z0 50\nR1 in out 50\n.SP LIN 2 1k 2k\n.END\n").unwrap();
    let result = cli(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");
    let result = cli(&[
        "compare",
        output.to_str().unwrap(),
        output.to_str().unwrap(),
    ]);
    assert!(result.status.success(), "{result:?}");
    let result = convert(&output, &json, &[]);
    assert!(result.status.success(), "{result:?}");
}

#[test]
fn dc_network_points_survive_touchstone_import_and_reexport() {
    let dir = test_dir("touchstone_dc");
    for (extension, text) in [
        ("s1p", "# Hz S RI R 50\n0 0.25 0\n1 0.5 0\n"),
        (
            "ts",
            "[Version] 2.0\n# Hz S RI R 50\n[Number of Ports] 1\n[Number of Frequencies] 2\n[Network Data]\n0 0.25 0\n1 0.5 0\n[End]\n",
        ),
    ] {
        let input = dir.join(format!("source.{extension}"));
        let output = dir.join("decoded.json");
        std::fs::write(&input, text).unwrap();
        let result = convert(&input, &output, &[]);
        assert!(result.status.success(), "{extension}: {result:?}");
        let table = common::read_json(&output);
        assert_eq!(table["scale"]["values"], serde_json::json!([0.0, 1.0]));
        assert_eq!(table["signals"][0]["real"], serde_json::json!([0.25, 0.5]));
        let dataset =
            rspice_formats::read_touchstone_bytes(&format!("source.{extension}"), text.as_bytes())
                .unwrap();
        let encoded =
            rspice_formats::WaveformWriter::new(rspice_formats::WaveformFormat::Touchstone)
                .write_text(&dataset)
                .unwrap();
        let decoded =
            rspice_formats::read_touchstone_bytes("roundtrip.s1p", encoded.as_bytes()).unwrap();
        assert_eq!(decoded.x_signal.unwrap().data, [0.0, 1.0]);
    }
}

#[test]
fn overflowing_port_count_extensions_are_not_inferred_as_another_network() {
    let dir = test_dir("touchstone_overflow_count");
    let input = dir.join("source.s99999999999999999999999999999999999p");
    let output = dir.join("decoded.json");
    std::fs::write(&input, "# Hz S RI R 50\n1 0.25 0\n2 0.5 0\n").unwrap();
    std::fs::write(&output, "previous result").unwrap();
    let result = convert(&input, &output, &[]);
    assert!(!result.status.success(), "{result:?}");
    assert_eq!(std::fs::read_to_string(output).unwrap(), "previous result");
}
