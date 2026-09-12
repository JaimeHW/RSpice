//! One fingerprint of one transient result, and the two things a suite does
//! with it: pin it, and prove a second process reproduces it.
//!
//! # What a fingerprint is worth
//!
//! A fingerprint is a point count and two FNV-1a hashes — one over the
//! accepted time grid, one over every node waveform in node order. A golden of
//! this kind proves self-agreement and nothing else: it says the route
//! reproduces its own answer, not that the answer is physically right. Read it
//! as a tripwire for an unintended change of the accepted grid, never as an
//! oracle.
//!
//! What makes it worth more than a point count is that it is cheap enough to
//! state for every fixture rather than one, and that the same three numbers can
//! be re-derived in another process and compared. A grid that differs between
//! two runs of one binary is an address, an iteration order or a shared cache
//! that has reached the answer, and no per-deck assertion in any suite is
//! shaped to catch it.
//!
//! # Regenerating
//!
//! Set `RSPICE_TRI_FAMILY_EMIT=1` and run the suite with `--nocapture`. Every
//! pinned quantity prints as `TRIFAMILY <key>=<value>` and no pinned equality
//! is asserted, so one run prints the complete set for the route it was built
//! with. Copy the printed values into the constants — per route, because the
//! interpreter and the x64 JIT are separate routes and may land on different
//! grids — and re-run without the variable.
//!
//! One printed line is one record: the key is what precedes the first `=`
//! after the `TRIFAMILY ` marker and the value is the rest of the line, which
//! is the rule `tools/ci/compare_trifamily_fingerprints.py` reads the same
//! lines by. The marker is looked for anywhere in the line rather than at its
//! start, because `--test-threads=1 --nocapture` leaves libtest's own
//! `test <name> ... ` in front of each case's first line.

#![allow(dead_code)]

use rspice_core::engine::TransientResult;
use std::collections::BTreeMap;
use std::process::Command;
use std::time::Instant;

/// Environment variable that turns every pinned comparison into a print.
pub const EMIT_ENV: &str = "RSPICE_TRI_FAMILY_EMIT";

/// Whether this process is re-measuring rather than asserting.
pub fn emitting() -> bool {
    std::env::var_os(EMIT_ENV).is_some()
}

/// FNV-1a over the raw bits of a float sequence.
fn sequence_hash(values: impl Iterator<Item = f64>) -> u64 {
    let mut hash: u64 = 1469598103934665603;
    for value in values {
        hash ^= value.to_bits();
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

/// A transient result's accepted grid and analog solution, in three numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fingerprint {
    /// Accepted timepoints.
    pub points: usize,
    /// FNV-1a over the accepted grid.
    pub grid_hash: u64,
    /// FNV-1a over every node waveform, in node order.
    pub volt_hash: u64,
}

impl Fingerprint {
    /// The `key=value` records [`pin_fingerprint`] prints for this fixture.
    ///
    /// The caller builds this from its pinned constants, so a cross-process
    /// comparison states child against pin rather than child against a value
    /// the child itself produced.
    pub fn records(&self, key: &str) -> Vec<(String, String)> {
        vec![
            (format!("{key}_points"), self.points.to_string()),
            (
                format!("{key}_grid_hash"),
                format!("{:016x}", self.grid_hash),
            ),
            (
                format!("{key}_volt_hash"),
                format!("{:016x}", self.volt_hash),
            ),
        ]
    }
}

/// The fingerprint of one transient result.
///
/// A node the run published no voltage for contributes nothing: `voltages` is
/// a column per node and an event-only net's column is empty, so the hash is
/// over the analog solution the deck actually has.
pub fn fingerprint(result: &TransientResult) -> Fingerprint {
    Fingerprint {
        points: result.time.len(),
        grid_hash: sequence_hash(result.time.iter().copied()),
        volt_hash: sequence_hash(result.voltages.iter().flatten().copied()),
    }
}

/// Print `key`'s three records, and assert them unless this run is emitting.
pub fn pin_fingerprint(key: &str, observed: Fingerprint, expected: Fingerprint) {
    for (record, value) in observed.records(key) {
        println!("TRIFAMILY {record}={value}");
    }
    if emitting() {
        return;
    }
    assert_eq!(
        observed.points, expected.points,
        "{key}_points moved off its pinned value; re-measure with {EMIT_ENV}=1"
    );
    assert_eq!(
        observed.grid_hash, expected.grid_hash,
        "{key}_grid_hash moved off its pinned value ({:016x} vs {:016x}); \
         re-measure with {EMIT_ENV}=1",
        observed.grid_hash, expected.grid_hash
    );
    assert_eq!(
        observed.volt_hash, expected.volt_hash,
        "{key}_volt_hash moved off its pinned value ({:016x} vs {:016x}); \
         re-measure with {EMIT_ENV}=1",
        observed.volt_hash, expected.volt_hash
    );
}

/// Every `TRIFAMILY` record in a child's stdout, and the test count it ran.
fn records_of(stdout: &str) -> (BTreeMap<String, String>, Option<usize>) {
    let mut records = BTreeMap::new();
    let mut ran = None;
    for line in stdout.lines() {
        let line = line.trim();
        if let Some(count) = line
            .strip_prefix("running ")
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|count| count.parse::<usize>().ok())
        {
            ran = Some(count);
            continue;
        }
        // Not `strip_prefix`: under `--test-threads=1 --nocapture` libtest
        // writes `test <name> ... ` with no newline and the case's own first
        // line lands on the end of it, so the first record of every test would
        // be the one record a prefix match dropped.
        let Some((_, record)) = line.split_once("TRIFAMILY ") else {
            continue;
        };
        let Some((key, value)) = record.split_once('=') else {
            continue;
        };
        records.insert(key.trim().to_string(), value.trim().to_string());
    }
    (records, ran)
}

/// Re-run `fixtures` in `children` fresh processes and compare what they print.
///
/// `fixtures` are test names in THIS binary; each child runs exactly those,
/// serially, with the emit variable set, so it prints its fingerprints and
/// asserts no pinned equality of its own. Three things are then asserted:
///
/// * every child ran every fixture — a filter that matched nothing would
///   otherwise leave an empty comparison looking like agreement;
/// * every child produced the same records as the first; and
/// * unless this process is itself emitting, those records are the `pinned`
///   ones — which are the parent's own pinned constants, so the statement is
///   child against pin rather than child against child.
///
/// A shared cache, an address, or an iteration order that leaked into the
/// answer shows up here and nowhere else in a suite.
pub fn assert_fingerprints_survive_new_processes(
    label: &str,
    fixtures: &[&str],
    children: usize,
    pinned: &[(String, String)],
) {
    assert!(!fixtures.is_empty(), "{label}: no fixture to re-run");
    assert!(children > 0, "{label}: a cross-process claim needs a child");
    let Ok(executable) = std::env::current_exe() else {
        println!("TRIFAMILY {label}_cross_process=skipped_no_executable_path");
        return;
    };

    let started = Instant::now();
    let mut reference: Option<BTreeMap<String, String>> = None;
    for attempt in 0..children {
        let output = Command::new(&executable)
            .args(["--exact", "--nocapture", "--test-threads=1"])
            .args(fixtures)
            .env(EMIT_ENV, "1")
            .output();
        let Ok(output) = output else {
            println!("TRIFAMILY {label}_cross_process=skipped_no_spawn");
            return;
        };
        assert!(
            output.status.success(),
            "{label}: child {attempt} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8(output.stdout).expect("child stdout is utf-8");
        let (records, ran) = records_of(&stdout);
        assert_eq!(
            ran,
            Some(fixtures.len()),
            "{label}: child {attempt} ran {ran:?} of the {} named fixtures; a filter that \
             selects nothing compares nothing:\n{stdout}",
            fixtures.len()
        );
        match &reference {
            None => reference = Some(records),
            Some(first) => assert_eq!(
                &records, first,
                "{label}: child {attempt} printed different fingerprints than child 0"
            ),
        }
    }
    let elapsed = started.elapsed();
    let reference = reference.expect("a child ran");
    assert!(
        !reference.is_empty(),
        "{label}: the children printed no TRIFAMILY record at all"
    );

    if !emitting() {
        for (key, value) in pinned {
            assert_eq!(
                reference.get(key),
                Some(value),
                "{label}: a fresh process disagrees with the pinned {key}"
            );
        }
    }
    println!("TRIFAMILY {label}_cross_process_children={children}");
    println!(
        "cross-process {label}: {children} children x {} fixtures in {:.2} s",
        fixtures.len(),
        elapsed.as_secs_f64()
    );
}
