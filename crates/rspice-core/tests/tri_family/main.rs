//! The tri-family mixed-signal suites and generated-model route parity.

#[cfg(feature = "veriloga")]
#[path = "../common/determinism_fingerprint.rs"]
mod determinism_fingerprint;
#[cfg(feature = "veriloga")]
#[path = "../common/digital_trace_invariants.rs"]
mod digital_trace_invariants;

#[cfg(all(feature = "veriloga", feature = "veriloga-model-diode-cmc"))]
mod generated_route_parity;
#[cfg(feature = "veriloga")]
mod tri_family_events;
#[cfg(feature = "veriloga")]
mod tri_family_small_signal;
#[cfg(feature = "veriloga")]
mod tri_family_transient;
#[cfg(feature = "veriloga")]
mod veriloga_master_shadowing;
#[cfg(feature = "veriloga")]
mod veriloga_terminal_currents;
