mod common;

use common::{read_json, test_dir};
use serde_json::Value;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json"])
        .args(args)
        .output()
        .unwrap()
}

fn run_document(dir: &Path, family: &str) -> PathBuf {
    let deck = dir.join(format!("{family}.cir"));
    let output = dir.join(format!("{family}.json"));
    let source = match family {
        "pac" => {
            "* PAC qualifiers\nV1 in 0 SIN(0 1 1k) AC 1\nR1 in out 1k\nC1 out 0 1u\n.HB 1k\n.PAC LIN 2 100 200 INPUT=V1 OUT=V(out) MAXSIDEBAND=1\n.END\n"
        }
        "pxf" => {
            "* PXF qualifier\nV1 in 0 SIN(0 1 1k) AC 1\nE1 drive 0 in 0 2\nR1 drive out 1k\nC1 out 0 1u\n.HB 1k\n.PXF DEC 3 10 10k INPUT=V1 OUT=V(out) INPUTSIDEBAND=0 OUTSIDEBAND=0 MAXSIDEBAND=1\n.END\n"
        }
        "disto" => {
            "* distortion qualifiers\nV1 out 0 DC 0.5 DISTOF1 1m 0 DISTOF2 1m 20\nD1 out 0 DMOD\n.model DMOD D(IS=1e-12 N=1 CJO=0 TT=0)\n.DISTO LIN 2 1k 2k 0.8\n.END\n"
        }
        _ => unreachable!(),
    };
    std::fs::write(&deck, source).unwrap();
    let result = cli(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        "-f",
        "json",
    ]);
    assert!(result.status.success(), "{family}: {result:?}");
    if family == "disto" {
        output
    } else {
        dir.join(format!("{family}.{family}-001.json"))
    }
}

fn qualified_name(signal: &Value) -> String {
    let name = signal["descriptor"]["displayName"].as_str().unwrap();
    let qualifier = &signal["qualifier"];
    match qualifier["kind"].as_str() {
        Some("pac-sideband") => format!("{name}:sb{}", qualifier["sideband"]),
        Some("distortion-fundamental") => {
            format!("peak({}:{name})", qualifier["tone"].as_str().unwrap())
        }
        Some("distortion-product") => {
            let label = match qualifier["product"].as_str().unwrap() {
                "second-harmonic" => "2f1",
                "third-harmonic" => "3f1",
                "sum" => "f1+f2",
                "difference" => "f1-f2",
                "third-order-difference" => "2f1-f2",
                other => panic!("{other}"),
            };
            format!("peak({label}:{name})")
        }
        Some("pxf-conversion") => {
            format!("{name}:sb{}->sb{}", qualifier["input"], qualifier["output"])
        }
        None => name.to_string(),
        other => panic!("{other:?}"),
    }
}

#[test]
fn qualified_responses_remain_distinct_through_every_table_format() {
    let dir = test_dir("qualified_roundtrip");
    for family in ["pac", "disto", "pxf"] {
        let input = run_document(&dir, family);
        let document = read_json(&input);
        for format in ["csv", "tsv", "ascii", "raw", "hdf5", "json"] {
            let flat = dir.join(format!("flat-{family}.{format}"));
            let readback = dir.join(format!("readback-{family}-{format}.json"));
            for (input, output, format) in [(&input, &flat, format), (&flat, &readback, "json")] {
                let result = cli(&[
                    "convert",
                    input.to_str().unwrap(),
                    output.to_str().unwrap(),
                    "--to",
                    format,
                ]);
                assert!(result.status.success(), "{family}, {format}: {result:?}");
            }
            let table = read_json(&readback);
            let columns = table["signals"].as_array().unwrap();
            let names = columns
                .iter()
                .map(|column| column["name"].as_str().unwrap())
                .collect::<HashSet<_>>();
            assert_eq!(
                names.len(),
                columns.len(),
                "{family}, {format}: duplicate names"
            );
            for source in document["signals"].as_array().unwrap() {
                let name = qualified_name(source);
                let column = columns
                    .iter()
                    .find(|column| column["name"] == name)
                    .unwrap_or_else(|| panic!("{family}, {format}: missing {name}"));
                for (index, sample) in source["values"]["samples"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    if let Some(real) = sample.get("real") {
                        assert_eq!(&column["real"][index], real);
                        assert_eq!(column["imag"][index], sample["imaginary"]);
                    } else {
                        assert_eq!(
                            column.get("values").or_else(|| column.get("real")).unwrap()[index],
                            *sample
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn comparison_checks_each_qualified_response_independently() {
    let dir = test_dir("qualified_compare");
    for family in ["pac", "disto", "pxf"] {
        let input = run_document(&dir, family);
        let original = read_json(&input);
        let result = cli(&[
            "compare",
            input.to_str().unwrap(),
            input.to_str().unwrap(),
            "--json",
        ]);
        assert!(result.status.success(), "{family}: {result:?}");
        for (index, signal) in original["signals"].as_array().unwrap().iter().enumerate() {
            if signal["qualifier"].is_null() {
                continue;
            }
            let mut changed = original.clone();
            changed["signals"][index]["values"]["samples"][0]["real"] = 1234.0.into();
            let other = dir.join("changed.json");
            std::fs::write(&other, serde_json::to_vec(&changed).unwrap()).unwrap();
            let result = cli(&[
                "compare",
                input.to_str().unwrap(),
                other.to_str().unwrap(),
                "--json",
            ]);
            assert_eq!(result.status.code(), Some(3), "{family}: {result:?}");
            let report: Value = serde_json::from_slice(&result.stdout).unwrap();
            assert_eq!(report["comparison_passed"], false, "{report}");
            assert!(report["num_differences"].as_u64().unwrap() > 0, "{report}");
        }
    }
}

#[test]
fn literal_names_cannot_overwrite_qualified_response_identity() {
    let dir = test_dir("qualified_collision");
    let input = run_document(&dir, "pac");
    let mut document = read_json(&input);
    let name = qualified_name(&document["signals"][0]);
    let mut literal = document["signals"][0].clone();
    literal["qualifier"] = Value::Null;
    literal["descriptor"]["canonicalName"] = name.to_ascii_lowercase().into();
    literal["descriptor"]["displayName"] = name.into();
    document["signals"].as_array_mut().unwrap().push(literal);
    std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
    let output = dir.join("protected.csv");
    std::fs::write(&output, "predecessor").unwrap();
    let result = cli(&[
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        "csv",
    ]);
    assert!(!result.status.success(), "{result:?}");
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("duplicate column"),
        "{result:?}"
    );
    assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
}
