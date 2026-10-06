mod common;
use serde_json::json;
use std::process::Command;

#[test]
fn comparison_retains_quantity_and_coordinate_types_in_every_typed_format() {
    let dir = common::test_dir("quantities");
    let document = |coordinate: &str, quantity: &str| {
        json!({
            "analysis":"converted", "scale":{"name":"x","type":coordinate,"values":[0,1]},
            "signals":[{"name":"probe","type":quantity,"real":[1,2],"imag":[0,1]}]
        })
    };
    for (name, coordinate, quantity) in [
        ("golden", "time", "voltage"),
        ("current", "time", "current"),
        ("frequency", "frequency", "voltage"),
        ("alias", "s", "V"),
        ("conductance", "S", "voltage"),
    ] {
        std::fs::write(
            dir.join(format!("{name}.json")),
            document(coordinate, quantity).to_string(),
        )
        .unwrap();
    }
    for (format, extension) in [("json", "json"), ("raw", "raw"), ("hdf5", "h5")] {
        for name in ["golden", "current", "frequency", "alias", "conductance"] {
            if format != "json" {
                let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "convert"])
                    .arg(dir.join(format!("{name}.json")))
                    .arg(dir.join(format!("{name}.{extension}")))
                    .args(["--to", format])
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{output:?}");
            }
        }
        for (name, interpolate, expected) in [
            ("current", false, 3),
            ("frequency", false, 3),
            ("frequency", true, 3),
            ("alias", true, 0),
            ("conductance", true, 3),
        ] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command
                .args(["--quiet", "compare"])
                .arg(dir.join(format!("{name}.{extension}")))
                .arg(dir.join(format!("golden.{extension}")))
                .args(["--variables", "probe"]);
            if interpolate {
                command.arg("--interpolate");
            }
            let output = command.output().unwrap();
            assert_eq!(
                output.status.code(),
                Some(expected),
                "{format}/{name}: {output:?}"
            );
        }
    }
}

#[test]
fn legacy_untyped_columns_do_not_acquire_invented_voltage_units() {
    let dir = common::test_dir("legacy-quantity");
    std::fs::write(dir.join("legacy.csv"), "x,probe\n0,1\n1,2\n").unwrap();
    std::fs::write(dir.join("typed.json"), json!({"scale":{"name":"x","type":"frequency","values":[0,1]},"signals":[{"name":"probe","type":"resistance","values":[1,2]}]}).to_string()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "compare"])
        .arg(dir.join("legacy.csv"))
        .arg(dir.join("typed.json"))
        .arg("--interpolate")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn explicit_units_survive_conversion_and_cannot_silently_compare_equal() {
    let dir = common::test_dir("comparison-units");
    for (name, coordinate, unit) in [
        ("golden", "s", "mV"),
        ("volts", "s", "V"),
        ("megavolts", "s", "MV"),
        ("milliseconds", "ms", "mV"),
    ] {
        std::fs::write(
            dir.join(format!("{name}.json")),
            json!({
                "scale":{"name":"time", "type":"time", "unit":coordinate, "values":[0,1]},
                "signals":[
                    {"name":"probe", "type":"voltage", "unit":unit, "real":[1,2], "imag":[0,1]},
                    {"name":"bias", "type":"voltage", "unit":unit, "values":[3,4]}
                ]
            })
            .to_string(),
        )
        .unwrap();
    }
    for (format, extension) in [
        ("json", "json"),
        ("raw", "raw"),
        ("ascii", "ascii.raw"),
        ("hdf5", "h5"),
    ] {
        for name in ["golden", "volts", "megavolts", "milliseconds"] {
            if format != "json" {
                let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "convert"])
                    .arg(dir.join(format!("{name}.json")))
                    .arg(dir.join(format!("{name}.{extension}")))
                    .args(["--to", format])
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{output:?}");
            }
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "compare"])
                .arg(dir.join(format!("{name}.{extension}")))
                .arg(dir.join("golden.json"))
                .args(["--interpolate", "--variables", "probe"])
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(if name == "golden" { 0 } else { 3 }),
                "{format}/{name}: {output:?}"
            );
            if name == "golden" {
                let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                    .args(["--quiet", "convert"])
                    .arg(dir.join(format!("{name}.{extension}")))
                    .arg(dir.join("roundtrip.json"))
                    .args(["--to", "json"])
                    .output()
                    .unwrap();
                assert!(output.status.success(), "{output:?}");
                let roundtrip: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(dir.join("roundtrip.json")).unwrap())
                        .unwrap();
                assert_eq!(roundtrip["scale"]["unit"], "s");
                assert_eq!(roundtrip["signals"][0]["unit"], "mV");
                assert_eq!(roundtrip["signals"][1]["unit"], "mV");
                assert!(roundtrip["signals"][1].get("values").is_some());
            }
        }
    }
}

#[test]
fn typed_scalar_units_survive_flattening() {
    let dir = common::test_dir("scalar-units");
    let deck = dir.join("deck.cir");
    let result = dir.join("result.json");
    let flat = dir.join("flat.json");
    std::fs::write(
        &deck,
        "units\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) V1\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .args(["-f", "json", "-o"])
        .arg(&result)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(result)
        .arg(&flat)
        .args(["--to", "json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let table: serde_json::Value = serde_json::from_slice(&std::fs::read(flat).unwrap()).unwrap();
    let impedances: Vec<_> = table["signals"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|signal| signal["name"].as_str().unwrap().contains("impedance"))
        .collect();
    assert_eq!(impedances.len(), 2);
    for signal in impedances {
        assert_eq!(signal["unit"], "ohm");
        assert_eq!(signal["type"], "resistance");
    }
}
