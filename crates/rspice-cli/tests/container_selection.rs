//! Selecting a result must never hide malformed data elsewhere in its container.
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

fn raw_plot(name: &str, value: f64) -> String {
    format!(
        "Title: fixture\nPlotname: {name}\nFlags: real\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 {value}\n1 1 {value}\n"
    )
}

fn hdf5(path: &Path, second_family: &str, second: &[f64]) {
    use rustyhdf5::{AttrValue, FileBuilder};
    let mut file = FileBuilder::new();
    file.set_attr("schema_version", AttrValue::String("1".into()));
    for (name, family, values) in [
        ("first", "transient", &[1.0, 1.0][..]),
        ("second", second_family, second),
    ] {
        let mut group = file.create_group(name);
        group.set_attr("section_type", AttrValue::String(family.into()));
        group.set_attr("independent_name", AttrValue::String("time".into()));
        group.set_attr("signal_count", AttrValue::I64(1));
        group.set_attr("signal_0000_name", AttrValue::String("V(out)".into()));
        group.set_attr("signal_0000_type", AttrValue::String("voltage".into()));
        group
            .create_dataset("independent")
            .with_f64_data(&[0.0, 1.0]);
        group.create_dataset("signal_0000").with_f64_data(values);
        file.add_group(group.finish());
    }
    std::fs::write(path, file.finish().unwrap()).unwrap();
}

#[test]
fn raw_requires_selection_and_validates_the_unselected_tail() {
    let dir = test_dir("raw_sections");
    let left = dir.join("left.raw");
    let right = dir.join("right.raw");
    std::fs::write(&left, raw_plot("first", 1.0) + &raw_plot("second", 2.0)).unwrap();
    std::fs::write(&right, raw_plot("first", 1.0) + &raw_plot("second", 99.0)).unwrap();
    let compare = |flags: &[&str]| {
        let mut args = vec![
            "compare",
            left.to_str().unwrap(),
            right.to_str().unwrap(),
            "--json",
        ];
        args.extend_from_slice(flags);
        cli(&args)
    };
    let ambiguous = compare(&[]);
    assert!(!ambiguous.status.success(), "{ambiguous:?}");
    assert!(String::from_utf8_lossy(&ambiguous.stderr).contains("2 result sections"));
    assert!(compare(&["--section", "first"]).status.success());
    assert_eq!(compare(&["--section", "2"]).status.code(), Some(3));
    std::fs::write(
        &right,
        raw_plot("first", 1.0) + "Title: truncated second plot\n",
    )
    .unwrap();
    assert!(!compare(&["--section", "first"]).status.success());
    let destination = dir.join("selected.csv");
    std::fs::write(&destination, "predecessor").unwrap();
    let result = cli(&[
        "convert",
        right.to_str().unwrap(),
        destination.to_str().unwrap(),
        "--to",
        "csv",
        "--section",
        "1",
    ]);
    assert!(!result.status.success());
    assert_eq!(std::fs::read_to_string(destination).unwrap(), "predecessor");
}

#[test]
fn repeated_hdf5_families_are_selected_by_group_and_all_sections_are_validated() {
    let dir = test_dir("hdf5_sections");
    let left = dir.join("left.h5");
    let right = dir.join("right.h5");
    let csv = dir.join("selected.csv");
    for family in ["transient", "dc_sweep"] {
        hdf5(&left, family, &[2.0, 2.0]);
        hdf5(&right, family, &[99.0, 99.0]);
        assert!(
            !cli(&["compare", left.to_str().unwrap(), right.to_str().unwrap()])
                .status
                .success()
        );
        assert_eq!(
            cli(&[
                "compare",
                left.to_str().unwrap(),
                right.to_str().unwrap(),
                "--section",
                "second"
            ])
            .status
            .code(),
            Some(3)
        );
        let converted = cli(&[
            "convert",
            left.to_str().unwrap(),
            csv.to_str().unwrap(),
            "--to",
            "csv",
            "--section",
            "second",
        ]);
        assert!(converted.status.success(), "{converted:?}");
        for line in std::fs::read_to_string(&csv).unwrap().lines().skip(1) {
            assert_eq!(line.split(',').nth(1).unwrap().parse::<f64>().unwrap(), 2.0);
        }
        hdf5(&right, family, &[99.0]);
        assert!(
            !cli(&[
                "compare",
                left.to_str().unwrap(),
                right.to_str().unwrap(),
                "--section",
                "first"
            ])
            .status
            .success()
        );
        hdf5(&right, family, &[f64::NAN, f64::NAN]);
        let result = cli(&[
            "compare",
            left.to_str().unwrap(),
            right.to_str().unwrap(),
            "--section",
            "first",
        ]);
        assert!(!result.status.success(), "{result:?}");
        assert!(String::from_utf8_lossy(&result.stderr).contains("non-finite"));
    }
}

#[test]
fn raw_container_resource_limits_cover_all_plots_before_selection() {
    let dir = test_dir("raw_container_budget");
    let input = dir.join("multi.raw");
    let config = dir.join("config.toml");
    let output = dir.join("selected.csv");
    std::fs::write(&input, raw_plot("first", 1.0) + &raw_plot("second", 2.0)).unwrap();
    for (resource, limit) in [("max_external_data_values", 7), ("max_result_values", 15)] {
        std::fs::write(&config, format!("[resources]\n{resource}={limit}\n")).unwrap();
        let result = cli(&[
            "--config",
            config.to_str().unwrap(),
            "convert",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
            "--to",
            "csv",
            "--section",
            "1",
        ]);
        assert_eq!(result.status.code(), Some(75), "{result:?}");
        assert!(!output.exists());
    }
}
