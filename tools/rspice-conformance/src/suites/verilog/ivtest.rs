//! The Icarus Verilog `ivtest` suite run through RSpice (oracle O3).
//!
//! ivtest is the regression suite of Icarus Verilog: about three thousand
//! small self-checking Verilog programs, each judged by a line reading
//! `PASSED`, by a gold file of expected output, or by whether it compiles.
//! It is GPL, so it is never vendored: the harness reads a checkout named by
//! `RSPICE_IVTEST_DIR` at run time, and reports the oracle absent when there
//! is none, as the other oracles do. What the repository keeps is
//! `tests/verilog/ivtest-manifest.tsv`: each test's name and RSpice's outcome,
//! and nothing of the suite's text.
//!
//! # Layers
//!
//! - [`list`] reads the regression lists the suite's own runner reads.
//! - [`select`] applies the selection: the IEEE 1364-2005 tests, each with
//!   the bucket of the language lane it waits for.
//! - [`runner`] runs each selected test in worker processes through
//!   `run_digital_testbench`, on both schedules, under a wall-clock budget.
//! - [`judge`] decides each run: the `PASSED` rule, gold comparison after a
//!   fixed normalization, compile errors told from refusals, and the racy
//!   rule.
//! - [`manifest`] holds the recorded outcomes and ratchets a run against them.
//! - [`icarus`] runs the selection through Icarus itself, which checks the
//!   harness: every selected test must pass on the simulator its gold files
//!   came from.
//!
//! The `rspice-ivtest` binary drives all of it:
//!
//! ```text
//! rspice-ivtest run [--jobs N] [--budget-s S] [--only a,b] [--record]
//! rspice-ivtest oracle [--jobs N] [--only a,b]
//! rspice-ivtest select
//! rspice-ivtest worker
//! ```
//!
//! # Outcomes
//!
//! `pass`, `fail`, `refused:<token>` (RSpice declined a construct or option by
//! name), `racy` (the verdict depends on the order of same-tick activations)
//! and `excluded:<reason>` (outside the selection). A compile-error test that
//! RSpice rejects with a language error is a `pass`; one it refuses by name
//! is `refused`.
//!
//! A row's detail opens with `testbench gap: ...` when Icarus's run of the test
//! sets up something the testbench entry point has no option for, a root
//! module (`-s`) or a define ([`select::testbench_gap`]). The test still runs,
//! as the entry point allows, and its outcome is recorded like any other.

pub mod icarus;
pub mod judge;
pub mod list;
pub mod manifest;
pub mod runner;
pub mod select;

use std::path::PathBuf;

/// The environment variable naming the ivtest checkout: the directory that
/// holds `regress-vlg.list`, `ivltests/` and `gold/`.
pub const IVTEST_DIR_ENV: &str = "RSPICE_IVTEST_DIR";

/// Where the checkout is, or why it is absent.
pub fn ivtest_dir() -> Result<PathBuf, String> {
    let Some(value) = std::env::var_os(IVTEST_DIR_ENV).filter(|value| !value.is_empty()) else {
        return Err(format!(
            "ivtest oracle absent: {IVTEST_DIR_ENV} is not set. Point it at the `ivtest` \
             directory of an Icarus Verilog v13_0 checkout \
             (https://github.com/steveicarus/iverilog, tag v13_0)."
        ));
    };
    let dir = PathBuf::from(value);
    if !dir.join(list::SourceList::Vlg.file_name()).is_file() {
        return Err(format!(
            "ivtest oracle absent: {IVTEST_DIR_ENV}={} has no {}",
            dir.display(),
            list::SourceList::Vlg.file_name()
        ));
    }
    Ok(dir)
}

/// `tests/verilog/ivtest-manifest.tsv`.
pub fn manifest_path() -> PathBuf {
    super::corpus_dir().join(manifest::FILE_NAME)
}
