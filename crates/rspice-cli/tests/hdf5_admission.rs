//! A small file cannot authorize arbitrary decoded counts or allocations.
mod common;

use common::test_dir;
use rustyhdf5::{AttrValue, FileBuilder};
use std::process::Command;

fn fixture(path: &std::path::Path, family: &str, count: i64, shape: Option<&[u64]>) {
    let mut file = FileBuilder::new();
    file.set_attr("schema_version", AttrValue::String("1".into()));
    let mut group = file.create_group(if family == "measurements" {
        "measurements"
    } else {
        "data"
    });
    if family != "measurements" {
        group.set_attr("section_type", AttrValue::String(family.into()));
        group.set_attr("independent_name", AttrValue::String("time".into()));
    }
    group.set_attr(
        if family == "measurements" {
            "measurement_count"
        } else {
            "signal_count"
        },
        AttrValue::I64(count),
    );
    let dataset = group.create_dataset(if family == "ac" {
        "frequency"
    } else {
        "independent"
    });
    dataset.with_f64_data(&[0.0]);
    if let Some(shape) = shape {
        dataset.with_shape(shape);
    }
    file.add_group(group.finish());
    std::fs::write(path, file.finish().unwrap()).unwrap();
}

#[test]
fn negative_and_implausible_counts_are_structured_failures_without_publication() {
    let dir = test_dir("hdf_counts");
    for family in ["transient", "ac", "measurements"] {
        for count in [-1, i64::MAX] {
            let input = dir.join("malformed.h5");
            fixture(&input, family, count, None);
            let destination = dir.join("result.csv");
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "--error-format", "json", "convert"])
                .arg(input)
                .arg(&destination)
                .args(["--to", "csv"])
                .output()
                .unwrap();
            assert!(!result.status.success(), "{result:?}");
            assert_ne!(result.status.code(), Some(101), "{result:?}");
            let json: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
            assert!(json["error"]["message"].is_string());
            assert!(!destination.exists());
        }
    }
}

#[test]
fn dataset_shape_is_admitted_before_reading_its_values() {
    let dir = test_dir("hdf_shape");
    let input = dir.join("large-shape.h5");
    fixture(&input, "transient", 0, Some(&[1_000_000]));
    let config = dir.join("limits.toml");
    std::fs::write(&config, "[resources]\nmax_external_data_values=8\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--config")
        .arg(config)
        .args(["--quiet", "--error-format", "json", "convert"])
        .arg(input)
        .arg(dir.join("out.csv"))
        .args(["--to", "csv"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(json["error"]["limit"], 8);
    assert_eq!(json["error"]["requested"], 1_000_000);
}
