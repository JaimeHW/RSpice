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
    let text = common::delimited_data_text(text, delimiter as u8);
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

/// `-f raw` is the binary rawfile for the operating point too; `-f ascii`
/// is the text one. Both carry the same values.
#[test]
fn op_raw_is_binary_and_ascii_is_text() {
    let dir = test_dir("op_raw_encoding");
    let binary = std::fs::read(run_export(&dir, "op_binary", OP_DECK, "raw")).unwrap();
    let marker = b"Binary:\n";
    let start = binary
        .windows(marker.len())
        .position(|window| window == marker)
        .expect("binary rawfile has a Binary: section")
        + marker.len();
    let values: Vec<f64> = binary[start..]
        .chunks_exact(8)
        .map(|bytes| f64::from_le_bytes(bytes.try_into().unwrap()))
        .collect();
    assert_eq!(values.len(), 3, "V(in), V(out), I(v1): {values:?}");
    assert_eq!(&values[..2], &[5.0, 2.5]);
    assert!((values[2] + 2.5e-3).abs() < 1e-15, "{values:?}");

    let ascii = std::fs::read_to_string(run_export(&dir, "op_ascii", OP_DECK, "ascii")).unwrap();
    assert!(
        ascii.contains("Values:\n") && !ascii.contains("Binary:"),
        "{ascii}"
    );
    let _ = std::fs::remove_dir_all(&dir);
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

/// `.PRINT AC VR(out) VI(out) VM(out) VP(out) VDB(out)` prints one real
/// column each, as Xyce does; only the bare `V(out)` prints a complex pair.
#[test]
fn ac_rendering_accessors_export_one_real_column_each() {
    let dir = test_dir("ac_rendering_csv");
    let deck = "* ac rendering accessors
v1 in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 1n
.ac dec 1 100k 1meg
.print ac vr(out) vi(out) vm(out) vp(out) vdb(out) ir(v1) v(out)
.end
";
    let path = run_export(&dir, "ac_rendering", deck, "csv");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let mut lines = text.lines();
    let header = lines
        .next()
        .expect("csv header")
        .split(',')
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    assert_eq!(
        header,
        [
            "frequency",
            "vr(out)",
            "vi(out)",
            "vm(out)",
            "vp(out)",
            "vdb(out)",
            "ir(v1)",
            "re(v(out))",
            "im(v(out))"
        ],
        "{text}"
    );
    let row = lines
        .next()
        .expect("a first frequency")
        .split(',')
        .map(|field| field.parse::<f64>().expect("a number"))
        .collect::<Vec<_>>();
    let (re, im) = (row[1], row[2]);
    let close = |left: f64, right: f64| (left - right).abs() <= 1e-12 * right.abs().max(1.0);
    assert!(close(row[3], re.hypot(im)), "{text}");
    assert!(close(row[4], im.atan2(re).to_degrees()), "{text}");
    assert!(close(row[5], 20.0 * re.hypot(im).log10()), "{text}");
    assert!(close(row[7], re) && close(row[8], im), "{text}");
    assert!(
        row[4] < 0.0 && row[4] > -90.0,
        "an RC lag in degrees: {text}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// `.PRINT NOISE` prints its own operands, as Xyce does: the small-signal
/// solution as on an AC card, and `ONOISE`, `INOISE` and `DNO()` as spectral
/// densities. A 1k/1k divider has an exact answer: the output sees
/// 4kT·500 Ω V²/Hz, half of it from R1, through a gain of one half.
#[test]
fn noise_print_card_exports_its_own_columns() {
    let dir = test_dir("noise_print_csv");
    let deck = "* noise print columns
v1 in 0 dc 0 ac 1
r1 in out 1k
r2 out 0 1k
.noise v(out) v1 lin 3 100 1000
.print noise vr(out) vm(out) onoise inoise dno(r1) {sqrt(onoise)} onoise_spectrum v(out)
.end
";
    let path = run_export(&dir, "noise_print", deck, "csv");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let mut lines = text.lines();
    let header = lines
        .next()
        .expect("csv header")
        .split(',')
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    assert_eq!(
        header,
        [
            "frequency",
            "vr(out)",
            "vm(out)",
            "onoise",
            "inoise",
            "dno(r1)",
            "{sqrt(onoise)}",
            "onoise_spectrum",
            "re(v(out))",
            "im(v(out))"
        ],
        "{text}"
    );
    let four_kt = 4.0 * 1.380649e-23 * 300.15;
    let close = |left: f64, right: f64| (left - right).abs() <= 1e-9 * right.abs().max(1e-30);
    let mut rows = 0;
    for line in lines.filter(|line| !line.starts_with('#')) {
        let row = line
            .split(',')
            .map(|value| value.parse::<f64>().expect("numeric csv cell"))
            .collect::<Vec<_>>();
        assert!(close(row[1], 0.5) && close(row[2], 0.5), "{text}");
        assert!(close(row[3], four_kt * 500.0), "{text}");
        assert!(close(row[4], four_kt * 2000.0), "{text}");
        assert!(close(row[5], four_kt * 250.0), "{text}");
        assert!(close(row[6], (four_kt * 500.0).sqrt()), "{text}");
        assert!(close(row[7], (four_kt * 500.0).sqrt()), "{text}");
        assert!(close(row[8], 0.5) && row[9].abs() <= 1e-15, "{text}");
        rows += 1;
    }
    assert_eq!(rows, 3, "{text}");

    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT DC` card prints a deck's `.OP`, which Xyce runs as a
/// one-point DC analysis: Xyce 7.10 prints one row of V(1) 1, V(2) 0.5 and
/// I(V1) -5e-4 for this deck.
#[test]
fn xyce_print_dc_card_prints_an_operating_point() {
    let dir = test_dir("print_dc_operating_point");
    let deck = "* print dc over op\nV1 1 0 1\nR1 1 2 1k\nR2 2 0 1k\n\
                .op\n.print dc v(1) v(2) i(v1)\n.end\n";
    let path = run_export_with_args(&dir, "op", deck, "csv", &["--spice-dialect", "xyce"]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    for (name, expected) in [("V(1)", 1.0), ("V(2)", 0.5), ("I(V1)", -5e-4)] {
        let row = text
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{name},")))
            .unwrap_or_else(|| panic!("no {name} row: {text}"));
        let value: f64 = row.parse().expect("value");
        assert!(
            (value - expected).abs() <= 1e-9 * f64::abs(expected),
            "{name}: {value}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT AC` or `.PRINT NOISE` card prints a `device:param` on every
/// row, the value `.PRINT DC` prints at the analysis's own bias point: Xyce
/// 7.10 prints R1:R 1e3, C1:C 1e-6, M1:W 1e-5 and M1:L 2e-6 on both rows of
/// this deck's AC and NOISE sweeps. M1:GM, which RSpice also reports, is the
/// LEVEL 1 transconductance KP*W/L*(VGS-VTO) = 1.5e-4 at VGS = 1.
#[test]
fn xyce_prints_device_parameters_on_ac_and_noise_cards_at_their_bias() {
    let dir = test_dir("print_device_parameters");
    let devices = "V1 1 0 DC 1 AC 1\nR1 1 2 1k\nC1 2 0 1u\nM1 4 2 0 0 NMOD W=10u L=2u\n\
                   RD 5 4 1k\nVDD 5 0 DC 3\n.model NMOD NMOS LEVEL=1 VTO=0.7 KP=1e-4\n";
    let parameters = "R1:R C1:C M1:W M1:L M1:GM";
    for (tag, analysis) in [
        (
            "ac",
            format!(".ac dec 1 100 1k\n.print ac vr(2) {parameters}\n"),
        ),
        (
            "noise",
            format!(".noise v(2) V1 dec 1 100 1k\n.print noise onoise {parameters}\n"),
        ),
    ] {
        let deck = format!("* device parameters\n{devices}{analysis}.end\n");
        let path = run_export_with_args(&dir, tag, &deck, "csv", &["--spice-dialect", "xyce"]);
        let text = std::fs::read_to_string(&path).expect("read csv");
        for (name, expected) in [
            ("R1:R", 1e3),
            ("C1:C", 1e-6),
            ("M1:W", 1e-5),
            ("M1:L", 2e-6),
            ("M1:GM", 1.5e-4),
        ] {
            let column = common::csv_column(&text, name);
            assert_eq!(column.len(), 2, "{tag} {name}: {text}");
            for value in column {
                assert!(
                    (value - expected).abs() <= 1e-9 * expected,
                    "{tag} {name}: {value}, expected {expected}"
                );
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT` card prints a temperature coefficient and a top-level
/// model card's parameter in every analysis. Xyce 7.10 prints, on every DC,
/// TRAN, AC and NOISE row of this deck: DMOD:IS 1e-14, R1:TC1 0.01, R2:TC1 0 and R1:TC2 0 (neither
/// authored), R3:TC1 0.02 (its model card's), RMOD:TC1 0.02, C1:TC1 3e-3,
/// L1:TC1 4e-3, X1:RS:TC1 5e-3 and {R1:TC1*2} 0.02.
#[test]
fn xyce_prints_temperature_coefficients_and_model_parameters_in_every_analysis() {
    let dir = test_dir("print_model_parameters");
    let devices = "V1 1 0 DC 1 AC 1\nR1 1 2 1k TC1=0.01\nR2 2 3 1k\nD1 3 0 DMOD\n\
                   .model DMOD D IS=1e-14\n.model RMOD R TC1=0.02\nR3 2 0 RMOD 2k\n\
                   C1 2 0 1u TC1=0.003\nL1 2 4 1m TC1=0.004\nR4 4 0 1k\nX1 2 0 SUB\n\
                   .subckt SUB a b\nRS a c 1k TC1=0.005\nDS c b DMS\n.model DMS D IS=3e-14\n\
                   .ends\n";
    let parameters =
        "DMOD:IS R1:TC1 R2:TC1 R1:TC2 R3:TC1 RMOD:TC1 C1:TC1 L1:TC1 X1:RS:TC1 {R1:TC1*2}";
    for (tag, analysis) in [
        ("dc", format!(".dc V1 1 2 1\n.print dc v(2) {parameters}\n")),
        (
            "tran",
            format!(".tran 0 1u\n.print tran v(2) {parameters}\n"),
        ),
        (
            "ac",
            format!(".ac dec 1 100 1k\n.print ac vr(2) {parameters}\n"),
        ),
        (
            "noise",
            format!(".noise v(2) V1 dec 1 100 1k\n.print noise onoise {parameters}\n"),
        ),
    ] {
        let deck = format!("* model parameters\n{devices}{analysis}.end\n");
        let path = run_export_with_args(&dir, tag, &deck, "csv", &["--spice-dialect", "xyce"]);
        let text = std::fs::read_to_string(&path).expect("read csv");
        for (name, expected) in [
            ("R1:TC1", 0.01),
            ("R2:TC1", 0.0),
            ("R1:TC2", 0.0),
            ("R3:TC1", 0.02),
            ("RMOD:TC1", 0.02),
            ("C1:TC1", 3e-3),
            ("L1:TC1", 4e-3),
            ("X1:RS:TC1", 5e-3),
            ("{R1:TC1*2}", 0.02),
        ] {
            let column = common::csv_column(&text, name);
            assert!(column.len() >= 2, "{tag} {name}: {text}");
            for value in column {
                assert!(
                    (value - expected).abs() <= 1e-12,
                    "{tag} {name}: {value}, expected {expected}"
                );
            }
        }
        let is = common::csv_column(&text, "DMOD:IS");
        assert!(is.len() >= 2, "{tag} DMOD:IS: {text}");
        for value in is {
            assert!((value - 1e-14).abs() <= 1e-23, "{tag} DMOD:IS: {value}");
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.DC` sweep may name a model card's parameter; the swept value is
/// what `.PRINT` reports for it. Xyce 7.10 prints these two points, the
/// second 1.8e-6 V short of the root, inside its Newton tolerance.
#[test]
fn xyce_dc_sweeps_a_model_parameter_and_prints_its_value() {
    let dir = test_dir("dc_model_parameter");
    let deck = "* model parameter sweep\nV1 1 0 DC 1\nR1 1 2 1k\nD1 2 0 DMOD\n\
                .model DMOD D IS=1e-14\n.dc DMOD:IS 1e-14 2e-14 1e-14\n\
                .print dc v(2) DMOD:IS\n.end\n";
    let path = run_export_with_args(&dir, "dc", deck, "csv", &["--spice-dialect", "xyce"]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    let v2 = common::csv_column(&text, "V(2)");
    let is = common::csv_column(&text, "DMOD:IS");
    assert_eq!(v2.len(), 2, "{text}");
    assert_eq!(is.len(), 2, "{text}");
    for (index, (expected_v2, expected_is)) in [(6.29424162e-01, 1e-14), (6.12643787e-01, 2e-14)]
        .into_iter()
        .enumerate()
    {
        assert!((v2[index] - expected_v2).abs() <= 1e-5, "{text}");
        assert!((is[index] - expected_is).abs() <= 1e-23, "{text}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT AC` expression is evaluated in complex arithmetic: one
/// column when its type is real, `Re()` and `Im()` columns when a `V()`
/// phasor reaches its root. Xyce 7.10 prints, at 100 Hz and 200 Hz of this
/// RC pole (H = 1/(1 + j f/159.15 Hz)), the values below.
#[test]
fn xyce_prints_an_ac_expression_in_complex_arithmetic() {
    let dir = test_dir("print_ac_expression");
    let deck = "* ac expression\nV1 1 0 AC 1\nR1 1 2 1k\nC1 2 0 1u\n.param k=2\n\
                .ac lin 2 100 200\n.print ac {vr(2)*k} {(vm(2)-vr(2))/k} {v(2)*2} {vp(2)} \
                {abs(v(2))}\n.end\n";
    let path = run_export_with_args(&dir, "xyce", deck, "csv", &["--spice-dialect", "xyce"]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    for (name, expected) in [
        ("{vr(2)*k}", [1.43391360e+00, 7.75453273e-01]),
        ("{(vm(2)-vr(2))/k}", [6.48881078e-02, 1.17475178e-01]),
        ("Re({v(2)*2})", [1.43391360e+00, 7.75453273e-01]),
        ("Im({v(2)*2})", [-9.00954487e-01, -9.74463323e-01]),
        ("{vp(2)}", [-3.21419076e+01, -5.14881127e+01]),
        ("{abs(v(2))}", [8.46733016e-01, 6.22676992e-01]),
    ] {
        let column = common::csv_column(&text, name);
        assert_eq!(column.len(), 2, "{name}: {text}");
        for (value, expected) in column.iter().zip(expected) {
            assert!(
                (value - expected).abs() <= 1e-8 * expected.abs(),
                "{name}: {value}, expected {expected}"
            );
        }
    }

    let ngspice_deck = dir.join("ngspice.sp");
    std::fs::write(
        &ngspice_deck,
        "* ac expression\nV1 1 0 AC 1\nR1 1 2 1k\nC1 2 0 1u\n.ac lin 2 100 200\n\
         .print ac {v(2)*2}\n.end\n",
    )
    .expect("write deck");
    let ngspice = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            "--spice-dialect",
            "ngspice",
            "-f",
            "csv",
            "-o",
        ])
        .arg(dir.join("ngspice.csv"))
        .arg(&ngspice_deck)
        .output()
        .expect("run rspice");
    assert!(
        !ngspice.status.success(),
        "the ngspice dialect keeps its refusal"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT HB` expression is evaluated over the spectrum in complex
/// arithmetic, bin by bin: a constant lands on every bin, the DC one and each
/// harmonic alike. Xyce 7.10 prints, for this divider driven by a 1 V sine,
/// `Re({V(OUT)+1.0})` = 1 on all eleven bins, `Im({V(OUT)+1.0})` =
/// `Im(V(OUT))`, and `{V(OUT)*2}` = twice `V(OUT)`.
#[test]
fn xyce_prints_an_hb_expression_in_complex_arithmetic() {
    let dir = test_dir("print_hb_expression");
    let deck = "* hb expression\n.hb 1e5\n.print hb v(out) {v(out)+1.0} {v(out)*2}\n\
                .options hbint numfreq=5\nV1 in 0 sin 0 1 1e5 0 0\nR1 in out 1k\n\
                R2 out 0 1k\n.end\n";
    let path = run_export_with_args(&dir, "xyce", deck, "csv", &["--spice-dialect", "xyce"]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    let re = common::csv_column(&text, "Re(V(OUT))");
    let im = common::csv_column(&text, "Im(V(OUT))");
    let shifted_re = common::csv_column(&text, "Re({v(out)+1.0})");
    let shifted_im = common::csv_column(&text, "Im({v(out)+1.0})");
    let doubled_re = common::csv_column(&text, "Re({v(out)*2})");
    let doubled_im = common::csv_column(&text, "Im({v(out)*2})");
    assert!(re.len() > 1, "{text}");
    for bin in 0..re.len() {
        assert!((shifted_re[bin] - (re[bin] + 1.0)).abs() <= 1e-12, "{text}");
        assert!((shifted_im[bin] - im[bin]).abs() <= 1e-12, "{text}");
        assert!((doubled_re[bin] - 2.0 * re[bin]).abs() <= 1e-12, "{text}");
        assert!((doubled_im[bin] - 2.0 * im[bin]).abs() <= 1e-12, "{text}");
    }
    assert!(
        im.iter().any(|value| value.abs() > 0.1),
        "the drive reaches V(OUT): {text}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT HB` reads a resistor's and a capacitor's current from the
/// solved spectrum, as it reads an inductor's or a source's branch unknown.
/// Xyce 7.10 prints, at the 100 kHz bin of this RLC driven by a 1 V sine,
/// V(OUT) 0.249824676-0.259361152j, I(R1) -2.49824676e-4-2.40638848e-4j and
/// I(C1) 1.62961418e-4+1.56969473e-4j: I(R1)/V(OUT) = -9.63230901e-4j and
/// I(C1)/V(OUT) = 6.28318530e-4j (jwC). A deck that names no device current
/// keeps the node voltages and branch unknowns alone.
#[test]
fn xyce_prints_an_hb_resistor_and_capacitor_current() {
    let dir = test_dir("print_hb_device_current");
    let circuit = ".options hbint numfreq=5\nV1 in 0 sin 0 1 1e5 0 0\nR1 in out 1k\n\
                   C1 out 0 1n\nL1 out 0 1m\n.end\n";
    let deck = format!("* hb currents\n.hb 1e5\n.print hb v(out) i(R1) i(C1) i(L1)\n{circuit}");
    let path = run_export_with_args(&dir, "named", &deck, "csv", &["--spice-dialect", "xyce"]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    let at_fundamental = |name: &str| {
        let frequency = common::csv_column(&text, "frequency");
        let row = frequency
            .iter()
            .position(|value| (value - 1e5).abs() < 1e-3)
            .unwrap_or_else(|| panic!("no 100 kHz row: {text}"));
        let re = common::csv_column(&text, &format!("Re({name})"));
        let im = common::csv_column(&text, &format!("Im({name})"));
        (re[row], im[row])
    };
    let (v_re, v_im) = at_fundamental("V(OUT)");
    let norm = v_re * v_re + v_im * v_im;
    assert!(norm > 0.01, "the drive reaches V(OUT): {text}");
    for (name, expected) in [("I(R1)", -9.63230901e-4), ("I(C1)", 6.28318530e-4)] {
        let (i_re, i_im) = at_fundamental(name);
        // I / V = (i_re + j i_im)(v_re - j v_im) / |V|^2
        let ratio_re = (i_re * v_re + i_im * v_im) / norm;
        let ratio_im = (i_im * v_re - i_re * v_im) / norm;
        assert!(ratio_re.abs() <= 1e-9, "{name}: {ratio_re}+{ratio_im}j");
        assert!(
            (ratio_im - expected).abs() <= 1e-8 * expected.abs(),
            "{name}: {ratio_im}j, expected {expected}j"
        );
    }

    let bare = format!("* hb currents\n.hb 1e5\n{circuit}");
    let path = run_export_with_args(&dir, "bare", &bare, "csv", &["--spice-dialect", "xyce"]);
    let header = std::fs::read_to_string(&path).expect("read csv");
    let header = header.lines().next().expect("csv header");
    assert!(header.contains("Re(I(L1))"), "{header}");
    assert!(
        !header.contains("I(R1)") && !header.contains("I(C1)"),
        "{header}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT HB` lead accessor, `ID(M1)` ... `IE(Q1)`, reads the
/// current into that terminal from the solved spectrum, as `I(D1)` reads a
/// diode's. Xyce 7.10 prints, at the 100 kHz bin of this deck, these
/// currents over V(D) (each ratio cancels the bin convention): ID(M1)
/// -3.37902494e-2-1.06070555e-5j, IG(M1) 1.47302841e-10-6.63089803e-7j,
/// IS(M1) 3.37902492e-2+1.12701453e-5j, I(D1) 3.32902494e-2+1.06070555e-5j,
/// IC(Q1) -8.45207099e-4+1.17219572e-6j, IB(Q1) -8.45173265e-6-3.92718064e-6j
/// and IE(Q1) 8.53658829e-4+2.75498492e-6j.
#[test]
fn xyce_prints_hb_lead_currents_by_their_accessors() {
    let dir = test_dir("print_hb_lead_current");
    let deck = "* hb leads\n.hb 1e5\n\
                .print hb v(d) id(M1) ig(M1) is(M1) i(D1) ic(Q1) ib(Q1) ie(Q1)\n\
                .options hbint numfreq=5\nVDD vdd 0 3\nVG g 0 sin 1.5 0.2 1e5 0 0\n\
                RD vdd d 2k\nM1 d g 0 0 nm L=1u W=10u\n\
                .model nm NMOS level=1 VTO=0.7 KP=1e-4 CGSO=1e-9 CGDO=1e-9\n\
                D1 d 0 dm\n.model dm D IS=1e-14 CJO=10p\nVB b 0 sin 0.65 0.01 1e5 0 0\n\
                RC vdd c 1k\nQ1 c b 0 qm\n.model qm NPN IS=1e-16 BF=100 CJE=1p CJC=1p\n.end\n";
    let path = run_export_with_args(&dir, "leads", deck, "csv", &["--spice-dialect", "xyce"]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    let frequency = common::csv_column(&text, "frequency");
    let row = frequency
        .iter()
        .position(|value| (value - 1e5).abs() < 1e-3)
        .unwrap_or_else(|| panic!("no 100 kHz row: {text}"));
    let phasor = |name: &str| {
        let re = common::csv_column(&text, &format!("Re({name})"));
        let im = common::csv_column(&text, &format!("Im({name})"));
        (re[row], im[row])
    };
    let (v_re, v_im) = phasor("V(D)");
    let norm = v_re * v_re + v_im * v_im;
    for (name, expected) in [
        ("id(M1)", (-3.37902494e-2, -1.06070555e-5)),
        ("ig(M1)", (1.47302841e-10, -6.63089803e-7)),
        ("is(M1)", (3.37902492e-2, 1.12701453e-5)),
        ("I(D1)", (3.32902494e-2, 1.06070555e-5)),
        ("ic(Q1)", (-8.45207099e-4, 1.17219572e-6)),
        ("ib(Q1)", (-8.45173265e-6, -3.92718064e-6)),
        ("ie(Q1)", (8.53658829e-4, 2.75498492e-6)),
    ] {
        let (i_re, i_im) = phasor(name);
        let ratio = (
            (i_re * v_re + i_im * v_im) / norm,
            (i_im * v_re - i_re * v_im) / norm,
        );
        let error = (ratio.0 - expected.0).hypot(ratio.1 - expected.1);
        assert!(
            error <= 1e-5 * expected.0.hypot(expected.1),
            "{name}: {ratio:?}, expected {expected:?}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// A Xyce `.PRINT HB` writes the two-sided spectrum, a row per bin from
/// -NUMFREQ f0 to NUMFREQ f0: Xyce 7.10 prints eleven rows for NUMFREQ=5,
/// V(OUT) 0.249824676+0.259361152j at -100 kHz and its conjugate at
/// +100 kHz, half the peak phasor. The default dialect keeps the one-sided
/// peak phasors, DC to the fifth harmonic.
#[test]
fn xyce_prints_the_two_sided_hb_spectrum() {
    let dir = test_dir("print_hb_two_sided");
    let deck = "* hb rows\n.hb 1e5\n.print hb v(out)\n.options hbint numfreq=5\n\
                V1 in 0 sin 0 1 1e5 0 0\nR1 in out 1k\nC1 out 0 1n\nL1 out 0 1m\n.end\n";
    let path = run_export_with_args(&dir, "xyce", deck, "csv", &["--spice-dialect", "xyce"]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    let frequency = common::csv_column(&text, "frequency");
    let expected = (-5..=5).map(|k| f64::from(k) * 1e5).collect::<Vec<_>>();
    assert_eq!(frequency.len(), expected.len(), "{text}");
    for (actual, expected) in frequency.iter().zip(&expected) {
        assert!((actual - expected).abs() <= 1e-6, "{text}");
    }
    let re = common::csv_column(&text, "Re(V(OUT))");
    let im = common::csv_column(&text, "Im(V(OUT))");
    for (row, sign) in [(4, 1.0), (6, -1.0)] {
        assert!((re[row] - 0.249824676).abs() <= 1e-9, "{text}");
        assert!((im[row] - sign * 0.259361152).abs() <= 1e-9, "{text}");
    }

    let path = run_export_with_args(&dir, "best", deck, "csv", &[]);
    let text = std::fs::read_to_string(&path).expect("read csv");
    let frequency = common::csv_column(&text, "frequency");
    assert_eq!(frequency.len(), 6, "{text}");
    let re = common::csv_column(&text, "Re(V(OUT))");
    let im = common::csv_column(&text, "Im(V(OUT))");
    assert!((re[1] - 2.0 * 0.249824676).abs() <= 2e-9, "{text}");
    assert!((im[1] + 2.0 * 0.259361152).abs() <= 2e-9, "{text}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A two-point LIN sweep visits both ends in every dialect, as Xyce, LTspice,
/// QSPICE and ngspice's noise analysis step it; ngspice's AC analysis alone
/// visits only the start (acan.c:104-113). `.DISTO`, which only ngspice runs,
/// keeps its N + 2 points (distoan.c:79-85).
#[test]
fn a_two_point_lin_sweep_visits_both_ends() {
    let dir = test_dir("lin_two_point");
    for (dialect, card, expected) in [
        ("best", ".ac lin 2 100 1k", vec![100.0, 1000.0]),
        ("xyce", ".ac lin 2 100 1k", vec![100.0, 1000.0]),
        ("ngspice", ".ac lin 2 100 1k", vec![100.0, 1000.0]),
        ("best", ".noise v(out) v1 lin 2 100 1k", vec![100.0, 1000.0]),
        ("xyce", ".noise v(out) v1 lin 2 100 1k", vec![100.0, 1000.0]),
        (
            "ngspice",
            ".noise v(out) v1 lin 2 100 1k",
            vec![100.0, 1000.0],
        ),
        (
            "ngspice",
            ".disto lin 2 100 400",
            vec![100.0, 200.0, 300.0, 400.0],
        ),
    ] {
        let deck = format!(
            "* lin two\nv1 in 0 dc 0.5 ac 1 distof1 1m 0\nr1 in out 1k\nd1 out 0 dmod\n\
             .model dmod d is=1e-14\n{card}\n.end\n"
        );
        let tag = format!("lin_{dialect}_{}", &card[1..3]);
        let path = run_export_with_args(&dir, &tag, &deck, "csv", &["--spice-dialect", dialect]);
        let text = std::fs::read_to_string(&path).expect("read csv");
        let frequencies = text
            .lines()
            .skip(1)
            .filter(|line| !line.starts_with('#'))
            .map(|line| {
                line.split(',')
                    .next()
                    .and_then(|cell| cell.parse::<f64>().ok())
                    .expect("frequency cell")
            })
            .collect::<Vec<_>>();
        assert_eq!(frequencies, expected, "{dialect} {card}: {text}");
    }

    let _ = std::fs::remove_dir_all(&dir);
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
    let text = common::delimited_data_text(&text, b',');
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
    let data = common::delimited_data_text(&text, b',');
    let times = data
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
    let times = common::delimited_data_text(&plain_text, b',')
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
    let text = common::delimited_data_text(text, b',');
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
    let text = common::delimited_data_text(&text, b',');
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
    let text = common::delimited_data_text(&text, b',');
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

/// The authored `.PRINT` spelling titles a printed table. A rawfile is the
/// waveform database, as in ngspice `-r`: it keeps every vector under its own
/// name and converts back unchanged.
#[test]
fn a_spaced_print_card_leaves_a_rawfile_every_vector_under_its_own_name() {
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
        let header = csv.lines().next().expect("CSV header");
        let columns = header.split(',').collect::<Vec<_>>();
        assert_eq!(columns[0], "time");
        for vector in ["V(IN)", "V(OUT)", "I(V1)"] {
            assert!(columns.contains(&vector), "{vector} missing: {header}");
        }
        assert!(
            !header.contains("{V(out)+1}") && !header.contains("I ( V1 )"),
            "a print column reached the rawfile: {header}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn transient_export_includes_xspice_digital_traces() {
    let dir = test_dir("tran_xspice_digital");
    let path = run_export(&dir, "tran_xspice_digital", XSPICE_DIGITAL_TRAN_DECK, "csv");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let text = common::delimited_data_text(&text, b',');
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

        let file = rspice_core::io::parse_raw_plots_bytes_with_limits(
            &std::fs::read(&path).expect("read the raw file"),
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

        // A real event node is a held `E(node)` column of the analysis plot,
        // as of every table, and the event plot carries its own timeline.
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
            analysis
                .variables
                .iter()
                .any(|variable| variable.name.eq_ignore_ascii_case("E(rnode)")),
            "{format}: a real event node is a column of the analysis plot"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// `convert` and `compare` read the analysis plot and are untroubled by the
/// plots behind it.
#[test]
fn explicit_grid_selection_reads_a_multi_plot_rawfile() {
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
            .args(["--section", "Transient Analysis"])
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
            .args(["--section", "Transient Analysis"])
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
    let text = common::delimited_data_text(&text, b',');
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
fn transient_voltage_save_refuses_an_xspice_event_only_net() {
    let dir = test_dir("tran_xspice_digital_voltage_save");
    let deck_path = dir.join("voltage_save.sp");
    let out_path = dir.join("voltage_save.csv");
    std::fs::write(&deck_path, XSPICE_DIGITAL_VOLTAGE_SAVE_TRAN_DECK).expect("write deck");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json", "--quiet", "run"])
        .arg(&deck_path)
        .arg("-o")
        .arg(&out_path)
        .args(["-f", "csv"])
        .output()
        .expect("run rspice");

    assert_eq!(output.status.code(), Some(65));
    let diagnostic: serde_json::Value =
        serde_json::from_slice(&output.stderr).expect("one JSON diagnostic");
    assert_eq!(diagnostic["error"]["category"], "netlist");
    assert_eq!(diagnostic["error"]["code"], "netlist_error");
    assert_eq!(diagnostic["error"]["analysis"], "Transient");
    assert!(
        diagnostic["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("V(d)") && message.contains("event-only net")),
        "the unavailable voltage must be identified: {diagnostic}"
    );
    assert!(
        !out_path.exists(),
        "a refused output must not publish a file"
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
        &rspice_core::io::parse_raw_plots_bytes_with_limits(
            &std::fs::read(&raw).expect("read the raw file"),
            rspice_core::ResourceLimits::default(),
        )
        .expect("read the reference rawfile"),
    )
    .expect("decode the reference event plots");

    let document = rspice_core::io::parse_vcd_bytes_with_limits(
        &std::fs::read(&vcd).expect("read the VCD file"),
        Default::default(),
    )
    .expect("read the published dump");
    assert_eq!(
        document.signals.len(),
        reference.digital_traces.len() + reference.real_traces.len(),
        "every captured event node is one signal: {document:?}"
    );
    let period = document.timescale.femtoseconds();
    // The nearest femtosecond to the exact time, as the dump rounds it: the
    // rounded product alone can land on the wrong side of a half.
    let femtoseconds = |time: f64| {
        let integral = (time * 1e15).round_ties_even();
        let correction = time.mul_add(1e15, -integral).round_ties_even();
        (integral + correction) as u64
    };

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
                femtoseconds(point.time),
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
            assert_eq!(change.tick * period, femtoseconds(point.time));
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

    let document = rspice_core::io::parse_vcd_bytes_with_limits(
        &std::fs::read(&out).expect("read the VCD file"),
        Default::default(),
    )
    .expect("an empty dump is still valid VCD");
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

/// Run a deck in the Xyce dialect without `-o`, from `dir`, and return what
/// the run printed.
fn run_xyce_files(dir: &Path, name: &str, deck: &str, extra_args: &[&str]) -> std::process::Output {
    let deck_path = dir.join(name);
    std::fs::write(&deck_path, deck).expect("write deck");
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("run")
        .arg(&deck_path)
        .args(["--spice-dialect", "xyce"])
        .args(extra_args)
        .output()
        .expect("run rspice")
}

fn measure_line(text: &str, name: &str) -> f64 {
    let value = text
        .strip_prefix(&format!("{name} = "))
        .and_then(|rest| rest.strip_suffix('\n'))
        .unwrap_or_else(|| panic!("not one `{name} = value` line: {text:?}"));
    let (mantissa, exponent) = value.split_once('e').expect("scientific value");
    assert_eq!(
        mantissa.trim_start_matches('-').len(),
        8,
        "%e keeps six digits: {value}"
    );
    assert!(
        exponent.len() == 3 && exponent.starts_with(['+', '-']),
        "{value}"
    );
    value.parse().expect("number")
}

/// In the Xyce dialect without `-o`, a run writes the files Xyce writes beside
/// the deck: the `.PRINT TRAN` table as `deck.cir.prn` in Xyce's padded
/// layout (`Index`, then `TIME`, %.8e in 17 columns) and the measures as
/// `deck.cir.mt0`. Xyce 7.10 on this deck: `MX = 4.999385e-01`.
#[test]
fn xyce_writes_the_print_table_and_the_measures_beside_the_deck() {
    let dir = test_dir("xyce_files_tran");
    let deck = "* xyce files\nV1 1 0 sin 0 1 1k\nR1 1 2 1k\nR2 2 0 1k\n.tran 10u 1m\n\
                .print tran v(2)\n.meas tran mx max v(2)\n.end\n";
    let output = run_xyce_files(&dir, "rc.cir", deck, &[]);
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    for file in ["rc.cir.prn", "rc.cir.mt0"] {
        let shown = dir.join(file).display().to_string();
        assert_eq!(
            stdout
                .lines()
                .filter(|line| *line == format!("wrote {shown}"))
                .count(),
            1,
            "{stdout}"
        );
    }

    let table = std::fs::read_to_string(dir.join("rc.cir.prn")).expect("deck.cir.prn");
    let lines: Vec<&str> = table.lines().collect();
    assert_eq!(lines[0], "Index       TIME              V(2)       ");
    assert_eq!(lines[1], "0        0.00000000e+00    0.00000000e+00");
    assert_eq!(*lines.last().unwrap(), "End of Xyce(TM) Simulation");
    let rows = &lines[1..lines.len() - 1];
    assert!(rows.len() > 100, "{table}");
    for (index, row) in rows.iter().enumerate() {
        let fields: Vec<&str> = row.split_whitespace().collect();
        assert_eq!(fields.len(), 3, "{row}");
        assert_eq!(fields[0], index.to_string());
    }
    assert!(rows.last().unwrap().contains("1.00000000e-03"), "{table}");

    let measures = std::fs::read_to_string(dir.join("rc.cir.mt0")).expect("deck.cir.mt0");
    assert!(
        (measure_line(&measures, "MX") - 0.5).abs() < 1e-3,
        "{measures}"
    );

    // `-o` names the run's one result: no Xyce files beside the deck.
    let other = test_dir("xyce_files_dash_o");
    let output = run_xyce_files(
        &other,
        "rc.cir",
        deck,
        &["-o", other.join("out.csv").to_str().unwrap(), "-f", "csv"],
    );
    assert!(output.status.success(), "{output:?}");
    assert!(other.join("out.csv").exists());
    assert!(!other.join("rc.cir.prn").exists());
    assert!(!other.join("rc.cir.mt0").exists());
}

/// An AC table goes to `deck.cir.FD.prn`, each phasor as `Re(..)` and
/// `Im(..)`, and AC measures to `deck.cir.ma0`. `.PRINT FILE=` names a card's
/// own file and `FORMAT=CSV` its layout (comma separated, no `Index`, no
/// closing line). Each row is Xyce 7.10's own for this deck. A `FILE=` that
/// leaves the deck's directory is refused with a warning, as a control
/// script's files are, and the run still succeeds.
#[test]
fn xyce_writes_ac_tables_where_the_cards_name_them() {
    let dir = test_dir("xyce_files_ac");
    let deck_dir = dir.join("deck");
    std::fs::create_dir_all(&deck_dir).expect("deck directory");
    let deck = "* ac files\nV1 1 0 dc 0 ac 1\nR1 1 2 1k\nC1 2 0 1u\n.ac dec 1 100 1k\n\
                .print ac vm(2) v(2)\n.print ac file=gain.csv format=csv vdb(2)\n\
                .print ac file=../escape.prn vm(2)\n.meas ac g3 find vdb(2) at=1k\n.end\n";
    let output = run_xyce_files(&deck_dir, "rc.cir", deck, &[]);
    assert!(output.status.success(), "{output:?}");

    let table = std::fs::read_to_string(deck_dir.join("rc.cir.FD.prn")).expect("deck.cir.FD.prn");
    let lines: Vec<&str> = table.lines().collect();
    assert_eq!(
        lines[0].split_whitespace().collect::<Vec<_>>(),
        ["Index", "FREQ", "VM(2)", "Re(V(2))", "Im(V(2))"]
    );
    assert_eq!(
        lines[1..],
        [
            "0        1.00000000e+02    8.46733016e-01    7.16956800e-01   -4.50477243e-01",
            "1        1.00000000e+03    1.57176725e-01    2.47045230e-02   -1.55223096e-01",
            "End of Xyce(TM) Simulation",
        ]
    );
    assert_eq!(
        std::fs::read_to_string(deck_dir.join("gain.csv")).expect("FILE= table"),
        "FREQ,VDB(2)\n1.00000000e+02,-1.44507012e+00\n1.00000000e+03,-1.60722353e+01\n"
    );
    assert_eq!(
        std::fs::read_to_string(deck_dir.join("rc.cir.ma0")).expect("AC measures"),
        "G3 = -1.607224e+01\n"
    );

    assert!(!dir.join("escape.prn").exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("warning: ../escape.prn is not written")
            && stderr.contains("outside the deck's directory"),
        "{stderr}"
    );
}

/// Under `.STEP`, Xyce 7.10 writes every coordinate's rows into one table,
/// each restarting its `Index`, closes it with `End of Xyce(TM) Parameter
/// Sweep`, writes coordinate k's measures to `deck.cir.mt<k>`, and lists each
/// coordinate's stepped value in `deck.cir.res`. Xyce on this deck:
/// `MX = 4.999385e-01` and `MX = 3.333277e-01`.
#[test]
fn xyce_writes_one_measure_file_per_step_and_the_step_table() {
    let dir = test_dir("xyce_files_step");
    let deck = "* step files\nV1 1 0 sin 0 1 1k\nR1 1 2 {r}\nR2 2 0 1k\n.param r=1k\n\
                .step r list 1k 2k\n.tran 10u 1m\n.print tran v(2)\n\
                .meas tran mx max v(2)\n.end\n";
    let output = run_xyce_files(&dir, "st.cir", deck, &[]);
    assert!(output.status.success(), "{output:?}");

    let table = std::fs::read_to_string(dir.join("st.cir.prn")).expect("deck.cir.prn");
    let lines: Vec<&str> = table.lines().collect();
    assert_eq!(lines[0], "Index       TIME              V(2)       ");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("Index"))
            .count(),
        1
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("0        0.00000000e+00"))
            .count(),
        2,
        "{table}"
    );
    assert_eq!(*lines.last().unwrap(), "End of Xyce(TM) Parameter Sweep");

    for (step, expected) in [(0, 0.5), (1, 1.0 / 3.0)] {
        let measures = std::fs::read_to_string(dir.join(format!("st.cir.mt{step}")))
            .unwrap_or_else(|error| panic!("st.cir.mt{step}: {error}"));
        assert!(
            (measure_line(&measures, "MX") - expected).abs() < 1e-3,
            "{measures}"
        );
    }
    assert!(!dir.join("st.cir.mt2").exists());
    assert_eq!(
        std::fs::read_to_string(dir.join("st.cir.res")).expect("deck.cir.res"),
        "STEP                    R\n\
         0          1.00000000e+03      \n\
         1          2.00000000e+03      \n\
         End of Xyce(TM) Parameter Sweep\n"
    );
}

/// A file that cannot be written is a warning: the run's result and exit
/// status stand, and the other files are still written.
#[test]
fn an_unwritable_xyce_file_warns_and_keeps_the_exit_status() {
    let dir = test_dir("xyce_files_unwritable");
    std::fs::create_dir_all(dir.join("rc.cir.prn")).expect("a directory where the table goes");
    let deck = "* blocked\nV1 1 0 1\nR1 1 2 1k\nR2 2 0 1k\n.dc V1 0 2 1\n.print dc v(2)\n\
                .meas dc mx max v(2)\n.end\n";
    let output = run_xyce_files(&dir, "rc.cir", deck, &[]);
    assert!(output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("warning: rc.cir.prn is not written"),
        "{stderr}"
    );
    assert!(dir.join("rc.cir.prn").is_dir());
    assert_eq!(
        std::fs::read_to_string(dir.join("rc.cir.ms0")).expect("DC measures"),
        "MX = 1.000000e+00\n"
    );
}

/// A `.PRINT HB` card writes Xyce's two files: the two-sided spectrum as
/// `deck.cir.HB.FD.prn` and one period of the waveform as
/// `deck.cir.HB.TD.prn`, sampled at as many points as the spectrum has bins.
/// The waveform row is Xyce 7.10's own for this deck:
/// `1  9.09090909e-07  7.00774234e-01 -1.60133416e-04  1.04455529e-04
/// -2.64588945e-04  1.60133416e-04`.
#[test]
fn xyce_writes_the_hb_spectrum_and_waveform_files() {
    let dir = test_dir("xyce_files_hb");
    let deck = "* hb files\n.hb 1e5\n.print hb v(out) i(R1) i(C1) i(L1) i(V1)\n\
                .options hbint numfreq=5\nV1 in 0 sin 0 1 1e5 0 0\nR1 in out 1k\n\
                C1 out 0 1n\nL1 out 0 1m\n.end\n";
    let output = run_xyce_files(&dir, "rc.cir", deck, &[]);
    assert!(output.status.success(), "{output:?}");

    let spectrum = std::fs::read_to_string(dir.join("rc.cir.HB.FD.prn")).expect("HB.FD.prn");
    let lines: Vec<&str> = spectrum.lines().collect();
    assert_eq!(
        lines[0].split_whitespace().collect::<Vec<_>>(),
        [
            "Index",
            "FREQ",
            "Re(V(OUT))",
            "Im(V(OUT))",
            "Re(I(R1))",
            "Im(I(R1))",
            "Re(I(C1))",
            "Im(I(C1))",
            "Re(I(L1))",
            "Im(I(L1))",
            "Re(I(V1))",
            "Im(I(V1))",
        ]
    );
    assert_eq!(lines.len(), 13, "{spectrum}");
    assert!(
        lines[1].starts_with("0       -5.00000000e+05"),
        "{spectrum}"
    );
    assert!(
        lines[11].starts_with("10       5.00000000e+05"),
        "{spectrum}"
    );

    let waveform = std::fs::read_to_string(dir.join("rc.cir.HB.TD.prn")).expect("HB.TD.prn");
    let lines: Vec<&str> = waveform.lines().collect();
    assert_eq!(
        lines[0].split_whitespace().collect::<Vec<_>>(),
        [
            "Index", "TIME", "V(OUT)", "I(R1)", "I(C1)", "I(L1)", "I(V1)"
        ]
    );
    assert_eq!(lines.len(), 13, "{waveform}");
    assert_eq!(lines[12], "End of Xyce(TM) Simulation");
    let row: Vec<f64> = lines[2]
        .split_whitespace()
        .map(|field| field.parse().expect("number"))
        .collect();
    let xyce = [
        1.0,
        9.09090909e-07,
        7.00774234e-01,
        -1.60133416e-04,
        1.04455529e-04,
        -2.64588945e-04,
        1.60133416e-04,
    ];
    for (actual, expected) in row.iter().zip(xyce) {
        assert!(
            (actual - expected).abs() <= 1e-6 * expected.abs(),
            "{actual} vs Xyce {expected}: {waveform}"
        );
    }

    // A lead current keeps its `Re(..)` and `Im(..)` around the upper-cased
    // name in the spectrum, and is one column in the waveform. Xyce 7.10 on
    // this deck: `1  9.09090909e-07  4.12348384e-04`.
    let deck = "* hb lead\n.hb 1e5\n.print hb id(M1)\n.options hbint numfreq=5\n\
                VDD vdd 0 3\nVG g 0 sin 1.5 0.2 1e5 0 0\nRD vdd d 2k\n\
                M1 d g 0 0 nm L=1u W=10u\n.model nm NMOS level=1 VTO=0.7 KP=1e-4\n.end\n";
    let output = run_xyce_files(&dir, "lead.cir", deck, &[]);
    assert!(output.status.success(), "{output:?}");
    let spectrum = std::fs::read_to_string(dir.join("lead.cir.HB.FD.prn")).expect("HB.FD.prn");
    assert_eq!(
        spectrum
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>(),
        ["Index", "FREQ", "Re(ID(M1))", "Im(ID(M1))"]
    );
    let waveform = std::fs::read_to_string(dir.join("lead.cir.HB.TD.prn")).expect("HB.TD.prn");
    let lines: Vec<&str> = waveform.lines().collect();
    assert_eq!(
        lines[0].split_whitespace().collect::<Vec<_>>(),
        ["Index", "TIME", "ID(M1)"]
    );
    let current: f64 = lines[2].split_whitespace().nth(2).unwrap().parse().unwrap();
    assert!(
        (current - 4.12348384e-04).abs() <= 1e-6 * 4.12348384e-04,
        "{waveform}"
    );
}
