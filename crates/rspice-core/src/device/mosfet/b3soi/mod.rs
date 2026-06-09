//! BSIM3SOI device family (SPICE levels 55-57).
//!
//! Shared scaffolding for the Berkeley SOI MOSFET compact models:
//! - `dd/`: B3SOIDD, "dynamic depletion" BSIMDD2.x (MOS level 56) — ported from
//!   ngspice-46 `src/spicelib/devices/bsim3soi_dd/`.
//! - `fd/` (level 55, fully depleted) and `pd/` (level 57, partially depleted)
//!   are planned siblings that should follow the same module structure
//!   (`params.rs` model card, `temp.rs` size/temperature setup, `eval.rs`
//!   faithful load transcription, `mod.rs` device + stamping glue).
//!
//! All three variants share the SOI node topology (drain, gate, source,
//! back-gate/substrate `E`, optional body contact `P`, internal floating body
//! `B`, optional internal drain/source primes and an optional self-heating
//! temperature node) and the smoothing-function vocabulary collected in
//! [`common`].

pub mod common;
pub mod dd;

pub use dd::{B3SoiDd, B3SoiDdModel};
