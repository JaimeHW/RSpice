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
//!
//! # Implementation status (DD / level 56)
//!
//! Done and unit-tested ([`dd`]):
//! - Model card transcription + defaults ([`dd::params`]).
//! - Size/temperature setup ([`dd::temp`], full `B3SOIDDtemp` port).
//! - The **DC current path** ([`dd::eval::eval_dc`]): SOI body-coupled
//!   threshold chain (Vbs0t/Vbs0/Vbs0mos/Vthfd/Vbs0teff/Vbs0eff/Vbsdio/
//!   Vbseff/nfb), Vgsteff smoothing, Abulk/Abeff, MOBMOD 0/1/2/3 mobility,
//!   Vdsat/Vdseff, CLM/DIBL/Va, Ids, impact ionization, GIDL, and the body
//!   source/drain diodes + parasitic BJT — i.e. every `here->B3SOIDD*`
//!   conductance/current the DC solve needs.
//! - The **DC matrix/RHS stamping** for `bodyMod` 0 (floating) and 2 (ideal
//!   tie), no series R, no temp node ([`dd::B3SoiDd`] `stamp_op`).
//!
//! Deferred (clearly bounded, see `dd::eval` module docs for line refs):
//! - **CAPMOD=3 charge model** (b3soiddld.c:2640-3400) and its transient
//!   capacitor companion stamping + charge-history/LTE integration. Required
//!   for `RampVg2`/`inv2`/`ring51` transient and for AC.
//! - **Self-heating** (SHMOD=1) and the temperature node — SHMOD=0 in every
//!   supported deck; the parameter is recognized and (per spec) should error
//!   if enabled rather than be silently ignored. Not yet enforced.
//! - **Body resistor** (`rbody`/`rbsh` > 0, `bodyMod==1`) and internal
//!   drain/source primes (series R) — absent in the supported decks.
//!
//! Because the charge model is not yet ported, [`dd::B3SoiDd`] is **not wired
//! into the builder dispatch** (`engine/builder.rs` still routes level 56 to
//! the generic MOSFET). This keeps all non-SOI paths byte-identical and the
//! existing regression suite green. Flipping the dispatch is the final step and
//! must wait until the transient charge path lands, so the SOI transient decks
//! are not left half-working.
//!
//! # Next steps for the FD (55) / PD (57) siblings
//!
//! The DD module is the template: copy `dd/` to `fd/` and `pd/`, retitle the
//! provenance to `bsim3soi_fd` / `bsim3soi_pd`, and re-derive the few places
//! where the body-charge / dynamic-depletion equations differ (FD has a fixed
//! depletion, PD a different `Vbs0` floor). The node topology, stamping
//! skeleton, and `common` smoothing helpers are shared verbatim. The builder
//! `is_bsimsoi_level` seam already brackets 55..=57; route each level to its
//! device once that variant's eval + charge are complete.

pub mod common;
pub mod dd;

pub use dd::{B3SoiDd, B3SoiDdModel};
