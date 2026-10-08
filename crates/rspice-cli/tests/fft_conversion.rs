//! FFT conversion must preserve provenance, metrics, and unavailable requests.
mod common;

use common::{read_json, test_dir};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .unwrap()
}

fn convert(source: &Path, destination: &Path, from: &str, to: &str, extra: &[&str]) -> Output {
    let mut args = vec![
        "convert",
        source.to_str().unwrap(),
        destination.to_str().unwrap(),
        "--from",
        from,
        "--to",
        to,
    ];
    args.extend(extra);
    cli(&args)
}

fn source(directory: &Path) -> PathBuf {
    let deck = directory.join("fft.cir");
    let requested = directory.join("waveform.json");
    std::fs::write(&deck, "FFT conversion fixture\n.param amplitude=1\nV1 out 0 SIN(0 {amplitude} 1k)\nR1 out 0 1k\n.options fft fftout=1\n.tran 1u 1m\n.step param amplitude list 1 2\n.fft v(out) np=8 format=unorm window=rect freq=1k\n.fft i(V1) np=16 window=hann freq=2k fmin=1k\n.fft v(out) np=16 window=gauss alfa=3\n.fft v(out) np=16 window=kaiser alfa=3\n.end\n").unwrap();
    let output = cli(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        requested.to_str().unwrap(),
        "-f",
        "json",
    ]);
    assert!(output.status.success(), "{output:?}");
    let mut paths: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.to_string_lossy().ends_with(".fft.json"))
        .collect();
    paths.sort();
    assert_eq!(paths.len(), 2);
    paths.remove(0)
}

#[test]
fn numeric_tables_can_use_fft_envelope_column_names() {
    let directory = test_dir("numeric_fft_header_names");
    let fft = source(&directory);
    let fft_csv = directory.join("fft.csv");
    assert!(convert(&fft, &fft_csv, "json", "csv", &[]).status.success());
    let text = std::fs::read_to_string(&fft_csv).unwrap();
    let full_header: Vec<_> = text.lines().next().unwrap().split(',').collect();
    let numeric = directory.join("numeric.json");
    for names in [&full_header[..2], full_header.as_slice()] {
        let expected = serde_json::json!({
            "analysis": "converted", "plot_name": "Converted Data",
            "scale": {"name": names[0], "type": "value", "values": [0.0, 1.0]},
            "signals": names[1..].iter().enumerate().map(|(index, name)| {
                serde_json::json!({"name": name, "type": "value", "values": [index as f64 + 2.0, index as f64 + 3.0]})
            }).collect::<Vec<_>>()
        });
        std::fs::write(&numeric, serde_json::to_vec(&expected).unwrap()).unwrap();
        for format in ["csv", "tsv"] {
            let encoded = directory.join(format!("numeric-{}.{format}", names.len()));
            let recovered = directory.join("recovered.json");
            assert!(
                convert(&numeric, &encoded, "json", format, &[])
                    .status
                    .success()
            );
            let output = convert(&encoded, &recovered, format, "json", &[]);
            assert!(output.status.success(), "{format}: {output:?}");
            let actual = read_json(&recovered);
            assert_eq!(actual["scale"]["name"], expected["scale"]["name"]);
            assert_eq!(actual["scale"]["values"], expected["scale"]["values"]);
            assert_eq!(actual["signals"], expected["signals"]);

            let output = cli(&[
                "compare",
                encoded.to_str().unwrap(),
                numeric.to_str().unwrap(),
            ]);
            assert!(output.status.success(), "{output:?}");
            let golden = directory.join(format!("golden-{}.{format}", names.len()));
            let output = cli(&[
                "compare",
                encoded.to_str().unwrap(),
                golden.to_str().unwrap(),
                "--bless",
            ]);
            assert!(output.status.success(), "{output:?}");
            assert_eq!(
                std::fs::read(&golden).unwrap(),
                std::fs::read(&encoded).unwrap()
            );
        }
    }
}

#[test]
fn damaged_fft_envelopes_cannot_fall_back_to_numeric_tables() {
    let directory = test_dir("damaged_fft_delimited_envelopes");
    let fft = source(&directory);
    for (format, separator) in [("csv", ','), ("tsv", '\t')] {
        let golden = directory.join(format!("valid.{format}"));
        assert!(convert(&fft, &golden, "json", format, &[]).status.success());
        let original = std::fs::read_to_string(&golden).unwrap();
        let (header, records) = original.split_once('\n').unwrap();
        let cases = [
            format!("schema_version{separator}analysis\n2{separator}fft\n"),
            format!(
                "{header}\n{}",
                records.replacen("fft", "unknown-analysis", 1)
            ),
            format!(
                "{header}\n999{separator}{}",
                records.split_once(separator).unwrap().1
            ),
        ];
        for (index, content) in cases.iter().enumerate() {
            let invalid = directory.join(format!("damaged-{index}.{format}"));
            std::fs::write(&invalid, content).unwrap();
            let protected = directory.join("protected.json");
            std::fs::write(&protected, "predecessor").unwrap();
            let output = convert(&invalid, &protected, format, "json", &[]);
            assert_eq!(output.status.code(), Some(1), "{output:?}");
            assert!(
                String::from_utf8_lossy(&output.stderr).contains("FFT"),
                "{output:?}"
            );
            assert_eq!(std::fs::read_to_string(&protected).unwrap(), "predecessor");
            let output = cli(&[
                "compare",
                invalid.to_str().unwrap(),
                golden.to_str().unwrap(),
                "--bless",
            ]);
            assert_eq!(output.status.code(), Some(1), "{output:?}");
            assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
            let missing = directory.join(format!("missing-{index}.{format}"));
            let output = cli(&[
                "compare",
                invalid.to_str().unwrap(),
                missing.to_str().unwrap(),
                "--bless",
            ]);
            assert_eq!(output.status.code(), Some(1), "{output:?}");
            assert!(!missing.exists());
        }
    }
}

#[test]
fn fft_json_rejects_repeated_coefficients_before_conversion_or_blessing() {
    let directory = test_dir("fft_duplicate_json_fields");
    let input = source(&directory);
    let original = std::fs::read_to_string(&input).unwrap();
    let invalid = original.replacen("\"real\":", "\"real\": 99, \"real\":", 1);
    assert_ne!(invalid, original);
    std::fs::write(&input, invalid).unwrap();
    let golden = directory.join("golden.json");
    std::fs::write(&golden, &original).unwrap();
    let missing = directory.join("missing.json");
    for (command, destination, options) in [
        ("convert", &golden, vec!["--to", "json"]),
        ("compare", &golden, vec![]),
        ("compare", &golden, vec!["--bless"]),
        ("compare", &missing, vec!["--bless"]),
    ] {
        let mut args = vec![
            command,
            input.to_str().unwrap(),
            destination.to_str().unwrap(),
        ];
        args.extend(options);
        let output = cli(&args);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("duplicate JSON field"),
            "{output:?}"
        );
        assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
        assert!(!missing.exists());
    }
}

#[test]
fn fft_tsv_cannot_skip_a_record_whose_fields_are_all_empty() {
    let directory = test_dir("fft_empty_tsv_record");
    let json = source(&directory);
    let input = directory.join("input.tsv");
    let golden = directory.join("golden.tsv");
    let result = convert(&json, &golden, "json", "tsv", &[]);
    assert!(result.status.success(), "{result:?}");
    let original = std::fs::read_to_string(&golden).unwrap();
    let (header, rows) = original.split_once('\n').unwrap();
    let empty_row = "\t".repeat(header.split('\t').count() - 1);
    std::fs::write(&input, format!("{header}\n{empty_row}\n{rows}")).unwrap();
    let check = |output: Output| {
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("schema_version"),
            "{output:?}"
        );
    };
    for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5"] {
        let output = directory.join(format!("protected.{format}"));
        std::fs::write(&output, "predecessor").unwrap();
        check(convert(&input, &output, "tsv", format, &[]));
        assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
    }
    for bless in [false, true] {
        let mut args = vec!["compare", input.to_str().unwrap(), golden.to_str().unwrap()];
        if bless {
            args.push("--bless");
        }
        check(cli(&args));
        assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
    }
    let missing = directory.join("missing.tsv");
    check(cli(&[
        "compare",
        input.to_str().unwrap(),
        missing.to_str().unwrap(),
        "--bless",
    ]));
    assert!(!missing.exists());
}

/// Replace only FFT provenance, preserving the exact ASCII or binary payload.
fn rewrite_raw_metadata(
    bytes: &[u8],
    legacy: bool,
    change: impl FnOnce(&mut serde_json::Value),
) -> Vec<u8> {
    rewrite_raw_metadata_text(bytes, legacy, |text| {
        let mut metadata = serde_json::from_str(text).unwrap();
        change(&mut metadata);
        serde_json::to_string(&metadata).unwrap()
    })
}

fn rewrite_raw_metadata_text(
    bytes: &[u8],
    legacy: bool,
    change: impl FnOnce(&str) -> String,
) -> Vec<u8> {
    let parsed = rspice_core::io::parse_raw_reader(&mut std::io::Cursor::new(bytes)).unwrap();
    let metadata = change(&parsed.header.command);
    let marker = if parsed.header.is_binary {
        b"\nBinary:\n".as_slice()
    } else {
        b"\nValues:\n".as_slice()
    };
    let offset = bytes
        .windows(marker.len())
        .position(|part| part == marker)
        .unwrap()
        + 1;
    let header = std::str::from_utf8(&bytes[..offset]).unwrap();
    let mut output = Vec::new();
    let mut replaced = false;
    for line in header.lines() {
        if line.starts_with("Option: rspice_metadata_v") || line.starts_with("Command:") {
            if !replaced {
                if legacy {
                    output.extend_from_slice(format!("Command: {metadata}\n").as_bytes());
                } else {
                    rspice_core::io::ltspice_raw::write_raw_metadata_options(
                        &mut output,
                        &metadata,
                    )
                    .unwrap();
                }
                replaced = true;
            }
        } else {
            output.extend_from_slice(line.as_bytes());
            output.push(b'\n');
        }
    }
    assert!(replaced);
    output.extend_from_slice(&bytes[offset..]);
    output
}

fn numeric_literal(source: &str, path: &str, literal: &str) -> String {
    let mut value: serde_json::Value = serde_json::from_str(source).unwrap();
    *value.pointer_mut(path).unwrap() = serde_json::json!("numeric-test-marker");
    serde_json::to_string(&value)
        .unwrap()
        .replace("\"numeric-test-marker\"", literal)
}

fn precision_refusal(input: &Path, golden: &Path, output: &Path, from: &str, expected: &str) {
    let original = std::fs::read(golden).unwrap();
    let input_bytes = std::fs::read(input).unwrap();
    std::fs::write(output, "predecessor").unwrap();
    let check = |result: Output| {
        assert_eq!(result.status.code(), Some(1), "{result:?}");
        assert!(
            String::from_utf8_lossy(&result.stderr).contains(expected),
            "{result:?}"
        );
    };
    check(convert(input, output, from, "json", &[]));
    check(convert(input, input, from, from, &[]));
    for bless in [false, true] {
        let mut args = vec!["compare", input.to_str().unwrap(), golden.to_str().unwrap()];
        if bless {
            args.push("--bless");
        }
        check(cli(&args));
    }
    let missing = golden.with_file_name(format!(
        "missing.{}",
        golden.extension().unwrap().to_str().unwrap()
    ));
    check(cli(&[
        "compare",
        input.to_str().unwrap(),
        missing.to_str().unwrap(),
        "--bless",
    ]));
    assert!(!missing.exists());
    assert_eq!(std::fs::read(input).unwrap(), input_bytes);
    assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
    assert_eq!(std::fs::read(golden).unwrap(), original);
}

#[test]
fn fft_json_integer_precision_is_checked_before_conversion_comparison_and_blessing() {
    let directory = test_dir("fft_json_integer_precision");
    let golden = source(&directory);
    let text = std::fs::read_to_string(&golden).unwrap();
    let input = directory.join("input.json");
    let output = directory.join("protected.json");
    for path in [
        "/results/0/transform/alpha",
        "/results/0/spectrum/bins/0/value/real",
        "/results/0/metrics/enob_bits",
    ] {
        for literal in ["9007199254740993", "18446744073709551617"] {
            std::fs::write(&input, numeric_literal(&text, path, literal)).unwrap();
            precision_refusal(
                &input,
                &golden,
                &output,
                "json",
                "cannot be represented exactly",
            );
        }
    }
}

#[test]
fn fft_raw_metadata_cannot_round_integers_or_underflow_decimals() {
    let directory = test_dir("fft_raw_numeric_precision");
    let json = source(&directory);
    let input = directory.join("input.raw");
    let output = directory.join("protected.json");
    for format in ["raw", "ascii"] {
        let golden = directory.join(format!("golden.{format}.raw"));
        let result = convert(&json, &golden, "json", format, &[]);
        assert!(result.status.success(), "{result:?}");
        let bytes = std::fs::read(&golden).unwrap();
        for legacy in [false, true] {
            for (path, literal, message) in [
                ("/results/0/sampling/start_time_s", "1e-999", "underflow"),
                (
                    "/results/0/transform/alpha",
                    "9007199254740993",
                    "cannot be represented exactly",
                ),
            ] {
                std::fs::write(
                    &input,
                    rewrite_raw_metadata_text(&bytes, legacy, |text| {
                        numeric_literal(text, path, literal)
                    }),
                )
                .unwrap();
                precision_refusal(&input, &golden, &output, "raw", message);
            }
        }
    }
}

#[test]
fn fft_json_keeps_exact_float_boundaries_and_native_integer_metadata() {
    let directory = test_dir("fft_exact_numeric_boundaries");
    let source = source(&directory);
    let mut data = read_json(&source);
    data["coordinate"]["ordinal"] = serde_json::json!(usize::MAX);
    let text = serde_json::to_string(&data).unwrap();
    let input = directory.join("input.json");
    let output = directory.join("output.json");
    for (literal, expected) in [
        ("18446744073709551616", 18446744073709551616.0_f64),
        ("5e-324", 5e-324_f64),
        ("-0e-999", -0.0_f64),
    ] {
        std::fs::write(
            &input,
            numeric_literal(&text, "/results/0/transform/alpha", literal),
        )
        .unwrap();
        let result = convert(&input, &output, "json", "json", &[]);
        assert!(result.status.success(), "{literal}: {result:?}");
        let data = read_json(&output);
        assert_eq!(
            data["coordinate"]["ordinal"].as_u64().unwrap(),
            usize::MAX as u64
        );
        assert_eq!(
            data["results"][0]["transform"]["alpha"]
                .as_f64()
                .unwrap()
                .to_bits(),
            expected.to_bits()
        );
    }
}

fn roundtrip_all_formats(directory: &Path, source: &Path, expected: &serde_json::Value, tag: &str) {
    for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5"] {
        let intermediate = directory.join(format!("{tag}.{format}"));
        let recovered = directory.join(format!("{tag}.{format}.json"));
        let output = convert(source, &intermediate, "json", format, &[]);
        assert!(output.status.success(), "write {format}: {output:?}");
        let output = convert(&intermediate, &recovered, format, "json", &[]);
        assert!(output.status.success(), "read {format}: {output:?}");
        assert_eq!(
            &read_json(&recovered),
            expected,
            "{format} changed FFT document contents"
        );
    }
}

#[test]
fn all_six_formats_preserve_ragged_spectra_metrics_windows_and_run_identity() {
    let directory = test_dir("complete");
    let source = source(&directory);
    let expected = read_json(&source);
    assert!(expected["coordinate"].is_object());
    assert!(expected["results"][0]["metrics"].is_object());
    assert_ne!(
        expected["results"][0]["sampling"]["point_count"],
        expected["results"][1]["sampling"]["point_count"]
    );
    roundtrip_all_formats(&directory, &source, &expected, "complete");
}

#[test]
fn incomplete_requests_survive_mixed_and_empty_spectrum_bundles() {
    let directory = test_dir("incomplete");
    let source = source(&directory);
    let mut document = read_json(&source);
    for (tag, count) in [("mixed", 1), ("empty", 4)] {
        for result in document["results"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .take(count)
        {
            result["status"] = serde_json::json!({"kind":"incomplete-history","availableStart":0.0,"availableStop":0.0002});
            result["metrics"] = serde_json::Value::Null;
            result["spectrum"]["bins"] = serde_json::json!([]);
        }
        let input = directory.join(format!("{tag}-input.json"));
        std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
        roundtrip_all_formats(&directory, &input, &document, tag);
    }
}

#[test]
fn quoted_multiline_fft_coordinate_text_survives_delimited_conversion() {
    let directory = test_dir("multiline-coordinate");
    let source = source(&directory);
    let mut expected = read_json(&source);
    let assignment = expected["coordinate"]["assignment"].as_str().unwrap();
    expected["coordinate"]["assignment"] =
        format!(" {assignment}\r\n\"quoted, annotation\" ").into();
    std::fs::write(&source, serde_json::to_vec(&expected).unwrap()).unwrap();
    for format in ["csv", "tsv"] {
        let intermediate = directory.join(format!("metadata.{format}"));
        let recovered = directory.join(format!("{format}-metadata.json"));
        let output = convert(&source, &intermediate, "json", format, &[]);
        assert!(output.status.success(), "{output:?}");
        let output = convert(&intermediate, &recovered, format, "json", &[]);
        assert!(output.status.success(), "{output:?}");
        assert_eq!(read_json(&recovered), expected);
    }
}

#[test]
fn utf8_signatures_preserve_fft_schema_detection_and_metadata() {
    let directory = test_dir("fft_utf8_signatures");
    let source = source(&directory);
    let expected = read_json(&source);
    for format in ["csv", "tsv"] {
        let encoded = directory.join(format!("fft.{format}"));
        let recovered = directory.join("recovered.json");
        let output = convert(&source, &encoded, "json", format, &[]);
        assert!(output.status.success(), "{output:?}");
        let content = std::fs::read_to_string(&encoded).unwrap();
        for quoted in [false, true] {
            let content = if quoted {
                content.replacen("schema_version", "\"schema_version\"", 1)
            } else {
                content.clone()
            };
            std::fs::write(&encoded, format!("\u{feff}{content}")).unwrap();
            let output = convert(&encoded, &recovered, format, "json", &[]);
            assert!(
                output.status.success(),
                "{format}, quoted={quoted}: {output:?}"
            );
            assert_eq!(read_json(&recovered), expected);
            let compared = cli(&[
                "compare",
                encoded.to_str().unwrap(),
                source.to_str().unwrap(),
            ]);
            assert!(compared.status.success(), "{compared:?}");
        }
    }
}

#[test]
fn fft_raw_metadata_is_inert_and_legacy_command_files_remain_readable() {
    let directory = test_dir("inert_raw_metadata");
    let source = source(&directory);
    let mut expected = read_json(&source);
    expected["coordinate"]["assignment"] =
        "α \"; $literal `text`\r\n annotation ".repeat(100).into();
    std::fs::write(&source, serde_json::to_vec(&expected).unwrap()).unwrap();
    for format in ["raw", "ascii"] {
        let encoded = directory.join(format!("fft.{format}"));
        let recovered = directory.join("recovered.json");
        let output = convert(&source, &encoded, "json", format, &[]);
        assert!(output.status.success(), "{format}: {output:?}");
        let bytes = std::fs::read(&encoded).unwrap();
        let text = String::from_utf8_lossy(&bytes);
        let header: Vec<_> = text
            .lines()
            .take_while(|line| *line != "Binary:" && *line != "Values:")
            .collect();
        assert!(!header.iter().any(|line| line.starts_with("Command:")));
        let options: Vec<_> = header
            .iter()
            .filter(|line| line.starts_with("Option:"))
            .collect();
        assert!(options.len() > 1);
        for line in options {
            assert!(line.starts_with("Option: rspice_metadata_v1_"));
            assert!(line.len() < 320);
            assert!(
                line.split_once(" = x")
                    .unwrap()
                    .1
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit())
            );
        }
        for legacy in [false, true] {
            std::fs::write(
                &encoded,
                if legacy {
                    rewrite_raw_metadata(&bytes, true, |_| {})
                } else {
                    bytes.clone()
                },
            )
            .unwrap();
            let output = convert(&encoded, &recovered, format, "json", &[]);
            assert!(
                output.status.success(),
                "{format} legacy={legacy}: {output:?}"
            );
            assert_eq!(read_json(&recovered), expected);
            let compared = cli(&[
                "compare",
                encoded.to_str().unwrap(),
                source.to_str().unwrap(),
            ]);
            assert!(
                compared.status.success(),
                "{format} legacy={legacy}: {compared:?}"
            );
        }
    }
}

#[test]
fn damaged_fft_metadata_options_preserve_conversion_and_bless_destinations() {
    let directory = test_dir("damaged_raw_metadata");
    let source = source(&directory);
    let encoded = directory.join("valid.raw");
    assert!(
        convert(&source, &encoded, "json", "ascii", &[])
            .status
            .success()
    );
    let original = std::fs::read_to_string(&encoded).unwrap();
    let lines: Vec<_> = original.lines().collect();
    let chunk_indices: Vec<_> = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            line.starts_with("Option: rspice_metadata_v1_")
                .then_some(index)
        })
        .collect();
    assert!(chunk_indices.len() > 1);
    let input = directory.join("invalid.raw");
    let output = directory.join("protected.json");
    let golden = directory.join("golden.raw");
    for mutation in 0..5 {
        let mut changed = lines.clone();
        match mutation {
            0 => {
                changed.remove(*chunk_indices.last().unwrap());
            }
            1 => changed.swap(chunk_indices[0], chunk_indices[1]),
            2 => changed.insert(chunk_indices[0], lines[chunk_indices[0]]),
            3 | 4 => {}
            _ => unreachable!(),
        }
        let changed = changed.join("\n") + "\n";
        let changed = match mutation {
            3 => changed.replacen("rspice_metadata_v1_", "rspice_metadata_v2_", 1),
            4 => changed.replacen(" = x", " = xz", 1),
            _ => changed,
        };
        std::fs::write(&input, changed).unwrap();
        std::fs::write(&output, "predecessor").unwrap();
        std::fs::write(&golden, &original).unwrap();
        let converted = convert(&input, &output, "ascii", "json", &[]);
        assert_eq!(
            converted.status.code(),
            Some(1),
            "{mutation}: {converted:?}"
        );
        assert!(String::from_utf8_lossy(&converted.stderr).contains("metadata"));
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
        let compared = cli(&[
            "compare",
            input.to_str().unwrap(),
            golden.to_str().unwrap(),
            "--bless",
        ]);
        assert_eq!(compared.status.code(), Some(1), "{mutation}: {compared:?}");
        assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
    }
}

#[test]
fn invalid_metadata_bins_and_partial_transform_requests_preserve_existing_output() {
    let directory = test_dir("invalid");
    let source = source(&directory);
    let original = read_json(&source);
    let destination = directory.join("protected.json");
    std::fs::write(&destination, "predecessor").unwrap();
    for pointer in [
        "/results/3/spectrum/bins/2/magnitude",
        "/results/3/transform/fundamental_bin",
        "/results/3/metrics/thd_ratio",
    ] {
        let mut document = original.clone();
        *document.pointer_mut(pointer).unwrap() = serde_json::json!(999999);
        let input = directory.join("invalid.json");
        std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
        let output = convert(&input, &destination, "json", "json", &[]);
        assert_eq!(output.status.code(), Some(1), "{pointer}: {output:?}");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "predecessor"
        );
    }
    for flags in [
        vec!["--variables", "V(out)"],
        vec!["--start", "1k"],
        vec!["--stop", "2k"],
    ] {
        let output = convert(&source, &destination, "json", "json", &flags);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "predecessor"
        );
    }
    let csv = directory.join("invalid.csv");
    assert!(convert(&source, &csv, "json", "csv", &[]).status.success());
    let text = std::fs::read_to_string(&csv).unwrap();
    let (head, tail) = text.split_once('\n').unwrap();
    let (row, rest) = tail.split_once('\n').unwrap();
    let broken = row.replacen("tran-001", "tran-002", 1);
    std::fs::write(&csv, format!("{head}\n{row}\n{broken}\n{rest}")).unwrap();
    let output = convert(&csv, &destination, "csv", "json", &[]);
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert_eq!(std::fs::read_to_string(destination).unwrap(), "predecessor");
}

#[test]
fn all_fft_readers_enforce_numeric_admission_before_publication() {
    let directory = test_dir("budget");
    let source = source(&directory);
    let config = directory.join("limited.toml");
    let destination = directory.join("protected.json");
    std::fs::write(&destination, "predecessor").unwrap();
    for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5"] {
        let input = directory.join(format!("budget.{format}"));
        assert!(
            convert(&source, &input, "json", format, &[])
                .status
                .success()
        );
        for budget in ["max_external_data_values", "max_result_values"] {
            std::fs::write(&config, format!("[resources]\n{budget}=1\n")).unwrap();
            let output = convert(
                &input,
                &destination,
                format,
                "json",
                &["--config", config.to_str().unwrap()],
            );
            assert_eq!(
                output.status.code(),
                Some(75),
                "{format} {budget}: {output:?}"
            );
            assert_eq!(
                std::fs::read_to_string(&destination).unwrap(),
                "predecessor"
            );
            for bless in [false, true] {
                let golden = directory.join(format!("golden.{format}"));
                let original = std::fs::read(&input).unwrap();
                std::fs::write(&golden, &original).unwrap();
                let mut args = vec![
                    "--config",
                    config.to_str().unwrap(),
                    "--error-format",
                    "json",
                    "compare",
                    input.to_str().unwrap(),
                    golden.to_str().unwrap(),
                ];
                if bless {
                    args.push("--bless");
                }
                let compared = cli(&args);
                assert_eq!(
                    compared.status.code(),
                    Some(75),
                    "{format} {budget} bless={bless}: {compared:?}"
                );
                let error: serde_json::Value = serde_json::from_slice(&compared.stderr).unwrap();
                assert_eq!(error["error"]["category"], "resource_limit");
                assert_eq!(error["error"]["limit"], 1);
                if format == "json" {
                    assert_eq!(
                        error["error"]["requested"], 2,
                        "JSON decoding must stop at the first excess numeric field"
                    );
                }
                assert_eq!(std::fs::read(&golden).unwrap(), original);
            }
        }
    }
}

#[test]
fn raw_fft_metadata_counts_toward_the_whole_file_numeric_budget() {
    fn count(value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Number(_) => 1,
            serde_json::Value::Array(values) => values.iter().map(count).sum(),
            serde_json::Value::Object(values) => values.values().map(count).sum(),
            _ => 0,
        }
    }
    let directory = test_dir("raw_fft_metadata_budget");
    let json = source(&directory);
    let original = read_json(&json);
    let input = directory.join("input.raw");
    let output = directory.join("protected.json");
    let config = directory.join("limit.toml");
    for incomplete in [false, true] {
        let mut document = original.clone();
        if incomplete {
            for result in document["results"].as_array_mut().unwrap() {
                result["status"] = serde_json::json!({"kind":"incomplete-history","availableStart":0.0,"availableStop":0.0002});
                result["metrics"] = serde_json::Value::Null;
                result["spectrum"]["bins"] = serde_json::json!([]);
            }
        }
        std::fs::write(&json, serde_json::to_vec(&document).unwrap()).unwrap();
        for format in ["raw", "ascii"] {
            let baseline = directory.join("baseline.raw");
            assert!(
                convert(&json, &baseline, "json", format, &[])
                    .status
                    .success()
            );
            let bytes = std::fs::read(&baseline).unwrap();
            let raw = rspice_core::io::parse_raw_reader(&mut std::io::Cursor::new(&bytes)).unwrap();
            let metadata = count(&serde_json::from_str(&raw.header.command).unwrap());
            let samples = raw.header.no_variables * raw.header.no_points;
            for copies in [1, 2] {
                std::fs::write(&input, bytes.repeat(copies)).unwrap();
                for (budget, total) in [
                    ("max_external_data_values", (samples + metadata) * copies),
                    ("max_result_values", (2 * samples + metadata) * copies),
                ] {
                    std::fs::write(&config, format!("[resources]\n{budget}={total}\n")).unwrap();
                    let result = convert(
                        &input,
                        &output,
                        "raw",
                        "json",
                        &["--section", "1", "--config", config.to_str().unwrap()],
                    );
                    assert!(
                        result.status.success(),
                        "{incomplete}/{format}/{copies}/{budget}: {result:?}"
                    );
                    std::fs::write(&config, format!("[resources]\n{budget}={}\n", total - 1))
                        .unwrap();
                    std::fs::write(&output, "predecessor").unwrap();
                    for to in ["json", "vcd"] {
                        let result = convert(
                            &input,
                            &output,
                            "raw",
                            to,
                            &[
                                "--section",
                                "1",
                                "--config",
                                config.to_str().unwrap(),
                                "--error-format",
                                "json",
                            ],
                        );
                        assert_eq!(
                            result.status.code(),
                            Some(75),
                            "{incomplete}/{format}/{copies}/{budget}/{to}: {result:?}"
                        );
                        let error: serde_json::Value =
                            serde_json::from_slice(&result.stderr).unwrap();
                        assert_eq!(error["error"]["requested"], total);
                        assert_eq!(error["error"]["limit"], total - 1);
                    }
                    assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
                    if copies == 1 {
                        for bless in [false, true] {
                            let mut args = vec![
                                "--config",
                                config.to_str().unwrap(),
                                "compare",
                                input.to_str().unwrap(),
                                baseline.to_str().unwrap(),
                            ];
                            if bless {
                                args.push("--bless");
                            }
                            let result = cli(&args);
                            assert_eq!(result.status.code(), Some(75), "{result:?}");
                            assert_eq!(std::fs::read(&baseline).unwrap(), bytes);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn selecting_a_raw_plot_still_validates_other_fft_plot_metadata() {
    let directory = test_dir("raw_container");
    let source = source(&directory);
    let fft = directory.join("spectrum.raw");
    assert!(
        convert(&source, &fft, "json", "ascii", &[])
            .status
            .success()
    );
    let bytes = rewrite_raw_metadata(&std::fs::read(fft).unwrap(), false, |metadata| {
        metadata["results"][3]["analysis_id"] = "fft-999".into();
    });
    let text = String::from_utf8(bytes).unwrap();
    let multi = directory.join("container.raw");
    std::fs::write(&multi, format!("Title: Simple\nPlotname: Transient Analysis\nFlags: real double\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 1\n{text}")).unwrap();
    let output = convert(
        &multi,
        &directory.join("selected.json"),
        "ascii",
        "json",
        &["--section", "1"],
    );
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("FFT"),
        "{output:?}"
    );
}

#[test]
fn raw_event_exports_validate_companion_fft_metadata() {
    let directory = test_dir("event_fft_container");
    let source = source(&directory);
    for format in ["raw", "ascii"] {
        let fft = directory.join("spectrum.raw");
        assert!(convert(&source, &fft, "json", format, &[]).status.success());
        let mut bytes = b"Title: Events\nPlotname: Real Events (rspice-real-events/1)\nFlags: real double\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 E(ctrl) real\nValues:\n0 0 1\n".to_vec();
        let fft_bytes = std::fs::read(fft).unwrap();
        let header_length = bytes.len();
        bytes.extend(&fft_bytes);
        let input = directory.join("events.raw");
        let output = directory.join("events.vcd");
        std::fs::write(&input, &bytes).unwrap();
        let valid = convert(&input, &output, format, "vcd", &[]);
        assert!(valid.status.success(), "{format}: {valid:?}");
        let original = std::fs::read(&output).unwrap();
        bytes.truncate(header_length);
        bytes.extend(rewrite_raw_metadata(&fft_bytes, false, |metadata| {
            metadata["results"][3]["analysis_id"] = "fft-999".into();
        }));
        std::fs::write(&input, bytes).unwrap();
        for extra in [&[][..], &["--section", "1"][..]] {
            let invalid = convert(&input, &output, format, "vcd", extra);
            assert_eq!(invalid.status.code(), Some(1), "{format}: {invalid:?}");
            assert!(
                String::from_utf8_lossy(&invalid.stderr).contains("FFT"),
                "{invalid:?}"
            );
            assert_eq!(std::fs::read(&output).unwrap(), original);
        }
    }
}

#[test]
fn typed_fft_comparison_retains_transform_contracts_across_formats() {
    let directory = test_dir("comparison");
    let source = source(&directory);
    for (format, extension) in [
        ("json", "json"),
        ("csv", "csv"),
        ("tsv", "tsv"),
        ("raw", "raw"),
        ("ascii", "raw"),
        ("hdf5", "h5"),
    ] {
        let destination = directory.join(format!("compare-{format}.{extension}"));
        assert!(
            convert(&source, &destination, "json", format, &[])
                .status
                .success()
        );
        let compared = cli(&[
            "compare",
            destination.to_str().unwrap(),
            source.to_str().unwrap(),
            "--json",
        ]);
        assert!(compared.status.success(), "{format}: {compared:?}");
        let report: serde_json::Value = serde_json::from_slice(&compared.stdout).unwrap();
        assert_eq!(report["comparison_passed"], true);
        assert_eq!(report["num_variables"], 4);
    }
    let mut altered = read_json(&source);
    altered["results"][0]["sampling"]["accurate_sampling"] = serde_json::json!(false);
    let changed = directory.join("different-policy.json");
    std::fs::write(&changed, serde_json::to_vec(&altered).unwrap()).unwrap();
    let compared = cli(&[
        "compare",
        changed.to_str().unwrap(),
        source.to_str().unwrap(),
        "--json",
    ]);
    assert!(!compared.status.success(), "{compared:?}");
    assert!(String::from_utf8_lossy(&compared.stdout).contains("sampling contract differs"));
    let selected = cli(&[
        "compare",
        changed.to_str().unwrap(),
        source.to_str().unwrap(),
        "--variables",
        "I(V1)",
        "--json",
    ]);
    assert!(selected.status.success(), "{selected:?}");
    let report: serde_json::Value = serde_json::from_slice(&selected.stdout).unwrap();
    assert_eq!(report["num_variables"], 1);
    for result in altered["results"].as_array_mut().unwrap() {
        result["status"] = serde_json::json!({"kind":"incomplete-history","availableStart":0.0,"availableStop":0.0002});
        result["metrics"] = serde_json::Value::Null;
        result["spectrum"]["bins"] = serde_json::json!([]);
    }
    std::fs::write(&changed, serde_json::to_vec(&altered).unwrap()).unwrap();
    let compared = cli(&[
        "compare",
        changed.to_str().unwrap(),
        changed.to_str().unwrap(),
        "--json",
    ]);
    assert!(
        !compared.status.success(),
        "unavailable spectra must not pass: {compared:?}"
    );
}

#[test]
fn fft_comparison_applies_numeric_tolerances_to_spectral_values() {
    let directory = test_dir("numeric_comparison");
    let deck = directory.join("deck.cir");
    let mut spectra = Vec::new();
    for (name, amplitude) in [("golden", 1.0), ("result", 1.01)] {
        let out = directory.join(format!("{name}.json"));
        std::fs::write(&deck, format!("numeric FFT comparison\nV1 out 0 SIN(0 {amplitude} 1k)\nR1 out 0 1k\n.tran 1u 1m\n.fft v(out) np=8 format=unorm window=rect\n.end\n")).unwrap();
        let run = cli(&[
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "json",
        ]);
        assert!(run.status.success(), "{run:?}");
        spectra.push(directory.join(format!("{name}.fft.json")));
    }
    let compare = |extra: &[&str]| {
        let mut args = vec![
            "compare",
            spectra[1].to_str().unwrap(),
            spectra[0].to_str().unwrap(),
            "--json",
        ];
        args.extend_from_slice(extra);
        cli(&args)
    };
    let failed = compare(&[]);
    assert!(!failed.status.success(), "{failed:?}");
    let report: serde_json::Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert!(report["num_differences"].as_u64().unwrap() > 0);
    let passed = compare(&["--abstol", "1"]);
    assert!(passed.status.success(), "{passed:?}");
    let fast = compare(&["--fail-fast"]);
    let report: serde_json::Value = serde_json::from_slice(&fast.stdout).unwrap();
    assert_eq!(report["num_differences"], 1);
}

#[test]
fn bootstrap_bless_refuses_fft_interpolation_without_creating_a_golden() {
    let directory = test_dir("fft_bless_interpolation");
    let source = source(&directory);
    for json in [false, true] {
        let golden = directory.join(format!("golden-{json}.json"));
        let mut args = vec![
            "compare",
            source.to_str().unwrap(),
            golden.to_str().unwrap(),
            "--bless",
            "--interpolate",
        ];
        if json {
            args.push("--json");
        }
        let output = cli(&args);
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains("discrete transform grids"));
        assert!(!golden.exists());
    }
}
