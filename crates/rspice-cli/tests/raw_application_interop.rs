//! CLI RAW output must carry the same signal meaning into the application.
mod common;

use common::test_dir;
use rspice_formats::{WaveformDomain, spice_raw::decode_spice_raw};
use std::path::Path;
use std::process::Command;

fn convert(source: &Path, output: &Path, format: &str) {
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(output)
        .args(["--to", format])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
}

#[test]
fn raw_exports_retain_explicit_signal_units_in_the_application_reader() {
    let dir = test_dir("raw_application_units");
    let source = dir.join("source.json");
    let data = serde_json::json!({
        "plot_name":"Exact units",
        "scale":{"name":"frequency", "type":"frequency", "unit":"Hz", "values":[1.0,2.0]},
        "signals":[
            {"name":" voltage ", "type":"voltage", "unit":"mV", "values":[1.0,-0.0]},
            {"name":"transfer", "type":"value", "unit":"1", "real":[2.0,3.0], "imag":[-4.0,5.0]},
            {"name":"unstated", "type":"value", "values":[6.0,7.0]}
        ]
    });
    std::fs::write(&source, serde_json::to_vec(&data).unwrap()).unwrap();
    for format in ["raw", "ascii"] {
        let output = dir.join(format!("result.{format}"));
        convert(&source, &output, format);
        let decoded =
            decode_spice_raw(&std::fs::read(output).unwrap(), Default::default()).unwrap();
        assert_eq!(decoded.coordinate, [1.0, 2.0]);
        assert_eq!(decoded.signals.len(), 3);
        assert_eq!(decoded.signals[0].name, " voltage ");
        assert_eq!(decoded.signals[0].unit.as_deref(), Some("mV"));
        assert_eq!(decoded.signals[0].real[1].to_bits(), (-0.0_f64).to_bits());
        assert!(decoded.signals[0].imag.is_none());
        assert_eq!(decoded.signals[1].unit.as_deref(), Some("1"));
        assert_eq!(decoded.signals[1].real, [2.0, 3.0]);
        assert_eq!(
            decoded.signals[1].imag.as_deref(),
            Some([-4.0, 5.0].as_slice())
        );
        assert!(decoded.signals[2].unit.is_none());
    }
}

#[test]
fn nullable_raw_columns_are_samples_with_gaps_not_extra_validity_signals() {
    let dir = test_dir("raw_application_nullable");
    let source = dir.join("source.json");
    let data = serde_json::json!({
        "plot_name":"Nullable samples",
        "scale":{"name":"time", "type":"time", "unit":"s", "values":[0.0,1.0,2.0]},
        "signals":[
            {"name":"voltage", "type":"voltage", "unit":"V", "values":[1.0,null,-0.0]},
            {"name":"transfer", "type":"value", "unit":"1", "real":[2.0,null,3.0], "imag":[-4.0,null,5.0]},
            {"name":"Valid(voltage)", "type":"current", "unit":"A", "values":[7.0,8.0,9.0]}
        ]
    });
    std::fs::write(&source, serde_json::to_vec(&data).unwrap()).unwrap();
    for format in ["raw", "ascii"] {
        let output = dir.join(format!("result.{format}"));
        convert(&source, &output, format);
        let decoded =
            decode_spice_raw(&std::fs::read(output).unwrap(), Default::default()).unwrap();
        assert_eq!(decoded.coordinate, [0.0, 1.0, 2.0]);
        assert_eq!(
            decoded.signals.len(),
            3,
            "{format}: validity is not a measured signal"
        );
        assert_eq!(decoded.signals[0].name, "voltage");
        assert_eq!(decoded.signals[1].name, "transfer");
        assert_eq!(decoded.signals[2].name, "Valid(voltage)");
        assert!(decoded.signals[0].real[1].is_nan());
        assert_eq!(decoded.signals[0].real[2].to_bits(), (-0.0_f64).to_bits());
        assert!(decoded.signals[1].real[1].is_nan());
        assert!(decoded.signals[1].imag.as_ref().unwrap()[1].is_nan());
        assert_eq!(decoded.signals[2].real, [7.0, 8.0, 9.0]);
    }
}

#[test]
fn raw_coordinates_use_canonical_physical_values_in_the_application_reader() {
    let dir = test_dir("raw_application_coordinate_units");
    let source = dir.join("source.json");
    for (name, kind, unit, canonical, values, expected) in [
        ("time", "time", "ns", "s", [0.0, 2.0], [0.0, 2e-9]),
        (
            "frequency",
            "frequency",
            "MHz",
            "Hz",
            [1.0, 2.0],
            [1e6, 2e6],
        ),
        ("bias", "current", "mA", "A", [-1.0, 1.0], [-1e-3, 1e-3]),
        (
            "temperature",
            "temperature",
            "degC",
            "K",
            [0.0, 25.0],
            [273.15, 298.15],
        ),
    ] {
        let data = serde_json::json!({"plot_name":"Coordinate units", "scale":{"name":name,"type":kind,"unit":unit,"values":values}, "signals":[{"name":"out","values":[3.0,4.0]}]});
        std::fs::write(&source, serde_json::to_vec(&data).unwrap()).unwrap();
        for format in ["raw", "ascii"] {
            let output = dir.join(format!("result.{format}"));
            convert(&source, &output, format);
            let decoded =
                decode_spice_raw(&std::fs::read(output).unwrap(), Default::default()).unwrap();
            assert_eq!(decoded.coordinate, expected, "{name} [{unit}], {format}");
            assert_eq!(decoded.coordinate_unit.as_deref(), Some(canonical));
            assert_eq!(decoded.signals[0].real, [3.0, 4.0]);
        }
    }
}

#[test]
fn coordinate_meaning_precedes_plot_titles_and_signal_complexity() {
    let dir = test_dir("raw_application_domain");
    let source = dir.join("source.json");
    for (name, kind, plot, domain, values) in [
        (
            "time",
            "time",
            "AC package",
            WaveformDomain::Transient,
            [0.0, 1.0],
        ),
        (
            "point",
            "index",
            "AC Operating Point",
            WaveformDomain::DcSweep,
            [0.0, 1.0],
        ),
        (
            "frequency",
            "frequency",
            "Transient Analysis",
            WaveformDomain::Ac,
            [1.0, 2.0],
        ),
        (
            "bias",
            "voltage",
            "package transfer",
            WaveformDomain::DcSweep,
            [-1.0, 1.0],
        ),
        (
            "parameter",
            "value",
            "AC Analysis",
            WaveformDomain::DcSweep,
            [-1.0, 1.0],
        ),
    ] {
        for complex in [false, true] {
            let signal = if complex {
                serde_json::json!({"name":"x", "real":[1.0,2.0], "imag":[3.0,4.0]})
            } else {
                serde_json::json!({"name":"x", "values":[1.0,2.0]})
            };
            let data = serde_json::json!({"plot_name":plot, "scale":{"name":name,"type":kind,"values":values}, "signals":[signal]});
            std::fs::write(&source, serde_json::to_vec(&data).unwrap()).unwrap();
            for format in ["raw", "ascii"] {
                let output = dir.join(format!("result.{format}"));
                convert(&source, &output, format);
                let decoded =
                    decode_spice_raw(&std::fs::read(output).unwrap(), Default::default()).unwrap();
                assert_eq!(decoded.domain, domain, "{plot}, complex={complex}");
                assert_eq!(decoded.coordinate_name, name);
                assert_eq!(decoded.coordinate, values);
                assert_eq!(decoded.signals.len(), 1);
            }
        }
    }
}

#[test]
fn scalar_raw_results_retain_the_first_signal_and_use_an_ordinal_axis() {
    for (plot, complex) in [
        ("DC Operating Point", false),
        ("AC Operating Point", true),
        ("Pole-Zero Analysis", true),
        ("Transfer Function", false),
        ("Integrated Noise - V^2 or A^2", false),
        ("Sensitivity Analysis", false),
    ] {
        let flags = if complex { "complex" } else { "real" };
        let values = if complex { "2,3 4,5" } else { "2 4" };
        let source = format!(
            "Title: scalars\nPlotname: {plot}\nFlags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 first voltage\n1 second current\nValues:\n0 {values}\n"
        );
        let decoded = decode_spice_raw(source.as_bytes(), Default::default()).unwrap();
        assert_eq!(decoded.coordinate_name, "point", "{plot}");
        assert_eq!(decoded.coordinate, [0.0]);
        assert_eq!(decoded.domain, WaveformDomain::DcSweep);
        assert_eq!(decoded.signals.len(), 2);
        assert_eq!(decoded.signals[0].name, "first");
        assert_eq!(decoded.signals[0].real, [2.0]);
        assert_eq!(
            decoded.signals[0].imag.as_deref(),
            complex.then_some([3.0].as_slice())
        );
    }
}

#[test]
fn waveform_coordinates_cannot_silently_discard_imaginary_values() {
    let source = b"Title: axis\nPlotname: AC Analysis\nFlags: complex\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 frequency frequency\n1 V(out) voltage\nValues:\n0 1,9 2,3\n";
    let error = decode_spice_raw(source, Default::default()).unwrap_err();
    assert!(
        error.to_string().contains("nonzero imaginary component"),
        "{error}"
    );
}

#[test]
fn multiple_raw_plots_cannot_be_imported_as_a_successful_partial_dataset() {
    let first = "Title: first\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 1\n1 1 2\n";
    let second = first
        .replace("Title: first", "Title: second")
        .replace("1 1 2", "1 1 3");
    let combined = format!("{first}{second}");
    let error = decode_spice_raw(combined.as_bytes(), Default::default())
        .expect_err("one dataset cannot silently discard a second plot");
    assert!(error.to_string().contains("2 plots"), "{error}");
    assert!(error.to_string().contains("--section"), "{error}");
    let broken = format!("{first}Title: incomplete second plot\n");
    assert!(decode_spice_raw(broken.as_bytes(), Default::default()).is_err());

    let directory = test_dir("raw_select_for_application");
    let source = directory.join("multiple.raw");
    let selected = directory.join("selected.raw");
    std::fs::write(&source, combined).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(&selected)
        .args(["--to", "raw", "--section", "2"])
        .output()
        .unwrap();
    assert!(result.status.success(), "{result:?}");
    let decoded = decode_spice_raw(&std::fs::read(selected).unwrap(), Default::default()).unwrap();
    assert_eq!(decoded.signals[0].real, [1.0, 3.0]);
}
