mod common;

use common::{read_json, test_dir};
use std::process::Command;

fn cli(args: &[&str]) {
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json"])
        .args(args)
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
}

fn check_formats(magnitude: f64, include_dc: bool) {
    let dir = test_dir("pac_formats");
    let deck = dir.join("network.cir");
    std::fs::write(&deck, format!("* PAC format contract\nV1 in 0 SIN(0 1 1k) AC 1\nR1 in out 1k\nC1 out 0 1u\n.HB 1k\n.PAC LIN 3 100 300 INPUT=V1 OUT=V(out) MAXSIDEBAND=1 PACMAG={magnitude} INCLUDEDC={}\n.END\n", if include_dc { "YES" } else { "NO" })).unwrap();
    let typed = dir.join("typed.json");
    cli(&[
        "run",
        deck.to_str().unwrap(),
        "-o",
        typed.to_str().unwrap(),
        "-f",
        "json",
    ]);
    let document = read_json(&dir.join("typed.pac-001.json"));
    let expected = document["signals"].as_array().unwrap();
    let bands = document["payload"]["sidebands"].as_array().unwrap();
    for format in ["csv", "tsv", "ascii", "raw", "hdf5"] {
        let output = dir.join(format!("output.{format}"));
        let pac = dir.join(format!("output.pac-001.{format}"));
        let converted = dir.join(format!("table-{format}.json"));
        cli(&[
            "run",
            deck.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
            "-f",
            format,
        ]);
        cli(&[
            "convert",
            pac.to_str().unwrap(),
            converted.to_str().unwrap(),
            "--to",
            "json",
        ]);
        let table = read_json(&converted);
        let actual = table["signals"].as_array().unwrap();
        assert_eq!(
            actual.len(),
            expected.len() + bands.len() + 1,
            "{format}: sideband selection differs from JSON"
        );
        let fundamental = actual
            .iter()
            .find(|column| column["name"] == "fundamental_frequency")
            .unwrap();
        assert_eq!(
            fundamental
                .get("values")
                .or_else(|| fundamental.get("real"))
                .unwrap(),
            &serde_json::json!([1000.0, 1000.0, 1000.0])
        );
        for band in bands {
            let name = format!("frequency(sb{})", band["sideband"]);
            let column = actual.iter().find(|column| column["name"] == name).unwrap();
            assert_eq!(
                column.get("values").or_else(|| column.get("real")).unwrap(),
                &band["absoluteFrequencies"]
            );
            if !matches!(format, "csv" | "tsv") {
                assert_eq!(column["unit"], "Hz");
            }
        }
        for signal in expected {
            let name = format!(
                "{}:sb{}",
                signal["descriptor"]["displayName"].as_str().unwrap(),
                signal["qualifier"]["sideband"]
            );
            let column = actual
                .iter()
                .find(|column| column["name"].as_str().unwrap().eq_ignore_ascii_case(&name))
                .unwrap();
            if !matches!(format, "csv" | "tsv") {
                assert_eq!(
                    column["unit"],
                    if signal["descriptor"]["kind"] == "voltage" {
                        "V"
                    } else {
                        "A"
                    },
                    "{format}: {name}"
                );
            }
            for (index, sample) in signal["values"]["samples"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
            {
                assert_eq!(column["real"][index], sample["real"], "{format}: {name}");
                assert_eq!(
                    column["imag"][index], sample["imaginary"],
                    "{format}: {name}"
                );
            }
        }
    }
}

#[test]
fn pac_drive_amplitude_and_physical_units_agree_across_formats() {
    check_formats(0.25, true);
}

#[test]
fn excluded_central_sideband_is_absent_in_every_format() {
    check_formats(1.0, false);
}
