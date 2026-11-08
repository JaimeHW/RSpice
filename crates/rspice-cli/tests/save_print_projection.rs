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

/// The `.dc-001`/`.tran-001` raw file a control-block analysis writes.
fn control_raw(run: &Run, analysis: &str) -> rspice_core::io::RawWaveformData {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(run.dir.join("input.cir"))
        .arg("-o")
        .arg(run.dir.join("control.raw"))
        .args(["-f", "raw"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    rspice_core::io::parse_raw_file(&run.dir.join(format!("control.{analysis}.raw"))).unwrap()
}

/// The named column's sample at sweep index `point`, within 1e-4
/// of the value ngspice-46 writes for the same deck.
fn assert_column_at(
    raw: &rspice_core::io::RawWaveformData,
    column: &str,
    point: usize,
    expected: f64,
) {
    let value = raw_column(raw, column).unwrap_or_else(|| panic!("{column} is kept"))[point];
    assert!(
        (value - expected).abs() <= 1e-4 * expected.abs(),
        "{column}: {value:e} against {expected:e}"
    );
}

/// A control `save` keeps the operating-point quantities ngspice keeps, and
/// keeps an empty vector for a name no device quantity answers to.
#[test]
fn a_control_save_keeps_a_bjts_conductances_charges_and_capacitances() {
    let deck = "bjt saved quantities
VB b 0 0.7
VC c 0 5
Q1 c b 0 qmod
.model qmod NPN(IS=1e-14 BF=100 VAF=50 TF=1n CJE=1p CJC=0.5p TR=10n)
.control
save @q1[gm] @q1[gpi] @q1[go] @q1[gmu]
save @q1[qbe] @q1[qbc] @q1[cmu] @q1[cpi]
save @q1[pi]
dc vb 0.6 0.8 0.1
.endc
.end
";
    let run = run_deck("control_save_bjt", deck, &[]);
    let raw = control_raw(&run, "dc-001");
    assert_eq!(raw.variables.len(), 10, "{:?}", raw.variables);
    // ngspice-46 at VB = 0.7.
    for (column, expected) in [
        ("@q1[gm]", 2.379695759743e-1),
        ("@q1[gpi]", 2.192292661310e-3),
        ("@q1[go]", 1.134069354286e-4),
        ("@q1[gmu]", 1.000000002041e-12),
        ("@q1[qbe]", 7.040796360280e-12),
        ("@q1[qbc]", -1.448792984825e-12),
        ("@q1[cmu]", 2.664735247060e-13),
        ("@q1[cpi]", 2.396995021093e-10),
    ] {
        assert_column_at(&raw, column, 1, expected);
    }
    let unanswered = raw_column(&raw, "@q1[pi]").expect("ngspice keeps the empty vector");
    assert_eq!(unanswered, [0.0; 3]);
    cleanup(&run.dir);
}

/// ngspice-46 reads the stored charge of a diode from a control `save` too.
#[test]
fn a_control_save_keeps_a_diodes_charge() {
    let deck = "diode saved quantities
V1 1 0 0.5
D1 1 0 dmod
.model dmod D(IS=1e-14 N=1.1 TT=5n CJO=1p VJ=0.7 M=0.4 RS=2)
.control
save @d1[gd] @d1[cd] @d1[qd]
dc v1 -1 0.6 0.4
.endc
.end
";
    let run = run_deck("control_save_diode", deck, &[]);
    let raw = control_raw(&run, "dc-001");
    assert_eq!(raw.variables.len(), 4, "{:?}", raw.variables);
    assert_column_at(&raw, "@d1[qd]", 0, -8.201478273262e-13);
    assert_column_at(&raw, "@d1[qd]", 4, 8.458830854119e-13);
    assert_column_at(&raw, "@d1[cd]", 4, 4.226251727145e-12);
    cleanup(&run.dir);
}

/// ngspice spells a source inside a subcircuit `v.x1.vs#branch`, and a
/// transient that saves it keeps that branch alone.
#[test]
fn a_control_save_keeps_a_subcircuit_branch_by_its_ngspice_flat_name() {
    let deck = "subcircuit branch save
X1 1 0 sub
R1 1 0 1k
.subckt sub a b
Vs a b 2
.ends
.control
save v.x1.vs#branch
tran 1n 5n
.endc
.end
";
    let run = run_deck("control_save_flat_branch", deck, &[]);
    let raw = control_raw(&run, "tran-001");
    assert_eq!(raw.variables.len(), 2, "{:?}", raw.variables);
    let branch = raw.waveforms.iter().next_back().expect("the branch vector");
    assert!(branch.name.eq_ignore_ascii_case("I(x1.vs)"), "{}", branch.name);
    assert!((branch.y[0] + 2e-3).abs() < 1e-12);
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
