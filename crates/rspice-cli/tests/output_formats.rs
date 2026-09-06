//! Integration tests: every analysis type run with `-o` must produce a
//! parseable output file in the requested format.
//!
//! Regression coverage for two CLI export bugs:
//! - `-f csv` (and tsv/json) used to fall through to the binary rawfile
//!   writer for transient/DC sweeps, producing binary bytes in a .csv file.
//! - AC (and noise) analyses wrote no output file at all for most formats
//!   while still reporting "Simulation complete".

mod common;

use common::test_dir;

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

const TRAN_OUTPUT_INTERVAL_DECK: &str = "* transient interval schedule compression preflight
v1 in 0 sin(0 1 1k)
r1 in out 1k
c1 out 0 1u
.tran 10u 200u
.options output initial_interval=25u
.print tran v(out)
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

const XSPICE_EVENT_TRAN_DECK: &str = "* xspice event export test
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
aobs out rnode obs
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.model obs v_to_real(gain=2)
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

/// The compressor keeps every sample an authored `INITIAL_INTERVAL` lattice
/// reads and fails closed if the retained grid could not reproduce those rows,
/// so `--compress` composes with the schedule instead of being refused.
#[test]
fn compression_preserves_the_authored_interval_output_lattice() {
    let dir = test_dir("compressed_interval");
    let uncompressed = run_export(&dir, "interval_plain", TRAN_OUTPUT_INTERVAL_DECK, "csv");
    let compressed = run_export_with_args(
        &dir,
        "interval_compressed",
        TRAN_OUTPUT_INTERVAL_DECK,
        "csv",
        &["--compress"],
    );

    let plain_text = std::fs::read_to_string(&uncompressed).expect("read scheduled csv");
    let compressed_text = std::fs::read_to_string(&compressed).expect("read compressed csv");
    assert_eq!(
        compressed_text, plain_text,
        "INITIAL_INTERVAL must retain its exact rows when compression is requested"
    );

    // The lattice really is the authored one, not an accident of both runs
    // being uncompressed.
    let times = plain_text
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
    assert!(
        times.len() > 2,
        "interval lattice has too few rows: {times:?}"
    );
    for pair in times.windows(2) {
        assert!(
            (pair[1] - pair[0] - 25.0e-6).abs() < 1.0e-12,
            "interval lattice is not the authored 25us cadence: {times:?}"
        );
    }
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

/// The rawfile carries each event node's own timeline as its own plot, and
/// leaves the analysis plot exactly where it was.
///
/// The grid `D()` column answers what a node held at each analysis time point;
/// it cannot answer when the node changed, and it says nothing at all about
/// drive strength or about real-valued event nodes, which reach no table.
#[test]
fn transient_rawfile_appends_lossless_event_plots() {
    use rspice_core::xspice::{DigitalState, DigitalStrength};

    let dir = test_dir("tran_xspice_event_plots");
    for format in ["raw", "ascii"] {
        let tag = format!("events_{format}");
        let path = run_export(&dir, &tag, XSPICE_EVENT_TRAN_DECK, format);

        // The single-plot reader every existing caller uses still reads the
        // analysis plot, with its grid-sampled digital column intact.
        let analysis = rspice_core::io::parse_raw_file(&path)
            .unwrap_or_else(|error| panic!("{format}: legacy read of plot 1: {error}"));
        assert_eq!(analysis.header.plotname, "Transient Analysis");
        let digital_column = analysis
            .variables
            .iter()
            .position(|variable| variable.name.eq_ignore_ascii_case("D(d)"))
            .unwrap_or_else(|| panic!("{format}: plot 1 must still carry D(d)"));
        assert_eq!(analysis.variables[digital_column].var_type, "digital");
        assert_eq!(
            analysis.waveforms[digital_column].y.len(),
            analysis.header.no_points,
            "{format}: the digital column is still on the analysis grid"
        );

        let file = rspice_core::io::parse_raw_plots_file_with_limits(
            &path,
            rspice_core::ResourceLimits::default(),
        )
        .unwrap_or_else(|error| panic!("{format}: multi-plot read: {error}"));
        assert_eq!(
            file.plots.len(),
            3,
            "{format}: one analysis plot, one digital node, one real node"
        );
        let traces = rspice_core::execution::decode_event_plots(&file)
            .unwrap_or_else(|error| panic!("{format}: decode event plots: {error}"));

        let digital = traces
            .digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case("d"))
            .unwrap_or_else(|| panic!("{format}: the digital node's timeline: {traces:?}"));
        assert!(
            digital
                .points
                .windows(2)
                .all(|pair| pair[0].time < pair[1].time),
            "{format}: event times are strictly increasing: {digital:?}"
        );
        assert!(
            digital.points.len() < analysis.header.no_points,
            "{format}: an event timeline is irregular, not the analysis grid"
        );
        let states: Vec<DigitalState> = digital.points.iter().map(|p| p.value.state).collect();
        assert!(
            states.contains(&DigitalState::Zero) && states.contains(&DigitalState::One),
            "{format}: the pulsed node must toggle: {states:?}"
        );
        assert!(
            digital
                .points
                .iter()
                .all(|point| point.value.strength == DigitalStrength::Strong),
            "{format}: a driven bridge output carries strong drive: {digital:?}"
        );

        // Real event nodes reach no table at all; the rawfile is the first
        // flat artifact that carries them.
        let real = traces
            .real_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case("rnode"))
            .unwrap_or_else(|| panic!("{format}: the real node's timeline: {traces:?}"));
        assert!(
            !real.points.is_empty() && real.points.iter().all(|point| point.value.is_finite()),
            "{format}: real event values are carried as themselves: {real:?}"
        );
        assert!(
            !analysis
                .variables
                .iter()
                .any(|variable| variable.name.eq_ignore_ascii_case("E(rnode)")),
            "{format}: real event nodes are not columns of the analysis plot"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// `convert` and `compare` read the analysis plot and are untroubled by the
/// plots behind it.
#[test]
fn generic_conversion_still_reads_plot_one_of_a_multi_plot_rawfile() {
    let dir = test_dir("tran_xspice_event_convert");
    // `compare` resolves each side's format from its extension, so the golden
    // is published under its own.
    let golden = dir.join("events_golden.csv");
    let golden_deck = dir.join("events_golden.sp");
    std::fs::write(&golden_deck, XSPICE_EVENT_TRAN_DECK).expect("write golden deck");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .arg("run")
        .arg(&golden_deck)
        .arg("-o")
        .arg(&golden)
        .arg("-f")
        .arg("csv")
        .output()
        .expect("publish the CSV golden");
    assert!(
        output.status.success(),
        "golden run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    for format in ["raw", "ascii"] {
        let tag = format!("events_convert_{format}");
        let path = run_export(&dir, &tag, XSPICE_EVENT_TRAN_DECK, format);
        let converted = dir.join(format!("{tag}.csv"));

        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .arg("--quiet")
            .arg("convert")
            .arg(&path)
            .arg(&converted)
            .arg("--to")
            .arg("csv")
            .output()
            .expect("convert a rawfile carrying event plots");
        assert!(
            output.status.success(),
            "{format} convert failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let header = std::fs::read_to_string(&converted).expect("read converted CSV");
        let header = header.lines().next().expect("converted CSV header");
        assert!(
            header
                .split(',')
                .any(|column| column.eq_ignore_ascii_case("D(d)")),
            "{format}: conversion must carry plot 1's columns: {header:?}"
        );

        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .arg("--quiet")
            .arg("compare")
            .arg(&path)
            .arg(&golden)
            .arg("--abstol")
            .arg("1e-9")
            .output()
            .expect("compare a rawfile carrying event plots");
        assert!(
            output.status.success(),
            "{format} compare failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
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

/// Every column a run publishes as HDF5 states the unit of its quantity.
///
/// The layout writes `signal_NNNN_unit` only for a column whose producer
/// named a quantity, and this is the assertion that the CLI is such a
/// producer: without it the attribute is simply absent and nothing fails,
/// which is exactly how the CLI shipped a unit-less file while the GUI's
/// writer published one and its reader read it.
///
/// The listing this walks is the file's own: group attributes read back
/// through the same `rustyhdf5` reader `rspice convert --from hdf5` uses.
#[test]
fn every_signal_a_run_publishes_as_hdf5_states_its_unit() {
    let dir = test_dir("hdf5_units");
    let mut checked = 0_usize;
    // Node voltages and branch currents are the only quantities these four
    // decks project, so the whole vocabulary a correct file may use is two
    // symbols; a third would mean a column was given someone else's unit.
    let expected = ["V", "A"];
    for (tag, deck) in [
        ("op", OP_DECK),
        ("dc", DC_DECK),
        ("tran", TRAN_DECK),
        ("ac", AC_DECK),
    ] {
        let path = run_export(&dir, &format!("hdf5_units_{tag}"), deck, "hdf5");
        let file = rustyhdf5::File::open(&path).expect("open the published HDF5 file");
        let root = file.root();
        for group_name in root.groups().expect("list groups") {
            let group = file.group(&group_name).expect("open group");
            let attrs = group.attrs().expect("read group attributes");
            let Some(rustyhdf5::AttrValue::I64(count)) = attrs.get("signal_count") else {
                // `measurements` is the one group with no columns on it.
                continue;
            };
            for index in 0..usize::try_from(*count).expect("a non-negative signal count") {
                let prefix = format!("signal_{index:04}");
                let name = attrs.get(&format!("{prefix}_name"));
                let unit = attrs.get(&format!("{prefix}_unit"));
                let Some(rustyhdf5::AttrValue::String(unit)) = unit else {
                    panic!(
                        "{tag}: group '{group_name}' column {name:?} states no {prefix}_unit; \
                         attributes are {:?}",
                        attrs.keys().collect::<Vec<_>>()
                    );
                };
                assert!(
                    expected.contains(&unit.as_str()),
                    "{tag}: group '{group_name}' column {name:?} states unit {unit:?}, \
                     expected one of {expected:?}"
                );
                checked += 1;
            }
        }
    }
    assert!(
        checked >= 4,
        "expected the four analyses to publish columns, checked {checked}"
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

/// `-f vcd` publishes the event timelines themselves: every captured node, at
/// its own times, in the four states VCD can name.
///
/// The rawfile's event plots are the reference, because they are the lossless
/// carrier this dump is projected from. Every edge must land on the same
/// femtosecond, and an XSPICE `Unknown` — which the grid `D()` column
/// flattens to 0.5 along with high impedance — must read back as `x`.
#[test]
fn transient_vcd_carries_every_event_node_at_its_own_time() {
    use rspice_core::io::{VcdBit, VcdSignalKind, VcdValue};
    use rspice_core::xspice::DigitalState;

    let dir = test_dir("tran_xspice_event_vcd");
    let vcd = run_export(&dir, "events_vcd", XSPICE_EVENT_TRAN_DECK, "vcd");
    let raw = run_export(&dir, "events_reference", XSPICE_EVENT_TRAN_DECK, "raw");

    let reference = rspice_core::execution::decode_event_plots(
        &rspice_core::io::parse_raw_plots_file_with_limits(
            &raw,
            rspice_core::ResourceLimits::default(),
        )
        .expect("read the reference rawfile"),
    )
    .expect("decode the reference event plots");

    let document = rspice_core::io::parse_vcd_file(&vcd).expect("read the published dump");
    assert_eq!(
        document.signals.len(),
        reference.digital_traces.len() + reference.real_traces.len(),
        "every captured event node is one signal: {document:?}"
    );
    let period = document.timescale.femtoseconds();

    let signal_of = |node: &str| {
        document
            .signals
            .iter()
            .find(|signal| {
                signal
                    .variables
                    .iter()
                    .any(|variable| variable.name.eq_ignore_ascii_case(node))
            })
            .unwrap_or_else(|| panic!("node '{node}' is not in the dump: {document:?}"))
    };

    let mut unknowns = 0_usize;
    for trace in &reference.digital_traces {
        let signal = signal_of(&trace.node_name);
        assert_eq!(signal.kind, VcdSignalKind::Logic);
        assert_eq!(signal.width, 1);
        assert_eq!(
            signal.variables[0].scope,
            vec!["events".to_string()],
            "the run's nodes are declared under one scope"
        );
        assert_eq!(signal.changes.len(), trace.points.len());
        for (change, point) in signal.changes.iter().zip(&trace.points) {
            assert_eq!(
                change.tick * period,
                (point.time * 1e15).round() as u64,
                "an edge moved: tick {} at {period} fs/tick is not {} s",
                change.tick,
                point.time
            );
            let expected = match point.value.state {
                DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => VcdBit::Zero,
                DigitalState::One | DigitalState::OneR | DigitalState::OneZ => VcdBit::One,
                DigitalState::Unknown | DigitalState::UnknownR | DigitalState::UnknownZ => {
                    unknowns += 1;
                    VcdBit::Unknown
                }
                DigitalState::HighZ => VcdBit::HighImpedance,
            };
            assert_eq!(change.value, VcdValue::Logic(vec![expected]));
        }
    }
    assert!(
        unknowns > 0,
        "the bridged node passes through Unknown, which is what `x` is for"
    );

    for trace in &reference.real_traces {
        let signal = signal_of(&trace.node_name);
        assert_eq!(signal.kind, VcdSignalKind::Real);
        assert_eq!(signal.changes.len(), trace.points.len());
        for (change, point) in signal.changes.iter().zip(&trace.points) {
            assert_eq!(change.tick * period, (point.time * 1e15).round() as u64);
            assert_eq!(change.value, VcdValue::Real(point.value));
        }
    }

    let _ = std::fs::remove_dir_all(&dir);
}

/// A transient with nothing to dump still publishes a valid dump, and says so.
#[test]
fn transient_vcd_without_event_nodes_declares_nothing_and_warns() {
    let dir = test_dir("tran_vcd_empty");
    let deck = dir.join("empty_events.sp");
    std::fs::write(&deck, TRAN_DECK).expect("write deck");
    let out = dir.join("empty_events.vcd");

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().unwrap(),
            "-o",
            out.to_str().unwrap(),
            "-f",
            "vcd",
        ])
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "an event-free transient still publishes: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("captured no digital or real event node"),
        "the run must say the dump is empty: {stderr}"
    );

    let document = rspice_core::io::parse_vcd_file(&out).expect("an empty dump is still valid VCD");
    assert!(document.signals.is_empty());
    let text = std::fs::read_to_string(&out).expect("read the dump");
    assert!(
        text.contains("$enddefinitions $end"),
        "declarations are still closed: {text}"
    );
    assert!(
        !text.lines().any(|line| line.starts_with('#')),
        "there is no change to record: {text}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Only a transient captures events, so every other analysis refuses `-f vcd`
/// by name rather than publishing a dump of nothing.
#[test]
fn analyses_without_an_event_timeline_refuse_vcd_output() {
    let dir = test_dir("vcd_refusal");
    for (tag, deck, what) in [
        ("op", OP_DECK, "operating point"),
        ("dc", DC_DECK, "dc_sweep"),
        ("ac", AC_DECK, "ac"),
        ("noise", NOISE_DECK, "noise"),
    ] {
        let deck_path = dir.join(format!("{tag}.sp"));
        std::fs::write(&deck_path, deck).expect("write deck");
        let out = dir.join(format!("{tag}.vcd"));

        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "--quiet",
                "run",
                deck_path.to_str().unwrap(),
                "-o",
                out.to_str().unwrap(),
                "-f",
                "vcd",
            ])
            .output()
            .expect("run rspice");
        assert!(
            !output.status.success(),
            "{tag}: a result with no event timeline must not publish a dump"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(&format!("VCD output is not supported for {what} results")),
            "{tag}: unexpected refusal: {stderr}"
        );
        assert!(
            !out.exists(),
            "{tag}: a refused publication leaves no file behind"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}
