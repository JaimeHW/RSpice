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

use common::{TestDirectory, test_dir};

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
    csv.lines()
        .next()
        .expect("CSV header")
        .split(',')
        .map(str::to_string)
        .collect()
}

fn csv_column(csv: &str, name: &str) -> Vec<f64> {
    let mut lines = csv.lines();
    let header = lines.next().expect("CSV header");
    let index = header
        .split(',')
        .position(|column| column.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing column {name} in {header}"));
    lines
        .map(|line| {
            line.split(',')
                .nth(index)
                .expect("CSV cell")
                .parse()
                .expect("numeric CSV value")
        })
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
// The `.STEP` sweep table publishes node voltages, so a branch current is the
// symbol it cannot supply; it used to disappear from the table instead.
const STEP_TABLE_DECK: &str = "* implicit .STEP sweep table with an unsupplied save\n\
     V1 in 0 5\n\
     R1 in out {rval}\n\
     R2 out 0 1k\n\
     .param rval=1k\n\
     .step param rval 1k 3k 1k\n\
     .save I(V1)\n\
     .end\n";

#[test]
fn a_step_sweep_table_refuses_a_signal_it_cannot_supply() {
    let run = run_deck("step_table_unknown", STEP_TABLE_DECK, &[]);
    assert_typed_unavailable(&run, "I(V1)", "Step");
    cleanup(&run.dir);
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
