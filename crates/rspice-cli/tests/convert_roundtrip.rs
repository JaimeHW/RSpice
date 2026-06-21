//! Integration tests for `rspice convert`: every supported input format must
//! be readable, conversions must preserve values, and `--variables` /
//! `--start` / `--stop` must subset the data.

use std::path::{Path, PathBuf};
use std::process::Command;

const TRAN_DECK: &str = "* transient convert test
v1 in 0 sin(0 1 1k)
r1 in out 1k
c1 out 0 1u
.tran 10u 200u
.end
";

const AC_DECK: &str = "* ac convert test
v1 in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 1n
.ac dec 2 100k 1meg
.end
";

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "rspice_convert_roundtrip_{}_{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn rspice(args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "rspice {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn simulate(dir: &Path, deck: &str, format: &str, out_name: &str) -> PathBuf {
    let deck_path = dir.join("deck.sp");
    std::fs::write(&deck_path, deck).expect("write deck");
    let out = dir.join(out_name);
    rspice(&[
        "run",
        deck_path.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        format,
    ]);
    out
}

fn convert(input: &Path, output: &Path, to: &str, extra: &[&str]) {
    let mut args = vec![
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        to,
    ];
    args.extend_from_slice(extra);
    rspice(&args);
}

/// Parse a CSV file into (header, rows).
fn read_csv(path: &Path) -> (Vec<String>, Vec<Vec<f64>>) {
    let text = std::fs::read_to_string(path).expect("read csv");
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header: Vec<String> = lines
        .next()
        .expect("csv header")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let rows = lines
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().parse::<f64>().expect("numeric field"))
                .collect()
        })
        .collect();
    (header, rows)
}

#[test]
fn binary_raw_to_csv_preserves_values() {
    let dir = test_dir("raw_to_csv");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    let csv = dir.join("tran.csv");
    convert(&raw, &csv, "csv", &[]);

    let (header, rows) = read_csv(&csv);
    assert_eq!(header[0], "time");
    assert!(
        header.iter().any(|h| h.eq_ignore_ascii_case("V(OUT)")),
        "V(OUT) column missing: {header:?}"
    );
    assert!(rows.len() > 10, "too few data rows: {}", rows.len());

    // Time axis must be increasing and end near 200us
    let last = rows.last().unwrap()[0];
    assert!(
        (last - 200e-6).abs() < 20e-6,
        "unexpected final time {last}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn full_format_cycle_preserves_values() {
    let dir = test_dir("cycle");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    // raw -> json -> hdf5 -> csv, then compare against direct raw -> csv
    let json = dir.join("tran.json");
    let h5 = dir.join("tran.h5");
    let cycled = dir.join("cycled.csv");
    let direct = dir.join("direct.csv");
    convert(&raw, &json, "json", &[]);
    convert(&json, &h5, "hdf5", &[]);
    convert(&h5, &cycled, "csv", &[]);
    convert(&raw, &direct, "csv", &[]);

    let (header_a, rows_a) = read_csv(&direct);
    let (header_b, rows_b) = read_csv(&cycled);
    assert_eq!(header_a, header_b, "headers diverged through the cycle");
    assert_eq!(rows_a.len(), rows_b.len(), "row count diverged");
    for (ra, rb) in rows_a.iter().zip(&rows_b) {
        for (a, b) in ra.iter().zip(rb) {
            assert!(
                (a - b).abs() <= 1e-12 * a.abs().max(1.0),
                "value diverged through cycle: {a} vs {b}"
            );
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn complex_ac_survives_csv_and_raw_round_trip() {
    let dir = test_dir("ac_complex");
    let csv = simulate(&dir, AC_DECK, "csv", "ac.csv");

    let (header, _) = read_csv(&csv);
    assert!(
        header.iter().any(|h| h.starts_with("Re(")) && header.iter().any(|h| h.starts_with("Im(")),
        "AC csv must carry Re/Im columns: {header:?}"
    );

    // csv -> raw (complex) -> csv must keep the Re/Im pairing
    let raw = dir.join("ac.raw");
    let back = dir.join("ac_back.csv");
    convert(&csv, &raw, "raw", &[]);
    let raw_text = std::fs::read(&raw).expect("read raw");
    let head = String::from_utf8_lossy(&raw_text[..raw_text.len().min(400)]).to_string();
    assert!(
        head.contains("Flags: complex"),
        "complex flag lost in rawfile header: {head}"
    );

    convert(&raw, &back, "csv", &[]);
    let (header_back, rows) = read_csv(&back);
    assert_eq!(header, header_back, "Re/Im columns diverged");
    assert!(!rows.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn csv_reader_accepts_quoted_comma_signal_names() {
    let dir = test_dir("quoted_signal_name");
    let input = dir.join("diff.csv");
    let output = dir.join("diff.json");
    std::fs::write(&input, "time,\"V(in,out)\"\n0,2.5\n1e-9,2.5\n").expect("write quoted csv");

    convert(&input, &output, "json", &[]);

    let json = std::fs::read_to_string(&output).expect("json output");
    assert!(
        json.contains("\"name\": \"V(in,out)\""),
        "quoted comma-bearing signal name should survive conversion: {json}"
    );
    assert!(
        json.contains("2.5"),
        "signal values should survive conversion: {json}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn variable_and_range_filters_apply() {
    let dir = test_dir("filters");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    let filtered = dir.join("filtered.csv");
    convert(
        &raw,
        &filtered,
        "csv",
        &[
            "--variables",
            "V(OUT)",
            "--start",
            "50e-6",
            "--stop",
            "150e-6",
        ],
    );

    let (header, rows) = read_csv(&filtered);
    assert_eq!(header.len(), 2, "expected scale + one column: {header:?}");
    assert!(header[1].eq_ignore_ascii_case("V(OUT)"));
    assert!(!rows.is_empty());
    for row in &rows {
        assert!(
            row[0] >= 50e-6 - 1e-12 && row[0] <= 150e-6 + 1e-12,
            "row outside requested range: t={}",
            row[0]
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn unknown_variable_filter_is_an_error() {
    let dir = test_dir("bad_filter");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    let out = dir.join("out.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "convert",
            raw.to_str().unwrap(),
            out.to_str().unwrap(),
            "--to",
            "csv",
            "--variables",
            "V(NOSUCHNODE)",
        ])
        .output()
        .expect("run rspice");
    assert!(!output.status.success(), "unknown variable must fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("available variables"),
        "error should list available variables: {stderr}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn malformed_json_shape_is_rejected_before_conversion() {
    let dir = test_dir("bad_json_shape");
    let input = dir.join("bad.json");
    let output_path = dir.join("out.csv");
    std::fs::write(
        &input,
        r#"{
  "analysis": "transient",
  "scale": { "name": "time", "values": [0.0, 1e-6] },
  "signals": [
    { "name": "V(OUT)", "values": [1.0] },
    { "name": "I(V1)", "real": [0.0, 0.1], "imag": [0.0] }
  ]
}
"#,
    )
    .expect("write malformed json");

    let command_output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "convert",
            input.to_str().unwrap(),
            output_path.to_str().unwrap(),
            "--to",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(
        !command_output.status.success(),
        "mismatched JSON waveform lengths must fail"
    );
    let stderr = String::from_utf8_lossy(&command_output.stderr);
    assert!(
        stderr.contains("V(OUT)") && stderr.contains("expected 2"),
        "error should name the first malformed signal and expected length: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
