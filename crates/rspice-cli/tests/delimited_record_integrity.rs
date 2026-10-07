//! Empty fields remain records, rather than disappearing as blank lines.
mod common;

use common::{read_json, test_dir};
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn delimiter_only_headers_do_not_consume_the_first_data_row() {
    let directory = test_dir("delimiter_only_header");
    for (format, separator) in [("csv", ','), ("tsv", '\t')] {
        let input = directory.join(format!("source.{format}"));
        let output = directory.join("output.json");
        std::fs::write(
            &input,
            format!(" \r\n{separator}\r\n0{separator}1\r\n1{separator}2\r\n"),
        )
        .unwrap();
        let result = cli(&[
            "convert",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
            "--to",
            "json",
        ]);
        assert!(result.status.success(), "{result:?}");
        let actual = read_json(&output);
        assert_eq!(actual["scale"]["name"], "");
        assert_eq!(actual["scale"]["values"], serde_json::json!([0.0, 1.0]));
        assert_eq!(actual["signals"][0]["name"], "");
        assert_eq!(
            actual["signals"][0]["values"],
            serde_json::json!([1.0, 2.0])
        );
    }
}

#[test]
fn delimiter_only_data_rows_fail_before_conversion_comparison_or_bless() {
    let directory = test_dir("missing_delimited_samples");
    for (format, separator) in [("csv", ','), ("tsv", '\t')] {
        let input = directory.join(format!("source.{format}"));
        let golden = directory.join(format!("golden.{format}"));
        let missing = directory.join(format!("missing.{format}"));
        for report in [false, true] {
            let valid = if report {
                " \nsignal,value\nV(out),1\n \nV(in),2\n"
            } else {
                " \ntime,D(clk)\n0,0\n \n1,1\n"
            }
            .replace(',', &separator.to_string());
            let invalid = valid.replacen("\n \n", &format!("\n {separator} \n"), 1);
            std::fs::write(&input, &invalid).unwrap();
            std::fs::write(&golden, &valid).unwrap();
            let check = |output: Output| {
                assert_eq!(
                    output.status.code(),
                    Some(1),
                    "{format}, report={report}: {output:?}"
                );
                assert!(
                    String::from_utf8_lossy(&output.stderr).contains("row 4"),
                    "{output:?}"
                );
            };
            for output_format in ["json", "csv", "tsv", "raw", "ascii", "hdf5", "vcd"] {
                let destination = directory.join(format!("protected.{output_format}"));
                std::fs::write(&destination, "predecessor").unwrap();
                check(cli(&[
                    "convert",
                    input.to_str().unwrap(),
                    destination.to_str().unwrap(),
                    "--to",
                    output_format,
                ]));
                assert_eq!(std::fs::read_to_string(destination).unwrap(), "predecessor");
            }
            for bless in [false, true] {
                let mut args = vec!["compare", input.to_str().unwrap(), golden.to_str().unwrap()];
                if bless {
                    args.push("--bless");
                }
                check(cli(&args));
                assert_eq!(std::fs::read_to_string(&golden).unwrap(), valid);
            }
            check(cli(&[
                "compare",
                input.to_str().unwrap(),
                missing.to_str().unwrap(),
                "--bless",
            ]));
            assert!(!missing.exists());
            // Truly blank, separator-free lines retain their supported meaning.
            let output = directory.join("valid.json");
            let result = cli(&[
                "convert",
                golden.to_str().unwrap(),
                output.to_str().unwrap(),
                "--to",
                "json",
            ]);
            assert!(result.status.success(), "{result:?}");
            let actual = read_json(&output);
            if report {
                assert_eq!(actual["signals"].as_array().unwrap().len(), 2);
            } else {
                assert_eq!(actual["scale"]["values"], serde_json::json!([0.0, 1.0]));
            }
        }
    }
}
