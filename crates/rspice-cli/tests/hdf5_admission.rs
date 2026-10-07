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
    for budget in ["max_external_data_values", "max_result_values"] {
        std::fs::write(&config, format!("[resources]\n{budget}=8\n")).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .arg("--config")
            .arg(&config)
            .args(["--quiet", "--error-format", "json", "convert"])
            .arg(&input)
            .arg(dir.join("out.csv"))
            .args(["--to", "csv"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(75), "{output:?}");
        let json: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert_eq!(json["error"]["limit"], 8);
        assert_eq!(json["error"]["requested"], 1_000_000);
    }
}

fn container(input: &std::path::Path, multiple: bool) {
    let mut file = FileBuilder::new();
    file.set_attr("schema_version", AttrValue::String("1".into()));
    let sections = if multiple {
        vec![("selected", vec![0.0]), ("other", vec![0.0, 1.0, 2.0])]
    } else {
        vec![("selected", vec![0.0, 1.0, 2.0, 3.0])]
    };
    for (name, samples) in sections {
        let mut group = file.create_group(name);
        group.set_attr("section_type", AttrValue::String("transient".into()));
        group.set_attr("independent_name", AttrValue::String("time".into()));
        group.set_attr("signal_count", AttrValue::I64(1));
        group.set_attr("signal_0000_name", AttrValue::String("V(out)".into()));
        group.create_dataset("independent").with_f64_data(&samples);
        group.create_dataset("signal_0000").with_f64_data(&samples);
        file.add_group(group.finish());
    }
    let mut group = file.create_group("measurements");
    group.set_attr("measurement_count", AttrValue::I64(2));
    for index in 0..2 {
        group.set_attr(
            &format!("measurement_{index:04}_name"),
            AttrValue::String(format!("m{index}")),
        );
        group.set_attr(
            &format!("measurement_{index:04}_value"),
            AttrValue::F64(index as f64),
        );
    }
    file.add_group(group.finish());
    std::fs::write(input, file.finish().unwrap()).unwrap();
}

#[test]
fn selecting_or_blessing_cannot_bypass_container_value_limits() {
    let dir = test_dir("hdf_container_values");
    let multi = dir.join("container.h5");
    container(&multi, true);
    let single = dir.join("single.h5");
    container(&single, false);
    let config = dir.join("limits.toml");
    for budget in ["max_external_data_values", "max_result_values"] {
        // Eight dataset samples and two measurement values are retained, even
        // though the selected table contains just two samples.
        for limit in [7, 9, 10] {
            std::fs::write(&config, format!("[resources]\n{budget}={limit}\n")).unwrap();
            for operation in ["convert", "compare", "bless"] {
                let input = if operation == "bless" {
                    &single
                } else {
                    &multi
                };
                let destination = dir.join(if operation == "convert" {
                    "out.csv"
                } else {
                    "golden.h5"
                });
                let original = std::fs::read(input).unwrap();
                std::fs::write(&destination, &original).unwrap();
                let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
                command
                    .arg("--config")
                    .arg(&config)
                    .args(["--quiet", "--error-format", "json"]);
                command
                    .arg(if operation == "convert" {
                        "convert"
                    } else {
                        "compare"
                    })
                    .arg(input)
                    .arg(&destination);
                if operation == "bless" {
                    command.arg("--bless");
                } else {
                    command.args(["--section", "selected"]);
                }
                if operation == "convert" {
                    command.args(["--to", "csv"]);
                }
                let output = command.output().unwrap();
                if limit == 10 {
                    assert!(output.status.success(), "{budget} {operation}: {output:?}");
                } else {
                    assert_eq!(
                        output.status.code(),
                        Some(75),
                        "{budget} {limit} {operation}: {output:?}"
                    );
                    let json: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
                    assert_eq!(json["error"]["limit"], limit);
                    assert!(json["error"]["requested"].as_u64().unwrap() > limit);
                    assert_eq!(std::fs::read(&destination).unwrap(), original);
                }
            }
        }
    }
}
