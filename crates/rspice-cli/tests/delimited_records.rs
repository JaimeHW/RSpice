//! Quoted field identity, logical records and failed publication.
mod common;
use std::path::Path;
use std::process::{Command, Output};

fn convert(input: &Path, output: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", format])
        .output()
        .unwrap()
}

fn success(output: Output) {
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn csv_and_tsv_roundtrip_quoted_multiline_and_padded_names() {
    let directory = common::test_dir("quoted-records");
    let input = directory.join("input.json");
    let original = serde_json::json!({
        "analysis":"tran", "scale":{"name":" time\naxis ","type":"time","values":[0.0,1.0]},
        "signals":[
            {"name":"  padded  ","type":"value","values":[1.0,2.0]},
            {"name":"V(α\r\nout,\"tap\")","type":"voltage","real":[3.0,4.0],"imag":[-1.0,2.0]},
            {"name":"middle\tcolumn\rname","type":"value","values":[5.0,6.0]}
        ]
    });
    std::fs::write(&input, serde_json::to_vec(&original).unwrap()).unwrap();
    for format in ["csv", "tsv"] {
        let intermediate = directory.join(format!("output.{format}"));
        let recovered = directory.join(format!("{format}-recovered.json"));
        success(convert(&input, &intermediate, format));
        success(convert(&intermediate, &recovered, "json"));
        let actual = common::read_json(&recovered);
        assert_eq!(actual["scale"]["name"], original["scale"]["name"]);
        assert_eq!(actual["scale"]["values"], original["scale"]["values"]);
        assert_eq!(actual["signals"], original["signals"]);
        success(
            Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "compare"])
                .arg(&input)
                .arg(&intermediate)
                .output()
                .unwrap(),
        );
    }
}

#[test]
fn operating_point_reports_preserve_quoted_names_and_admit_values_incrementally() {
    let directory = common::test_dir("quoted-op-report");
    let input = directory.join("input.csv");
    let output = directory.join("output.json");
    let first = "signal,value\r\n\" A\r\nB \"\"C\"\" \" \t,1.25\r\n";
    std::fs::write(&input, first).unwrap();
    success(convert(&input, &output, "json"));
    let actual = common::read_json(&output);
    assert_eq!(actual["signals"][0]["name"], " A\r\nB \"C\" ");
    assert_eq!(actual["signals"][0]["values"], serde_json::json!([1.25]));
    std::fs::write(&input, format!("{first}V(second),2\nV(third),3\n")).unwrap();
    for resource in ["max_external_data_values", "max_result_values"] {
        let config = directory.join("config.toml");
        std::fs::write(&config, format!("[resources]\n{resource}=2\n")).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .arg("--config")
            .arg(&config)
            .args(["--quiet", "--error-format", "json", "convert"])
            .arg(&input)
            .arg(&output)
            .args(["--to", "json"])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(75), "{result:?}");
        let error: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
        assert_eq!(error["error"]["requested"], 3);
        assert_eq!(error["error"]["limit"], 2);
        assert_eq!(common::read_json(&output), actual);
    }
}

#[test]
fn malformed_quoting_and_data_keep_outputs_and_report_physical_rows() {
    let directory = common::test_dir("malformed-records");
    let input = directory.join("input.csv");
    let output = directory.join("output.json");
    let golden = directory.join("golden.csv");
    std::fs::write(&output, "prior output").unwrap();
    std::fs::write(&golden, "prior golden").unwrap();
    for (source, diagnostic) in [
        ("time,\"V(out)\"junk\n0,1\n", "after closing quote"),
        ("time,V(a\"b)\n0,1\n", "quote in an unquoted field"),
        ("time,\"unfinished\n0,1\n", "unterminated quoted field"),
        ("\n\n\"time\naxis\",V(out)\n\n0,nope\n", "row 6"),
    ] {
        std::fs::write(&input, source).unwrap();
        let result = convert(&input, &output, "json");
        assert!(!result.status.success());
        assert!(
            String::from_utf8_lossy(&result.stderr).contains(diagnostic),
            "{result:?}"
        );
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "compare"])
            .arg(&input)
            .arg(&golden)
            .arg("--bless")
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(
            String::from_utf8_lossy(&result.stderr).contains(diagnostic),
            "{result:?}"
        );
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "prior output");
        assert_eq!(std::fs::read_to_string(&golden).unwrap(), "prior golden");
    }
}
