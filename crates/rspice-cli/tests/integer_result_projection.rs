mod common;

use serde_json::{Value, json};
use std::path::Path;
use std::process::Command;

fn document(dir: &Path) -> Value {
    let deck = dir.join("deck.cir");
    let output = dir.join("template.json");
    std::fs::write(&deck, "integers\nV1 n 0 1\nR1 n 0 1k\n.op\n.end\n").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .args(["-f", "json", "-o"])
        .arg(&output)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    common::read_json(&output)
}

fn set_axis(document: &mut Value, value: i64) {
    document["axes"] = json!([{
        "name":"sample", "displayName":"sample", "kind":"index",
        "unit":{"unit":"dimensionless"},
        "values":{"representation":"integer", "values":[value]}
    }]);
}

#[test]
fn unrepresentable_integer_coordinates_cannot_be_rounded_or_blessed() {
    let dir = common::test_dir("integer-axis-admission");
    let mut data = document(&dir);
    let source = dir.join("source.json");
    let golden = dir.join("golden.json");
    set_axis(&mut data, 1_i64 << 53);
    let original = data.to_string();
    std::fs::write(&golden, &original).unwrap();
    for value in [
        (1_i64 << 53) + 1,
        -(1_i64 << 53) - 1,
        i64::MAX,
        i64::MIN + 1,
    ] {
        set_axis(&mut data, value);
        std::fs::write(&source, data.to_string()).unwrap();
        for (format, ext) in [
            ("csv", "csv"),
            ("json", "json"),
            ("raw", "raw"),
            ("hdf5", "h5"),
        ] {
            let destination = dir.join(format!("result.{ext}"));
            std::fs::write(&destination, "previous artifact").unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "convert"])
                .arg(&source)
                .arg(&destination)
                .args(["--to", format])
                .output()
                .unwrap();
            assert!(!output.status.success(), "{value}/{format}: {output:?}");
            assert!(
                String::from_utf8_lossy(&output.stderr).contains("represented exactly"),
                "{output:?}"
            );
            assert_eq!(
                std::fs::read_to_string(destination).unwrap(),
                "previous artifact"
            );
        }
        for bless in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command
                .args(["--quiet", "compare"])
                .arg(&source)
                .arg(&golden)
                .args(["--abstol", "0", "--reltol", "0"]);
            if bless {
                command.arg("--bless");
            }
            let output = command.output().unwrap();
            assert!(!output.status.success(), "{value}: {output:?}");
            assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
        }
    }
}

#[test]
fn exactly_representable_large_integer_axes_and_scalars_are_supported() {
    let dir = common::test_dir("exact-integer-projection");
    let mut data = document(&dir);
    let source = dir.join("source.json");
    let output = dir.join("result.json");
    for value in [
        i64::MIN,
        -(1_i64 << 60),
        (1_i64 << 53) + 2,
        1_i64 << 60,
        i64::MAX - 1023,
    ] {
        set_axis(&mut data, value);
        let count = u64::MAX - 2047;
        data["scalars"] = json!([
            {"name":"integer", "displayName":"integer", "unit":null, "value":{"representation":"integer", "value":value}},
            {"name":"count", "displayName":"count", "unit":null, "value":{"representation":"count", "value":count}}
        ]);
        std::fs::write(&source, data.to_string()).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&source)
            .arg(&output)
            .args(["--to", "json"])
            .output()
            .unwrap();
        assert!(result.status.success(), "{value}: {result:?}");
        let table = common::read_json(&output);
        assert_eq!(
            table["scale"]["values"][0].as_f64().unwrap() as i128,
            i128::from(value)
        );
        for (name, expected) in [("integer", i128::from(value)), ("count", i128::from(count))] {
            let signal = table["signals"]
                .as_array()
                .unwrap()
                .iter()
                .find(|signal| signal["name"] == name)
                .unwrap();
            assert_eq!(signal["values"][0].as_f64().unwrap() as i128, expected);
        }
    }
}
