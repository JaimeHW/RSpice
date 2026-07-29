//! BSIM3SOI device family (SPICE levels 55-57).
//!
//! Shared scaffolding for the Berkeley SOI MOSFET compact models:
//! - `dd/`: B3SOIDD, "dynamic depletion" BSIMDD2.x (MOS level 56) — ported from
//!   ngspice-46 `src/spicelib/devices/bsim3soi_dd/`.
//! - `fd/`: B3SOIFD, "fully depleted" BSIMFD2.x (MOS level 55) — ported from
//!   ngspice-46 `src/spicelib/devices/bsim3soi_fd/`. The fully-depleted body is
//!   pinned algebraically (`Vbs = Vbs0eff`), has no body circuit node, and all
//!   body currents are zero (see [`fd`]).
//! - `pd/`: B3SOIPD, "partially depleted" BSIMPD2.x (MOS level 57) — ported from
//!   ngspice-46 `src/spicelib/devices/bsim3soi_pd/`. PD keeps a real floating-
//!   body node and the full body-diode + parasitic-BJT current set, with a body
//!   resistor for the nonideal body tie (see [`pd`]).
//!
//! Each variant follows the same module layout (`params.rs` model card,
//! `temp.rs` size/temperature setup, `eval.rs` faithful load transcription,
//! `mod.rs` device + stamping glue).
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
//!   tie), with drain/source series R externalized by the builder and no temp
//!   node ([`dd::B3SoiDd`] `stamp_op`).
//!
//! Done since: the **CAPMOD=3 charge model** (b3soiddld.c:2646-3784) with its
//! transient companion stamping + charge-history/LTE integration, the back-gate
//! (E) DC coupling, the floating-body convergence aids (`B3SOIDDlimit` /
//! `B3SOIDDSmartVbs`), and the live LEVEL=56 builder dispatch. The DC sweep
//! decks pass against the ngspice references.
//!
//! Deferred (clearly bounded, see `dd::eval` module docs for line refs):
//! - **Self-heating** (SHMOD=1) and the temperature node — SHMOD=0 in every
//!   supported deck; the parameter is recognized and (per spec) errors if RTH0
//!   makes it active rather than being silently ignored.
//! - **Body resistor** (`rbody`/`rbsh` > 0, `bodyMod==1`).
//! - **Fast-edge floating-body transient amplitude** (`RampVg2`) and the
//!   **ring-oscillator DC startup** (`ring51`) need further calibration of the
//!   body LTE / continuation against ngspice.
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
pub mod fd;
pub mod pd;

pub use dd::{B3SoiDd, B3SoiDdModel, BodyMode};
pub use fd::{B3SoiFd, B3SoiFdModel};
pub use pd::{B3SoiPd, B3SoiPdModel};
