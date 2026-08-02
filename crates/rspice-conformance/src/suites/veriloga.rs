//! Hand-written harnesses that measure the generated Verilog-A built-ins.
//!
//! Everything here drives `rspice_core::device::veriloga_builtins` from the outside and is
//! maintained by hand. It lives in its own module for one reason: the generated
//! tree is generator-owned output that is rewritten wholesale by
//! `rspice-veriloga-gen`, and hand-written code sitting inside it cannot be
//! told apart from emitted code by a reader, a reviewer, or a regeneration
//! diff. The only hand-written file that belongs under `veriloga_builtins/` is
//! its `mod.rs`, because that is the module root the generated folders attach
//! to.
//!
//! - [`bench`] — stamp throughput, the runtime gate.
//! - [`reference`] — the same measurement on a hand-written model, which is
//!   what the runtime gate is stated against.
//! - [`golden`] — numerical fingerprints and the derivative oracle, the
//!   correctness gate.
//! - [`fixture`] — the on-disk form of a captured fingerprint.
//!
//! All of it outlives any particular backend: these observe devices through the
//! public instance API and know nothing about how a stamp was emitted.

// `bench` and `reference` are timing harnesses, and `std::time::Instant::now()`
// aborts at runtime on `wasm32-unknown-unknown` — there is no clock in the bare
// wasm ABI. `rspice_core::time_compat` is the shim used inside the engine, but
// it is deliberately not the answer here: its `elapsed()` is always zero, which
// is right for threshold-triggered diagnostics that should simply stay quiet,
// and wrong for a benchmark, where it would report a fabricated `0.0 ns/stamp`
// instead of failing. A measurement that cannot be taken must not be reported,
// so these are compiled out of wasm entirely and a wasm caller gets a name
// error rather than a plausible number. Their only consumer is `rspice-bench`,
// which is native-only.
#[cfg(all(feature = "veriloga-builtins-base", not(target_arch = "wasm32")))]
pub mod bench;

#[cfg(feature = "veriloga-builtins-base")]
pub mod fixture;

#[cfg(feature = "veriloga-builtins-base")]
pub mod golden;

#[cfg(all(feature = "veriloga-builtins-base", not(target_arch = "wasm32")))]
pub mod reference;
