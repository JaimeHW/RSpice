//! Digital Verilog checked against independent simulators.
//!
//! The corpus lives at `tests/verilog/` and the oracles are Icarus Verilog and
//! Verilator — two simulators written by other people, sharing no code with
//! RSpice and none with each other.
//!
//! # Why this suite exists before there is anything to test
//!
//! Nothing in RSpice executes digital Verilog yet. The front end that will
//! (W2.3, the CFG interpreter) is being built now, and the order matters: a
//! harness written *after* an implementation tends to encode that
//! implementation's idea of what the answer is, because the easiest oracle to
//! reach for is the thing already on the machine. Written first, it can only
//! encode what the LRM and two independent simulators say.
//!
//! So the RSpice arm is present and refuses. [`VerilogEngine::Rspice`] resolves
//! to [`RunError::RspiceExecutionUnimplemented`] — a named variant, not a
//! panic, not a silent skip — and [`rspice_execution_is_not_implemented_yet`]
//! asserts that it does. When W2.3 lands, that test fails, and the failure is
//! the checklist item that says the arm is now real.
//!
//! # What is trustworthy today
//!
//! Four things, and it is worth being exact, because a suite that overstates
//! what it proves is worse than one that proves less:
//!
//! 1. **Corpus integrity.** Every case has a stimulus, every stimulus parses,
//!    every declared port matches the module header of the design beside it,
//!    and the manifest and the directory agree in both directions.
//! 2. **Testbench synthesis.** The generated testbench is deterministic and
//!    structurally well-formed for every case in the corpus.
//! 3. **Oracle detection.** Absence is a typed value, not an exception.
//! 4. **Trace comparison.** Agreement and disagreement are both exercised, on
//!    synthetic traces, so the comparator is tested independently of whether
//!    any simulator is installed.
//!
//! What is *not* proven today is the only thing that ultimately matters: that
//! two real simulators agree on this corpus. That check is written and runs,
//! but on a machine without the binaries it reports absence and stops. See
//! [`oracle`] for how to make it a hard failure instead.
//!
//! # The harness convention
//!
//! ## Case layout
//!
//! A case is two files in `tests/verilog/` sharing a stem:
//!
//! - `<case>.v` — the design. Structural or behavioural Verilog with **no
//!   `$display`, no `$monitor`, no `initial` block, and no delays**. This is
//!   the load-bearing rule of the whole design: a case that prints its own
//!   results is testing each simulator's number formatting as much as its
//!   semantics, and the two disagree about formatting for reasons that are
//!   nobody's bug. Everything observable about a case must travel through its
//!   module ports.
//! - `<case>.stim` — port declarations, timing, and input vectors, in the
//!   `RSPICE-VERILOG-STIMULUS 1` format below.
//!
//! `tests/verilog/verilog-manifest.tsv` lists every case with the oracles it
//! may be compared against. That column is not bookkeeping: Icarus is a
//! four-state event simulator and Verilator is a two-state cycle simulator, so
//! on a case whose answer depends on X or Z they cannot agree, and demanding
//! that they do would make the harness report a defect that does not exist.
//! Every case is two-state-clean except `xz_propagation.v`, which covers the
//! four-state rules and is therefore Icarus-only.
//!
//! ## The testbench is generated, never checked in
//!
//! Both oracles receive a testbench synthesised from the `.stim` file by
//! [`testbench`]. RSpice therefore owns the output format, and both simulators
//! are asked to produce byte-identical text rather than each producing its own
//! and the harness reconciling them afterwards. Reconciliation is where an
//! oracle harness goes wrong: every normalisation rule is a place a real
//! disagreement can be normalised away.
//!
//! ## Timing
//!
//! Two numbers, and one rule that makes them unambiguous. For vector `k`:
//!
//! - inputs are applied at `t = k * step`
//! - outputs are sampled at `t = k * step + settle`
//!
//! with `0 < settle < step`. A `clock <port> <half_period>` line adds a free
//! running clock — `initial clk = 0; always #half_period clk = ~clk;` — whose
//! port is excluded from the vector columns. Choosing `step = 2 * half_period`
//! gives exactly one rising edge per vector, at `k * step + half_period`, and a
//! `settle` greater than `half_period` samples after it. The corpus's clocked
//! cases all use `half_period 5`, `step 10`, `settle 8`.
//!
//! ## Trace grammar
//!
//! The generated testbench writes to stdout:
//!
//! ```text
//! # RSPICE-VERILOG-TRACE 1
//! @0 N22=1 N23=1
//! @1 N22=1 N23=0
//! ```
//!
//! One row per vector, in order. Values are `%b` — binary, MSB first, exactly
//! as wide as the port, with `x` and `z` appearing literally. Comparison is
//! token-based on these rows and nothing else; tool preamble before the header
//! is discarded, and any non-empty line after it that is not a row is an error
//! rather than something to skip past.
//!
//! ## Adding a case
//!
//! Write the `.v` and the `.stim`, add a manifest row naming the oracles it is
//! comparable across, and stop. Discovery is by directory sweep and the
//! manifest is checked against it in both directions, so a case that is added
//! to one and not the other fails rather than being quietly ignored.

pub mod ams;
/// The LRM-clause-tagged semantics suite. Behind `verilog-digital` because
/// every case runs either a mixed deck or a discrete-domain design, and both
/// need the front end that feature links.
#[cfg(feature = "verilog-digital")]
pub mod ams_semantics;
pub mod corpus;
/// The mixed-versus-all-analog exit-gate benchmarks. Behind `verilog-digital`
/// for the reason [`ams_semantics`] is: half of each benchmark is a mixed deck.
#[cfg(feature = "verilog-digital")]
pub mod mixed_benchmarks;
pub mod oracle;
/// The dual-representation reference blocks. Behind `verilog-digital` because
/// the RNM half of every block is a call into the digital host, and a module
/// that could not run one would be half a suite.
#[cfg(feature = "verilog-digital")]
pub mod rnm;
pub mod run;
pub mod scale;
pub mod testbench;
pub mod trace;

pub use ams::{
    AmsCase, AmsCorpus, AmsCorpusError, AmsDirection, AmsPort, AmsPortValue, AmsStimulus,
};
pub use corpus::{Case, CaseDirection, CasePort, Corpus, CorpusError, Stimulus};
pub use oracle::{OracleAvailability, OracleTools, VerilogEngine};
pub use run::{RunError, RunOutcome, run_case};
pub use trace::{Divergence, Trace, TraceError, compare_traces};

use std::path::{Path, PathBuf};

/// Root of the vendored digital corpus.
///
/// Resolved from `CARGO_MANIFEST_DIR` rather than the process working
/// directory, which differs between `cargo test` and a debugger and is not
/// something a corpus path may depend on.
pub fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the conformance crate sits two levels below the workspace root")
        .join("tests")
        .join("verilog")
}

/// Root of the Verilog-AMS real-net corpus.
///
/// A subdirectory of [`corpus_dir`], which is what keeps the two apart without
/// either having to know about the other: [`corpus::Corpus`] discovers by
/// reading files rather than by recursing, so these cases are invisible to the
/// corpus beside them and the Icarus and Verilator arms never see one.
pub fn ams_corpus_dir() -> PathBuf {
    corpus_dir().join("ams")
}
