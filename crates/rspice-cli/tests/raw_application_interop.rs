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

/// A noise rawfile is its spectrum followed by ngspice's one-point
/// "Integrated Noise" totals; the application imports the spectrum.
#[test]
fn a_noise_rawfile_imports_its_spectrum_beside_its_totals() {
    let dir = test_dir("raw_application_noise");
    let deck = dir.join("noise.cir");
    std::fs::write(
        &deck,
        "* resistor noise
v1 in 0 dc 0 ac 1
r1 in out 1k
r2 out 0 1k
.noise v(out) v1 lin 3 100 1k
.end
",
    )
    .unwrap();
    for format in ["raw", "ascii"] {
        let output = dir.join(format!("noise.{format}"));
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .arg("-o")
            .arg(&output)
            .args(["-f", format])
            .output()
            .unwrap();
        assert!(result.status.success(), "{result:?}");
        let decoded =
            decode_spice_raw(&std::fs::read(output).unwrap(), Default::default()).unwrap();
        assert_eq!(decoded.domain, WaveformDomain::Ac);
        assert_eq!(decoded.coordinate, [100.0, 550.0, 1000.0]);
        let names = decoded
            .signals
            .iter()
            .map(|signal| signal.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, ["onoise_spectrum", "inoise_spectrum"]);
    }
}

/// The `Flags:` line of a rawfile's first plot.
fn first_plot_flags(bytes: &[u8]) -> String {
    bytes
        .split(|byte| *byte == b'\n')
        .take_while(|line| *line != b"Binary:" && *line != b"Values:")
        .find_map(|line| line.strip_prefix(b"Flags: "))
        .map(|flags| String::from_utf8(flags.to_vec()).unwrap())
        .expect("the first plot has a Flags: line")
}

/// LTspice and the readers built for its files decode a real plot's binary
/// values as single precision unless `Flags:` says `double`, and take
/// `forward` and `log` from it as LTspice writes them. Every binary rawfile
/// the run writes says what it holds; an ASCII one says only `real` or
/// `complex` and `log`, since no reader decodes text by `double` or
/// `forward` and ngspice warns on both. Each still reads back.
#[test]
fn run_rawfiles_carry_the_flags_ltspice_readers_decode_by() {
    let dir = test_dir("raw_application_flags");
    let rc = "v1 in 0 dc 1 ac 1 pulse(0 1 0 1n 1n 5n 10n)\nr1 in out 1k\nc1 out 0 1p\n";
    for (name, analysis, binary, ascii, coordinate) in [
        ("op", ".op", "real forward double", "real", None),
        ("tran", ".tran 5n 10n", "real forward double", "real", None),
        (
            "ac_dec",
            ".ac dec 1 1 100",
            "complex forward log",
            "complex log",
            Some(vec![1.0, 10.0, 100.0]),
        ),
        (
            "ac_lin",
            ".ac lin 3 100 300",
            "complex forward",
            "complex",
            Some(vec![100.0, 200.0, 300.0]),
        ),
        (
            "dc_down",
            ".dc v1 1 0 -0.5",
            "real double",
            "real",
            Some(vec![1.0, 0.5, 0.0]),
        ),
        (
            "noise_dec",
            ".noise v(out) v1 dec 1 100 1k",
            "real forward log double",
            "real log",
            Some(vec![100.0, 1000.0]),
        ),
    ] {
        let deck = dir.join(format!("{name}.cir"));
        std::fs::write(&deck, format!("* {name}\n{rc}{analysis}\n.end\n")).unwrap();
        for (format, flags) in [("raw", binary), ("ascii", ascii)] {
            let output = dir.join(format!("{name}.{format}"));
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "run"])
                .arg(&deck)
                .arg("-o")
                .arg(&output)
                .args(["-f", format])
                .output()
                .unwrap();
            assert!(result.status.success(), "{name} {format}: {result:?}");
            let bytes = std::fs::read(&output).unwrap();
            assert_eq!(first_plot_flags(&bytes), flags, "{name} {format}");
            let decoded = decode_spice_raw(&bytes, Default::default()).unwrap();
            if let Some(coordinate) = &coordinate {
                // A decade sweep's points come from a power, whose last bit
                // is the platform's.
                assert_eq!(
                    decoded.coordinate.len(),
                    coordinate.len(),
                    "{name} {format}"
                );
                for (read, swept) in decoded.coordinate.iter().zip(coordinate) {
                    assert!(
                        (read - swept).abs() <= 1e-12 * swept.abs(),
                        "{name} {format}: {:?}",
                        decoded.coordinate
                    );
                }
            }
            assert!(!decoded.signals.is_empty(), "{name} {format}");
        }
    }

    // A converted rawfile keeps the `log` its source declared.
    let source = dir.join("ltspice_ac.raw");
    std::fs::write(
        &source,
        b"Title: ac\nPlotname: AC Analysis\nFlags: complex forward log\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 frequency frequency\n1 V(out) voltage\nValues:\n0 1,0 1,-0\n1 10,0 0.5,-0.5\n",
    )
    .unwrap();
    let converted = dir.join("converted.raw");
    convert(&source, &converted, "raw");
    assert_eq!(
        first_plot_flags(&std::fs::read(converted).unwrap()),
        "complex forward log"
    );
}

/// `rspice convert` opens the rawfiles LTspice wrote itself (UTF-16LE
/// headers, single-precision signals, compression marks, fast access,
/// complex AC), and its RSpice rawfile carries every value through to the
/// application's reader unchanged.
#[test]
fn rawfiles_ltspice_wrote_convert_with_every_value() {
    let fixtures =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-core-base/tests/fixtures/ltspice");
    let dir = test_dir("raw_application_ltspice");
    for (name, domain, points) in [
        ("rc_tran.raw", WaveformDomain::Transient, 23),
        ("rc_tran_double.raw", WaveformDomain::Transient, 23),
        ("rc_fast.raw", WaveformDomain::Transient, 277),
        ("rc_ac.raw", WaveformDomain::Ac, 121),
        ("rc_noise.raw", WaveformDomain::Ac, 61),
        ("rc_dc.raw", WaveformDomain::DcSweep, 21),
        ("rc_step.raw", WaveformDomain::Transient, 105),
    ] {
        let source = fixtures.join(name);
        let original = rspice_core::io::ltspice_raw::parse_raw_file(&source)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        let output = dir.join(name);
        convert(&source, &output, "raw");
        let decoded =
            decode_spice_raw(&std::fs::read(output).unwrap(), Default::default()).unwrap();
        assert_eq!(
            (decoded.domain, decoded.coordinate.len()),
            (domain, points),
            "{name}"
        );
        let bits = |values: &[f64]| {
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            bits(&decoded.coordinate),
            bits(&original.waveforms[0].y),
            "{name}"
        );
        for (signal, column) in decoded.signals.iter().zip(&original.waveforms[1..]) {
            assert_eq!(signal.name, column.name, "{name}");
            assert_eq!(
                bits(&signal.real),
                bits(&column.y),
                "{name} {}",
                signal.name
            );
            assert_eq!(
                signal.imag.as_deref().map(bits),
                column.y_imag.as_deref().map(bits),
                "{name} {}",
                signal.name
            );
        }
    }
}

/// LTspice's operating point converts too: one point of every node voltage
/// and branch current, drawn over no scale, each value carried unchanged.
#[test]
fn an_operating_point_ltspice_wrote_converts_with_every_value() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../rspice-core-base/tests/fixtures/ltspice/rc_op.raw");
    let original = rspice_core::io::ltspice_raw::parse_raw_file(&source).unwrap();
    let dir = test_dir("raw_application_ltspice_op");
    let output = dir.join("rc_op.raw");
    convert(&source, &output, "raw");
    let decoded = decode_spice_raw(&std::fs::read(output).unwrap(), Default::default()).unwrap();
    let values: Vec<_> = decoded
        .signals
        .iter()
        .map(|signal| {
            (
                signal.name.as_str(),
                signal.real.iter().map(|v| v.to_bits()).collect(),
            )
        })
        .collect();
    let expected: Vec<(&str, Vec<u64>)> = original
        .waveforms
        .iter()
        .map(|column| {
            (
                column.name.as_str(),
                column.y.iter().map(|v| v.to_bits()).collect(),
            )
        })
        .collect();
    assert_eq!(values, expected);
}

/// `rspice compare` opens every rawfile LTspice wrote, each layout, and finds
/// it equal to itself at zero tolerance.
#[test]
fn every_rawfile_ltspice_wrote_compares_equal_to_itself() {
    let fixtures =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-core-base/tests/fixtures/ltspice");
    for name in [
        "rc_tran.raw",
        "rc_tran_double.raw",
        "rc_fast.raw",
        "rc_ac.raw",
        "rc_noise.raw",
        "rc_dc.raw",
        "rc_op.raw",
        "rc_step.raw",
    ] {
        let source = fixtures.join(name);
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "compare"])
            .arg(&source)
            .arg(&source)
            .args(["--json", "--abstol", "0", "--reltol", "0"])
            .output()
            .unwrap();
        assert!(result.status.success(), "{name}: {result:?}");
    }
}

/// An ASCII rawfile of one transient plot: `names` after `time`, each row
/// its time then one value per name.
fn ascii_rawfile(names: &[&str], rows: &[(f64, Vec<f64>)]) -> String {
    let mut text = format!(
        "Title: refused\nDate: today\nPlotname: Transient Analysis\nFlags: real\n\
         No. Variables: {}\nNo. Points: {}\nVariables:\n\t0\ttime\ttime\n",
        names.len() + 1,
        rows.len()
    );
    for (index, name) in names.iter().enumerate() {
        text.push_str(&format!("\t{}\t{name}\tvoltage\n", index + 1));
    }
    text.push_str("Values:\n");
    for (point, (time, values)) in rows.iter().enumerate() {
        text.push_str(&format!("{point}\t{time:e}\n"));
        for value in values {
            text.push_str(&format!("\t{value:e}\n"));
        }
    }
    text
}

/// A conversion never writes a table the application's importer refuses:
/// two columns of one name, or a time that runs backwards, are refused with
/// the column or the point, and no file is written.
#[test]
fn a_conversion_refuses_a_table_the_importer_would_refuse() {
    let dir = common::test_dir("convert-refusals");
    let duplicated = dir.join("duplicated.raw");
    std::fs::write(
        &duplicated,
        ascii_rawfile(
            &["V(out)", "V(out)"],
            &[(0.0, vec![1.0, 2.0]), (1.0e-6, vec![3.0, 4.0])],
        ),
    )
    .unwrap();
    // The importer compares names ignoring case.
    let cased = dir.join("cased.raw");
    std::fs::write(
        &cased,
        ascii_rawfile(
            &["V(out)", "v(OUT)"],
            &[(0.0, vec![1.0, 2.0]), (1.0e-6, vec![3.0, 4.0])],
        ),
    )
    .unwrap();
    let falling = dir.join("falling.raw");
    std::fs::write(
        &falling,
        ascii_rawfile(
            &["V(out)"],
            &[(0.0, vec![1.0]), (2.0e-6, vec![2.0]), (1.0e-6, vec![3.0])],
        ),
    )
    .unwrap();
    for (source, reason) in [
        (&duplicated, "two columns are named 'V(out)'"),
        (&cased, "two columns are named 'v(OUT)'"),
        (&falling, "falls from 0.000002 to 0.000001 at point 3"),
    ] {
        for to in ["csv", "tsv", "json"] {
            let out = dir.join(format!("out.{to}"));
            let refused = common::convert_with_flags(source, &out, to, &[]);
            assert!(!refused.status.success(), "{to}: {refused:?}");
            let message = String::from_utf8_lossy(&refused.stderr);
            assert!(message.contains(reason), "{to}: {message}");
            assert!(!out.exists(), "{to}: nothing is written");
        }
    }
}

/// An LTspice stepped rawfile converts one run at a time, each run a section
/// named by the values its log gives it, never as one table whose time
/// restarts at every run.
#[test]
fn a_stepped_ltspice_rawfile_converts_each_run_as_its_own_section() {
    let fixtures =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-core-base/tests/fixtures/ltspice");
    let dir = common::test_dir("stepped-convert");
    let source = dir.join("rc_step.raw");
    std::fs::copy(fixtures.join("rc_step.raw"), &source).unwrap();
    std::fs::copy(fixtures.join("rc_step.log"), dir.join("rc_step.log")).unwrap();

    let refused = common::convert_with_flags(&source, &dir.join("all.csv"), "csv", &[]);
    assert!(!refused.status.success(), "{refused:?}");
    let message = String::from_utf8_lossy(&refused.stderr);
    for run in [
        "run 1 of 3, r=1000",
        "run 2 of 3, r=2000",
        "run 3 of 3, r=4000",
    ] {
        assert!(message.contains(run), "{message}");
    }
    assert!(!dir.join("all.csv").exists());

    let mut points = 0;
    for run in 1..=3 {
        let out = dir.join(format!("run{run}.csv"));
        let section = run.to_string();
        let converted = common::convert_with_flags(&source, &out, "csv", &["--section", &section]);
        assert!(converted.status.success(), "run {run}: {converted:?}");
        let (names, rows) = common::read_numeric_csv(&out);
        assert_eq!(names[0], "time", "run {run}");
        let time: Vec<f64> = rows.iter().map(|row| row[0].unwrap()).collect();
        assert_eq!(time[0], 0.0, "run {run} starts at its own zero");
        assert!(
            time.windows(2).all(|pair| pair[0] < pair[1]),
            "run {run}: its time ascends"
        );
        points += time.len();
    }
    assert_eq!(points, 105, "the runs hold every point of the plot");
}
