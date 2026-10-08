//! Delimited conversions retain declared quantities, units and physical samples.
mod common;

use rspice_formats::delimited::{DelimitedReadLimits, decode_delimited_waveforms};
use serde_json::json;
use std::path::Path;
use std::process::Command;

fn convert(source: &Path, destination: &Path, format: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(destination)
        .args(["--to", format])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
}

fn source() -> serde_json::Value {
    json!({
        "analysis":"authored", "plot_name":"Exact physical samples",
        "scale":{"name":"time", "type":"time", "unit":"ns", "values":[0.0,2.0,4.0]},
        "signals":[
            {"name":"charge [C]", "type":"charge", "unit":"C", "values":[1.0,null,-0.0]},
            {"name":"gain", "type":"value", "unit":"V/sqrt(Hz)", "real":[2.0,null,4.0], "imag":[-3.0,null,-0.0]},
            {"name":"Re(real signal)", "type":"value", "unit":"1", "values":[1.0,2.0,3.0]},
            {"name":"Im(real signal)", "type":"value", "unit":"1", "values":[4.0,5.0,6.0]},
            {"name":"unstated", "type":"value", "values":[7.0,8.0,9.0]}
        ]
    })
}

#[test]
fn cli_csv_and_tsv_round_trip_exact_quantity_metadata_and_nullable_samples() {
    let directory = common::test_dir("delimited_units");
    let input = directory.join("source.json");
    let restored = directory.join("restored.json");
    let original = source();
    std::fs::write(&input, original.to_string()).unwrap();
    for format in ["csv", "tsv"] {
        let encoded = directory.join(format!("encoded.{format}"));
        convert(&input, &encoded, format);
        convert(&encoded, &restored, "json");
        let decoded = common::read_json(&restored);
        assert_eq!(decoded, original, "{format}");
        assert_eq!(
            decoded["signals"][0]["values"][2]
                .as_f64()
                .unwrap()
                .to_bits(),
            (-0.0_f64).to_bits()
        );
    }
}

#[test]
fn application_reader_normalizes_coordinates_without_reinterpreting_signal_units() {
    let directory = common::test_dir("delimited_application_units");
    let input = directory.join("source.json");
    std::fs::write(&input, source().to_string()).unwrap();
    for (format, delimiter) in [("csv", b','), ("tsv", b'\t')] {
        let encoded = directory.join(format!("encoded.{format}"));
        convert(&input, &encoded, format);
        let decoded = decode_delimited_waveforms(
            &std::fs::read_to_string(encoded).unwrap(),
            delimiter,
            DelimitedReadLimits {
                max_columns: 10,
                max_rows: 3,
                max_header_bytes: 128,
                min_rows: 1,
            },
        )
        .unwrap();
        assert_eq!(decoded.coordinate, [0.0, 2e-9, 4e-9]);
        assert_eq!(decoded.columns[0].canonical_unit(), Some("s"));
        assert_eq!(decoded.columns[1].name, "charge [C]");
        assert_eq!(decoded.columns[1].canonical_unit(), Some("C"));
        assert_eq!(decoded.signal_values[0][0], 1.0);
        assert!(decoded.signal_values[0][1].is_nan());
        assert_eq!(decoded.signal_values[0][2].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(decoded.columns[2].canonical_unit(), Some("V/sqrt(Hz)"));
        assert_eq!(decoded.columns[3].canonical_unit(), Some("V/sqrt(Hz)"));
        assert_eq!(decoded.columns.last().unwrap().canonical_unit(), None);
    }
}

#[test]
fn delimited_round_trips_preserve_opaque_quantity_labels() {
    let directory = common::test_dir("delimited_opaque_quantities");
    let input = directory.join("source.json");
    let restored = directory.join("restored.json");
    let mut original = source();
    original["scale"]["type"] = json!("");
    original["signals"][0]["type"] = json!("custom \"charge\"\tquantity\n");
    original["signals"][1]["type"] = json!("");
    std::fs::write(&input, original.to_string()).unwrap();
    for format in ["csv", "tsv"] {
        let encoded = directory.join(format!("encoded.{format}"));
        convert(&input, &encoded, format);
        convert(&encoded, &restored, "json");
        assert_eq!(common::read_json(&restored), original, "{format}");
    }
}

#[test]
fn invalid_metadata_cannot_replace_converted_results_or_golden_files() {
    let directory = common::test_dir("delimited_metadata_integrity");
    let metadata = json!({
        "analysis":"ac", "title":"Integrity",
        "columns":[
            {"name":"frequency", "kind":"real", "quantity":"frequency", "unit":"Hz"},
            {"name":"Re(x)", "kind":"complex_real", "quantity":"voltage", "unit":"V"},
            {"name":"Im(x)", "kind":"complex_imag", "quantity":"voltage", "unit":"V"}
        ]
    });
    let encode = |delimiter, payload: &str, extra: Option<&[&str]>| {
        let mut writer = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(vec![]);
        writer
            .write_record(["frequency", "Re(x)", "Im(x)"])
            .unwrap();
        writer.write_record(["1", "2", "3"]).unwrap();
        writer.write_record(["2", "3", "4"]).unwrap();
        writer
            .write_record([
                rspice_formats::delimited::metadata::RECORD_MARKER,
                payload,
                "",
            ])
            .unwrap();
        if let Some(record) = extra {
            writer.write_record(record).unwrap();
        }
        writer.into_inner().unwrap()
    };
    for (format, delimiter) in [("csv", b','), ("tsv", b'\t')] {
        let input = directory.join(format!("invalid.{format}"));
        let golden = directory.join(format!("golden.{format}"));
        let destination = directory.join("protected.json");
        let valid_payload = metadata.to_string();
        let valid = encode(delimiter, &valid_payload, None);
        let mut cases = Vec::new();
        for (field, value) in [
            ("name", json!("Re(other)")),
            ("unit", json!(false)),
            ("unit", json!("A")),
            ("unit", json!("")),
            ("quantity", json!("current")),
        ] {
            let mut invalid = metadata.clone();
            invalid["columns"][1][field] = value;
            cases.push(encode(delimiter, &invalid.to_string(), None));
        }
        let mut oversized = metadata.clone();
        oversized["columns"] = json!([null, null, null, null]);
        cases.push(encode(delimiter, &oversized.to_string(), None));
        cases.push(encode(delimiter, &valid_payload, Some(&["3", "4", "5"])));
        cases.push(encode(
            delimiter,
            &valid_payload,
            Some(&[
                rspice_formats::delimited::metadata::RECORD_MARKER,
                &valid_payload,
                "",
            ]),
        ));
        for (index, invalid) in cases.iter().enumerate() {
            std::fs::write(&input, invalid).unwrap();
            std::fs::write(&golden, &valid).unwrap();
            std::fs::write(&destination, "predecessor").unwrap();
            for args in [
                vec![
                    "convert",
                    input.to_str().unwrap(),
                    destination.to_str().unwrap(),
                    "--to",
                    "json",
                ],
                vec![
                    "compare",
                    input.to_str().unwrap(),
                    golden.to_str().unwrap(),
                    "--bless",
                ],
            ] {
                let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .arg("--quiet")
                    .args(args)
                    .output()
                    .unwrap();
                assert_eq!(
                    output.status.code(),
                    Some(1),
                    "{format} case {index}: {output:?}"
                );
            }
            assert_eq!(std::fs::read(&golden).unwrap(), valid);
            assert_eq!(
                std::fs::read_to_string(&destination).unwrap(),
                "predecessor"
            );
            assert!(
                decode_delimited_waveforms(
                    std::str::from_utf8(invalid).unwrap(),
                    delimiter,
                    DelimitedReadLimits {
                        max_columns: 3,
                        max_rows: 3,
                        max_header_bytes: 128,
                        min_rows: 1
                    }
                )
                .is_err(),
                "application accepted {format} case {index}"
            );
        }
    }
}
