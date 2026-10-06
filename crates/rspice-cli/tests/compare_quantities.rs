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
