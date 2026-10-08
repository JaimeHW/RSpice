//! Complex missingness survives typed projection, table conversion and comparison.
mod common;

use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, Output};

fn convert(input: &Path, output: &Path, format: &str, flags: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", format])
        .args(flags)
        .output()
        .unwrap()
}

fn compare(result: &Path, golden: &Path, flags: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(result)
        .arg(golden)
        .args(flags)
        .output()
        .unwrap()
}

#[test]
fn typed_complex_missing_samples_survive_all_table_formats_and_clipping() {
    let directory = common::test_dir("typed_nullable_complex");
    let deck = directory.join("source.sp");
    let typed = directory.join("typed.json");
    std::fs::write(&deck, "* nullable AC projection\nV1 in 0 dc 0 ac 1\nR1 in out 1k\nC1 out 0 1u\n.AC LIN 3 10 100\n.END\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .arg("-o")
        .arg(&typed)
        .args(["-f", "json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let original = common::read_json(&typed);
    assert_eq!(original["pointCount"], 3);
    let source = directory.join("source.json");
    for all_missing in [false, true] {
        let mut document = original.clone();
        let samples = document["signals"][0]["values"]["samples"]
            .as_array_mut()
            .unwrap();
        for (index, sample) in samples.iter_mut().enumerate() {
            if all_missing || index == 1 {
                *sample = Value::Null;
            }
        }
        let expected: Vec<_> = samples.iter().map(|v| v["real"].clone()).collect();
        let expected_imag: Vec<_> = samples.iter().map(|v| v["imaginary"].clone()).collect();
        let content = serde_json::to_string(&document).unwrap();
        rspice_core::execution::AnalysisResultDocument::from_json(&content).unwrap();
        std::fs::write(&source, content).unwrap();
        for (format, extension) in [
            ("json", "json"),
            ("csv", "csv"),
            ("tsv", "tsv"),
            ("raw", "raw"),
            ("ascii", "ascii.raw"),
            ("hdf5", "h5"),
        ] {
            let encoded = directory.join(format!("encoded.{extension}"));
            let decoded = directory.join("decoded.json");
            let output = convert(&source, &encoded, format, &[]);
            assert!(output.status.success(), "{format}: {output:?}");
            let output = convert(&encoded, &decoded, "json", &[]);
            assert!(output.status.success(), "{format}: {output:?}");
            let table = common::read_json(&decoded);
            let signals = table["signals"].as_array().unwrap();
            assert_eq!(
                signals.len(),
                original["signals"].as_array().unwrap().len(),
                "{format}"
            );
            assert_eq!(signals[0]["name"], "V(IN)");
            assert_eq!(signals[0]["real"], json!(expected), "{format}");
            assert_eq!(signals[0]["imag"], json!(expected_imag), "{format}");
            if !matches!(format, "csv" | "tsv") {
                assert_eq!(signals[0]["type"], "voltage");
                assert_eq!(signals[0]["unit"], "V");
            }
            let output = convert(
                &encoded,
                &decoded,
                "json",
                &["--variables", "V(IN)", "--start", "55", "--stop", "55"],
            );
            assert!(output.status.success(), "{format}: {output:?}");
            let clipped = common::read_json(&decoded);
            assert_eq!(clipped["signals"].as_array().unwrap().len(), 1);
            assert_eq!(clipped["signals"][0]["real"], json!([null]));
            assert_eq!(clipped["signals"][0]["imag"], json!([null]));
            let output = compare(&encoded, &encoded, &["--json"]);
            assert_eq!(
                output.status.code(),
                Some(if all_missing { 3 } else { 0 }),
                "{format}: {output:?}"
            );
        }
    }
}

#[test]
fn nullable_complex_comparison_checks_both_components_and_interpolation() {
    let directory = common::test_dir("compare_nullable_complex");
    let result = directory.join("result.json");
    let golden = directory.join("golden.json");
    let source = json!({"scale":{"name":"frequency","values":[0,2,4]},"signals":[
        {"name":"V(out)","real":[null,2,4],"imag":[null,4,8]}
    ]});
    let reference = json!({"scale":{"name":"frequency","values":[0,1,2,3,4]},"signals":[
        {"name":"V(out)","real":[null,null,2,3,4],"imag":[null,null,4,6,8]}
    ]});
    std::fs::write(&result, source.to_string()).unwrap();
    for (real, imag, status) in [(None, None, 0), (Some(0.0), Some(0.0), 3)] {
        let mut document = reference.clone();
        document["signals"][0]["real"][1] = json!(real);
        document["signals"][0]["imag"][1] = json!(imag);
        std::fs::write(&golden, document.to_string()).unwrap();
        let output = compare(&result, &golden, &["--interpolate", "--json"]);
        assert_eq!(output.status.code(), Some(status), "{output:?}");
    }
    let mut changed = reference;
    changed["signals"][0]["imag"][3] = json!(7);
    std::fs::write(&golden, changed.to_string()).unwrap();
    let output = compare(&result, &golden, &["--interpolate", "--json"]);
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["num_differences"], 1);
    assert_eq!(report["differences"][0]["variable"], "Im(V(out))");
}

#[test]
fn mismatched_complex_availability_cannot_publish_or_be_blessed() {
    let directory = common::test_dir("invalid_nullable_complex");
    let source = directory.join("source.json");
    let golden = directory.join("golden.json");
    let valid = json!({"scale":{"name":"time","values":[0,1]},"signals":[
        {"name":"V(out)","real":[null,1],"imag":[null,2]}
    ]});
    let original = valid.to_string();
    std::fs::write(&golden, &original).unwrap();
    for part in ["real", "imag"] {
        let mut invalid = valid.clone();
        invalid["signals"][0][part][0] = json!(0);
        std::fs::write(&source, invalid.to_string()).unwrap();
        let output = convert(&source, &golden, "json", &[]);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("matching availability"),
            "{output:?}"
        );
        assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
        let output = compare(&source, &golden, &["--bless"]);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
    }
}

#[test]
fn nullable_complex_dense_masks_reject_invalid_padding_and_flags() {
    let directory = common::test_dir("nullable_complex_masks");
    let source = directory.join("source.json");
    let encoded = directory.join("encoded.raw");
    let invalid = directory.join("invalid.raw");
    let decoded = directory.join("decoded.json");
    std::fs::write(
        &source,
        json!({"scale":{"name":"time","values":[0,1]},"signals":[
            {"name":"V(out)","type":"voltage","real":[null,1],"imag":[null,2]}
        ]})
        .to_string(),
    )
    .unwrap();
    let output = convert(&source, &encoded, "ascii", &[]);
    assert!(output.status.success(), "{output:?}");
    let original = std::fs::read_to_string(&encoded).unwrap();
    let (header, data) = original.split_once("Values:\n").unwrap();
    let mut rows = data.lines();
    let row: Vec<_> = rows.next().unwrap().split_whitespace().collect();
    let rest = rows.collect::<Vec<_>>().join("\n");
    assert_eq!(row.len(), 4);
    for (column, replacement) in [(2, "9,0"), (2, "0,9"), (3, "2,0")] {
        let mut fields = row.clone();
        fields[column] = replacement;
        std::fs::write(
            &invalid,
            format!("{header}Values:\n{}\n{rest}\n", fields.join("\t")),
        )
        .unwrap();
        std::fs::write(&decoded, "existing output").unwrap();
        let output = convert(&invalid, &decoded, "json", &[]);
        assert_eq!(output.status.code(), Some(1), "{column}: {output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("invalid nullable value or validity flag"),
            "{output:?}"
        );
        assert_eq!(
            std::fs::read_to_string(&decoded).unwrap(),
            "existing output"
        );
        let output = compare(&invalid, &encoded, &["--bless"]);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert_eq!(std::fs::read_to_string(&encoded).unwrap(), original);
    }
}

#[test]
fn missing_complex_components_count_toward_budgets_and_cannot_be_blessed_without_coverage() {
    let directory = common::test_dir("nullable_complex_admission");
    let source = directory.join("source.json");
    let golden = directory.join("golden.json");
    let config = directory.join("limited.toml");
    std::fs::write(
        &source,
        json!({"scale":{"name":"time","values":[0,1]},"signals":[
            {"name":"V(out)","real":[null,null],"imag":[null,null]}
        ]})
        .to_string(),
    )
    .unwrap();
    for resource in ["max_external_data_values", "max_result_values"] {
        std::fs::write(&config, format!("[resources]\n{resource}=5\n")).unwrap();
        let output = convert(
            &source,
            &golden,
            "json",
            &["--config", config.to_str().unwrap()],
        );
        assert_eq!(output.status.code(), Some(75), "{output:?}");
        assert!(!golden.exists());
    }
    let output = compare(&source, &golden, &["--bless"]);
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("no defined samples"),
        "{output:?}"
    );
    assert!(!golden.exists());
}
