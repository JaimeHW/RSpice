//! Delimited headers must not turn separate scalar results into complex pairs.
mod common;

use common::{read_json, test_dir};
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

#[test]
fn scalar_component_names_keep_their_identity_and_availability() {
    let dir = test_dir("delimited_scalar_components");
    let source = dir.join("source.json");
    let original = serde_json::json!({
        "analysis":"table", "plot_name":"Independent scalar components",
        "scale":{"name":"frequency", "type":"frequency", "values":[1.0,2.0,3.0]},
        "signals":[
            {"name":"Re(x)", "type":"value", "values":[1.0,null,3.0]},
            {"name":"Im(x)", "type":"value", "values":[4.0,5.0,null]},
            {"name":"y", "type":"value", "real":[6.0,null,8.0], "imag":[9.0,null,11.0]},
            {"name":"Re(a,\"b\")", "type":"value", "values":[12.0,13.0,14.0]},
            {"name":"Im(a,\"b\")", "type":"value", "values":[15.0,16.0,17.0]}
        ]
    });
    std::fs::write(&source, serde_json::to_vec(&original).unwrap()).unwrap();
    for format in ["csv", "tsv"] {
        let encoded = dir.join(format!("result.{format}"));
        let decoded = dir.join(format!("result.{format}.json"));
        let result = convert(&source, &encoded, format, &[]);
        assert!(result.status.success(), "{result:?}");
        let result = convert(&encoded, &decoded, "json", &[]);
        assert!(result.status.success(), "{result:?}");
        let recovered = read_json(&decoded);
        assert_eq!(recovered["scale"], original["scale"]);
        assert_eq!(recovered["signals"], original["signals"], "{format}");
        let selected = dir.join("selected.json");
        let result = convert(
            &encoded,
            &selected,
            "json",
            &["--variables", "Re(x)", "--start", "2"],
        );
        assert!(result.status.success(), "{result:?}");
        let selected = read_json(&selected);
        assert_eq!(selected["signals"].as_array().unwrap().len(), 1);
        assert_eq!(
            selected["signals"][0]["values"],
            serde_json::json!([null, 3.0])
        );
    }
}

#[test]
fn explicit_layout_preserves_real_pairs_and_validates_complex_pairs() {
    let dir = test_dir("delimited_explicit_layout");
    let source = dir.join("source.csv");
    let output = dir.join("decoded.json");
    for (layout, count) in [("real,real", 2), ("complex_real,complex_imag", 1)] {
        std::fs::write(
            &source,
            format!("time,Re(x),Im(x)\n0,1,2\n# RSpiceTableLayoutV1,{layout}\n"),
        )
        .unwrap();
        let result = convert(&source, &output, "json", &[]);
        assert!(result.status.success(), "{result:?}");
        assert_eq!(
            read_json(&output)["signals"].as_array().unwrap().len(),
            count
        );
    }
}

#[test]
fn malformed_layout_cannot_replace_a_destination() {
    let dir = test_dir("delimited_invalid_layout");
    let source = dir.join("source.csv");
    let output = dir.join("decoded.json");
    for footer in [
        "# RSpiceTableLayoutV2,real,real\n",
        "# RSpiceTableLayoutV1,real\n",
        "# RSpiceTableLayoutV1,unknown,real\n",
        "# RSpiceTableLayoutV1,complex_imag,complex_real\n",
        "# RSpiceTableLayoutV1,complex_real,real\n",
        "# RSpiceTableLayoutV1,real,real\n1,3,4\n",
        "# RSpiceTableLayoutV1,real,real\n# RSpiceTableLayoutV1,real,real\n",
    ] {
        std::fs::write(&source, format!("time,Re(x),Im(x)\n0,1,2\n{footer}")).unwrap();
        std::fs::write(&output, "previous result").unwrap();
        let result = convert(&source, &output, "json", &[]);
        assert!(!result.status.success(), "{footer}: {result:?}");
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "previous result");
    }
}

#[test]
fn ordinary_complex_exports_keep_the_plain_delimited_layout() {
    let dir = test_dir("delimited_plain_complex");
    let source = dir.join("source.json");
    let output = dir.join("encoded.csv");
    std::fs::write(&source, r#"{"scale":{"name":"frequency","values":[1]},"signals":[{"name":"x","real":[2],"imag":[3]}]}"#).unwrap();
    let result = convert(&source, &output, "csv", &[]);
    assert!(result.status.success(), "{result:?}");
    let text = std::fs::read_to_string(&output).unwrap();
    assert_eq!(text.lines().count(), 2);
    assert!(text.starts_with("frequency,Re(x),Im(x)\n"));
}

#[test]
fn dense_scalar_component_exports_remain_readable_by_the_application_decoder() {
    use rspice_formats::delimited::{DelimitedReadLimits, decode_delimited_waveforms};
    let dir = test_dir("delimited_application_layout");
    let source = dir.join("source.json");
    let original = serde_json::json!({
        "scale":{"name":"time", "type":"time", "values":[0.0]},
        "signals":[
            {"name":"Re(x)", "type":"value", "values":[1.0]},
            {"name":"Im(x)", "type":"value", "values":[2.0]},
            {"name":"y", "type":"value", "real":[3.0], "imag":[4.0]}
        ]
    });
    std::fs::write(&source, serde_json::to_vec(&original).unwrap()).unwrap();
    for (format, delimiter) in [("csv", b','), ("tsv", b'\t')] {
        let output = dir.join(format!("encoded.{format}"));
        let result = convert(&source, &output, format, &[]);
        assert!(result.status.success(), "{result:?}");
        let text = std::fs::read_to_string(&output).unwrap();
        let decoded = decode_delimited_waveforms(
            &text,
            delimiter,
            DelimitedReadLimits {
                max_columns: 5,
                max_rows: 1,
                max_header_bytes: 32,
                min_rows: 1,
            },
        )
        .unwrap();
        assert_eq!(decoded.coordinate, [0.0]);
        assert_eq!(
            decoded.signal_values,
            [vec![1.0], vec![2.0], vec![3.0], vec![4.0]]
        );
        assert_eq!(
            decoded
                .columns
                .iter()
                .map(|header| header.name.as_str())
                .collect::<Vec<_>>(),
            ["time", "Re(x)", "Im(x)", "Re(y)", "Im(y)"]
        );
        let recovered = dir.join("recovered.json");
        let result = convert(&output, &recovered, "json", &[]);
        assert!(result.status.success(), "{result:?}");
        assert_eq!(read_json(&recovered)["signals"], original["signals"]);
    }
}

#[test]
fn layout_records_do_not_consume_sample_limits_or_bypass_them() {
    let dir = test_dir("delimited_layout_limits");
    let source = dir.join("source.csv");
    let output = dir.join("result.json");
    let config = dir.join("config.toml");
    for resource in ["max_external_data_values", "max_result_values"] {
        std::fs::write(&config, format!("[resources]\n{resource}=3\n")).unwrap();
        for (data, success) in [("0,1,2\n", true), ("0,1,2\n1,3,4\n", false)] {
            std::fs::write(
                &source,
                format!("time,Re(x),Im(x)\n{data}# RSpiceTableLayoutV1,real,real\n"),
            )
            .unwrap();
            std::fs::write(&output, "previous result").unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .arg("--config")
                .arg(&config)
                .args(["--quiet", "convert"])
                .arg(&source)
                .arg(&output)
                .args(["--to", "json"])
                .output()
                .unwrap();
            assert_eq!(result.status.success(), success, "{resource}: {result:?}");
            if !success {
                assert_eq!(result.status.code(), Some(75), "{result:?}");
                assert_eq!(std::fs::read_to_string(&output).unwrap(), "previous result");
            }
        }
    }
}
