//! Integration tests: every analysis type run with `-o` must produce a
//! parseable output file in the requested format.
//!
//! Regression coverage for two CLI export bugs:
//! - `-f csv` (and tsv/json) used to fall through to the binary rawfile
//!   writer for transient/DC sweeps, producing binary bytes in a .csv file.
//! - AC (and noise) analyses wrote no output file at all for most formats
//!   while still reporting "Simulation complete".

use std::path::{Path, PathBuf};
use std::process::Command;

const FORMATS: &[&str] = &["raw", "ascii", "csv", "tsv", "json", "hdf5"];

const OP_DECK: &str = "* op export test
v1 in 0 dc 5
r1 in out 1k
r2 out 0 1k
.op
.end
";

const DC_DECK: &str = "* dc sweep export test
v1 in 0 dc 5
r1 in out 1k
r2 out 0 1k
.dc v1 0 5 1
.end
";

const DC_CURRENT_PRINT_DECK: &str = "* dc current print export test
v1 in 0 dc 5
r1 in out 1k
r2 out 0 1k
.dc v1 0 5 1
.print dc v(out) i(v1)
.end
";

const TRAN_DECK: &str = "* transient export test
v1 in 0 sin(0 1 1k)
r1 in out 1k
c1 out 0 1u
.tran 10u 200u
.end
";

const TRAN_CURRENT_PRINT_DECK: &str = "* transient current print export test
v1 in 0 pulse(0 5 0 1n 1n 50u 100u)
r1 in out 1k
c1 out 0 1u
.tran 10u 200u
.print tran v(out) i(v1)
.end
";

const DC_ORDERED_PRINT_DECK: &str = "* ordered dc print export test
V1 in 0 0
R1 in out 1k
R2 out 0 1k
.dc V1 1 2 1
.print dc I(V1) V(out) R1:R {V(out)+1} V(out)
.end
";

const TRAN_ORDERED_PRINT_DECK: &str = "* ordered transient print export test
V1 in 0 pulse(0 5 0 1n 1n 50u 100u)
R1 in out 1k
C1 out 0 1u
.tran 10u 200u
.print tran I ( V1 ) V ( out ) {V(out)+1} V(out)
.end
";

const TRAN_OUTPUT_TIME_POINTS_DECK: &str = "* transient output schedule export test
voff a 0 2
vexp 1 a exp(0 5 0 1ms 1s)
r1 1 2 1
c1 2 0 1
.tran 1ms 5ms
.options output outputtimepoints=1ms,2ms,3ms,4ms
.print tran v(1)
.end
";

const XSPICE_DIGITAL_TRAN_DECK: &str = "* xspice digital export test
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.tran 1n 20n
.end
";

const XSPICE_DIGITAL_SAVE_TRAN_DECK: &str = "* xspice digital save export test
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.tran 1n 20n
.save d
.end
";

const XSPICE_DIGITAL_VOLTAGE_SAVE_TRAN_DECK: &str = "* xspice digital voltage save export test
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.tran 1n 20n
.save v(d)
.end
";

const AC_DECK: &str = "* ac export test
v1 in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 1n
.ac dec 2 100k 1meg
.print ac vm(out)
.end
";

const NOISE_DECK: &str = "* noise export test
v1 in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 1n
.noise v(out) v1 dec 2 100k 1meg
.end
";

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "rspice_output_formats_{}_{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

/// Run `rspice run <deck> -o <out> -f <format>` and return the output path.
fn run_export(dir: &Path, tag: &str, deck: &str, format: &str) -> PathBuf {
    run_export_with_args(dir, tag, deck, format, &[])
}

fn run_export_with_args(
    dir: &Path,
    tag: &str,
    deck: &str,
    format: &str,
    extra_args: &[&str],
) -> PathBuf {
    let deck_path = dir.join(format!("{tag}.sp"));
    std::fs::write(&deck_path, deck).expect("write deck");
    let out_path = dir.join(format!("{tag}_{format}.out"));

    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command
        .arg("--quiet")
        .arg("run")
        .arg(&deck_path)
        .arg("-o")
        .arg(&out_path)
        .arg("-f")
        .arg(format)
        .args(extra_args);
    let output = command.output().expect("run rspice");

    assert!(
        output.status.success(),
        "rspice run failed for {tag}/{format}: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        out_path.exists(),
        "no output file produced for {tag}/{format}"
    );
    out_path
}

/// Validate that the produced file is parseable in the requested format.
fn assert_parseable(path: &Path, tag: &str, format: &str) {
    let bytes = std::fs::read(path).expect("read output file");
    assert!(!bytes.is_empty(), "empty output file for {tag}/{format}");

    // The DC operating point writes `signal,value` rows (text label first);
    // sweep analyses write purely numeric tables.
    let label_column = tag == "op";

    match format {
        "csv" => assert_delimited(&bytes, ',', label_column, tag, format),
        "tsv" => assert_delimited(&bytes, '\t', label_column, tag, format),
        "json" => {
            let value: serde_json::Value = serde_json::from_slice(&bytes)
                .unwrap_or_else(|e| panic!("invalid JSON for {tag}/{format}: {e}"));
            assert!(
                value.get("analysis").is_some(),
                "JSON output for {tag}/{format} missing 'analysis' tag"
            );
        }
        "raw" | "ascii" => assert_rawfile(&bytes, tag, format),
        "hdf5" => {
            assert!(
                bytes.starts_with(b"\x89HDF\r\n\x1a\n"),
                "missing HDF5 signature for {tag}/{format}"
            );
        }
        other => panic!("unknown format {other}"),
    }
}

/// Header row plus at least one data row of numeric fields.
///
/// With `label_column` set, the first field of each row may be a signal name.
fn assert_delimited(bytes: &[u8], delimiter: char, label_column: bool, tag: &str, format: &str) {
    let text = std::str::from_utf8(bytes)
        .unwrap_or_else(|_| panic!("non-UTF8 {format} output for {tag} (binary bytes?)"));
    let mut lines = text.lines().filter(|line| !line.trim().is_empty());

    let header = lines
        .next()
        .unwrap_or_else(|| panic!("missing header row for {tag}/{format}"));
    let columns = header.split(delimiter).count();
    assert!(
        columns >= 2,
        "expected at least 2 columns for {tag}/{format}, got header {header:?}"
    );

    let mut rows = 0;
    for line in lines {
        let fields: Vec<&str> = line.split(delimiter).collect();
        assert_eq!(
            fields.len(),
            columns,
            "row/header column mismatch for {tag}/{format}: {line:?}"
        );
        let skip = if label_column { 1 } else { 0 };
        for field in fields.iter().skip(skip) {
            field
                .trim()
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("non-numeric field {field:?} in {tag}/{format}"));
        }
        rows += 1;
    }
    assert!(rows > 0, "no data rows for {tag}/{format}");
}

/// SPICE rawfile header with a positive point count and a data section.
fn assert_rawfile(bytes: &[u8], tag: &str, format: &str) {
    let text = String::from_utf8_lossy(bytes);
    assert!(
        text.contains("Plotname:"),
        "missing Plotname in rawfile for {tag}/{format}"
    );

    let points: usize = text
        .lines()
        .find_map(|line| line.strip_prefix("No. Points:"))
        .unwrap_or_else(|| panic!("missing 'No. Points:' in rawfile for {tag}/{format}"))
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("unparseable point count for {tag}/{format}"));
    assert!(points > 0, "zero points in rawfile for {tag}/{format}");

    assert!(
        text.contains("Values:") || text.contains("Binary:"),
        "missing data section in rawfile for {tag}/{format}"
    );
}

fn check_all_formats(tag: &str, deck: &str) {
    let dir = test_dir(tag);
    for format in FORMATS {
        let path = run_export(&dir, tag, deck, format);
        assert_parseable(&path, tag, format);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn op_exports_every_format() {
    check_all_formats("op", OP_DECK);
}

#[test]
fn dc_sweep_exports_every_format() {
    check_all_formats("dc", DC_DECK);
}

#[test]
fn transient_exports_every_format() {
    check_all_formats("tran", TRAN_DECK);
}

#[test]
fn ac_exports_every_format() {
    check_all_formats("ac", AC_DECK);
}

#[test]
fn noise_exports_every_format() {
    check_all_formats("noise", NOISE_DECK);
}

/// `-f csv` must produce text CSV, not binary raw bytes (regression test).
#[test]
fn transient_csv_is_text_not_binary() {
    let dir = test_dir("csv_regression");
    let path = run_export(&dir, "tran_csv", TRAN_DECK, "csv");
    let bytes = std::fs::read(&path).expect("read csv");
    let text = std::str::from_utf8(&bytes).expect("csv output contains non-UTF8 (binary) bytes");
    let header = text.lines().next().expect("csv header");
    assert!(
        header.starts_with("time,"),
        "unexpected csv header: {header:?}"
    );
    assert!(
        !text.contains("Binary:"),
        "csv output contains rawfile binary marker"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_csv_preserves_subnanosecond_knots_at_large_times() {
    let dir = test_dir("csv_time_precision");
    let deck = "* transient csv precision test
vclk out 0 PWL(0 0 4.5 0 4.50000000001 1 4.50000000002 1)
rload out 0 1k
.tran 1 4.50000000002
.print tran v(out)
.end
";
    let path = run_export(&dir, "csv_time_precision", deck, "csv");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let times = text
        .lines()
        .skip(1)
        .filter_map(|line| line.split(',').next())
        .filter_map(|field| field.parse::<f64>().ok())
        .collect::<Vec<_>>();

    assert!(
        times
            .windows(2)
            .any(|pair| (pair[1] - pair[0]).abs() > 1.0e-12 && (pair[1] - pair[0]).abs() < 2.0e-11),
        "CSV time column must preserve 10 ps-scale spacing near 4.5 s: {text}"
    );
    assert!(
        text.contains("4.50000000001"),
        "CSV should print enough significant digits to expose the 10 ps knot: {text}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_output_time_points_project_the_export_without_truncating_the_solve() {
    let dir = test_dir("output_time_points");
    let path = run_export(
        &dir,
        "output_time_points",
        TRAN_OUTPUT_TIME_POINTS_DECK,
        "csv",
    );
    let text = std::fs::read_to_string(&path).expect("read scheduled csv");
    let times = text
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .next()
                .expect("time field")
                .parse::<f64>()
                .expect("numeric time")
        })
        .collect::<Vec<_>>();
    assert_eq!(times, vec![1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3, 5.0e-3]);
    assert!(
        !text.lines().skip(1).any(|line| line.starts_with("0,")),
        "an absent zero request must not leak the accepted t=0 row: {text}"
    );

    let compressed = run_export_with_args(
        &dir,
        "output_time_points_compressed",
        TRAN_OUTPUT_TIME_POINTS_DECK,
        "csv",
        &["--compress"],
    );
    let compressed_text = std::fs::read_to_string(compressed).expect("read compressed csv");
    assert_eq!(
        compressed_text, text,
        "OUTPUTTIMEPOINTS must retain its exact rows when compression is requested"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

fn current_column_index(header: &str, name: &str) -> usize {
    header
        .split(',')
        .position(|column| column.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("{name} column missing from header: {header:?}"))
}

fn numeric_csv_rows(text: &str) -> Vec<Vec<f64>> {
    text.lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(',')
                .map(|field| field.parse::<f64>().expect("numeric CSV field"))
                .collect()
        })
        .collect()
}

fn assert_dc_ordered_print_projection(text: &str) {
    let header = text.lines().next().expect("CSV header");
    assert_eq!(
        header, "V1,I(V1),V(out),R1:R,{V(out)+1},V(out)",
        "PRINT operands must retain exact authored spelling, order, and duplicates"
    );

    let rows = numeric_csv_rows(text);
    assert!(
        !rows.is_empty(),
        "projected CSV must contain samples: {text}"
    );
    for row in rows {
        assert_eq!(row.len(), 6, "projected row must match its header: {row:?}");
        assert!(
            (row[3] - 1_000.0).abs() < 1e-12,
            "R1:R must export its installed resistance: {row:?}"
        );
        assert!(
            (row[4] - (row[2] + 1.0)).abs() < 1e-9,
            "expression column must be evaluated from V(out): {row:?}"
        );
        assert_eq!(
            row[2].to_bits(),
            row[5].to_bits(),
            "duplicate PRINT operands must produce duplicate columns"
        );
    }
}

fn assert_tran_ordered_print_projection(text: &str) {
    let header = text.lines().next().expect("CSV header");
    assert_eq!(
        header, "time,I ( V1 ),V ( out ),{V(out)+1},V(out)",
        "PRINT operands must retain exact authored spelling, order, and duplicates"
    );

    let rows = numeric_csv_rows(text);
    assert!(
        !rows.is_empty(),
        "projected CSV must contain samples: {text}"
    );
    let mut saw_dynamic_sample = false;
    for row in rows {
        assert_eq!(row.len(), 5, "projected row must match its header: {row:?}");
        assert!(
            (row[3] - (row[2] + 1.0)).abs() < 1e-9,
            "expression column must be evaluated from V(out): {row:?}"
        );
        assert_eq!(
            row[2].to_bits(),
            row[4].to_bits(),
            "duplicate PRINT operands must produce duplicate columns"
        );
        saw_dynamic_sample |= row[1] != 0.0 || row[2] != 0.0;
    }
    assert!(
        saw_dynamic_sample,
        "transient projection must contain a nonzero current or voltage sample"
    );
}

#[test]
fn dc_sweep_print_exports_branch_current() {
    let dir = test_dir("dc_current_print");
    let path = run_export(&dir, "dc_current_print", DC_CURRENT_PRINT_DECK, "csv");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let header = text.lines().next().expect("csv header");
    let current_column = current_column_index(header, "I(v1)");
    assert!(
        text.lines().skip(1).any(|line| {
            line.split(',')
                .nth(current_column)
                .and_then(|field| field.parse::<f64>().ok())
                .is_some_and(|value| value.abs() > 0.0)
        }),
        "dc branch current column should contain non-zero source current: {text}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_print_exports_branch_current() {
    let dir = test_dir("tran_current_print");
    let path = run_export(&dir, "tran_current_print", TRAN_CURRENT_PRINT_DECK, "csv");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let header = text.lines().next().expect("csv header");
    let current_column = current_column_index(header, "I(v1)");
    assert!(
        text.lines().skip(1).any(|line| {
            line.split(',')
                .nth(current_column)
                .and_then(|field| field.parse::<f64>().ok())
                .is_some_and(|value| value.abs() > 0.0)
        }),
        "tran branch current column should contain non-zero source current: {text}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn dc_print_preserves_authored_projection_order_and_duplicates() {
    let dir = test_dir("dc_ordered_print");
    let path = run_export(&dir, "dc_ordered_print", DC_ORDERED_PRINT_DECK, "csv");
    let text = std::fs::read_to_string(&path).expect("read CSV");
    assert_dc_ordered_print_projection(&text);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_print_preserves_authored_projection_order_and_duplicates() {
    let dir = test_dir("tran_ordered_print");
    let path = run_export(&dir, "tran_ordered_print", TRAN_ORDERED_PRINT_DECK, "csv");
    let text = std::fs::read_to_string(&path).expect("read CSV");
    assert_tran_ordered_print_projection(&text);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn spaced_print_operands_round_trip_through_raw_formats() {
    let dir = test_dir("spaced_print_raw_round_trip");
    for format in ["raw", "ascii"] {
        let path = run_export(&dir, format, TRAN_ORDERED_PRINT_DECK, format);
        let converted = dir.join(format!("{format}.csv"));
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .arg("--quiet")
            .arg("convert")
            .arg(&path)
            .arg(&converted)
            .arg("--to")
            .arg("csv")
            .output()
            .expect("convert authored PRINT rawfile");
        assert!(
            output.status.success(),
            "{format} round trip failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let csv = std::fs::read_to_string(&converted).expect("read converted CSV");
        assert_eq!(
            csv.lines().next(),
            Some("time,I(V1),V(out),{V(out)+1},V(out)"),
            "rawfile declarations must compact authored probe whitespace"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_export_includes_xspice_digital_traces() {
    let dir = test_dir("tran_xspice_digital");
    let path = run_export(&dir, "tran_xspice_digital", XSPICE_DIGITAL_TRAN_DECK, "csv");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let header = text.lines().next().expect("csv header");
    let digital_column = current_column_index(header, "D(d)");
    let values: Vec<f64> = text
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .nth(digital_column)
                .expect("digital field")
                .parse::<f64>()
                .expect("numeric digital field")
        })
        .collect();

    assert!(
        values.iter().any(|value| (*value - 0.0).abs() < 1e-12),
        "digital export should include low samples: {text}"
    );
    assert!(
        values.iter().any(|value| (*value - 1.0).abs() < 1e-12),
        "digital export should include high samples: {text}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_save_selects_xspice_digital_trace_by_raw_node() {
    let dir = test_dir("tran_xspice_digital_save");
    let path = run_export(
        &dir,
        "tran_xspice_digital_save",
        XSPICE_DIGITAL_SAVE_TRAN_DECK,
        "csv",
    );
    let text = std::fs::read_to_string(&path).expect("read csv");
    let header = text.lines().next().expect("csv header");

    assert!(
        header
            .split(',')
            .any(|column| column.eq_ignore_ascii_case("D(d)")),
        "digital trace selected by .save d should be exported: {header:?}"
    );
    assert!(
        !header
            .split(',')
            .any(|column| column.eq_ignore_ascii_case("V(in)")),
        ".save d should still filter unrelated analog voltages: {header:?}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_voltage_save_does_not_select_xspice_digital_trace() {
    let dir = test_dir("tran_xspice_digital_voltage_save");
    let path = run_export(
        &dir,
        "tran_xspice_digital_voltage_save",
        XSPICE_DIGITAL_VOLTAGE_SAVE_TRAN_DECK,
        "csv",
    );
    let text = std::fs::read_to_string(&path).expect("read csv");
    let header = text.lines().next().expect("csv header");

    assert!(
        !header
            .split(',')
            .any(|column| column.eq_ignore_ascii_case("D(d)")),
        "typed voltage save v(d) must not export digital trace D(d): {header:?}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_hdf5_preserves_xspice_digital_type_when_converted() {
    let dir = test_dir("tran_xspice_digital_hdf5_type");
    let hdf5 = run_export(
        &dir,
        "tran_xspice_digital_hdf5_type",
        XSPICE_DIGITAL_TRAN_DECK,
        "hdf5",
    );
    let raw_ascii = dir.join("tran_xspice_digital_hdf5_type.raw");

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .arg("convert")
        .arg(&hdf5)
        .arg(&raw_ascii)
        .arg("--from")
        .arg("hdf5")
        .arg("--to")
        .arg("ascii")
        .output()
        .expect("convert hdf5 to ascii raw");

    assert!(
        output.status.success(),
        "rspice convert failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let text = std::fs::read_to_string(&raw_ascii).expect("read ascii raw");
    assert!(
        text.lines().any(|line| {
            line.split_whitespace()
                .collect::<Vec<_>>()
                .windows(2)
                .any(|fields| fields[0].eq_ignore_ascii_case("D(d)") && fields[1] == "digital")
        }),
        "converted rawfile should preserve digital variable type: {text}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
