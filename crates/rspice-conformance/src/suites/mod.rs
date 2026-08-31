//! The conformance suites, one module per reference simulator.
//!
//! Each suite owns its corpus layout, its reference-contract selection, and
//! its tolerance model, because those genuinely differ between reference
//! simulators: ngspice ships tabular `.out` captures, Xyce ships `.prn` files
//! with per-deck contract labels. What they share is the shape — discover
//! decks, select a contract, execute, compare, report.
//!
//! That shared shape is not yet expressed as a trait, deliberately. With both
//! suites here it can now be *derived* rather than guessed, and what they
//! actually share turns out to be the type quintet — runner, config, result,
//! statistics, mismatch — which each already spells out
//! (`TestRunner`/`XyceTestRunner`, and so on). What they do not share is
//! everything a trait would have to abstract over: contract selection,
//! reference format (`.out` tables versus `.prn` files), and the result codec
//! the case-runner subprocess speaks. Extracting a trait over those
//! differences would buy nothing today; the moment to do it is when a third
//! suite arrives and the shape has to be honoured rather than invented.
//!
//! [`execution`] is that third suite, and it settled the question the other
//! way. It does not implement the quintet at all: with no reference data
//! there is no mismatch type to implement, no tolerance to model, and no
//! reference format to select. The trait those two suites *would* have shared
//! turns out not to describe conformance testing in general — it describes
//! comparing against a reference simulator, which is one of two things this
//! crate does. So the boundary that matters is not runner-versus-runner but
//! oracle-versus-no-oracle, and it is drawn at the module level here.
//!
//! Adding a suite meanwhile: decide first whether the corpus carries a
//! numeric oracle. With one, implement the five types, drop the corpus under
//! `tests/<suite>/` at the workspace root, add a module here, and give it an
//! integration test plus a nightly step. Without one, it belongs in
//! [`execution`] as another [`ExecutionCorpus`](execution::ExecutionCorpus)
//! and needs only a corpus directory and a manifest.

#[cfg(feature = "circuit-suites")]
pub mod execution;
#[cfg(feature = "circuit-suites")]
pub mod gf180mcu;
#[cfg(feature = "circuit-suites")]
pub mod ngspice;
#[cfg(feature = "circuit-suites")]
pub mod xyce;

/// Digital Verilog checked against Icarus Verilog and Verilator.
///
/// A fourth kind again, and the boundary drawn above — oracle versus no oracle
/// — puts it firmly on the oracle side while it shares almost nothing else with
/// [`ngspice`] and [`xyce`]. Those two consume a reference *file* another
/// simulator published once; this one runs two other simulators live and diffs
/// them against each other. That difference is why it does not implement the
/// quintet either: with the oracle in the loop there is no checked-in reference
/// format to select and no tolerance to model, because logic values are
/// compared exactly or not at all.
///
/// Ungated, unlike its neighbours. It depends on nothing in `rspice-core` — the
/// corpus is Verilog, the oracles are other people's binaries, and the whole
/// module is text generation and process control. Gating it on
/// `circuit-suites` would hide it from exactly the build that wants it: the
/// Verilog-A tooling depends on this crate with `default-features = false`, and
/// the digital front end is grown from that side.
pub mod verilog;

/// The generated Verilog-A built-ins, checked against captured fingerprints
/// and a finite-difference derivative oracle.
///
/// A different kind of regression suite from its neighbours, and worth being
/// precise about: ngspice and Xyce compare against *another simulator's*
/// published output, while this compares against RSpice's own captured stamps
/// plus an oracle that shares no code with the chain rule under test. It is a
/// snapshot plus an independent mathematical check, not external conformance.
///
/// Gated on the feature that compiles the models it measures — with the
/// built-ins absent there is nothing to fingerprint — so it costs a default
/// build nothing.
#[cfg(feature = "veriloga-builtins-base")]
pub mod veriloga;
