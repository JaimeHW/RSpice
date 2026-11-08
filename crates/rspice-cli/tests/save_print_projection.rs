//! One authored output contract across `.SAVE` and `.PRINT`.
//!
//! A deck may write both cards. The ordered `.PRINT` columns and the
//! analysis-agnostic `.SAVE` symbols are one selection, so neither replaces
//! the other, and a symbol no analysis can supply fails the run with a typed
//! error instead of publishing a narrower artifact.
//!
//! The `.STEP` sweep table, PSS, and HB used to apply the save set without
//! checking it, so an unknown symbol silently vanished from those exports.

mod common;

use common::{TestDirectory, csv_column, test_dir};

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn cleanup(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
}

struct Run {
    dir: TestDirectory,
    output_path: PathBuf,
    output: Output,
}

fn run_deck(tag: &str, deck_body: &str, extra_args: &[&str]) -> Run {
    let dir = test_dir(tag);
    let deck = dir.join("input.cir");
    let output_path = dir.join("result.csv");
    std::fs::write(&deck, deck_body).expect("write deck");
    let mut args = vec![
        "--quiet".to_string(),
        "run".to_string(),
        deck.to_str().expect("UTF-8 deck path").to_string(),
        "-o".to_string(),
        output_path.to_str().expect("UTF-8 output path").to_string(),
        "-f".to_string(),
        "csv".to_string(),
    ];
    args.extend(extra_args.iter().map(|value| (*value).to_string()));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(&args)
        .output()
        .expect("run rspice");
    Run {
        dir,
        output_path,
        output,
    }
}

fn header_columns(csv: &str) -> Vec<String> {
    common::delimited_data_text(csv, b',')
        .lines()
        .next()
        .expect("CSV header")
        .split(',')
        .map(str::to_string)
        .collect()
}

fn assert_typed_unavailable(run: &Run, authored_symbol: &str, analysis: &str) {
    assert!(
        !run.output.status.success(),
        "an unavailable authored symbol must fail the run; stdout: {}",
        String::from_utf8_lossy(&run.output.stdout)
    );
    assert_ne!(
        run.output.status.code(),
        Some(101),
        "authored input must not panic"
    );
    let stderr = String::from_utf8_lossy(&run.output.stderr);
    assert!(
        stderr.contains("requested signal")
            && stderr.contains(authored_symbol)
            && stderr.contains("is unavailable")
            && stderr.contains(analysis),
        "missing typed unavailable-signal detail: {stderr}"
    );
    assert!(
        !run.output_path.exists(),
        "a failed projection must publish no artifact"
    );
}

#[test]
fn a_saved_device_observable_is_exported_beside_an_ordered_print_column() {
    let run = run_deck(
        "tran_save_beside_print",
        include_str!("fixtures/device_observables/tran_save_beside_print.cir"),
        &[],
    );
    assert!(
        run.output.status.success(),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&run.output.stdout),
        String::from_utf8_lossy(&run.output.stderr)
    );
    let csv = std::fs::read_to_string(&run.output_path).expect("read TRAN CSV");
    let columns = header_columns(&csv);
    assert!(
        columns
            .iter()
            .any(|column| column.eq_ignore_ascii_case("V(out)")),
        "the ordered .PRINT column is missing: {columns:?}"
    );
    assert!(
        columns
            .iter()
            .any(|column| column.eq_ignore_ascii_case("@D1[Id]")),
        "the .SAVE device observable is missing: {columns:?}"
    );

    let times = csv_column(&csv, "time");
    let currents = csv_column(&csv, "@D1[Id]");
    let voltages = csv_column(&csv, "V(out)");
    assert!(!times.is_empty());
    assert_eq!(currents.len(), times.len());
    assert_eq!(voltages.len(), times.len());
    assert!(currents.iter().all(|value| value.is_finite()));
    assert!(voltages.iter().all(|value| value.is_finite()));
    cleanup(&run.dir);
}

#[test]
fn a_saved_parameter_the_model_does_not_publish_fails_with_no_artifact() {
    let run = run_deck(
        "tran_save_beside_print_unavailable",
        include_str!("fixtures/device_observables/tran_save_beside_print_unavailable.cir"),
        &[],
    );
    assert_typed_unavailable(&run, "@D1[NotAParameter]", "TRAN");
    cleanup(&run.dir);
}

// A node that does not exist is already refused while parsing. A device
// parameter is not: its validity belongs to device metadata, so it reaches
// output projection and is exactly the symbol these paths used to drop.
//
// Implicit OP sweeps retain the same current inventory as explicit .OP.
const STEP_TABLE_DECK: &str = "* implicit .STEP sweep table with a current save\n\
     V1 in 0 5\n\
     R1 in out {rval}\n\
     R2 out 0 1k\n\
     .param rval=1k\n\
     .step param rval 1k 3k 1k\n\
     .save I(V1)\n\
     .end\n";

#[test]
fn a_step_sweep_table_publishes_the_selected_branch_current() {
    let run = run_deck("step_table_current", STEP_TABLE_DECK, &[]);
    assert!(run.output.status.success(), "{:?}", run.output);
    let csv = std::fs::read_to_string(&run.output_path).unwrap();
    let currents = csv_column(&csv, "I(V1)");
    assert_eq!(currents.len(), 3);
    for (current, resistance) in currents.iter().zip([1000.0, 2000.0, 3000.0]) {
        assert!((current + 5.0 / (resistance + 1000.0)).abs() < 1e-12);
    }
    assert_eq!(header_columns(&csv).len(), 2);
}

#[test]
fn implicit_step_raw_retains_current_type_and_destination_ownership() {
    for format in ["raw", "ascii"] {
        let run = run_deck("step_table_current_raw", STEP_TABLE_DECK, &[]);
        // Exercise both RAW carriers with the same projected current.
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(run.dir.join("input.cir"))
            .arg("-o")
            .arg(&run.output_path)
            .args(["-f", format])
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let raw = rspice_core::io::parse_raw_file(&run.output_path).unwrap();
        assert_eq!(raw.variables[1].var_type, "current");
        assert!(raw.variables[1].name.eq_ignore_ascii_case("I(V1)"));
    }
}

const PERIODIC_DECK: &str = "* periodic analysis with an unknown save\n\
     V1 in 0 SIN(0 100m 1meg)\n\
     R1 in out 1k\n\
     C1 out 0 1n\n\
     .save @R1[NotAParameter]\n\
     .end\n";

#[test]
fn pss_refuses_an_unknown_saved_signal() {
    let run = run_deck("pss_unknown", PERIODIC_DECK, &["--pss-freq", "1meg"]);
    assert_typed_unavailable(&run, "@R1[NotAParameter]", "PSS");
    cleanup(&run.dir);
}

#[test]
fn hb_refuses_an_unknown_saved_signal() {
    let run = run_deck("hb_unknown", PERIODIC_DECK, &["--hb-freq", "1meg"]);
    assert_typed_unavailable(&run, "@R1[NotAParameter]", "HB");
    cleanup(&run.dir);
}

const PRINTED_DC_DECK: &str = "* a printed DC sweep
     V1 in 0 5
     R1 in out 1k
     R2 out 0 1k
     .dc V1 0 4 1
     .print dc I(V1)
     .end
";

fn run_raw(run: &Run, format: &str) -> rspice_core::io::RawWaveformData {
    let path = run.dir.join(format!("result.{format}.raw"));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(run.dir.join("input.cir"))
        .arg("-o")
        .arg(&path)
        .args(["-f", format])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    rspice_core::io::parse_raw_file(&path).unwrap()
}

fn raw_column<'a>(raw: &'a rspice_core::io::RawWaveformData, name: &str) -> Option<&'a [f64]> {
    raw.waveforms
        .iter()
        .find(|waveform| waveform.name.eq_ignore_ascii_case(name))
        .map(|waveform| waveform.y.as_slice())
}

// A rawfile is a waveform database: ngspice -r, Xyce -r, LTspice and QSPICE
// all keep every vector whatever a .PRINT card prints.
#[test]
fn a_print_card_chooses_table_columns_but_a_rawfile_keeps_every_vector() {
    let run = run_deck("print_dc_raw", PRINTED_DC_DECK, &[]);
    assert!(run.output.status.success(), "{:?}", run.output);
    let csv = std::fs::read_to_string(&run.output_path).unwrap();
    assert_eq!(header_columns(&csv).len(), 2, "{csv}");
    assert_eq!(csv_column(&csv, "I(V1)").len(), 5);

    for format in ["raw", "ascii"] {
        let raw = run_raw(&run, format);
        let current = raw_column(&raw, "I(V1)").expect("the printed current");
        let output = raw_column(&raw, "V(out)").expect("an unprinted node voltage");
        let input = raw_column(&raw, "V(in)").expect("an unprinted node voltage");
        for ((volts, current), (output, input)) in [0.0, 1.0, 2.0, 3.0, 4.0]
            .iter()
            .zip(current)
            .zip(output.iter().zip(input))
        {
            assert!((output - volts / 2.0).abs() < 1e-12);
            assert!((input - volts).abs() < 1e-12);
            assert!((current + volts / 2000.0).abs() < 1e-15);
        }
    }
    cleanup(&run.dir);
}

#[test]
fn a_save_card_narrows_a_rawfile_that_a_print_card_does_not() {
    let deck = PRINTED_DC_DECK.replace(
        ".print dc I(V1)",
        ".save V(out)
.print dc V(out) I(V1)",
    );
    let run = run_deck("save_print_dc_raw", &deck, &[]);
    assert!(run.output.status.success(), "{:?}", run.output);
    for format in ["raw", "ascii"] {
        let raw = run_raw(&run, format);
        let names = raw
            .variables
            .iter()
            .map(|variable| variable.name.to_ascii_lowercase())
            .collect::<Vec<_>>();
        assert_eq!(names.len(), 2, "{names:?}");
        assert_eq!(names[1], "v(out)");
    }
    cleanup(&run.dir);
}
