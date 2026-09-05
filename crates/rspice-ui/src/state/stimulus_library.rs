//! Reusable stimulus definitions, and what a placed source remembers of them.
//!
//! The library is an authoring instrument, not a second owner of the design. A
//! definition is the same `(component type, value, params)` triple a placed
//! source carries; adopting one **copies** it onto the instance and leaves a
//! receipt behind. The instance keeps owning its card, which is what lets the
//! netlister, the deck, the Excitations page and every frozen manifest stay
//! exactly as they were — and what makes "modified" and "behind" comparisons
//! rather than flags anyone has to remember to set.
//!
//! This is the same bargain the model hub already strikes with
//! `ProjectModelLibrary::pack_pin`: adopt by copy, keep the provenance, offer
//! to re-adopt when the source moves.
//!
//! Realizing a definition — turning it into a card, a `SourceSpec` or a
//! waveform — is deliberately *not* here. It needs the netlister and the
//! engine's evaluator, both of which sit above `state`, so it lives in
//! `simulation::stimulus_realize` and the library stays a value model.

pub(crate) mod definition;
pub(crate) mod draft;
pub(crate) mod library;
pub(crate) mod provenance;

/// Wall-clock milliseconds, through the shim the browser build needs.
fn now_unix_ms() -> u64 {
    u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX)
}
