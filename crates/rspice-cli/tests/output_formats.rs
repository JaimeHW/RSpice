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
    let deck_path = dir.join(format!("{tag}.sp"));
    std::fs::write(&deck_path, deck).expect("write deck");
    let out_path = dir.join(format!("{tag}_{format}.out"));

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .arg("run")
        .arg(&deck_path)
        .arg("-o")
        .arg(&out_path)
        .arg("-f")
        .arg(format)
        .output()
        .expect("run rspice");

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

fn current_column_index(header: &str, name: &str) -> usize {
    header
        .split(',')
        .position(|column| column.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("{name} column missing from header: {header:?}"))
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
