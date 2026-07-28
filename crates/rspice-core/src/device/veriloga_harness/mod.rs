//! Hand-written harnesses that measure the generated Verilog-A built-ins.
//!
//! Everything here drives `super::veriloga_generated` from the outside and is
//! maintained by hand. It lives in its own module for one reason: the generated
//! tree is generator-owned output that is rewritten wholesale by
//! `rspice-veriloga-gen`, and hand-written code sitting inside it cannot be
//! told apart from emitted code by a reader, a reviewer, or a regeneration
//! diff. The only hand-written file that belongs under `veriloga_generated/` is
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

#[cfg(feature = "veriloga-builtins-base")]
pub mod bench;

#[cfg(feature = "veriloga-builtins-base")]
pub mod fixture;

#[cfg(feature = "veriloga-builtins-base")]
pub mod golden;

#[cfg(feature = "veriloga-builtins-base")]
pub mod reference;
