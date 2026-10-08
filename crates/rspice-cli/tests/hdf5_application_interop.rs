//! CLI HDF5 files must retain their meaning in the application's decoder.
mod common;

use rspice_formats::hdf5::{Hdf5Limits, decode_hdf5};
use serde_json::json;
use std::process::Command;

#[test]
fn native_sections_preserve_coordinate_identity_through_cli_conversion() {
    use rspice_core::io::{Hdf5Attribute, Hdf5Column, Hdf5Coordinate, Hdf5Document, Hdf5Table};

    for (family, label, unit, quantity) in [
        ("transient", "Elapsed time", Some("ns"), "time"),
        ("transient", "Clock", None, "time"),
        ("dc_sweep", "bias", Some("A"), "value"),
        ("dc_sweep", "ambient", Some("K"), "value"),
        ("dc_sweep", "control", None, "value"),
        ("dc_sweep", "time", None, "value"),
        ("dc_sweep", "frequency", Some("A"), "value"),
        ("ac", "Test frequency", Some("MHz"), "frequency"),
        ("ac", "Tone", None, "frequency"),
    ] {
        let mut document = Hdf5Document::new("Retained coordinate");
        document
            .add_table(&Hdf5Table {
                group: "selected".into(),
                section_type: family.into(),
                coordinate: if family == "ac" {
                    Hdf5Coordinate::Frequency(vec![1.0, 2.0])
                } else {
                    Hdf5Coordinate::Independent {
                        name: label.into(),
                        values: vec![1.0, 2.0],
                    }
                },
                columns: vec![if family == "ac" {
                    Hdf5Column::Complex {
                        name: "out".into(),
                        unit: None,
                        real: vec![-0.0, 4.0],
                        imag: vec![2.0, -1.0],
                    }
                } else {
                    Hdf5Column::Real {
                        name: "out".into(),
                        quantity: "value".into(),
                        unit: None,
                        values: vec![-0.0, 4.0],
                    }
                }],
            })
            .unwrap();
        let group = document.groups.last_mut().unwrap();
        if family == "ac" {
            group.set_attr("independent_name", Hdf5Attribute::Text(label.into()));
        }
        if let Some(unit) = unit {
            group.set_attr("coordinate_unit", Hdf5Attribute::Text(unit.into()));
        }
        let directory = common::test_dir("native_hdf5_coordinate");
        let source = directory.join("source.h5");
        let mut bytes = Vec::new();
        rspice_core::io::write_hdf5(&mut bytes, &document).unwrap();
        std::fs::write(&source, &bytes).unwrap();
        let json_path = directory.join("converted.json");
        let roundtrip = directory.join("converted.h5");
        let restored = directory.join("restored.json");
        for (input, output, format) in [
            (&source, &json_path, "json"),
            (&source, &roundtrip, "hdf5"),
            (&roundtrip, &restored, "json"),
        ] {
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "convert"])
                .arg(input)
                .arg(output)
                .args(["--to", format])
                .output()
                .unwrap();
            assert!(result.status.success(), "{family} {label}: {result:?}");
        }
        let limits = Hdf5Limits {
            max_columns: 10,
            max_values: 100,
            coordinate_names: &["time", "frequency", "x"],
        };
        let before = decode_hdf5(&bytes, limits, "hdf5").unwrap();
        let after = decode_hdf5(&std::fs::read(&roundtrip).unwrap(), limits, "hdf5").unwrap();
        assert_eq!(before.coordinate_name, label);
        assert_eq!(after.coordinate_name, label);
        assert_eq!(before.domain, after.domain);
        assert_eq!(before.coordinate_unit, after.coordinate_unit);
        assert_eq!(before.coordinate, after.coordinate);
        let original: serde_json::Value =
            serde_json::from_slice(&std::fs::read(json_path).unwrap()).unwrap();
        let decoded: serde_json::Value =
            serde_json::from_slice(&std::fs::read(restored).unwrap()).unwrap();
        assert_eq!(decoded, original);
        assert_eq!(decoded["scale"]["name"], label);
        assert_eq!(decoded["scale"]["unit"], json!(unit), "{family} {label}");
        assert_eq!(decoded["scale"]["type"], quantity);
        assert_eq!(decoded["scale"]["values"], json!([1.0, 2.0]));
        let values = if family == "ac" { "real" } else { "values" };
        assert_eq!(
            decoded["signals"][0][values][0].as_f64().unwrap().to_bits(),
            (-0.0_f64).to_bits()
        );
        if family == "ac" {
            assert_eq!(decoded["signals"][0]["imag"], json!([2.0, -1.0]));
        }
    }
}

fn decode_export(document: serde_json::Value) -> rspice_formats::numeric::DecodedNumericDataset {
    let directory = common::test_dir("hdf5_application_interop");
    let source = directory.join("source.json");
    let output = directory.join("result.h5");
    std::fs::write(&source, document.to_string()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&source)
        .arg(&output)
        .args(["--to", "hdf5"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    decode_hdf5(
        &std::fs::read(output).unwrap(),
        Hdf5Limits {
            max_columns: 32,
            max_values: 1000,
            coordinate_names: &["time", "frequency", "x"],
        },
        "hdf5",
    )
    .unwrap()
}

#[test]
fn nullable_hdf5_columns_do_not_become_zero_measurements_or_extra_signals() {
    let decoded = decode_export(json!({
        "scale":{"name":"time", "type":"time", "unit":"s", "values":[0.0,1.0,2.0]},
        "signals":[
            {"name":"voltage", "type":"voltage", "unit":"V", "values":[1.0,null,-0.0]},
            {"name":"transfer", "type":"value", "unit":"1", "real":[2.0,null,3.0], "imag":[-4.0,null,5.0]},
            {"name":"Valid(voltage)", "type":"current", "unit":"A", "values":[7.0,8.0,9.0]}
        ]
    }));
    assert_eq!(
        decoded.signals.len(),
        3,
        "validity is not a measured signal"
    );
    assert_eq!(decoded.signals[0].name, "voltage");
    assert_eq!(decoded.signals[0].unit.as_deref(), Some("V"));
    assert!(decoded.signals[0].real[1].is_nan());
    assert_eq!(decoded.signals[0].real[2].to_bits(), (-0.0_f64).to_bits());
    assert_eq!(decoded.signals[1].name, "transfer");
    assert_eq!(decoded.signals[1].real[0], 2.0);
    assert!(decoded.signals[1].real[1].is_nan());
    let imag = decoded.signals[1].imag.as_ref().unwrap();
    assert_eq!(imag[0], -4.0);
    assert!(imag[1].is_nan());
    assert_eq!(decoded.signals[2].name, "Valid(voltage)");
    assert_eq!(decoded.signals[2].real, [7.0, 8.0, 9.0]);
}

#[test]
fn hdf5_coordinate_units_are_normalized_before_application_use() {
    for (kind, unit, canonical, values, expected) in [
        ("time", "ns", "s", [0.0, 2.0], [0.0, 2e-9]),
        ("frequency", "MHz", "Hz", [1.0, 2.0], [1e6, 2e6]),
        ("current", "mA", "A", [-1.0, 1.0], [-1e-3, 1e-3]),
        ("temperature", "degC", "K", [0.0, 25.0], [273.15, 298.15]),
    ] {
        let decoded = decode_export(json!({
            "scale":{"name":kind, "type":kind, "unit":unit, "values":values},
            "signals":[{"name":"out", "unit":"mV", "values":[3.0,4.0]}]
        }));
        assert_eq!(decoded.coordinate, expected, "{kind} [{unit}]");
        assert_eq!(decoded.coordinate_unit.as_deref(), Some(canonical));
        assert_eq!(decoded.signals[0].real, [3.0, 4.0]);
        assert_eq!(decoded.signals[0].unit.as_deref(), Some("mV"));
    }
}

#[test]
fn mixed_complex_tables_retain_their_domain_and_signal_representation() {
    for (analysis, coordinate_type, domain) in [
        (
            "transient",
            "time",
            rspice_formats::WaveformDomain::Transient,
        ),
        ("dc_sweep", "value", rspice_formats::WaveformDomain::DcSweep),
        ("ac", "frequency", rspice_formats::WaveformDomain::Ac),
    ] {
        let decoded = decode_export(json!({
            "analysis": analysis,
            "scale": {"name":"axis", "type":coordinate_type, "values":[1.0, 2.0]},
            "signals": [
                {"name":"out", "unit":"mA", "real":[-0.0, 4.0], "imag":[2.0, -1.0]},
                {"name":"Re(out)", "values":[3.0, -0.0]}
            ]
        }));
        assert_eq!(decoded.domain, domain);
        assert_eq!(decoded.coordinate_name, "axis");
        assert_eq!(decoded.signals.len(), 2);
        assert_eq!(decoded.signals[0].real[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(
            decoded.signals[0].imag.as_deref(),
            Some([2.0, -1.0].as_slice())
        );
        assert_eq!(decoded.signals[0].unit.as_deref(), Some("mA"));
        assert!(decoded.signals[1].imag.is_none());
        assert_eq!(decoded.signals[1].real[1].to_bits(), (-0.0_f64).to_bits());
    }
}
